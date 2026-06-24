# System Patterns

## Monorepo Architecture

```
crustchan2/
├── Cargo.toml              # Workspace config (⚠️ references "frontend-ng" — needs fix)
├── .moon/                  # moon workspace configuration
├── packages/
│   ├── frontend/           # Tuono + React 19 frontend (SSR)
│   │   ├── src/routes/     # File-based routing (Rust .rs + React .tsx pairs)
│   │   ├── src/components/ # Reusable React components
│   │   ├── src/test/       # Vitest test setup
│   │   └── package.json    # npm deps (React, Tuono, Vitest, Testing Library)
│   ├── api/                # Rust API service (Axum)
│   │   ├── src/            # API handlers, routes, middleware
│   │   └── Cargo.toml
│   ├── entity/             # Sea-ORM database entities
│   │   └── src/            # Entity definitions
│   ├── lib/                # Shared Rust library code
│   ├── migration/          # Sea-ORM database migrations
│   └── image-processor/    # Image optimization pipeline
├── infrastructure/         # Terraform configuration
├── .github/workflows/      # GitHub Actions CI/CD
├── agents/                 # AI agent role prompts
├── skills/                 # Agent skill definitions
├── docs/                   # Development guidelines
└── memory-bank/            # Project documentation (this)
```

## Frontend Routing Pattern (Tuono)
Tuono uses file-based routing where each route requires BOTH a `.rs` (server-side) and `.tsx` (client-side) file:

```
src/routes/
├── index.rs / index.tsx           # Home page
├── boards/
│   ├── index.rs / index.tsx       # Board list
│   └── [boardid]/
│       ├── index.rs / index.tsx   # Individual board
├── users/
│   └── index.rs / index.tsx       # User profile/list
├── auth/
│   ├── index.rs / index.tsx       # Auth landing
│   ├── callback.rs / callback.tsx # OAuth callback
│   └── logout.rs / logout.tsx     # Logout
└── admin/
    ├── test.rs / test.tsx         # Admin test route
    └── middlewares.rs             # Admin route guards
```

### Middleware Pattern
Admin routes use `middlewares.rs` to enforce authentication/authorization before handling requests.

### State Management
- Server state managed in `state.rs`
- JWT handling in `jwt.rs`
- App configuration in `app.rs`

## Backend API Pattern (Axum)
- Axum routers with handlers
- OpenAPI documentation via `axum_openapi3`
- Sea-ORM for database operations
- Shared logic in `packages/lib`

## Database Pattern (Sea-ORM)
- Entities defined in `packages/entity`
- Migrations in `packages/migration`
- PostgreSQL as primary store
- SQLite used locally in `packages/api/db.sqlite` for development

## Authentication Pattern
- OAuth2/OIDC via Auth0
- JWT tokens exchanged after OAuth callback
- Middleware extracts and validates JWT on protected routes
- User permissions model (non-plural table name: `user_permission`)

## Image Processing Pipeline
- Upload → optimization (WebP conversion) → metadata stripping → storage
- Planned: OpenAI Moderation API for content classification

## Development Lifecycle Pattern
1. **Planning:** PRD Agent creates feature spec → GitHub Issue
2. **Implementation:** Developer Agent codes in feature branch → PR
3. **Review:** PR Review Agent validates against PRD → merge to `dev`
