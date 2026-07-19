use schema_ir::Schema;

pub fn generate_cxx(_schema: &Schema) -> String {
    let mut out = String::new();

    // ヘッダガード
    out.push_str("#pragma once\n\n");

    out
}
