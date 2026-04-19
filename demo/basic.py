from __future__ import annotations

import time

from lirays import (
    BooleanVar,
    Client,
    FloatVar,
    IntegerVar,
    TextVar,
    VariableMetadataPatch,
)


def run(client: Client) -> None:
    root = f"demo_{int(time.time() * 1000)}"
    root_path = f"/{root}"

    client.create_folders([root], timeout_ms=8000)
    client.create_folders(["assets"], parent_id=root_path, timeout_ms=8000)

    client.create_integer_variables(
        [IntegerVar(name="i1", unit="rpm", min=0.0, max=5000.0)],
        parent_id=root_path,
        timeout_ms=8000,
    )

    client.create_float_variables(
        [FloatVar(name="f1", unit="bar", min=0.0, max=30.0)],
        parent_id=root_path,
        timeout_ms=8000,
    )

    client.create_text_variables(
        [TextVar(name="t1", options=["AUTO", "MAN"], max_len=8)],
        parent_id=root_path,
        timeout_ms=8000,
    )

    client.create_boolean_variables(
        [BooleanVar(name="b1")],
        parent_id=root_path,
        timeout_ms=8000,
    )

    _, vars_found = client.list(folder_id=root_path, timeout_ms=8000)
    print(f"[basic] vars created under {root_path}: {len(vars_found)}")

    client.set_integer_variables([f"{root_path}/i1"], [1200], timeout_ms=8000)
    client.set_float_variables([f"{root_path}/f1"], [7.25], timeout_ms=8000)
    client.set_text_variables([f"{root_path}/t1"], ["AUTO"], timeout_ms=8000)
    client.set_boolean_variables([f"{root_path}/b1"], [True], timeout_ms=8000)

    values = client.get_values(
        [
            f"{root_path}/i1",
            f"{root_path}/f1",
            f"{root_path}/t1",
            f"{root_path}/b1",
        ],
        timeout_ms=8000,
    )
    print(f"[basic] current values: {values}")

    client.edit_variable_metadata(
        f"{root_path}/f1",
        VariableMetadataPatch(unit="psi", min=1.0, max=20.0),
        timeout_ms=8000,
    )

    client.edit_variable_metadata(
        f"{root_path}/t1",
        VariableMetadataPatch(options=["AUTO", "MAN", "OFF"], max_len=12),
        timeout_ms=8000,
    )

    client.delete_items([f"{root_path}/assets"], timeout_ms=8000)
    print("[basic] completed all core CRUD/set/get/meta operations")
