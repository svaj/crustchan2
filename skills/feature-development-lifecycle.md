---
title: Feature Development Lifecycle Management Skill
description: This skill orchestrates the end-to-end process of developing new features using GitHub resources (Issues, PRs) and dedicated AI agents. It defines the handoff sequence between planning, implementation, and review phases.
lifecycle_steps:
1. **PLANNING (Planner Agent):** Use `PRD.agent.md`. The agent takes a high-level requirement and drafts a comprehensive feature specification using the `ISSUE_TEMPLATE/feature_spec.md` template, creating a new GitHub Issue.
2. **IMPLEMENTATION (Developer Agent - either Frontend agent or backend agent or possibly both as subagents):** Once the issue is approved, use `developer.agent.md`. The agent reads the PRD from the Issue, creates a feature branch, writes the required code changes in the relevant packages/crates (`packages/api`, `packages/entity`, etc.), commits them, and submits a Pull Request against the `dev` development branch.
3. **REVIEW (PR Review Agent):** Automated review using `pr-review-agent.md`. The agent runs before merging to ensure quality, test coverage, security adherence, and full compliance with the initial PRD.

## Important Instructions
- This skill acts as the master process hook. All new features must pass through this documented lifecycle.
- **Hooks:** Implement automated checks in CI/CD pipelines (via `.github/workflows/`) to enforce this sequence.
---
