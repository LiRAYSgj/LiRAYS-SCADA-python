from __future__ import annotations

import pathlib
import py_compile


ROOT = pathlib.Path(__file__).resolve().parents[1]


def test_typing_files_exist() -> None:
    assert (ROOT / "python" / "lirays" / "__init__.pyi").exists()
    assert (ROOT / "python" / "lirays" / "py.typed").exists()


def test_demo_python_files_compile() -> None:
    for path in (ROOT / "demo").glob("*.py"):
        py_compile.compile(str(path), doraise=True)
