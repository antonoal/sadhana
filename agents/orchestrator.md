# Workspace Orchestrator Rules

This document explains how responsibilities are divided and enforced across agents.

## Global Repository Rules
1. Only modify migrations using the migration tool (`make migrate` / `make redo_migrate`).
2. Do not edit `server/src/schema.rs` manually — use `make gen_schema`.
3. Docker build and runtime must remain single-container.
4. Shared types in `common/` may be updated only with compatible Bump changes.
5. Forbidden changes: editing applied migrations, committing `.env`.

## Delegation Logic
- Changes inside `server/`: handled by Backend Agent
- Changes inside `frontend/`: handled by Frontend Agent
- Changes inside `static-react/`: handled by Static Site Agent
- Cross-cutting changes: ask before proceeding