//! schema-ir — Refactored to align with FlatBuffers reflection metadata.

use serde::Serialize;

pub mod parser;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Schema {
    pub objects: Vec<ObjectDef>,
    pub enums: Vec<EnumDef>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ObjectDef {
    pub name: String,
    pub fields: Vec<Field>,
    pub is_struct: bool,
    pub attributes: Vec<KeyValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct KeyValue {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Field {
    pub name: String,
    pub field_type: TypeRef,
    pub attributes: Vec<KeyValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum TypeRef {
    Primitive(PrimitiveType),
    Obj(String),
    Vector(Box<TypeRef>),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum PrimitiveType {
    Bool,
    Byte,
    Int,
    Float,
    String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EnumDef {
    pub name: String,
    pub base_type: PrimitiveType,
    pub variants: Vec<EnumVal>,
    pub attributes: Vec<KeyValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EnumVal {
    pub name: String,
    pub value: i64,
}
