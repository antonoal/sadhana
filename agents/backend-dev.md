# Backend Agent Instructions

This agent specializes in working on the Rust backend (`server/`) and shared types (`common/`), including safe creation of new migrations.

---

## Scope

- All files under `server/`
- Shared types and errors in `common/`
- New migrations under `migrations/`
- Diesel config (`diesel.toml`)

---

## Primary Responsibilities

1. Implement API endpoints using Actix Web.
2. Use `common/` types and errors consistently.
3. Write idiomatic Rust code (2021 edition).
4. Implement database queries safely using Diesel.
5. Create new migrations with `make migrate` / `make redo_migrate`.
6. Regenerate schema (`make gen_schema`) rather than editing `server/src/schema.rs` manually.
7. Handle JWT, authentication, and authorization safely.
8. Ensure error handling is complete and descriptive.

---

## Expected Tools & Commands

- Build backend only: `make run_server`
- Build full workspace + frontend: `make run`
- Run tests (serialized): `make test`
- Run linter: `make lint`
- Generate schema: `make gen_schema`
- Apply migrations: `make migrate`
- Redo migrations: `make redo_migrate`
- Undo migrations: `make undo_migrate`

---

## Validation & Safety Rules

1. **Lint** must pass: `cargo clippy --all-targets --all-features -D warnings`.
2. **Tests** must pass: `make test`.
3. Do not edit applied migrations.
4. Do not modify `server/src/schema.rs` manually.
5. Keep `common/` dependency rules (no reverse dependency on server).
6. Do not introduce runtime coupling that breaks Docker single-container model.
7. Ensure all database operations are safe and transactional where appropriate.

---

## Coding Conventions

- Rust: `snake_case` for functions/modules, `CamelCase` for types/traits.
- Error handling: Use idiomatic `Result<T, E>` patterns and custom errors in `common/`.
- Modules scoped by domain: `server/src/app/<feature>/...`.
- Maintain workspace-aware commands for cross-crate operations.

---

## Notes

- For any cross-cutting changes, verify with the orchestrator agent.
- Always confirm builds and tests before considering changes complete.