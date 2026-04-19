from __future__ import annotations

import sys

from common import connect_for_config, from_args_and_env

import basic
import bulk_test
import data_stress
import subscribe
import tree_stress


def main(argv: list[str] | None = None) -> int:
    config = from_args_and_env(argv)
    effective_auth = config.auth_enabled or config.pat_token is not None

    print(
        f"[demo] target={'wss' if config.tls else 'ws'}://{config.host}:{config.port} "
        f"auth={effective_auth} insecure_tls={config.allow_insecure_tls} demo={config.demo}"
    )

    demo = config.demo
    token = config.pat_token if (config.auth_enabled or config.pat_token) else None

    if demo == "all":
        client = connect_for_config(config)
        try:
            basic.run(client)
            bulk_test.run(client)
            subscribe.run(client)
        finally:
            client.disconnect()

        tree_stress.run(config.host, config.port, config.tls, config.allow_insecure_tls, token)
        data_stress.run(config.host, config.port, config.tls, config.allow_insecure_tls, token)
        return 0

    if demo == "basic":
        client = connect_for_config(config)
        try:
            basic.run(client)
        finally:
            client.disconnect()
        return 0

    if demo in {"bulk", "bulk_test"}:
        client = connect_for_config(config)
        try:
            bulk_test.run(client)
        finally:
            client.disconnect()
        return 0

    if demo in {"subscribe", "sub"}:
        client = connect_for_config(config)
        try:
            subscribe.run(client)
        finally:
            client.disconnect()
        return 0

    if demo in {"tree_stress", "tree"}:
        tree_stress.run(config.host, config.port, config.tls, config.allow_insecure_tls, token)
        return 0

    if demo in {"data_stress", "data"}:
        data_stress.run(config.host, config.port, config.tls, config.allow_insecure_tls, token)
        return 0

    raise ValueError(f"unknown demo: {demo}")


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
