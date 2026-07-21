use std::env;
use std::fs;
use std::path::Path;
use std::process;

use codegen_cxx::generator::generate_schema;
use codegen_rust::generate_rust_code;
use schema_ir::parser::parse_fbs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.fbs> <output_file>", args[0]);
        eprintln!("  Output file extension determines the language (.h for C++, .rs for Rust)");
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
            eprintln!("Failed to parse schema:\n{:?}", e);
            process::exit(1);
        }
    };

    let extension = Path::new(output_path)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    let generated_code = match extension {
        "h" | "hpp" => {
            println!("Generating C++ code...");
            generate_schema(&schema_ir)
        }
        "rs" => {
            println!("Generating Rust code...");
            generate_rust_code(&schema_ir)
        }
        _ => {
            eprintln!(
                "Unsupported output extension: '{}'. Use .h for C++ or .rs for Rust.",
                extension
            );
            process::exit(1);
        }
    };

    match fs::write(output_path, &generated_code) {
        Ok(_) => println!("Successfully wrote generated code to: {}", output_path),
        Err(e) => {
            eprintln!("Failed to write output file '{}': {}", output_path, e);
            process::exit(1);
        }
    }
}
