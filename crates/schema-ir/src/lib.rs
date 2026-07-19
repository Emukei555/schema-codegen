//! schema-ir — placeholder. Implementation TBD.

pub mod parser;

#[derive(Debug, PartialEq)]
pub struct Schema {
    pub tables: Vec<Table>,
}

#[derive(Debug, PartialEq)]
pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, PartialEq)]
pub struct Field {
    pub name: String,
    pub field_type: PrimitiveType,
}

#[derive(Debug, PartialEq)]
pub enum PrimitiveType {
    Float,
    Int,
    Bool,
    String,
}
