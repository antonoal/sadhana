# Static Site Agent Instructions

This agent specializes in the marketing/static site (`static-react/`) built with Vite + React + TypeScript.

---

## Scope

- All files under `static-react/`
- Asset directories under `static-react/src/assets/`
- No backend code or Rust workspace code

---

## Primary Responsibilities

1. Implement React + TypeScript features.
2. Follow existing frontend conventions and component naming.
3. Build and validate production-ready assets with Vite.
4. Maintain correct asset paths and imports.
5. Keep linter (ESLint/Prettier) passing.
6. Avoid introducing dependencies that couple with backend runtime.

---

## Expected Tools & Commands

```bash
cd static-react
npm run dev       # Local development
npm run build     # Production build
npm run lint      # Lint checks
npm run preview   # Preview production build
```

---

## Validation & Safety Rules

1. **Lint** must pass: `npm run lint`.
2. **Build** must succeed: `npm run build`.
3. Never assume a backend server is available at runtime.
4. Do not introduce runtime coupling with Rust workspace code.
5. Avoid unnecessary dependencies or unused assets.
6. Confirm production assets match expected directory structure (`src/assets/`, images, icons).

---

## Coding Conventions

- Use descriptive React component names: `LandingView.tsx`, `ParallaxSection.tsx`.
- Maintain TypeScript types and interfaces.
- Use modular CSS or scoped styles where appropriate.
- Keep Prettier formatting consistent with `.prettierrc` rules (2-space indentation).

---

## Notes

- If changes require PR screenshots, include them in the description.
- For any cross-cutting changes (e.g., shared `common/` types), confirm with orchestrator agent.
- All builds must succeed before considering changes complete.