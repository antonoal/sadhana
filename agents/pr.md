# Pull Request & Commit Guidelines

This agent role governs how commits and pull request descriptions should be generated.

## Purpose
Produce clear, informative, and standard-compliant commit messages and PR descriptions for changes made by other agents (e.g., backend or frontend).

## Commit Message Rules
1. Use **imperative tense** (e.g., `Fix bug in login handler`).
2. Start with a short subject line (≤ 72 chars).
3. Follow with a blank line then a concise body.
4. Include context about:
   - What changed
   - Why it changed
   - Risks/assumptions
   - Migration impacts (if any)

## Pull Request Rules
1. Provide a **clear summary** of:
   - Behavior changes
   - Component/domain impacted
   - Any cross-cutting effects
2. Link to:
   - Related task/issue (if available)
   - Migration notes (if relevant)
3. Include verification steps:
   - Which lint/test/build commands should pass
   - Screenshots for UI changes
4. Highlight any policy violations fixed or edge cases introduced.

## Validation
1. Verify all tests and lint checks passed.
2. If backend changes included `migrations/`, list migration notes.
3. For frontend UI changes, request screenshots or screenshots of failing/passing states.
4. Confirm no prohibited files (like applied migrations edits or manual schema edits) were touched unless explicitly allowed.

## Formatting
- PR title: `[<scope>] <short subject>`
- Break long descriptions into sections with headers.
- Use code fences for command output or examples.