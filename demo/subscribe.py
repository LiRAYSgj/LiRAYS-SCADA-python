from __future__ import annotations

import time

from lirays import Client, FloatVar


def run(client: Client) -> None:
    root = f"sub_{int(time.time() * 1000)}"
    root_path = f"/{root}"
    var_id = f"{root_path}/signal"

    client.create_folders([root], timeout_ms=8000)
    client.create_float_variables(
        [FloatVar(name="signal", unit="unit")],
        parent_id=root_path,
        timeout_ms=8000,
    )

    stream = client.subscribe_var_values([var_id], timeout_ms=8000)

    client.set_float_variables([var_id], [1.0], timeout_ms=8000)
    client.set_float_variables([var_id], [2.0], timeout_ms=8000)
    client.set_float_variables([var_id], [3.0], timeout_ms=8000)

    received = 0
    while True:
        event = stream.next_event(timeout_ms=8000)
        if event is None:
            break
        item_id, value = event
        print(f"[subscribe] event {item_id} -> {value!r}")
        if item_id == var_id:
            received += 1
        if received >= 3:
            break

    stream.close()
    print(f"[subscribe] received {received} value events")
