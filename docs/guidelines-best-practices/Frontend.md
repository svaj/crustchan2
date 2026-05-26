# Crustchan Frontend
Located at [/packages/frontend](/packages/frontend/)
This package is the Tuono-based frontend for Crustchan 2. Tuono combines a
React/TypeScript client with Rust server-side routing and middleware support.
Treat this package as the current priority frontend surface for the project.

## Agent Instructions

- Preserve user changes already present in the worktree. Do not revert
  unrelated edits.
- Prefer small, focused changes that match the existing Tuono, React,
  TypeScript, and Rust patterns in this package.

## Stack

- Frontend: TypeScript, React 19, Tuono.
- Server-side frontend code: Rust 2024 edition with Tuono and Axum-style
  middleware integration.
- Styling: global CSS is currently in `src/styles/global.css`; use Tailwind
  only where the project has configured it.
- Data fixtures: `fixtures/board.json` and `fixtures/boardList.json`.
- Local Tuono fork: `Cargo.toml` points at
  `/home/svaj/development/tuono/crates/*` for customized Tuono crates.

## Important Paths

- React routes and layouts: `src/routes/**/*.tsx`.
- Rust route and app integration: `src/routes/**/*.rs`, `src/app.rs`,
  `src/state.rs`, `src/jwt.rs`, `mod.rs`.
- Generated Tuono files: `.tuono/`.
- Client assets: `public/`.
- Build output: `out/`.
- Cache output: `target/`

Do not edit generated `.tuono/` files or `out/` artifacts unless the task explicitly requires generated output changes.  If you encounter an error in Tuono's generation of main.rs, or how it functions, create an issue on this [github repository](https://github.com/svaj/tuono/)

## Development Commands

This package currently has no npm scripts in `package.json`. Prefer direct
tool commands until scripts are added.

- Install frontend dependencies: `npm install`.
- Type-check TypeScript: `npx tsc --noEmit`.
- Run Rust checks from this package: `cargo check -p crustchan-fe-ng`.
- Format Rust from the repository root: `cargo fmt`.
- Run Rust tests from the repository root: `cargo test`.
- We might need to add a Front-end only component unit test framework like vitest. to separate FE and BE tests.
- Start or build Tuono with the Tuono CLI if needed, checking the installed
  `tuono` package for the exact command supported by the current version.

If a command fails because the local Tuono fork at `/home/svaj/development/tuono`
is unavailable, report that as an environment prerequisite instead of replacing
the dependency with the published crate.

## React and TypeScript

- Use function components and hooks; do not introduce class components.
- Use React 19 features where they simplify the implementation, including
  `use`, `useActionState`, `useOptimistic`, `useFormStatus`,
  `useEffectEvent`, Suspense, and transitions.
- Mark client-only components with `'use client'` when needed.
- Keep JSX semantic and accessible. Use native interactive elements such as
  `button`, `nav`, and `main` before ARIA-heavy custom controls.
- Ensure keyboard access and useful focus states for interactive UI.
- Keep TypeScript strict. Avoid `any`; prefer `unknown` only at trust
  boundaries with explicit narrowing.
- Prefer interfaces for object shapes and discriminated unions for UI or async
  state.
- Use named exports for reusable components and utilities.
- Keep helpers, static data, and local types close to the component unless they
  are shared across routes.

## Rust

- Write idiomatic, safe Rust that compiles without warnings.
- Use `Result<T, E>` for recoverable errors and avoid `unwrap()` or `expect()`
  in application code unless there is a clear invariant.
- Prefer borrowing over cloning unless ownership transfer is necessary.
- Use async/await patterns consistently with the existing Tuono server code.
- Document public APIs with rustdoc comments when adding public Rust items.
- Run `cargo fmt` and relevant `cargo check` or `cargo test` commands after
  Rust changes.

## Styling and UI

- Use mobile-first layouts.
- Keep page UI dense and usable for an imageboard workflow rather than
  marketing-style composition.
- Avoid nested cards and decorative gradient/orb backgrounds.
- Use stable dimensions for repeated UI elements, grids, toolbars, counters,
  and tiles so content changes do not cause layout shifts.
- Do not add visible instructional copy that explains the app's features or
  shortcuts unless the product surface explicitly needs onboarding text.

## Testing and Verification

- Add or update tests for behavior changes when the project has the relevant
  test harness available.
- For TypeScript-only changes, run `npx tsc --noEmit`.
- For Rust changes, run `cargo check -p crustchan-fe-ng` at minimum.
- For mixed Tuono changes, run both TypeScript and Rust checks where possible.
- If verification cannot run because dependencies, local services, or the local
  Tuono fork are missing, state the blocker clearly in the final response.

## Git and PR Workflow

- Use non-destructive git commands. Never reset or discard user changes unless
  explicitly requested.
- Branches normally originate from the repository's default development branch,
  but do not create or switch branches unless the user asks or the task requires
  it.
- Keep commits focused if the user asks for commits.
- Commit messages should reference the associated issue with `ref #<number>`
  when an issue number is known. Do not use closing keywords such as `fixes`,
  `closes`, or `resolves` before an issue reference.
