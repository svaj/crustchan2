# Progress

## What Works

### ✅ Frontend Scaffolding
- Tuono + React 19 + TypeScript project set up in `packages/frontend`
- File-based routing structure established with routes for:
  - Home (`/`)
  - Board list (`/boards`)
  - Individual board (`/boards/[boardid]`)
  - User pages (`/users`)
  - Auth flow (`/auth`, `/auth/callback`, `/auth/logout`)
  - Admin test route (`/admin/test`)
- Layout system via `__layout.tsx`
- Component library started (`Greeting.tsx`)
- Vitest test harness configured with Testing Library
- Global CSS/Tailwind set up

### ✅ Auth Infrastructure
- Tuono middleware support implemented via custom fork
- JWT handling module (`jwt.rs`)
- Auth0 callback routes established
- Login/logout flow wired up
- Admin route middleware pattern established

### ✅ Workspace Configuration
- Root `Cargo.toml` with shared dependencies pinned
- `packages/` structure defined for all crates
- `moon` monorepo tool configured

### ✅ Agent Tooling
- Agent role prompts defined (`agents/`)
- Skill definitions created (`skills/`)
- PRD workflow documented
- Feature development lifecycle skill defined
- Memory bank structure established

### ✅ Documentation
- `AGENTS.md` with comprehensive instructions
- `docs/guidelines-best-practices/` with detailed guidelines for:
  - Backend development
  - Frontend development
  - React patterns
  - Tailwind + TypeScript
  - CI/CD (GitHub Actions, Docker)
  - General best practices
  - Engineer workflow

## What's Left to Build

### 🔧 Critical Fixes
- [ ] **Fix workspace `Cargo.toml`:** Member `frontend-ng` should be `frontend`
- [ ] **Ensure all packages build:** Needs verification after name fix

### 🏗 Core Backend (API)
- [ ] Complete API route handlers for:
  - [ ] Board CRUD operations
  - [ ] Thread CRUD operations
  - [ ] Post CRUD operations
  - [ ] Image upload endpoint
- [ ] Connect API to database via Sea-ORM entities
- [ ] Implement proper error handling and logging
- [ ] Add OpenAPI documentation

### 🗄 Database
- [ ] Define complete entity models:
  - [ ] `Board` — board definitions
  - [ ] `Thread` — thread within a board
  - [ ] `Post` — individual posts
  - [ ] `Image` — uploaded image metadata
  - [ ] `User` — OAuth user profiles
  - [ ] `UserPermission` — role-based permissions
- [ ] Create migrations for all entities
- [ ] Seed data for development

### 🖼 Image Processing
- [ ] Image upload endpoint
- [ ] WebP conversion logic
- [ ] Metadata stripping (EXIF, geolocation)
- [ ] Thumbnail generation
- [ ] Storage backend (local/Azure Blob)

### 🎨 Frontend Features
- [ ] Replace mocked boardlist with real API data
- [ ] Thread view page
- [ ] Post creation form
- [ ] Image upload in post form
- [ ] User profile page
- [ ] Admin dashboard UI
- [ ] Responsive design implementation
- [ ] Dark/light mode toggle

### 🔐 Auth Completion
- [ ] Proper JWT session management (cookies/localStorage)
- [ ] Role-based route guards on frontend
- [ ] Admin middleware enforcement
- [ ] User profile data display

### 🧪 Testing
- [ ] Frontend: tests for all components
- [ ] Frontend: tests for routes/pages
- [ ] Backend: unit tests for handlers
- [ ] Backend: integration tests for API
- [ ] End-to-end testing strategy

### 🚀 CI/CD & Infrastructure
- [ ] GitHub Actions workflow for Rust builds/tests
- [ ] GitHub Actions workflow for frontend builds/tests
- [ ] Terraform modules for infrastructure
- [ ] Docker configuration for services
- [ ] Deployment pipeline to Azure

### 🤖 AI Moderation
- [ ] OpenAI Moderation API integration
- [ ] Image classification pipeline
- [ ] Automated moderation actions
- [ ] Admin moderation queue UI

### ⚡ Real-time Updates (Future)
- [ ] WebSocket or SSE implementation
- [ ] Thread watching functionality
- [ ] Live post updates

## Current Status Summary

| Area | Status | Notes |
|------|--------|-------|
| Frontend scaffolding | 🟢 Functional | Routes, layout, auth flow wired |
| Auth middleware | 🟡 In progress | Callbacks work; session persistence needs work |
| Backend API | 🔴 Minimal | SQLite dev DB exists; full endpoints missing |
| Database schema | 🔴 Early | Entity crate exists; models incomplete |
| Image processing | 🔴 Not started | Package exists; no logic yet |
| Tests | 🟡 Started | Vitest configured; only 1 component test |
| Infrastructure | 🔴 Not started | Terraform dir may be empty |
| CI/CD | 🔴 Not started | `.github/workflows` may need creation |
| Documentation | 🟢 Good | Guidelines and agent instructions comprehensive |

## Known Issues
1. Workspace `Cargo.toml` references non-existent `frontend-ng` package
2. Frontend boardlist is mocked — not connected to backend
3. API database uses SQLite for local dev but migrations may target PostgreSQL
4. Tuono fork dependency may complicate builds for new contributors
5. No CI/CD pipeline is currently configured
6. Very limited test coverage across the codebase
