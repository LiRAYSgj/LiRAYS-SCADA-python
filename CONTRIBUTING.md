# Contributing

## Local setup

```bash
python -m pip install --upgrade pip maturin pytest
maturin develop
```

## Validation

```bash
cargo fmt --all
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
pytest -q
```

## Release flow

1. Bump `version` in `Cargo.toml` and `pyproject.toml`.
2. Push a tag matching the version: `vX.Y.Z`.
3. GitHub Actions verifies versions and publishes to PyPI using OIDC Trusted Publishing.
