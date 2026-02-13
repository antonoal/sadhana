# Repository AI Operating Guide

This file defines global repository constraints and agent routing rules.

Detailed implementation behavior is defined in `/agents/*.md`.

---

## Repository Structure

Rust workspace with three crates:
- `server/` - Actix Web API (binary crate)
- `frontend/` - Yew WebAssembly app
- `common/` - shared types/errors

Separate marketing site:
- `static-react/` - Vite + React + TypeScript

Shared assets are in `frontend/images/`, `frontend/icons/`, and `static-react/src/assets/`.

Database migrations live in `migrations/` and Diesel config is in `diesel.toml`.

Single-container Docker deployment:
- Server serves prebuilt frontend assets.
- No multi-container runtime assumptions.

---

## Global Invariants (Must Never Be Violated)

1. Do not edit applied files in `migrations/`.
2. Do not manually modify `server/src/schema.rs` (use `make gen_schema`).
3. Do not introduce unnecessary new dependencies.
4. Do not split runtime responsibilities.
5. `common/` must not depend on `server/` or `frontend/`.
6. Do not commit `.env` files.
7. Do not reformat unrelated files.

---

## Agent Routing

Specialist instructions are located in:

- Backend: `/agents/backend-dev.md`
- Frontend: `/agents/frontend-dev.md`
- Static Site: `/agents/static-site-dev.md`
- PR Generator: `/agents/pr.md`
- Review Agent: `/agents/reviewer.md`
- Orchestrator: `/agents/orchestrator.md`

---

## Validation Requirements (High-Level)

After backend changes:
- `make lint`
- `make test`

After frontend changes:
- `make run`

After static-react changes:
- `npm run lint`
- `npm run build`

---

## Change Completion Rule

Changes are not complete unless:
- Relevant builds pass
- Lint passes
- Tests pass
- No prohibited changes were made