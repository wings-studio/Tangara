// This file was generated by tangara-gen
// All changes in this file will discard after rebuilding project
use tangara::context::{FnDtor, Context, Ptr, Property};

static mut TestStruct_id_getter: Option<fn(Ptr) -> Ptr> = None;
static mut TestStruct_id_setter: Option<fn(Ptr, Ptr)> = None;
static mut TestStruct_dtor: Option<FnDtor> = None;
static mut MyStruct_dtor: Option<FnDtor> = None;
static mut GenericsTest_dtor: Option<FnDtor> = None;

#[derive(Ord, PartialOrd, Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum EnumUnit {
	Variant=0,
}
pub enum EnumTuple {
}
pub enum EnumStruct {
}
pub enum EnumMixed {
}
pub enum EnumComplex {
}
pub trait MyTrait {
	fn foo(&self, a:String);
	fn bar(&self) -> String;
}
pub type BoxedStr=Box<str>;
pub struct TestStruct{
	ptr: Ptr
}
impl TestStruct {
	fn get_id(&self) -> u64 {
		unsafe {
			let raw_ptr: *mut u64 = (TestStruct_id_getter.unwrap())(self as *const TestStruct as Ptr) as *mut u64;
			if !raw_ptr.is_null() {
				*unsafe { Box::from_raw(raw_ptr) }
			} else {
				panic!("Pointer of gotten property is null")
			}
		}
	}
	fn set_id(&mut self, value: u64) {
		unsafe { (TestStruct_id_setter.unwrap())(self as *mut TestStruct as Ptr, &value as *const u64 as Ptr); }
	}
}
impl Drop for TestStruct {
	fn drop(&mut self) {
		unsafe {
			TestStruct_dtor.expect("Destructor wasn't loaded from library")(self.ptr);
		}
	}
}
pub struct MyStruct{
	ptr: Ptr
}
impl MyStruct {
}
impl Drop for MyStruct {
	fn drop(&mut self) {
		unsafe {
			MyStruct_dtor.expect("Destructor wasn't loaded from library")(self.ptr);
		}
	}
}
pub struct GenericsTest<T:MyTrait>{
	ptr: Ptr
}
impl GenericsTest<T:MyTrait> {
}
impl Drop for GenericsTest<T:MyTrait> {
	fn drop(&mut self) {
		unsafe {
			GenericsTest_dtor.expect("Destructor wasn't loaded from library")(self.ptr);
		}
	}
}

pub fn load_MyLib(ctx: &Context) {
	unsafe {
		let MyLib_package = ctx.get_package(17442259264759129316);
		let EnumTuple_type = MyLib_package.get_type(4625095818019192178);
		let EnumStruct_type = MyLib_package.get_type(14304033509570757938);
		let EnumMixed_type = MyLib_package.get_type(5027534704753303560);
		let EnumComplex_type = MyLib_package.get_type(7898830565185017404);
		let TestStruct_type = MyLib_package.get_type(662889698570043466);
		let TestStruct_id_prop = TestStruct_type.get_property(5824848936401749885);
		TestStruct_id_getter = Some(TestStruct_id_prop.getter);
		TestStruct_id_setter = Some(TestStruct_id_prop.setter.unwrap());
		TestStruct_dtor = Some(TestStruct_type.get_dtor());
		let MyStruct_type = MyLib_package.get_type(575532125808595608);
		MyStruct_dtor = Some(MyStruct_type.get_dtor());
		let GenericsTest_type = MyLib_package.get_type(7080624152791471576);
		GenericsTest_dtor = Some(GenericsTest_type.get_dtor());
	}
}
