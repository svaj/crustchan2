# Instructions for Crustchan

## Project Overview

**Crustchan** is a 4chan-style imageboard built for 2026 with a cloud-native architecture. This is an early-stage project (pre-release) that will serve as a successor to the original unfinished Crustchan.

**Current Status**: Planning and architectural phase. The codebase structure described in `AGENTS.md` reflects planned architecture, not current implementation. Expect incomplete, broken, or placeholder code.

## Architecture

### Stack
- **Backend**: Rust 1.93.1 (Axum framework with OpenAPI 3 support)
- **Frontend**: TypeScript, React, Tuono
- **Package Management**: pnpm (monorepo)
- **Orchestration**: Turborepo (planned)
- **Authentication**: OAuth2 + OIDC (Apple, Microsoft, Google, Twitch, Facebook)
- **Database**: PostgreSQL
- **Caching/Events**: Redis or KV store (potentially RabbitMQ for events)

### Planned Structure
The monorepo will contain:
- `apps/api/` - Rust backend using Axum
- `apps/frontend/` - React TypeScript frontend
- `apps/serverless/` - Rust serverless components
- `packages/` - Shared packages
- `infrastructure` - Anything to do with Infrastructure (terraform, etc).
- `tests/` - Integration and E2E tests
- `agents/` - Agent definition files
- `rules/` - Rules for AI Agents
- `skills` - Skills for AI Agents
- `docs/` - Documentation for the project, including guidelines for AI agents

### Key Design Decisions
- **RDBMS over NoSQL**: Postgres instead of DynamoDB for better transactional consistency
- **Cloud-Agnostic**: Avoids AWS-specific services; suitable for deployment to personal infrastructure
- **CQRS Pattern**: Read replicas or Redis for separating read/write concerns
- **Planned Future**: AI-assisted development, image optimization (WebP), admin/user UIs, OAuth2 sign-up

## Setup & Development

**Note**: The project is not currently set up for development. When infrastructure is in place, look for:

- `pnpm install` - Install dependencies (when pnpm-workspace.yaml is configured)
- `pnpm build` - Build all packages (when turbo.json is configured)
- `pnpm test` - Run all tests
- Backend tests: `cargo test` (from `apps/api/`)
- Frontend tests: `vitest` (from `apps/frontend/`)

## Key Conventions

### Documentation Structure
- **`docs/`** - Detailed best practices for specific tools and languages (Rust, Docker, GitHub Actions, Markdown)
- **`agents/`** - Specialized agent configurations for different roles (React engineer, GitHub Actions expert, etc.)
- **`skills/`** - Reusable skills for agents (PRD generation, agent configuration)
- **`AGENTS.md`** - High-level tech stack and planned project structure
- **`README.md`** - Project overview and vision

### Rust Development
- Follow conventions in `docs/guidelines-best-practices/Backend.md`
- Use `cargo fmt` for formatting and `cargo clippy` for linting
- Implement comprehensive error handling with `Result<T, E>`
- Prefer borrowing over cloning; use `Arc<T>` for thread-safe shared ownership
- Write rustdoc comments (`///`) for all public APIs

### Frontend Development
- TypeScript with React for type safety
- Use ESLint/Prettier
- Follow accessibility and React patterns detailed in agent instructions
- Use vitest for testing

### Markdown & Documentation
- Follow guidelines in `docs/guidelines-best-practices/markdown.instructions.md`
- All docs should have clear structure with H2/H3 headings
- Use code blocks with language specification
- Keep line length under 400 characters

### Docker & Containerization
- Multi-stage builds for efficiency
- Use minimal base images (alpine, slim, distroless)
- Non-root users in containers
- See `docs/guidelines-best-practices/ci-cd-docker.md`

### CI/CD & GitHub Actions
- Comprehensive guidance in `docs/guidelines-best-practices/ci-cd-github.md`
- Use least privilege for GITHUB_TOKEN permissions
- Leverage OIDC for cloud authentication (avoid long-lived secrets)
- Implement comprehensive testing strategy (unit, integration, E2E)

## Reference Documents

When working on specific areas, consult:

| Area | Document |
|------|----------|
| Rust best practices | `docs/guidelines-best-practices/Backend.md` |
| Docker/containerization | `docs/guidelines-best-practices/ci-cd-docker.md` |
| GitHub Actions workflows | `docs/guidelines-best-practices/ci-cd-github.md` |
| Markdown documentation | `docs/guidelines-best-practices/markdown.instructions.md` |
| Product requirements | `docs/guidelines-best-practices/prd.md` |
| React expertise | `docs/guidelines-best-practices/Frontend-React.md` |

## Development Workflow

1. **Create issues first** - All work should be tracked in GitHub issues
2. **Follow naming conventions** - Check existing code for patterns (see Rust and Markdown instructions)
3. **Run tests before committing** - Both Rust (`cargo test`) and frontend (`vitest`)
4. **Document public APIs** - Use rustdoc for Rust, JSDoc for TypeScript
5. **Use specialized agents** - Leverage `agents/` when working on specific domains

## Important Notes

- **Early stage**: Code may be incomplete or non-functional
- **AI-assisted development**: This project uses AI agents intentionally; review and test their outputs carefully
- **No AWS**: Infrastructure should be cloud-agnostic (deploy to personal servers, Portainer, etc.)
- **Avoid slop**: Maintain code quality standards despite automated assistance
