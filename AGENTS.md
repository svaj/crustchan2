# Crustchan 

Crustchan  is a 4chan-style imageboard built for the modern age. The repository is a
cloud-native monorepo with Rust backend services, a Tuono/React frontend,
PostgreSQL persistence, Redis caching, and Terraform-managed infrastructure.

## Agent Instructions

- This `AGENTS.md` file is the instruction source for agents in this repository root.
- Package-level `AGENTS.md` files override or extend these instructions for
  their subtrees. Check for one before making changes inside a package.
- Preserve user changes already present in the worktree. Do not revert
  unrelated edits. Do not commit anything unless you have the `engineer` agent role. (it is denoted by a line in your agent prompt beginning with `AGENT-ROLE:`, containing the text ("Engineer"), or use your agent name as your role)
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
- `packages/entity`: Rust crate defining database entities in Sea-ORM. 
- `packages/lib`: shared Rust library code.
- `packages/migration`: database migrations via Sea-ORM
- `packages/image-processor`: image processing components.
- `infrastructure`: Terraform and deployment infrastructure.
- `.github/workflows`: GitHub Actions workflows.
- `.moon`: moon workspace configuration.
- `agents`: role-specific agent prompts.
- `skills`: Agent skill definitions
- `docs`: Documentation for AI and Humans alike

## Outside the scope of this project, but in active development
- A customized fork of [Tuono](https://tuono.dev/),  The fork we use is https://github.com/svaj/tuono/tree/middleware-and-extractors.  If work should be done there, raise an issue in that fork's github issues and we'll try to make it implemented.


## Development Workflow

- Inspect the relevant issue and affected packages before changing code; several areas are still early-stage or incomplete.
- Keep changes scoped to the requested package or feature.
- Add or update tests for behavioral changes when the package has a test
  harness available.
- Run the narrowest reliable checks for the touched code before finishing.
- If a command cannot run because dependencies, local services, credentials, or
  generated files are missing, report the blocker clearly in the github issue and ask for advice.
- Follow these guidelines [here](/docs/agent-instructions.md), [here](/docs/guidelines-best-practices/AI-Guidelines.md), [here](/docs/guidelines-best-practices/general-best-practices.md), and [here](/docs/guidelines-best-practices/Engineer-workflow.md)

## Rust

- Write idiomatic, safe Rust and keep code warning-free.
- Use `Result<T, E>` for recoverable errors. Avoid `unwrap()` and `expect()` in
  application and library code unless there is a clear invariant.
- Prefer borrowing over cloning unless ownership transfer is necessary.
- Use modules and public interfaces to encapsulate behavior.
- Document public APIs with rustdoc comments.
- Run `cargo fmt`, `cargo clippy`, and `cargo test` when they are relevant and
  feasible for the touched package.
- Follow rust-specific instructions [here](/docs/guidelines-best-practices/Backend.md), [here](/docs/guidelines-best-practices/Engineer-workflow.md), [here](/docs/agent-instructions.md),  [here](/docs/guidelines-best-practices/general-best-practices.md)

## Frontend

- Use TypeScript, React 19, and Tuono patterns already present in the package.
- Prefer functional components, hooks, strict TypeScript, semantic HTML, and
  accessible interactive controls.
- Use React 19 features when they simplify the implementation.
- Keep UI work appropriate for an imageboard product: dense, readable, and
  workflow-oriented rather than marketing-oriented.
- Follow package-specific instructions  [here](/docs/guidelines-best-practices/Frontend.md), [here](/docs/guidelines-best-practices/Engineer-workflow.md), [here](/docs/agent-instructions.md),  [here](/docs/guidelines-best-practices/general-best-practices.md), [here](/docs/guidelines-best-practices/Frontend-React.md), [here](/docs/guidelines-best-practices/Frontend-Tailwind-Typescript-React.md), [here](/docs/guidelines-best-practices/Typescript.md)

## CI/CD and Infrastructure

- GitHub Actions workflows should use explicit least-privilege permissions.
- Prefer pinned major versions for trusted actions such as `actions/checkout`.
- Avoid long-lived cloud credentials.
- Keep workflows clear, modular, and scoped to the needed triggers.
- Terraform changes should be performed in test branches deploying to test environments and should be focused, formatted, and documented when behavior changes.
- Follow package-specific instructions  [here](/docs/guidelines-best-practices/ci-cd-docker.md), [here](/docs/guidelines-best-practices/Engineer-workflow.md), [here](/docs/agent-instructions.md),  [here](/docs/guidelines-best-practices/general-best-practices.md), [here](/docs/guidelines-best-practices/ci-cd-github.md)


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
- More details are [here](/docs/agent-instructions.md) and [here](/docs/guidelines-best-practices/Engineer-workflow.md)
