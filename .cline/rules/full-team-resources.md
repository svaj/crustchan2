The entirety of team resources can be found locally in the following places:

(Check ~/.cline and ~/.github first, and then workspace .cline and .github)

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

## If you feel an additional tool, skill, MCP server, GraphQL access, a database, dedicated cloud task runners, etc. would make your tasks easier, cheaper, and better to deal with, do some research and let me know (directly or via Slack (Chris Svajlenka) or Discord (svajy / mealworm)).
