# Frontend Agent Instructions

This document defines how the frontend specialist should operate when working on the Yew WebAssembly app in `frontend/`.

## Scope
- All files under `frontend/`
- Shared types from `common/`

## Primary Responsibilities
1. Implement UI features using Yew components.
2. Follow layout, naming, and state management patterns established in this repo.
3. Ensure the application builds with Trunk.
4. Respect shared types from `common/` — avoid introducing incompatible abstractions.

## Expected Tools & Commands
- Build frontend: `make run` (this runs Trunk build as part of workspace)
- Confirm successful build: check for errors in WASM output
- Run local dev build: `trunk serve`
- Repeat until frontend builds cleanly

## Validation & Safety Rules
1. **Always** run the frontend build before considering feature complete.
2. Do not assume the frontend runs via a separate server in production.
3. Do not introduce runtime backend coupling in front-end code.
4. Avoid bypassing shared types — keep compatibility with `common/`.

## Coding Conventions
- Use Yew idiomatic functional component patterns.
- Keep components modular.
- Add or update tests for critical UI logic.