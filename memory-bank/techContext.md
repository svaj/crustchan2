# Tech Context

## Technologies Used

### Backend
| Technology | Version | Purpose |
|------------|---------|---------|
| Rust | 1.93.1 | Primary backend language |
| Axum | (latest) | Web framework for API |
| axum_openapi3 | (latest) | OpenAPI documentation generation |
| Sea-ORM | (latest) | ORM for PostgreSQL |
| Tokio | (latest) | Async runtime |
| anyhow | 1.0.102 | Error handling |
| chrono | 0.4.44 | Date/time handling |
| serde | 1.0.228 | Serialization |
| serde_json | 1.0.149 | JSON serialization |
| uuid | 1.21.0 | UUID generation (v7) |

### Frontend
| Technology | Version | Purpose |
|------------|---------|---------|
| TypeScript | 5.6.3 | Type-safe JavaScript |
| React | 19.0.0 | UI library |
| React DOM | 19.0.0 | DOM rendering |
| Tuono | 0.19.7 | Rust-based SSR framework for React |
| Tailwind CSS | (latest) | Utility-first styling |
| Vite | (latest) | Build tool |
| Vitest | 4.1.7 | Unit testing |
| @testing-library/react | 16.3.2 | React component testing |
| @testing-library/jest-dom | 6.9.1 | DOM assertions |
| @testing-library/user-event | 14.6.1 | User interaction simulation |
| jsdom | 29.1.1 | DOM environment for tests |

### Database & Cache
| Technology | Purpose |
|------------|---------|
| PostgreSQL | Primary relational database |
| Redis | Caching and event pub/sub |
| Sea-ORM | Database ORM and migrations |

### Infrastructure
| Technology | Purpose |
|------------|---------|
| Terraform | Infrastructure as Code |
| GitHub Actions | CI/CD pipelines |
| moon | Monorepo task runner |
| Azure | Primary cloud target |

### Authentication
| Technology | Purpose |
|------------|---------|
| Auth0 | OAuth2/OIDC provider |
| JWT | Token-based session management |

### External Services
| Service | Purpose |
|---------|---------|
| OpenAI Moderation API | Content moderation for images |

## Development Setup

### Prerequisites
- Rust 1.93.1+ (via rustup)
- Node.js + npm (for frontend)
- PostgreSQL (local or Docker)
- Redis (local or Docker)
- moon (`npm install -g @moonrepo/cli`)

### Build Commands
```bash
# Rust formatting & linting
cargo fmt
cargo clippy

# Rust tests
cargo test

# Frontend tests
cd packages/frontend && npm test        # vitest run
cd packages/frontend && npm run test:watch   # vitest
```

### Workspace Dependencies
Shared dependencies are pinned in the root `Cargo.toml`:
- `anyway = "1.0.102"`
- `chrono = "0.4.44"`
- `serde = "1.0.228"` (with derive feature)
- `serde_json = "1.0.149"`
- `uuid = "1.21.0"` (with serde, v7 features)

## Technical Constraints
1. **Tuono Fork Dependency:** Using a custom fork (`svaj/tuono/tree/middleware-and-extractors`). This must be built from source or referenced via git dependency. Upstream compatibility is not guaranteed.
2. **Package Name Mismatch:** Workspace `Cargo.toml` lists `frontend-ng` but directory is `frontend`. This will break cargo workspace builds until fixed.
3. **Early Stage:** Many packages have minimal implementation. The entity crate may not have complete models yet.
4. **Local Dev DB:** `packages/api/db.sqlite` exists but the target DB is PostgreSQL. Migrations may need to be run against a real PostgreSQL instance.

## Tool Usage Patterns
- **moon:** Task runner for the monorepo. Each package can define tasks in `moon.yml`.
- **cargo:** Rust package manager and build tool.
- **npm:** Frontend package manager.
- **GitHub Issues:** Feature tracking and PRD references.
- **GitHub Actions:** Automated testing and deployment.
