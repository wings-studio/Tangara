use crate::{Package, Type, Visibility};
use xxhash_rust::const_xxh3::const_custom_default_secret;
use xxhash_rust::xxh3::xxh3_64_with_secret;
use crate::builder::class_builder::ClassBuilder;
use crate::builder::enum_builder::EnumBuilder;

pub mod enum_builder;
pub mod class_builder;

const PACKAGE_SECRET: [u8; 192] = const_custom_default_secret(772);
const TYPE_SECRET: [u8; 192] = const_custom_default_secret(4900);
const FUNC_SECRET: [u8; 192] = const_custom_default_secret(18257);

/// Generate XXHash id for type with given name
pub(crate) fn generate_typeid(name: &String) -> u64 {
    xxh3_64_with_secret(name.as_bytes(), &TYPE_SECRET)
}

/// Generate XXHash id for function with given name
pub(crate) fn generate_fnid(name: &String) -> u64 {
    xxh3_64_with_secret(name.as_bytes(), &FUNC_SECRET)
}

pub trait TypeBuilder {
    fn get_type(&self) -> Type;
    fn build(&mut self) -> &mut PackageBuilder;
}

pub struct PackageBuilder {
    name: String,
    namespace: String,
    type_visibility: Visibility,
    constructor_visibility: Visibility,
    types: Vec<Type>
}

impl PackageBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: name.to_string(),
            type_visibility: Visibility::Public,
            constructor_visibility: Visibility::Public,
            types: Vec::new()
        }
    }

    pub fn set_namespace(&mut self, namespace: &str) -> &mut Self {
        self.namespace = namespace.to_string();
        self
    }

    pub fn create_class(&mut self, name: &str) -> ClassBuilder {
        ClassBuilder::new(self, name)
    }

    pub fn create_enum(&mut self, name: &str) -> EnumBuilder {
        EnumBuilder::new(self, name)
    }

    pub fn build(&self) -> Package {
        Package {
            name: self.name.clone(),
            id: xxh3_64_with_secret(self.name.as_bytes(), &PACKAGE_SECRET),
            types: self.types.to_vec(),
        }
    }
}