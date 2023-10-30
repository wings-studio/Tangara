use crate::builder::{generate_type_id, PackageBuilder, TypeBuilder};
use crate::{Constructor, Method, Property, Type, TypeRef, Visibility};
use crate::builder::constructor_builder::{ConstructorBuilder, ConstructorCollector};
use crate::builder::method_builder::{MethodBuilder, MethodCollector};
use crate::builder::property_builder::{PropertyBuilder, PropertyCollector};
use crate::TypeKind::Class;

pub struct ClassBuilder<'a> {
    builder: &'a mut PackageBuilder,
    name: String,
    vis: Visibility,
    constructors: Vec<Constructor>,
    properties: Vec<Property>,
    methods: Vec<Method>
}

impl<'a> ClassBuilder<'a> {
    pub(crate) fn new(builder: &'a mut PackageBuilder, name: &str) -> Self {
        let vis = builder.type_visibility.clone();
        Self {
            builder,
            name: name.to_string(),
            vis,
            constructors: Vec::new(),
            properties: Vec::new(),
            methods: Vec::new()
        }
    }

    pub fn set_visibility(&mut self, vis: Visibility) -> &mut Self {
        self.vis = vis;
        self
    }

    pub fn add_constructor(&'a mut self) -> ConstructorBuilder<'a, Self> {
        ConstructorBuilder::new(self, self.builder.constructor_visibility)
    }

    pub fn add_property(&'a mut self, prop_type: TypeRef, name: &str) -> PropertyBuilder<'a, Self> {
        PropertyBuilder::new(self, prop_type, name)
    }

    pub fn add_method(&'a mut self, name: &str) -> MethodBuilder<'a, Self> {
        MethodBuilder::new(self, name)
    }
}

impl<'a> TypeBuilder for ClassBuilder<'a> {
    fn get_type(&self) -> Type {
        Type {
            vis: self.vis.clone(),
            namespace: self.builder.namespace.clone(),
            name: self.name.clone(),
            id: generate_type_id(&self.name),
            attrs: vec![],
            kind: Class(
                self.constructors.to_vec(),
                self.properties.to_vec(),
                self.methods.to_vec()
            )
        }
    }

    fn build(&mut self) -> &mut PackageBuilder {
        self.builder.types.push(self.get_type());
        self.builder
    }
}

impl<'a> ConstructorCollector for ClassBuilder<'a> {
    fn get_default_visibility(&self) -> Visibility {
        self.builder.constructor_visibility
    }

    fn add_constructor(&mut self, constructor: Constructor) {
        self.constructors.push(constructor)
    }
}

impl<'a> PropertyCollector for ClassBuilder<'a> {
    fn get_default_visibility(&self) -> Visibility {
        self.builder.property_visibility
    }

    fn add_property(&mut self, property: Property) {
        self.properties.push(property)
    }
}

impl<'a> MethodCollector for ClassBuilder<'a> {
    fn get_default_visibility(&self) -> Visibility {
        self.builder.method_visibility
    }

    fn add_method(&mut self, method: Method) {
        self.methods.push(method)
    }
}