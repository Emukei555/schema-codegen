use insta::glob;
use schema_ir::parser::parse_fbs;
use std::fs;

#[test]
fn test_ir_snapshots() {
    // e2e-tests/ 配下を基準として glob でファイルを探索
    glob!("golden/input/*.fbs", |path| {
        let input = fs::read_to_string(path).expect("Failed to read .fbs file");
        let result = parse_fbs(&input).expect("Failed to parse .fbs file");

        insta::assert_debug_snapshot!(result);
    });
}
