//! codegen-rust — placeholder. Implementation TBD.
use schema_ir::{PrimitiveType, Schema};

pub fn generate_rust_code(schema: &Schema) -> String {
    let mut code = String::new();

    for table in &schema.tables {
        code.push_str("#[derive(Debug, Clone, PartialEq)]\n");
        code.push_str(&format!("pub struct {} {{\n", table.name));

        for field in &table.fields {
            let rust_type = match field.field_type {
                PrimitiveType::Float => "f32",
                PrimitiveType::Int => "i32",
                PrimitiveType::Bool => "bool",
                PrimitiveType::String => "String",
            };
            code.push_str(&format!("    pub {}: {},\n", field.name, rust_type));
        }

        code.push_str("}\n\n");
    }

    code
}
