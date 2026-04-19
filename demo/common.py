from __future__ import annotations

import argparse
import os
import sys
from dataclasses import dataclass
from pathlib import Path

# Allow running demos without installation: python demo/main.py
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "python"))

from lirays import Client


@dataclass(frozen=True)
class DemoConfig:
    demo: str
    host: str
    port: int
    tls: bool
    allow_insecure_tls: bool
    auth_enabled: bool
    pat_token: str | None


def parse_bool(raw: str) -> bool:
    value = raw.strip().lower()
    if value in {"1", "true", "yes", "on"}:
        return True
    if value in {"0", "false", "no", "off"}:
        return False
    raise ValueError(f"invalid boolean value: {raw}")


def _env_bool(name: str, default: bool) -> bool:
    raw = os.getenv(name)
    return parse_bool(raw) if raw is not None else default


def from_args_and_env(argv: list[str] | None = None) -> DemoConfig:
    parser = argparse.ArgumentParser(
        description="LiRAYS Python client demos",
        formatter_class=argparse.ArgumentDefaultsHelpFormatter,
    )
    parser.add_argument("--demo", default=os.getenv("LIRAYS_DEMO", "all"))
    parser.add_argument("--host", default=os.getenv("LIRAYS_HOST", "127.0.0.1"))
    parser.add_argument("--port", type=int, default=int(os.getenv("LIRAYS_PORT", "8245")))
    parser.add_argument("--tls", type=parse_bool, default=_env_bool("LIRAYS_TLS", False))
    parser.add_argument(
        "--allow-insecure-tls",
        type=parse_bool,
        default=_env_bool("LIRAYS_ALLOW_INSECURE_TLS", False),
    )
    parser.add_argument(
        "--auth",
        dest="auth_enabled",
        type=parse_bool,
        default=_env_bool("LIRAYS_AUTH_ENABLED", False),
    )
    parser.add_argument("--pat-token", default=os.getenv("LIRAYS_PAT_TOKEN"))

    args = parser.parse_args(argv)
    return DemoConfig(
        demo=args.demo,
        host=args.host,
        port=args.port,
        tls=args.tls,
        allow_insecure_tls=args.allow_insecure_tls,
        auth_enabled=args.auth_enabled,
        pat_token=args.pat_token,
    )


def connect_client(
    host: str,
    port: int,
    tls: bool,
    allow_insecure_tls: bool,
    pat_token: str | None,
) -> Client:
    if pat_token:
        return Client.connect_with_pat(
            host,
            port,
            tls=tls,
            pat_token=pat_token,
            allow_insecure_tls=allow_insecure_tls,
        )
    return Client.connect(host, port, tls=tls, allow_insecure_tls=allow_insecure_tls)


def connect_for_config(config: DemoConfig) -> Client:
    if config.auth_enabled and not config.pat_token:
        raise ValueError(
            "auth is enabled but PAT token was not provided (--pat-token or LIRAYS_PAT_TOKEN)"
        )

    token = config.pat_token if (config.auth_enabled or config.pat_token) else None
    return connect_client(
        config.host,
        config.port,
        config.tls,
        config.allow_insecure_tls,
        token,
    )
