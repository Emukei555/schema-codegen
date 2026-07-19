use schema_ir::Schema;

pub fn generate_cxx(schema: &Schema) -> String {
    let mut out = String::new();

    // ヘッダガード（モダンな pragma once を採用）
    out.push_str("#pragma once\n\n");

    // TODO: ここで schema.objects や schema.enums をループして
    // C++の struct や class の定義を生成していく

    out
}
