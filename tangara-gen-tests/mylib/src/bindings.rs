// This file was generated by tangara-gen
// All changes in this file will discard after rebuilding project
use std::ptr;
use std::alloc::{dealloc, Layout};
use tangara::context::{Context, Ptr, Property};
use crate::*;

pub extern "C" fn EnumTuple_dtor(value: Ptr) {
    unsafe {
        ptr::drop_in_place(value);
        dealloc(value, Layout::new::<EnumTuple>());
    }
}

pub extern "C" fn EnumTuple_Variant(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let args_slice = std::slice::from_raw_parts_mut(args, args_size);
        let mut args_ptr = args_slice.as_mut_ptr();
        let field0: i32 = ptr::read(args_ptr as *const i32);
        args_ptr = args_ptr.add(std::mem::size_of::<i32>());
        let return_value = Box::new(EnumTuple::Variant(field0));
		Box::into_raw(return_value) as Ptr
    }
}

pub extern "C" fn EnumStruct_dtor(value: Ptr) {
    unsafe {
        ptr::drop_in_place(value);
        dealloc(value, Layout::new::<EnumStruct>());
    }
}

pub extern "C" fn EnumStruct_Variant(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let args_slice = std::slice::from_raw_parts_mut(args, args_size);
        let mut args_ptr = args_slice.as_mut_ptr();
        let a: i32 = ptr::read(args_ptr as *const i32);
        args_ptr = args_ptr.add(std::mem::size_of::<i32>());
        let return_value = Box::new(EnumStruct::Variant { a });
		Box::into_raw(return_value) as Ptr
    }
}

pub extern "C" fn EnumMixed_dtor(value: Ptr) {
    unsafe {
        ptr::drop_in_place(value);
        dealloc(value, Layout::new::<EnumMixed>());
    }
}

pub extern "C" fn EnumMixed_Unit(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let return_value = Box::new(EnumMixed::Unit);
		Box::into_raw(return_value) as Ptr
    }
}

pub extern "C" fn EnumMixed_Tuple(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let args_slice = std::slice::from_raw_parts_mut(args, args_size);
        let mut args_ptr = args_slice.as_mut_ptr();
        let field0: i32 = ptr::read(args_ptr as *const i32);
        args_ptr = args_ptr.add(std::mem::size_of::<i32>());
        let return_value = Box::new(EnumMixed::Tuple(field0));
		Box::into_raw(return_value) as Ptr
    }
}

pub extern "C" fn EnumComplex_dtor(value: Ptr) {
    unsafe {
        ptr::drop_in_place(value);
        dealloc(value, Layout::new::<EnumComplex>());
    }
}

pub extern "C" fn EnumComplex_Unit(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let return_value = Box::new(EnumComplex::Unit);
		Box::into_raw(return_value) as Ptr
    }
}

pub extern "C" fn EnumComplex_Tuple(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let args_slice = std::slice::from_raw_parts_mut(args, args_size);
        let mut args_ptr = args_slice.as_mut_ptr();
        let field0: i32 = ptr::read(args_ptr as *const i32);
        args_ptr = args_ptr.add(std::mem::size_of::<i32>());
        let return_value = Box::new(EnumComplex::Tuple(field0));
		Box::into_raw(return_value) as Ptr
    }
}

pub extern "C" fn EnumComplex_Struct(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let args_slice = std::slice::from_raw_parts_mut(args, args_size);
        let mut args_ptr = args_slice.as_mut_ptr();
        let a: i32 = ptr::read(args_ptr as *const i32);
        args_ptr = args_ptr.add(std::mem::size_of::<i32>());
        let return_value = Box::new(EnumComplex::Struct { a });
		Box::into_raw(return_value) as Ptr
    }
}

pub extern "C" fn TestStruct_dtor(value: Ptr) {
    unsafe {
        ptr::drop_in_place(value);
        dealloc(value, Layout::new::<TestStruct>());
    }
}

pub extern "C" fn TestStruct_get_id(this: Ptr) -> Ptr {
    unsafe {
        let this: *const TestStruct = this as *const TestStruct;
        let to_return = Box::new((*this).id);
        Box::into_raw(to_return) as Ptr
    }
}

pub extern "C" fn TestStruct_set_id(this: Ptr, object: Ptr) {
    unsafe {
        let this: *mut TestStruct = this as *mut TestStruct;
        let id: u64 = ptr::read(object as *const u64);
        (*this).id = id;
    }
}

