"""ビルド/検証タスクのオーケストレーション定義。

実行例:
    nox -s lint
    nox -s test
    nox -s build_rust
"""

import nox

nox.options.reuse_existing_virtualenvs = True


@nox.session(python="3.11")
def lint(session: nox.Session) -> None:
    """schema_tools側のPythonコードをlint(内容は未実装)。"""
    session.install(".[dev]")
    session.run("ruff", "check", ".")


@nox.session(python="3.11")
def test(session: nox.Session) -> None:
    """Python側のユニットテスト(内容は未実装)。"""
    session.install(".[dev]")
    session.run("pytest")


@nox.session(python=False)
def build_rust(session: nox.Session) -> None:
    """Cargoワークスペース全体をビルド(内容は未実装)。

    将来的に `cargo build --workspace --release` をここから叩き、
    生成物をUE側アダプタが参照するパスへ配置する処理を実装する。
    """
    session.run("cargo", "build", "--workspace", "--release", external=True)


@nox.session(python=False)
def generate(session: nox.Session) -> None:
    """schema-codegen CLIを実行してコード生成(内容は未実装)。"""
    session.run(
        "cargo", "run", "-p", "schema-codegen-cli", "--", "--help", external=True
    )
