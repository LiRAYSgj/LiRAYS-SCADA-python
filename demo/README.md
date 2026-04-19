# lirays Python demo

Demo scripts that mirror the Rust demo scenarios.

## Covered scenarios

- `basic`: create/list/set/get/edit-metadata/delete operations
- `bulk`: bulk namespace creation from JSON schema
- `subscribe`: realtime value subscription flow
- `tree_stress`: concurrent namespace create/delete/list stress
- `data_stress`: concurrent get/set load over large variable sets
- `all`: runs all scenarios in sequence

## Run

```sh
python -m pip install --upgrade pip maturin
maturin develop
python demo/main.py \
  --demo all \
  --host 127.0.0.1 \
  --port 8245 \
  --tls false \
  --allow-insecure-tls false \
  --auth true \
  --pat-token "pat_xxx.yyy"
```

## Parameters

- `--demo <all|basic|bulk|subscribe|tree_stress|data_stress>`
- `--host <host>`
- `--port <port>`
- `--tls <true|false>`
- `--allow-insecure-tls <true|false>` (local self-signed certs only)
- `--auth <true|false>`
- `--pat-token <token>` (required when `--auth true`)

Equivalent env vars:

- `LIRAYS_DEMO`
- `LIRAYS_HOST`
- `LIRAYS_PORT`
- `LIRAYS_TLS`
- `LIRAYS_ALLOW_INSECURE_TLS`
- `LIRAYS_AUTH_ENABLED`
- `LIRAYS_PAT_TOKEN`

Additional stress-tuning env vars:

- Tree stress: `LIRAYS_TREE_NUM_CLIENTS`, `LIRAYS_TREE_MAX_BATCH`, `LIRAYS_TREE_RUN_SECS`
- Data stress: `LIRAYS_DATA_NUM_FOLDERS`, `LIRAYS_DATA_VARS_PER_FOLDER`, `LIRAYS_DATA_NUM_CLIENTS`, `LIRAYS_DATA_PCT_TOUCH`, `LIRAYS_DATA_RUN_SECS`, `LIRAYS_DATA_SET_CLIENT_RATIO`
