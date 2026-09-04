# `translator` Crate (`iml-translator`)

## API Endpoints (Axum)
- `GET /ast`: Returns the current `Arena` state.
- `POST /translate`: Accepts `TranslateRequest`. Returns updated `Arena` via LLM mutation (`rewrite_node`).

## `TranslateRequest` Struct
- `node_index`: `usize`
- `updated_text`: `String`

## `AppState` Struct
- `arena`: `Arc<Mutex<Arena>>`
- `client`: `reqwest::Client`

## Frontend Service
- **Release builds**: Serves static assets from `frontend/dist` via `rust-embed`.
- **Debug builds**: Proxies unhandled routes to `http://localhost:5173`.
