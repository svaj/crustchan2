# Instructions for General Copilot Usage

## Key Conventions

### Documentation Structure
- **`.github/instructions/`** - Detailed best practices for specific tools and languages (Rust, Docker, GitHub Actions, Markdown)
- **`.github/agents/`** - Specialized agent configurations for different roles (React engineer, GitHub Actions expert, etc.)
- **`.cursor/agents/`** - Specialized agent configurations for different roles (React engineer, GitHub Actions expert, etc.) (cursor user specific)
- **`.github/skills/`** - Reusable skills for agents (PRD generation, agent configuration)- **
- **`.cursor/skills/`** - Reusable skills for agents (PRD generation, agent configuration) (cursor user specific)
- **`AGENTS.md`** - High-level tech stack and planned project structure, intended for all agents to understand the overall architecture and how their work fits into the bigger picture
- **`README.md`** - Project overview and vision
- **`.clineignore.md`** - Files and directories to ignore for any agent!  Just ignore the paths in this file.

### Rust Development
- Follow conventions in `.cline/rules/engineers/backend/rust.instructions.md`
- Use `cargo fmt` for formatting and `cargo clippy` for linting
- Implement comprehensive error handling with `Result<T, E>`
- Prefer borrowing over cloning; use `Arc<T>` for thread-safe shared ownership
- Write rustdoc comments (`///`) for all public APIs

### Frontend Development
- TypeScript with React for type safety
- Use ESLint/Prettier as configured by `.github/instructions/`
- Follow accessibility and React patterns detailed in agent instructions
- Use vitest for testing


### Markdown & Documentation
- Follow guidelines in `.cline/rules/markdown.instructions.md`
- All docs should have clear structure with H2/H3 headings
- Multi-page documentation should link together with relative links and live in the `docs/` directory or in the relevant service or agent directories
- Use code blocks with language specification
- Keep line length under 400 characters

### Docker & Containerization
- Multi-stage builds for efficiency
- Use minimal base images (alpine, slim, distroless)
- Prefer non-root users in containers if possible
- Only Traefik and Cline services should ever use the host network. All other services should not expose/define ports and should use overlay networking. This maintains security and avoids port conflicts on the host machine, and encourages proper service discovery and communication through Docker's internal DNS.
- If a service needs public exposure, it should be done via traefik deploy labels.
- See `.github/instructions/containerization-docker-best-practices.instructions.md`

### CI/CD & GitHub Actions
- Comprehensive guidance in `.github/instructions/github-actions-ci-cd-best-practices.instructions.md`
- Use least privilege for GITHUB_TOKEN permissions
- Leverage OIDC for cloud authentication (avoid long-lived secrets)
- Implement comprehensive testing strategy (unit, integration, E2E)

## Reference Documents

When working on specific areas, consult:

| Area | Document |
|------|----------|
| Rust best practices | `.cline/rules/engineers/backend/rust.instructions.md` |
| Docker/containerization | `.cline/rules/engineers/ci-cd/containerization-docker-best-practices.instructions.md` |
| GitHub Actions workflows | `.cline/rules/engineers/ci-cd/github-actions-ci-cd.instructions.md` |
| Product requirements | `.cline/agents/prd.agent.md` |
| React expertise | `.cline/rules/engineers/frontend/react.instructions.md` |
| Tailwind expertise | `.cline/rules/engineers/frontend/tailwind.instructions.md` |
| TypeScript expertise | `.cline/rules/engineers/frontend/typescript.instructions.md` |
| PR Review | `.cline/rules/engineers/pr-review/review.instructions.md` |
| AI Enablement | `.cline/rules/ai-enablement.instructions.md` |
| Engineer workflow | `.cline/rules/engineer-workflow.instructions.md` |
| General best practices | `.cline/rules/general-best-practices.md` |
| Markdown documentation | `.cline/rules/markdown.instructions.md` |
| Sync agent | `.cline/rules/sync-agent.instructions.md`

## Development Workflow

1. **Create issues first** - All work should be tracked in GitHub issues
2. **Follow naming conventions** - Check existing code for patterns (see Rust and Markdown instructions)
3. **Run tests before committing** - Both Rust (`cargo test`) and frontend (`vitest`)
4. **Document public APIs** - Use rustdoc for Rust, JSDoc for TypeScript
5. **Use specialized agents** - Leverage `.cline/agents/` when working on specific domains

## Important Notes

- **Early stage**: Code may be incomplete or non-functional
- **AI-assisted development**: This project uses AI agents intentionally; review and test their outputs carefully
- **No AWS**: Infrastructure should be cloud-agnostic (deploy to Docker swarm, maybe support k8s and/or helm charts, with terraform a low priority) With first attempt at deploying into docker swarm via github workflows.
- **Avoid slop**: Maintain code quality standards despite automated assistance
