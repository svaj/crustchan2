# Product Context

## Why This Project Exists
Crustchan v2 is a complete rewrite of a previously unfinished imageboard project. The original crustchan stalled due to architectural decisions that locked it into AWS-specific infrastructure (DynamoDB, Lambda, etc.) and a Warp-based backend that became difficult to maintain.

## Problems Solved
1. **Infrastructure Lock-in:** Moving from AWS-specific services to portable, open-source-friendly stack (PostgreSQL, Redis, Azure-compatible).
2. **Backend Complexity:** Replacing Warp with Axum via Tuono for a more coherent API contract and better middleware support.
3. **Database Limitations:** Replacing DynamoDB with PostgreSQL for relational data and better query flexibility.
4. **Frontend Modernization:** Introducing SSR React via Tuono instead of a separate SPA.
5. **Development Velocity:** Using AI agents to assist with development while maintaining quality control through PR review processes.

## How It Should Work
1. Users visit the site and see a list of boards
2. Users can view threads within boards
3. Anonymous users can post in anonymous-only boards
4. OAuth-registered users can post in mixed or registered-only boards
5. Images are automatically optimized and converted to WebP
6. Admins moderate content via an admin UI
7. Real-time updates push new posts to open threads (planned)

## User Experience Goals
- **Dense & Fast:** Imageboards are about consuming content quickly. No marketing fluff.
- **Familiar:** 4chan-style interaction patterns for users coming from existing imageboards.
- **Secure:** OAuth login, image moderation, metadata stripping from uploads.
- **Responsive:** Works on desktop and mobile without a separate mobile app.

## Key Differentiators from crustchan v1
| Aspect | v1 | v2 (this project) |
|--------|-----|-------------------|
| Backend Framework | Warp | Axum + Tuono |
| Database | DynamoDB | PostgreSQL |
| Cache/Events | AWS-specific | Redis |
| Frontend | Separate SPA | SSR React via Tuono |
| Auth | None / Custom | OAuth2/OIDC via Auth0 |
| Image Processing | Basic | WebP + metadata stripping |
| Infrastructure | AWS Free Tier | Azure + personal server |
| Moderation | Manual | OpenAI Moderation API |

## Development Approach
The project uses AI agents as development assistants. Features go through a documented lifecycle:
1. **Planning:** PRD creation via Planner Agent
2. **Implementation:** Code by Developer Agents (Frontend/Backend)
3. **Review:** Automated PR review for quality/compliance

This is for learning purposes — the author is committed to not allowing "slop" through PRs.
