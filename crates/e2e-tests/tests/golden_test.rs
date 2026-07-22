use insta::glob;
use schema_ir::parser::parse_fbs;
use std::fs;

#[test]
fn test_ir_snapshots() {
    glob!("golden/input/*.fbs", |path| {
        let input = fs::read_to_string(path).expect("Failed to read .fbs file");
        let result = parse_fbs(&input).expect("Failed to parse .fbs file");

        insta::assert_debug_snapshot!(&result);

        let json_output =
            serde_json::to_string_pretty(&result).expect("Failed to serialize IR to JSON");

        let file_stem = path.file_stem().unwrap().to_str().unwrap();

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let output_path = Path::new(manifest_dir)
            .join("../../target")
            .join(format!("{}_ir.json", file_stem));

        fs::write(&output_path, json_output).expect("Failed to write JSON file");
    });
}
