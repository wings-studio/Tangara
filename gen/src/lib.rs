use std::ops::Deref;
use std::path::Path;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{FnArg, ImplItem, Item, parse_file, Pat, PatType, ReturnType, Signature, Type, UseTree, Visibility};
use syn::parse::Parse;
use xxhash_rust::const_xxh3::const_custom_default_secret;
use xxhash_rust::xxh3::xxh3_64_with_secret;

const PACKAGE_SECRET: [u8; 192] = const_custom_default_secret(0x0304);
const TYPE_SECRET: [u8; 192] = const_custom_default_secret(0x1324);
const FUNC_SECRET: [u8; 192] = const_custom_default_secret(0x4751);

pub struct Generator {
    generate_internal: bool,
    package_ident: Ident,
    use_tokens: TokenStream,
    tgload_tokens: TokenStream,
    bindings_tokens: TokenStream
}

impl Generator {
    pub fn new(package_name: &str) -> Self {
        let mut use_tokens = TokenStream::new();
        let mut tgload_tokens = TokenStream::new();
        let mut bindings_tokens = TokenStream::new();

        let disclaimer = quote! {
            /// This file was generated by tangara-gen
            /// All changes in this file will discard after rebuilding project
            use std::ptr;
            use std::alloc::{dealloc, Layout};
            use tangara::context::{Context, Ptr};
        };
        use_tokens.extend(disclaimer);

        let package_ident = Ident::new(format!("{}_package", package_name).as_str(), Span::call_site());
        let package_id = xxh3_64_with_secret(package_name.as_bytes(), &PACKAGE_SECRET);
        let package_definition = quote! {
            let mut #package_ident = ctx.add_package(#package_id);
        };
        tgload_tokens.extend(package_definition);

        Self {
            generate_internal: false,
            package_ident,
            use_tokens,
            tgload_tokens,
            bindings_tokens
        }
    }

    fn gen_dtor(&mut self, struct_name: &Ident, struct_type: &Ident) {
        let dtor_ident = Ident::new(format!("{}_dtor", struct_name).as_str(), Span::call_site());
        self.bindings_tokens.extend(quote! {
            extern "C" fn #dtor_ident(object: Ptr) {
                unsafe {
                    ptr::drop_in_place(object);
                    dealloc(object, Layout::new::<#struct_name>());
                }
            }
        });
        self.tgload_tokens.extend(quote! {
            #struct_type.set_dtor(#dtor_ident);
        })
    }

    fn add_struct(&mut self, struct_name: &Ident, struct_ident: &Ident) {
        let struct_id = xxh3_64_with_secret(
            struct_name.to_string().as_bytes(), &TYPE_SECRET
        );
        let package_ident = &self.package_ident;
        self.tgload_tokens.extend(quote! {
            let mut #struct_ident = #package_ident.add_type(#struct_id);
        });
    }

