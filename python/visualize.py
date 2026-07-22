import json
import graphviz
from pathlib import Path
from typing import Tuple, Optional

def load_ir_data(target_dir: Path) -> list[dict]:
    """targetディレクトリ内のすべての *_ir.json を読み込む"""
    ir_files = target_dir.glob("*_ir.json")
    data = []
    for file in ir_files:
        with open(file, "r", encoding="utf-8") as f:
            data.append(json.load(f))
    return data

def extract_type_info(field_type: dict) -> Tuple[str, Optional[str]]:
    """
    フィールドの文字列表現と、他のオブジェクトへの依存(エッジの宛先)を抽出する
    戻り値: (画面表示用の型名, 依存先のノード名)
    """
    if "Primitive" in field_type:
        return (field_type["Primitive"], None)

    if "Obj" in field_type:
        obj_name = field_type["Obj"]
        return (obj_name, obj_name)

    if "Vector" in field_type:
        inner_type, inner_dep = extract_type_info(field_type["Vector"])
        return (f"[{inner_type}]", inner_dep)

    return ("Unknown", None)

def build_graph(ir_data_list: list[dict]) -> graphviz.Digraph:
    """IRデータのリストからGraphvizのグラフを純粋に構築する"""
    dot = graphviz.Digraph(
        comment='Engine-Agnostic Schema IR',
        node_attr={'shape': 'record', 'fontname': 'Helvetica'}
    )

    for ir in ir_data_list:

        for enum_def in ir.get("enums", []):
            name = enum_def["name"]
            variants = [v["name"] for v in enum_def.get("variants", [])]

            label = f"{{{name} (Enum)|{'|'.join(variants)}}}"
            dot.node(name, label, style="filled", fillcolor="#e8f4f8")

        for obj_def in ir.get("objects", []):
            name = obj_def["name"]
            fields = obj_def.get("fields", [])

            field_labels = []
            dependencies = []

            for field in fields:
                field_name = field["name"]
                type_str, dep = extract_type_info(field["field_type"])

                # "+ name: type" の形式
                field_labels.append(f"+ {field_name}: {type_str}")

                if dep:
                    dependencies.append(dep)

            label = f"{{{name}|{'|'.join(field_labels)}}}"
            dot.node(name, label, style="filled", fillcolor="#f9f2e8")

            for dep in dependencies:
                dot.edge(name, dep)

    return dot

def main():
    target_dir = Path("../target")

    if not target_dir.exists():
        print(f"Error: Target directory not found at {target_dir}")
        return

    ir_data_list = load_ir_data(target_dir)

    if not ir_data_list:
        print("Error: No *_ir.json files found. Run `nox -s test` first.")
        return

    dot = build_graph(ir_data_list)

    output_path = target_dir / "schema_graph"
    dot.render(output_path, format="png", cleanup=True)
    print(f"Success! Graph generated at: {output_path}.png")

if __name__ == "__main__":
    main()
