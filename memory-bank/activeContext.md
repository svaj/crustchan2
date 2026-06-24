# Active Context

## Current Work Focus

### 1. Agent Tooling & Workflow Setup
- Switching between Cline and Codex agent configurations
- Setting up agent memory banks and role-specific prompts
- Configuring `.cline/cline_mcp_settings.json` for MCP integrations

### 2. Authentication & Middleware
- Implemented Tuono middleware support via a customized fork (`svaj/tuono/tree/middleware-and-extractors`)
- Added auth callback routes (`/auth`, `/auth/callback`, `/auth/logout`)
- Set up JWT handling in frontend Rust code (`jwt.rs`)
- Auth0 integration in progress

### 3. Frontend Scaffolding
- Tuono-based frontend package (`packages/frontend`) set up with:
  - React 19 + TypeScript
  - Route structure: `/`, `/boards`, `/boards/[boardid]`, `/users`, `/auth/*`, `/admin`
  - Basic mocked boardlist interaction
  - Vitest testing harness with `@testing-library/react`
  - Tailwind/global CSS
- Layout system via `__layout.tsx`
- Middleware pattern for admin routes (`admin/middlewares.rs`)

### 4. Code Quality
- Recent formatting and readability refactor across files
- Workspace dependency pins established in root `Cargo.toml`

## Recent Changes (last 10 commits)
| Commit | Summary |
|--------|---------|
| `7ca1f9b` | Switched back to Cline; considering kanban-style workflow |
| `b3539b5` | Moved to Codex; fixed agent directory nesting issue |
| `161968e` | Moved Cline agents to Codex storage |
| `c6af469` | Added Cline agent config (then moved to Codex) |
| `b45e0b3` | Agent configuration setup |
| `5390b60` | Code formatting and readability cleanup |
| `c1748cf` | Router middleware working in Tuono |
| `0448654` | Auth middleware + Tuono extractors + Auth0 callbacks |
| `cb2c8ad` | Tuono frontend setup; mocked boardlist; removed plural from `user_permissions`; workspace deps |
| `0f8d780` | Initial frontend scaffolding |

## Active Decisions & Considerations

### Architecture Decisions
- **Tuono Fork:** Using a custom fork with middleware and extractor support. If upstream changes are needed, issues must be raised in the fork's GitHub repo.
- **Package Name Mismatch:** Workspace `Cargo.toml` references `packages/frontend-ng` but the actual directory is `packages/frontend`. This may cause build issues.
- **Monorepo Tool:** moon is configured but moon.yml files are sparse. The frontend has a `moon.yml` and the API has one too.

### Open Questions
- What is the current state of the `packages/api` backend? (Has a SQLite file but unclear if schema is current)
- Is the database schema in `packages/entity` complete enough for the frontend's mocked boardlist?
- Should the frontend package name in `Cargo.toml` be updated from `frontend-ng` to `frontend`?

## Next Steps
1. Fix workspace `Cargo.toml` member reference (`frontend-ng` → `frontend`)
2. Continue fleshing out auth flow (ensure callback properly exchanges tokens)
3. Connect frontend boardlist to real API instead of mocks
4. Expand database entities for core imageboard models (Board, Thread, Post)
5. Set up actual API endpoints for board/thread/post CRUD
6. Add frontend tests for existing components (only `Greeting.test.tsx` exists)
7. Set up CI/CD pipeline scaffolding in GitHub Actions

## Important Patterns & Preferences
- Prefer borrowing over cloning in Rust
- Use `Result<T, E>` for recoverable errors; avoid `unwrap()`
- Frontend: functional components, hooks, strict TypeScript, semantic HTML
- Imageboard UI should be dense and workflow-oriented
- Commit messages reference issues with `ref #<number>` (no closing keywords)
