# lirays (Python)

Python client for LiRAYS SCADA powered by the Rust [`lirays`](https://crates.io/crates/lirays) crate.

- PyPI: [pypi.org/project/lirays](https://pypi.org/project/lirays/)
- Rust core: [crates.io/crates/lirays](https://crates.io/crates/lirays)

## Features

- Same high-level API surface as the Rust client.
- Connect with or without PAT auth.
- Namespace operations: list, create, delete, metadata edit.
- Value operations: get/set for integer, float, text, and boolean.
- Bulk creation from JSON schema.
- Realtime subscription stream for variable values.
- Optional insecure TLS mode for local self-signed certificates.

## Installation

```bash
pip install lirays
```

## Quick Start

```python
from lirays import Client, ConnectionOptions, FloatVar

opts = ConnectionOptions("127.0.0.1", 8245, tls=False, pat_token=None)
client = Client.connect_with_options(opts)

client.create_folders(["demo"], timeout_ms=8000)
client.create_float_variables([FloatVar(name="signal")], parent_id="/demo", timeout_ms=8000)
print(client.get_values(["/demo/signal"], timeout_ms=8000))

client.disconnect()
```

## Self-signed TLS (local/dev only)

```python
from lirays import ConnectionOptions, Client

opts = ConnectionOptions("127.0.0.1", 8245, tls=True, allow_insecure_tls=True)
client = Client.connect_with_options(opts)
```

## Development

```bash
python -m pip install --upgrade pip maturin pytest
maturin develop
pytest -q
```

## Demo Scenarios

Demo scripts are under [`demo/`](demo/README.md) and mirror the Rust demos:

- `basic`
- `bulk`
- `subscribe`
- `tree_stress`
- `data_stress`
- `all`

## Release

Push a tag like `v0.1.0` to trigger GitHub Actions publishing to PyPI via Trusted Publishing (OIDC).

## License

MIT ([LICENSE](LICENSE)).
