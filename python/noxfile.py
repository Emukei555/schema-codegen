import nox
import os

PROJECT_ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))

@nox.session(python=False)
def test(session):
    """Run all Rust tests (unit, E2E, and Golden Tests)"""
    with session.chdir(PROJECT_ROOT):
        session.run("cargo", "test", "--workspace", external=True)

@nox.session(python=False)
def update_golden(session):
    """Golden Testのスナップショットを再生成（更新）します"""
    with session.chdir(PROJECT_ROOT):
        session.run(
            "cargo", "test", "--workspace",
            env={"UPDATE_EXPECT": "1"},
            external=True
        )

@nox.session(python=False)
def build(session):
    """リリースビルドを実行します"""
    with session.chdir(PROJECT_ROOT):
        session.run("cargo", "build", "--release", external=True)

@nox.session(python=False)
def check(session):
    """Linter(clippy)とフォーマッター(fmt)のチェックを実行します"""
    with session.chdir(PROJECT_ROOT):
        session.run("cargo", "fmt", "--all", "--", "--check", external=True)
        session.run("cargo", "clippy", "--workspace", "--", "-D", "warnings", external=True)

@nox.session
def visualize(session):
    """IRのJSONを読み込み、アーキテクチャ図を生成する"""
    session.install("graphviz")
    session.run("python", "visualize.py")
