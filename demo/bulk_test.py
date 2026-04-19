from __future__ import annotations

import time

from lirays import Client


def run(client: Client) -> None:
    root = f"bulk_{int(time.time() * 1000)}"
    root_path = f"/{root}"

    client.create_folders([root], timeout_ms=20000)

    schema = """
    {
      "LineA": {
        "Cell1": {
          "pump_speed": {"variable": {"var_d_type": "Float", "unit": "rpm", "min": 0, "max": 3600}},
          "pump_mode": {"variable": {"var_d_type": "Text", "options": ["AUTO", "MAN", "OFF"], "max_len": 8}},
          "pump_on": {"variable": {"var_d_type": "Boolean"}}
        },
        "Cell2": {
          "tank_level": {"variable": {"var_d_type": "Float", "unit": "%", "min": 0, "max": 100}},
          "alarm_count": {"variable": {"var_d_type": "Integer", "min": 0, "max": 9999}}
        }
      }
    }
    """

    client.create_bulk_from_json(schema, parent_id=root_path, timeout_ms=20000)

    _, vars_found = client.list(folder_id=f"{root_path}/LineA/Cell1", timeout_ms=8000)
    print(f"[bulk] variables under Cell1: {len(vars_found)}")
