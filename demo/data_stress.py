from __future__ import annotations

import math
import os
import random
import time
from concurrent.futures import ThreadPoolExecutor, as_completed

from lirays import FloatVar

from common import connect_client


NUM_FOLDERS = int(os.getenv("LIRAYS_DATA_NUM_FOLDERS", "200"))
VARS_PER_FOLDER = int(os.getenv("LIRAYS_DATA_VARS_PER_FOLDER", "5000"))
NUM_CLIENTS = int(os.getenv("LIRAYS_DATA_NUM_CLIENTS", "30"))
PCT_TOUCH = int(os.getenv("LIRAYS_DATA_PCT_TOUCH", "10"))
RUN_SECS = int(os.getenv("LIRAYS_DATA_RUN_SECS", "60"))
SET_CLIENT_RATIO = float(os.getenv("LIRAYS_DATA_SET_CLIENT_RATIO", "0.10"))


def run(host: str, port: int, tls: bool, allow_insecure_tls: bool, pat_token: str | None) -> None:
    setup_client = connect_client(host, port, tls, allow_insecure_tls, pat_token)
    root_name = f"DataStress_{int(time.time() * 1000)}"
    root_path = f"/{root_name}"

    all_var_ids: list[str] = []
    try:
        setup_client.create_folders([root_name], timeout_ms=10000)

        for folder_idx in range(NUM_FOLDERS):
            folder_name = f"f{folder_idx}"
            folder_path = f"{root_path}/{folder_name}"

            setup_client.create_folders([folder_name], parent_id=root_path, timeout_ms=10000)
            vars_to_create = [FloatVar(name=f"v{var_idx}") for var_idx in range(VARS_PER_FOLDER)]
            setup_client.create_float_variables(
                vars_to_create,
                parent_id=folder_path,
                timeout_ms=10000,
            )

            for var_idx in range(VARS_PER_FOLDER):
                all_var_ids.append(f"{folder_path}/v{var_idx}")
    finally:
        try:
            setup_client.disconnect()
        except Exception:
            pass

    set_clients = max(1, math.ceil(NUM_CLIENTS * SET_CLIENT_RATIO))

    with ThreadPoolExecutor(max_workers=NUM_CLIENTS) as executor:
        futures = [
            executor.submit(
                _load_worker,
                idx,
                host,
                port,
                tls,
                allow_insecure_tls,
                pat_token,
                all_var_ids,
                idx < set_clients,
            )
            for idx in range(NUM_CLIENTS)
        ]

        for fut in as_completed(futures):
            exc = fut.exception()
            if exc is not None:
                print(f"[data_stress] worker error: {exc}")


def _load_worker(
    idx: int,
    host: str,
    port: int,
    tls: bool,
    allow_insecure_tls: bool,
    pat_token: str | None,
    var_ids: list[str],
    is_setter: bool,
) -> None:
    client = connect_client(host, port, tls, allow_insecure_tls, pat_token)
    total = len(var_ids)
    sample_size = max(1, (total * PCT_TOUCH) // 100)
    deadline = time.monotonic() + RUN_SECS

    try:
        while time.monotonic() < deadline:
            subset = random.sample(var_ids, k=sample_size)
            values = [random.uniform(0.0, 100.0) for _ in range(sample_size)]

            if is_setter:
                try:
                    client.set_float_variables(subset, values, timeout_ms=5000)
                except Exception:
                    pass

            try:
                client.get_values(subset, timeout_ms=5000)
            except Exception:
                pass

            time.sleep(random.uniform(1.0, 5.0))
    finally:
        try:
            client.disconnect()
        except Exception:
            pass

    print(f"client {idx} done ({'set+get' if is_setter else 'get'})")
