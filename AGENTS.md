# Crustchan 2
A 4-chan style imageboard built for 2026

## Tech stack
* Monorepo: [moon](https://moonrepo.dev)
* Backend: Rust v1.93.1, axum, axum_openapi3,
* Database: postgresql
* Cache: Redis
* Serverless Components: Rust v1.93.1 
* Infrastructure: Terraform
* Frontend: Typescript, React
* Testing: cargo test, jest
* Authentication: Oauth2 and OIDC for authorization and authentication
  - supported providers: Apple, Microsoft, Google, Twitch, Facebook


## Reference Documentation

General Rules/instructions/guidelines may be found in `.cline/rules/**/*.md`.  The rules in `./cline/rules/*.md` apply to every agent.  Individual agents have subdirectories with rules that apply to them (such as `./cline/rules/engineers/backend/*.md`).  Many rules have a `paths` property that shows what files apply to it, if it has no `paths` property, it applies everywhere.

To note:
 * All Engineer agents should adhear to the `.cline/rules/engineers/*.md` instructions.
 * Frontend engineer agents should follow rules in `./cline/rules/engineers/frontend/*.md`
 * Backend engineer agents should also explicitly follow rules in `./cline/rules/engineers/backend/*.md`
 * CI-CD engineer agents should follow rules in `./cline/rules/engineers/ci-cd/*.md`
 * PR review agents should analyze the standards and guidelines of all rules defined to determine if a pull request is following our standards.
