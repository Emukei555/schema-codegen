//! schema-ir — Refactored to align with FlatBuffers reflection metadata.

pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub objects: Vec<ObjectDef>,
    pub enums: Vec<EnumDef>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectDef {
    pub name: String,
    pub fields: Vec<Field>,
    pub is_struct: bool,
    pub attributes: Vec<KeyValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyValue {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub field_type: TypeRef,
    pub attributes: Vec<KeyValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeRef {
    Primitive(PrimitiveType),
    Obj(String),
    Vector(Box<TypeRef>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimitiveType {
    Bool,
    Byte,
    Int,
    Float,
    String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDef {
    pub name: String,
    pub base_type: PrimitiveType,
    pub variants: Vec<EnumVal>,
    pub attributes: Vec<KeyValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumVal {
    pub name: String,
    pub value: i64,
}
