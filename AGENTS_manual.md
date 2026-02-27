# Crustchan 2
A 4-chan style imageboard built for 2026

## Tech stack
* Monorepo: turborepo
* Backend: Rust v1.93.1, axum, axum_openapi3,
* Serverless Components: Rust v1.93.1 
* Frontend: Typescript, React
* Testing: cargo test, jest
* Authentication: Oauth2 and OIDC for authorization and authentication
  - supported providers: Apple, Microsoft, Google, Twitch, Facebook

## Project Structure

```
/
├── apps/                     # contains indivudal projects of the monorepo
│   ├── api/                  # Backend API project
│   │   ├── main.py           # FastAPI entry point
│   │   ├── database.py       # SQLite connection
│   │   ├── models.py         # SQLAlchemy models
│   │   ├── schemas.py        # Pydantic schemas
│   │   ├── logging_config.py # structlog configuration
│   │   └── routers/          # API endpoints
│   └── pyproject.toml
|   ├── frontend/                 # Frontend API project
│   |   ├── src/                  # Frontend source files
│   |   │   ├── components/       # React components
│   |   │   ├── features/         # Feature modules
│   |   │   ├── api/              # API client
│   |   │   └── App.jsx
│   |   └── package.json
├── packages/                 # Shared packages used amongst individual projects
│   ├── typescript-eslint/                  # Frontend source files
│   |   ├── src/                  # Package's source files
|   └── └── package.json          # package's package.json
├── tests/
│   ├── integration/          # API integration tests
│   └── e2e/                  # E2E tests
├── .github/                  # Github specific files
│   ├── copilot-instructions.md # Instructions for Copilot 
│   ├── CODE_OF_CONDUCT.md    # Instructions for engaging with this community
│   ├── CONTRIBUTING.md       # Instructions for contributing
│   ├── PULL_REQUEST_TEMPLATE.md # Template for new PRs
│   ├── PRD.md                # Product Requirement Documentation
│   ├── agents/               # specific instructions for Agents
│   ├── instructions/         # specific instructions for Agents around various topics
│   ├── prompts/              # user-defined commands for Agents
│   ├── skills/               # user-defined skills for Agents
│   ├── reference/            # Best practices docs for Agents
│   ├── workflows/            # Github action workflows
│   └── ISSUE_TEMPLATE/       # Github issue templates\
├── .gitignore                # Files to ignore from git
├── README.md                 # Main repository README
├── meta.json                 # Turborepo meta file
├── package.json              # Repository-wide package.json
├── pnpm-lock.yaml            # Current dependency lock file
├── pnpm-workspace.yaml       # Monorepo workspace configuration
└── turbo.json                # Turborepo configuration file
```


## Reference Documentation

Read these documents when working on specific areas:

| Document | When to Read |
|----------|--------------|
| `.github/PRD.md` | Understanding requirements, features, API spec |