pub extern "C" fn MyStruct_dtor(value: Ptr) {
    unsafe {
        ptr::drop_in_place(value);
        dealloc(value, Layout::new::<MyStruct>());
    }
}

pub extern "C" fn MyStruct_ctor0(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let args_slice = std::slice::from_raw_parts_mut(args, args_size);
        let mut args_ptr = args_slice.as_mut_ptr();
        let name: &str = ptr::read(args_ptr as *const &str);
        args_ptr = args_ptr.add(std::mem::size_of::<&str>());
        let value = Box::new(MyStruct::new(name));
        Box::into_raw(value) as Ptr
    }
}

pub extern "C" fn MyStruct_repeat_name(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let args_slice = std::slice::from_raw_parts_mut(args, args_size);
        let mut args_ptr = args_slice.as_mut_ptr();
        let this: *const MyStruct = *(args_ptr as *mut Ptr) as *const MyStruct;
        args_ptr = args_ptr.add(std::mem::size_of::<*const MyStruct>());
        let times: u32 = ptr::read(args_ptr as *const u32);
        args_ptr = args_ptr.add(std::mem::size_of::<u32>());
        let to_return = Box::new((*this).repeat_name(times));
		Box::into_raw(to_return) as Ptr
    }
}

pub extern "C" fn MyStruct_set_name(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let args_slice = std::slice::from_raw_parts_mut(args, args_size);
        let mut args_ptr = args_slice.as_mut_ptr();
        let this: *mut MyStruct = *(args_ptr as *mut Ptr) as *mut MyStruct;
        args_ptr = args_ptr.add(std::mem::size_of::<*mut MyStruct>());
        let name: &str = ptr::read(args_ptr as *const &str);
        args_ptr = args_ptr.add(std::mem::size_of::<&str>());
        (*this).set_name(name);
		ptr::null_mut()
    }
}

pub extern "C" fn MyStruct_get_name(args_size: usize, args: *mut u8) -> Ptr {
    unsafe {
        let args_slice = std::slice::from_raw_parts_mut(args, args_size);
        let mut args_ptr = args_slice.as_mut_ptr();
        let this: *const MyStruct = *(args_ptr as *mut Ptr) as *const MyStruct;
        args_ptr = args_ptr.add(std::mem::size_of::<*const MyStruct>());
        let to_return = Box::new((*this).get_name());
		Box::into_raw(to_return) as Ptr
    }
}
#[no_mangle]
pub extern "C" fn tgLoad(ctx: &mut Context) {
	let mut MyLib_package = ctx.add_package(17442259264759129316);
	let mut EnumTuple_type = MyLib_package.add_type(4625095818019192178);
	EnumTuple_type.set_dtor(EnumTuple_dtor);
	EnumTuple_type.add_method(837492378273562681, EnumTuple_Variant);
	let mut EnumStruct_type = MyLib_package.add_type(14304033509570757938);
	EnumStruct_type.set_dtor(EnumStruct_dtor);
	EnumStruct_type.add_method(837492378273562681, EnumStruct_Variant);
	let mut EnumMixed_type = MyLib_package.add_type(5027534704753303560);
	EnumMixed_type.set_dtor(EnumMixed_dtor);
	EnumMixed_type.add_method(9260626685794967516, EnumMixed_Unit);
	EnumMixed_type.add_method(8975276260061643599, EnumMixed_Tuple);
	let mut EnumComplex_type = MyLib_package.add_type(7898830565185017404);
	EnumComplex_type.set_dtor(EnumComplex_dtor);
	EnumComplex_type.add_method(9260626685794967516, EnumComplex_Unit);
	EnumComplex_type.add_method(8975276260061643599, EnumComplex_Tuple);
	EnumComplex_type.add_method(12225383099421259715, EnumComplex_Struct);
	let mut TestStruct_type = MyLib_package.add_type(662889698570043466);
	TestStruct_type.set_dtor(TestStruct_dtor);
	TestStruct_type.add_property(5824848936401749885, Property { getter: TestStruct_get_id, setter: Some(TestStruct_set_id) });
	let mut MyStruct_type = MyLib_package.add_type(575532125808595608);
	MyStruct_type.set_dtor(MyStruct_dtor);
	MyStruct_type.add_ctor(MyStruct_ctor0);
	MyStruct_type.add_method(17567713076779176127, MyStruct_repeat_name);
	MyStruct_type.add_method(1641961565049420977, MyStruct_set_name);
	MyStruct_type.add_method(552281434682100053, MyStruct_get_name);
}
