use crate::builder::generate_method_id;
use crate::{Argument, ArgumentKind, Attribute, Method, TypeRef, Value, Visibility};

pub trait MethodCollector {
    fn get_default_visibility(&self) -> Visibility;
    fn add_method(&mut self, method: Method);
}

pub struct MethodBuilder<'a, T: MethodCollector> {
    builder: &'a mut T,
    attrs: Vec<Attribute>,
    vis: Visibility,
    name: String,
    arg_attrs: Vec<Attribute>,
    args: Vec<Argument>,
    return_type: Option<TypeRef>
}

impl<'a, T: MethodCollector> MethodBuilder<'a, T> {
    pub(crate) fn new(builder: &'a mut T, name: &str) -> Self {
        let vis = builder.get_default_visibility();
        Self {
            builder,
            attrs: vec![],
            vis,
            name: name.to_string(),
            arg_attrs: Vec::new(),
            args: Vec::new(),
            return_type: None
        }
    }

    pub fn add_attribute(&mut self, attr: Attribute) -> &mut Self {
        self.attrs.push(attr);
        self
    }

    pub fn set_visibility(&mut self, vis: Visibility) -> &mut Self {
        self.vis = vis;
        self
    }

    /// Set return type. If you don't (by not calling this method) it will be `void` - nothing.
    /// But if you will, you cannot return it back to nothing.
    pub fn return_type(&mut self, return_type: TypeRef) -> &mut Self {
        self.return_type = Some(return_type);
        self
    }

    /// Push attribute before next argument
    pub fn arg_attribute(&mut self, attribute: Attribute) -> &mut Self {
        self.arg_attrs.push(attribute);
        self
    }

    #[inline]
    fn add_argument(&mut self, arg_type: TypeRef, name: &str, kind: ArgumentKind) -> &mut Self {
        self.args.push(Argument(
            self.arg_attrs.to_vec(),
            arg_type,
            name.to_string(),
            kind
        ));
        self.arg_attrs.clear();
        self
    }

    /// Creates common argument with given type and name
    pub fn arg(&mut self, arg_type: TypeRef, name: &str) -> &mut Self {
        self.add_argument(arg_type, name, ArgumentKind::Default)
    }

    /// Creates common argument with given type, name and default value
    pub fn arg_value(&mut self, arg_type: TypeRef, name: &str, default_value: Value) -> &mut Self {
        self.add_argument(arg_type, name, ArgumentKind::DefaultValue(default_value))
    }

    /// Creates output argument with given type and name
    pub fn arg_out(&mut self, arg_type: TypeRef, name: &str) -> &mut Self {
        self.add_argument(arg_type, name, ArgumentKind::Out)
    }

    /// Creates reference argument with given type and name
    pub fn arg_ref(&mut self, arg_type: TypeRef, name: &str) -> &mut Self {
        self.add_argument(arg_type, name, ArgumentKind::Ref)
    }

    pub fn get_method(&self) -> Method {
        Method {
            attrs: self.attrs.to_vec(),
            vis: self.vis,
            name: self.name.clone(),
            id: generate_method_id(&self.name, &self.args),
            args: self.args.to_vec(),
            return_type: self.return_type.clone(),
        }
    }
    
    pub fn build(&'a mut self) -> &'a mut T {
        self.builder.add_method(self.get_method());
        self.builder
    }
}