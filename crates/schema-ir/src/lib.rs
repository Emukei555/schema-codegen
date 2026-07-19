//! schema-ir — Refactored to align with FlatBuffers reflection metadata.

pub mod parser;

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    // flatcに合わせ、structとtableを同一の「Objects」として管理できるようにすると移行が楽です
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
    // flatcの「BaseType」の概念（基本型か、他構造への参照か、配列か）を模倣
    Primitive(PrimitiveType),
    Obj(String),          // Custom(String) から名前を変更（他table/struct参照）
    Vector(Box<TypeRef>), // 配列（ゲームのコンポーネントでは超高頻度）
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PrimitiveType {
    // 将来 flatc の型（Int32, Float32等）に1対1で割当て直せるようにマッピング
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
