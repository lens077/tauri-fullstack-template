# Backend data layer starter

This optional backend layer keeps persistence out of the Tauri WebView. The desktop/mobile app should call a remote API, or use SQLite locally; do not ship Postgres/Redis credentials in the client.

## Services

```bash
docker compose up -d
```

- PostgreSQL: `postgres://app:app@localhost:5432/app`
- Redis: `redis://localhost:6379`
- SQLite: `sqlite://data/app.db`

## SQLx (Rust)

`examples/sqlx/todos.rs` contains parameterized CRUD for SQLite/Postgres using `sqlx::AnyPool`. In a real crate enable `runtime-tokio`, `sqlite`, `postgres`, and `macros`, run migrations at startup, and configure `DATABASE_URL` only on the server or local app data directory.

## sqlc (Go)

For a Go API, install sqlc and add a `sqlc.yaml` that points at `migrations/` and `queries/`, then generate typed CRUD with `sqlc generate`. Keep generated code out of the Tauri frontend. Redis should be used as a cache, never as the source of truth; use explicit TTLs and invalidate keys after writes.

## Production checklist

- Use SQLite for offline/local-first data; PostgreSQL for server-authoritative data.
- Put all secrets in environment variables or an OS keychain, never `tauri.conf.json` or frontend code.
- Add migrations and rollback policy before shipping.
- Bound Redis connection pools, set timeouts, and namespace keys by tenant/user.
- Validate all Tauri commands and API inputs in Rust/server code.
