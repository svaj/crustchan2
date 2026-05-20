# Crustchan 2

Crustchan 2 is a 4chan-style imageboard built for 2026. The project is a
cloud-native monorepo with Rust backend services, a Tuono/React frontend,
PostgreSQL persistence, Redis caching, and Terraform-managed infrastructure.

## Agent Instructions

- This `AGENTS.md` file is the Codex-facing instruction source for the
  repository root.
- Package-level `AGENTS.md` files override or extend these instructions for
  their subtrees. Check for one before making changes inside a package.
- Individual role prompts live in `.agents/agents/*.md`.
- Do not depend on `.cline`, `.github/instructions`, `.github/agents`, or
  GitHub Copilot-specific instruction files for current agent guidance.
- Preserve user changes already present in the worktree. Do not revert
  unrelated edits.
- Prefer the repository's existing patterns and local helper APIs over new
  abstractions.

## Tech Stack

- Monorepo: moon.
- Backend: Rust 1.93.1, Axum, axum_openapi3.
- Frontend: TypeScript, React, Tuono.
- Database: PostgreSQL.
- Cache and events: Redis.
- Serverless components: Rust 1.93.1.
- Infrastructure: Terraform.
- Testing: cargo test and frontend test tooling where configured.
- Authentication: OAuth2 and OIDC.
- Supported auth providers: Apple, Microsoft, Google, Twitch, Facebook.

## Repository Layout

- `packages/frontend`: Tuono React frontend and Rust server-side frontend code.
- `packages/api`: Rust API service.
- `packages/lib`: shared Rust library code.
- `packages/migration`: database migrations.
- `packages/image-processor`: image processing components.
- `infrastructure`: Terraform and deployment infrastructure.
- `.agents/agents`: role-specific agent prompts.
- `.github/workflows`: GitHub Actions workflows.
- `.moon`: moon workspace configuration.

## Development Workflow

- Inspect the relevant package before changing code; several areas are still
  early-stage or incomplete.
- Keep changes scoped to the requested package or feature.
- Add or update tests for behavioral changes when the package has a test
  harness available.
- Run the narrowest reliable checks for the touched code before finishing.
- If a command cannot run because dependencies, local services, credentials, or
  generated files are missing, report the blocker clearly.

## Rust

- Write idiomatic, safe Rust and keep code warning-free.
- Use `Result<T, E>` for recoverable errors. Avoid `unwrap()` and `expect()` in
  application and library code unless there is a clear invariant.
- Prefer borrowing over cloning unless ownership transfer is necessary.
- Use modules and public interfaces to encapsulate behavior.
- Document public APIs with rustdoc comments.
- Run `cargo fmt`, `cargo clippy`, and `cargo test` when they are relevant and
  feasible for the touched package.

## Frontend

- Use TypeScript, React 19, and Tuono patterns already present in the package.
- Prefer functional components, hooks, strict TypeScript, semantic HTML, and
  accessible interactive controls.
- Use React 19 features when they simplify the implementation.
- Keep UI work appropriate for an imageboard product: dense, readable, and
  workflow-oriented rather than marketing-oriented.
- Follow package-specific instructions in `packages/frontend/AGENTS.md`.

## CI/CD and Infrastructure

- GitHub Actions workflows should use explicit least-privilege permissions.
- Prefer pinned major versions for trusted actions such as `actions/checkout`.
- Avoid long-lived cloud credentials; use OIDC where possible.
- Keep workflows clear, modular, and scoped to the needed triggers.
- Terraform changes should be focused, formatted, and documented when behavior
  changes.

## Git and PR Workflow

- Use non-destructive git commands. Never reset or discard user changes unless
  explicitly requested.
- Branches normally originate from the repository's default development branch,
  but do not create or switch branches unless the user asks or the task requires
  it.
- If the user asks for commits, keep them focused.
- Commit messages should reference the associated issue with `ref #<number>`
  when an issue number is known.
- Do not use closing keywords such as `fixes`, `closes`, or `resolves` before
  an issue reference.

