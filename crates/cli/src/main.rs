use std::env;
use std::fs;
use std::process;

use codegen_cxx::generator::generate_schema;
use schema_ir::parser::parse_fbs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.fbs> <output.h>", args[0]);
        process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    println!("Reading schema from: {}", input_path);
    let schema_content = match fs::read_to_string(input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Failed to read file '{}': {}", input_path, e);
            process::exit(1);
        }
    };

    println!("Parsing schema...");
    let schema_ir = match parse_fbs(&schema_content) {
        Ok(ir) => ir,
        Err(e) => {
            // パースエラーの場合はここで弾く
            eprintln!("Failed to parse schema:\n{:?}", e);
            process::exit(1);
        }
    };

    // 4. C++コード生成 (IR -> C++ String)
    println!("Generating C++ code...");
    let cpp_code = generate_schema(&schema_ir);

    // 5. ファイルへ書き出し
    match fs::write(output_path, &cpp_code) {
        Ok(_) => println!("Successfully wrote C++ code to: {}", output_path),
        Err(e) => {
            eprintln!("Failed to write output file '{}': {}", output_path, e);
            process::exit(1);
        }
    }
}
