from __future__ import annotations

import os
import random
import time
from concurrent.futures import ThreadPoolExecutor, as_completed

from lirays import BooleanVar, FloatVar, IntegerVar, TextVar

from common import connect_client


NUM_CLIENTS = int(os.getenv("LIRAYS_TREE_NUM_CLIENTS", "100"))
MAX_BATCH = int(os.getenv("LIRAYS_TREE_MAX_BATCH", "100"))
RUN_SECS = int(os.getenv("LIRAYS_TREE_RUN_SECS", "60"))


def run(host: str, port: int, tls: bool, allow_insecure_tls: bool, pat_token: str | None) -> None:
    with ThreadPoolExecutor(max_workers=NUM_CLIENTS) as executor:
        futures = [
            executor.submit(_worker, idx, host, port, tls, allow_insecure_tls, pat_token)
            for idx in range(NUM_CLIENTS)
        ]
        for fut in as_completed(futures):
            exc = fut.exception()
            if exc is not None:
                print(f"[tree_stress] worker error: {exc}")


def _worker(
    idx: int,
    host: str,
    port: int,
    tls: bool,
    allow_insecure_tls: bool,
    pat_token: str | None,
) -> None:
    client = connect_client(host, port, tls, allow_insecure_tls, pat_token)
    root_name = f"Root{idx}"
    root_path = f"/{root_name}"

    try:
        try:
            client.create_folders([root_name], timeout_ms=5000)
        except Exception:
            pass

        create_phase = True
        last_created_folders: list[str] = []
        last_created_vars: list[str] = []
        deadline = time.monotonic() + RUN_SECS

        while time.monotonic() < deadline:
            time.sleep(random.uniform(1.0, 5.0))

            if create_phase:
                folders_n = random.randint(1, MAX_BATCH)
                vars_n = random.randint(1, MAX_BATCH)

                folders = [f"f{random.getrandbits(32) ^ i}" for i in range(folders_n)]
                try:
                    client.create_folders(folders, parent_id=root_path, timeout_ms=5000)
                except Exception:
                    pass
                last_created_folders = [f"{root_path}/{name}" for name in folders]

                int_vars: list[IntegerVar] = []
                float_vars: list[FloatVar] = []
                text_vars: list[TextVar] = []
                bool_vars: list[BooleanVar] = []

                for i in range(vars_n):
                    suffix = random.getrandbits(32) ^ i
                    kind = i % 4
                    if kind == 0:
                        int_vars.append(IntegerVar(name=f"vi_{suffix}"))
                    elif kind == 1:
                        float_vars.append(FloatVar(name=f"vf_{suffix}"))
                    elif kind == 2:
                        text_vars.append(
                            TextVar(name=f"vt_{suffix}", options=["A", "B", "C"], max_len=8)
                        )
                    else:
                        bool_vars.append(BooleanVar(name=f"vb_{suffix}"))

                last_created_vars = [
                    *[f"{root_path}/{v.name}" for v in int_vars],
                    *[f"{root_path}/{v.name}" for v in float_vars],
                    *[f"{root_path}/{v.name}" for v in text_vars],
                    *[f"{root_path}/{v.name}" for v in bool_vars],
                ]

                try:
                    if int_vars:
                        client.create_integer_variables(int_vars, parent_id=root_path, timeout_ms=5000)
                    if float_vars:
                        client.create_float_variables(float_vars, parent_id=root_path, timeout_ms=5000)
                    if text_vars:
                        client.create_text_variables(text_vars, parent_id=root_path, timeout_ms=5000)
                    if bool_vars:
                        client.create_boolean_variables(bool_vars, parent_id=root_path, timeout_ms=5000)
                except Exception:
                    pass

                try:
                    client.list(folder_id=root_path, timeout_ms=5000)
                    if last_created_folders:
                        client.list(folder_id=last_created_folders[0], timeout_ms=5000)
                except Exception:
                    pass
            else:
                if len(last_created_vars) > 1:
                    to_delete = last_created_vars[1:]
                    try:
                        client.delete_items(to_delete, timeout_ms=5000)
                    except Exception:
                        pass
                    last_created_vars = [last_created_vars[0]]

                if len(last_created_folders) > 1:
                    to_delete = last_created_folders[1:]
                    try:
                        client.delete_items(to_delete, timeout_ms=5000)
                    except Exception:
                        pass
                    last_created_folders = [last_created_folders[0]]

                try:
                    client.list(folder_id=root_path, timeout_ms=5000)
                    if last_created_folders:
                        client.list(folder_id=last_created_folders[0], timeout_ms=5000)
                except Exception:
                    pass

            create_phase = not create_phase
    finally:
        try:
            client.disconnect()
        except Exception:
            pass