    fn parse_pat_arg(pat_arg: &PatType, fn_body: &mut TokenStream) -> Ident {
        match pat_arg.pat.deref() {
            Pat::Ident(arg_ident) => {
                /*match *pat_arg.ty {
                    Type::Array(array_type) => {}
                    //Type::BareFn(_) => {}
                    //Type::ImplTrait(_) => {}
                    Type::Path(path_type) => {}
                    Type::Ptr(ptr_type) => {}
                    Type::Reference(ref_type) => {}
                    Type::Slice(slice_type) => {}
                    //Type::TraitObject(_) => {}
                    Type::Tuple(tuple_type) => {}
                    _ => {
                        unimplemented!("Unparseble type of argument")
                    }
                }*/
                let arg_type = pat_arg.ty.to_token_stream();
                let arg_type_ptr = if arg_ident.mutability.is_none() {
                    quote!(*const #arg_type)
                } else {
                    quote!(*mut #arg_type)
                };
                fn_body.extend(quote! {
                                let #arg_ident: #arg_type = *(args_ptr.add(prev_arg_size) as #arg_type_ptr);
                                prev_arg_size = std::mem::size_of::<#arg_type>();
                            });
                arg_ident.clone().ident
            }
            _ => unimplemented!("Strange pattern of argument")
        }
    }

    fn gen_ctor(&mut self, ctor_index: i32, fn_sig: &Signature, struct_ident: &Ident, struct_name: &Ident) {
        let ctor_ident = Ident::new(
            format!("{}_ctor{}", struct_name.to_string(), ctor_index).as_str(), Span::call_site()
        );
        self.tgload_tokens.extend(quote! {
           #struct_ident.add_ctor(#ctor_ident);
        });
        let mut ctor_body = if fn_sig.inputs.len() > 0 {
            quote! {
                let args_slice = std::slice::from_raw_parts_mut(args, args_size);
                let args_ptr = args_slice.as_mut_ptr();
                let mut prev_arg_size = 0usize;
            }
        } else {
            TokenStream::new()
        };

        // Parse args
        let mut args_tokens = TokenStream::new();
        for arg_fn in &fn_sig.inputs {
            match arg_fn {
                FnArg::Receiver(_) => {
                    panic!("self argument cannot be in constructor");
                }
                FnArg::Typed(arg) => {
                    let arg_ident = Self::parse_pat_arg(arg, &mut ctor_body);
                    args_tokens.extend(quote!(#arg_ident,));
                }
            }
        }

        // Check for correct return type
        match &fn_sig.output {
            ReturnType::Default => {
                panic!("Constructor cannot return nothing");
            }
            ReturnType::Type(_, return_type_boxed) => {
                let return_type = return_type_boxed.to_token_stream().to_string();
                if return_type != "Self" &&
                    return_type != struct_name.to_string() {
                    panic!("Return type of constructor cannot be not the type of impl: {} != {}",
                           return_type, struct_name.to_string());
                }
                ctor_body.extend(quote! {
                    let to_return = Box::new(#struct_name::new(#args_tokens));
                    Box::into_raw(to_return) as Ptr
                });
            }
        }

        self.bindings_tokens.extend(quote! {
            extern "C" fn #ctor_ident(args_size: usize, args: *mut u8) -> Ptr {
                unsafe {
                    #ctor_body
                }
            }
        });
    }

    fn gen_method(&mut self, fn_sig: &Signature, struct_ident: &Ident, struct_name: &Ident) {
        // Get Ident for Rust function
        let fn_name_ident = &fn_sig.ident;
        // Get String name of generated extern "C" binding for this function
        let fn_name = format!("{}_{}", struct_name.to_string(), fn_name_ident);
        let fn_id = xxh3_64_with_secret(fn_name.as_bytes(), &FUNC_SECRET);
        // Ident for `fn_name`
        let fn_ident = Ident::new(fn_name.as_str(), Span::call_site());
        self.tgload_tokens.extend(quote! {
           #struct_ident.add_method(#fn_id, #fn_ident);
        });

        let mut fn_body = if fn_sig.inputs.len() > 0 {
            quote! {
                let args_slice = std::slice::from_raw_parts_mut(args, args_size);
                let args_ptr = args_slice.as_mut_ptr();
                let mut prev_arg_size = 0usize;
            }
        } else {
            TokenStream::new()
        };

        // Parse args
        let mut use_this = false;
        let mut args_tokens = TokenStream::new();
        for arg_fn in &fn_sig.inputs {
            match arg_fn {
                FnArg::Receiver(self_arg) => {
                    use_this = true;
                    let this_type = if self_arg.mutability.is_none() {
                        quote!(*const #struct_name)
                    } else {
                        quote!(*mut #struct_name)
                    };
                    // TODO: add support for self: T types from https://doc.rust-lang.org/stable/reference/items/associated-items.html#methods
                    fn_body.extend(quote! {
                        let this: #this_type = *(args_ptr as *mut Ptr) as #this_type;
                        prev_arg_size = std::mem::size_of::<#this_type>();
                    });
                }
                FnArg::Typed(arg) => {
                    let arg_ident = Self::parse_pat_arg(arg, &mut fn_body);
                    args_tokens.extend(quote!(#arg_ident,));
                }
            }
        }

        // Parse return type
        match &fn_sig.output {
            ReturnType::Default => {
                fn_body.extend(quote!(ptr::null_mut()));
            }
            ReturnType::Type(_, return_type_boxed) => {
                let return_type = return_type_boxed.deref();
                let mut was_returned = false;
                if let Type::Tuple(tuple_type) = return_type {
                    // function returns () - nothing
                    if tuple_type.elems.len() == 0 {
                        fn_body.extend(quote!(ptr::null_mut()));
                        was_returned = true;
                    }
                }
                if !was_returned {
                    let to_return = if use_this {
                        quote!((*this).#fn_name_ident(#args_tokens))
                    }
                    else {
                        quote!(#struct_name::#fn_name_ident(#args_tokens))
                    };
                    fn_body.extend(quote! {
                        let to_return = Box::new(#to_return);
                        Box::into_raw(to_return) as Ptr
                    });
                }
            }
        }

        self.bindings_tokens.extend(quote! {
            extern "C" fn #fn_ident(args_size: usize, args: *mut u8) -> Ptr {
                unsafe {
                    #fn_body
                }
            }
        });
    }

    pub fn parse_code(mut self, code: &str) -> Self {
        let syntax_tree = parse_file(code).expect("Failed to parse Rust code");

        for item in syntax_tree.items {
            match item {
                Item::Impl(impl_item) => {
                    // Ident: raw name of struct using in this `impl`
                    let name_ident = match *impl_item.self_ty {
                        Type::Path(ref t) => {
                            let tp = &t.path.segments.last().unwrap().ident;
                            Ident::new(tp.to_string().as_str(), Span::call_site())
                        }
                        _ => {
                            panic!("Type in `impl` has strange syntax")
                        }
                    };
                    // Ident for using inside `tgLoad`
                    let struct_ident = Ident::new(format!("{}_type", name_ident).as_str(), Span::call_site());
                    self.add_struct(&name_ident, &struct_ident);
                    self.gen_dtor(&name_ident, &struct_ident);

                    let mut ctor_count = 0;
                    // Generate bindings for functions
                    for inner_impl_item in impl_item.items {
                        // From `impl` block we need only functions
                        if let ImplItem::Fn(ref impl_item_fn) = inner_impl_item {

                            // Parse only pub and pub(crate) functions
                            match &impl_item_fn.vis {
                                // just pub
                                Visibility::Public(_) => {
                                    let fn_sig = &impl_item_fn.sig;

                                    // Check is it constructor
                                    if fn_sig.ident.to_string() == "new" {
                                        // it's constructor
                                        self.gen_ctor(ctor_count, fn_sig, &struct_ident, &name_ident);
                                        ctor_count += 1;
                                    } else {
                                        // it's function
                                        self.gen_method(fn_sig, &struct_ident, &name_ident);
                                    }
                                }
                                // pub(super), pub(crate), pub(in some::shit)
                                Visibility::Restricted(restricted_vis) => {
                                    if self.generate_internal &&
                                        restricted_vis.path.to_token_stream().to_string() == "crate" {
                                        let fn_sig = &impl_item_fn.sig;

                                        // Check is it constructor
                                        if fn_sig.ident.to_string() == "new" {
                                            // it's constructor
                                            self.gen_ctor(ctor_count, fn_sig, &struct_ident, &name_ident);
                                            ctor_count += 1;
                                        } else {
                                            // it's function
                                            self.gen_method(fn_sig, &struct_ident, &name_ident);
                                        }
                                    }
                                }
                                // private
                                Visibility::Inherited => {}
                            }
                        }
                    }
                }
                _ => {
                    println!("why I need this");
                }
            }
        }
        self
    }

    pub fn parse_file<P: AsRef<Path>>(mut self, path: P) -> Self {
        let rust_code = std::fs::read_to_string(path).expect("Failed to read file");
        self.parse_code(&rust_code)
    }

    pub fn custom_uses(mut self, uses: Vec<&str>) -> Self {
        for use_mod in uses {
            let use_tree: UseTree  = syn::parse_str(use_mod).unwrap();
            self.use_tokens.extend(quote!(use #use_tree;));
        }
        self
    }

    /// Generates bindings for pub(crate) functions
    /// By default: `false`
    pub fn generate_internal(mut self, generate_internal: bool) -> Self {
        self.generate_internal = generate_internal;
        self
    }

    pub fn generate(self) -> TokenStream {
        let mut result = TokenStream::new();
        result.extend(self.use_tokens);
        result.extend(self.bindings_tokens);
        let tgload_tokens = self.tgload_tokens;
        result.extend(quote! {
           #[no_mangle]
            pub extern "C" fn tgLoad(ctx: &mut Context) {
                #tgload_tokens
            }
        });
        result
    }

    pub fn generate_to_file<P: AsRef<Path>>(self, path: P) -> std::io::Result<()> {
        std::fs::write(path, self.generate().to_string())
    }
}