// This file was generated by tangara-gen
// All changes in this file will discard after rebuilding project
use std::ptr;
use std::alloc::{dealloc, Layout};
use tangara::context::{Context, Ptr, Property};
use crate::*;

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
#[no_mangle]
pub extern "C" fn tgLoad(ctx: &mut Context) {
	let mut MyPackage_package = ctx.add_package(15027680195549333245);
	let mut MyStruct_type = MyPackage_package.add_type(7534407210518214439);
	MyStruct_type.set_dtor(MyStruct_dtor);
	MyStruct_type.add_ctor(MyStruct_ctor0);
	MyStruct_type.add_method(17567713076779176127, MyStruct_repeat_name);
	MyStruct_type.add_method(1641961565049420977, MyStruct_set_name);
	MyStruct_type.add_method(552281434682100053, MyStruct_get_name);
	let mut TestStruct_type = MyPackage_package.add_type(16038391058121417372);
	TestStruct_type.set_dtor(TestStruct_dtor);
	TestStruct_type.add_property(5824848936401749885, Property { getter: TestStruct_get_id, setter: Some(TestStruct_set_id) });
}
