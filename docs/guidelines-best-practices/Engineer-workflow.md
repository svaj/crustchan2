---
description: 'Engineer workflow guidelines'

---

# Engineer workflow guidelines

## General outline:
 1. Find issues/subissues assigned to you and have a status of  `Ready for Implementation`, `In Progress` or `PR Refinement` and have a label that designates you as assigned.  It will be formatted like `Assignee: frontend-engineer`, denoting that the issue is for the frontend-engineer to work on.
  - Note: There may be multiple engineer agents assigned to an issue, the parts that are relevant to you should be noted in the issue description.
 2. Analyze the the issue/subissue and comments on the issue and any associated pull request and their comments.
 3. Update the issue/subissue status to `In Progress` if not already.
 3. Fetch and checkout the `develop` git branch of the repository
 4. Create a new git branch (naming guidelines below) for your implementation and check it out, or, if a pull request already exists, checkout that associated branch (**not develop**).
 5. Analyze the source code to determine how to implement the issue 
 6. Make modifications to code for your implementation, committing using the commits guidelines below.
 7. Ensure tests have been added/updated for any new code, or modifications.
 8. Ensure all tests pass
 9. Run any linting (cargo rust-fmt or npm run lint) and fix any warnings or errors that may present.
 10. Verify you have implemented the feature in the application.
 11. Add or modify documentation in the repository and github wiki.
 11. Create a pull request (guidelines below).
 12. Update the associated issue's status to `Ready for Review` and set the `Current Agent` field to `PR Review`

## Branching
 **Unless you are a PRD agent, you must first create a new git branch to make any modifications in, then create a worktree to work in from that branch.**  Almost exclusively the branch it should originate from is `develop`.  Only certain circumstances, such as hotfixes, may the origininating branch be different.
 - Branch naming convention: Use the format `CRSTCHN-[github issue number]/[one of the following: "feature","fix","chore","ci","task","test","other"]-[issue or subisssue title formatted in kebab case]-agent[agent-type]`.  Replace portions in brackets `[]`, with actual values from the associated github issue, and your agent engineer type (frontend-engineer, ci-cd-engineer, etc.)

## Commits

### Commit size: 
  - keep individual commit sizes relatively low, (below 1000 additions or 1000 removals (lockfiles excluded)) if possible.
  - Add new functionality that doesn't modify any existing code in it's own commits if possible.
  - Add big feature functionality in separate commits from tests for that feature.
  - Add remaining modifications in commits as needed.
  - Small commits combining a test and a modification for a fix are okay to be one commit if small (less than 200 line changes)
### Commit messages:
  - Always refer to the github issue by suffixing the first line of your commit message with `ref #[Issue number]` replacing `[Issue number]` with the associated github issue number.
  - **NEVER** use the closing keywords (close, closes, closed, fix, fixes, fixed, resolve, resolves, resolved) in your commit messages followed by `#[Issue number]` (replaced with an actual issue number).  We have a process to follow to close and resolve issues after safely reviewing, testing, deploying, and verifying them first.
  - Keep messages short and descriptive of the changes done in the commit, one to 10 lines of text is fine.  The first line should always be a summary of the total changes.
  - Add a final line of the message to indicate what agent created the commit, and some information as to how long the changes for the individual commit took, and pricing information if available E.g. (`Created by Backend Engineer Agent in 5.02 seconds using model moonshotai/Kimi-K2.5, using 1000 tokens, costing $1.02`).

## Pull Requests
  When an issue's implementation is complete on a git branch, the application builds fine, passes tests and has undergone initial engineer review, then the engineering agent may create a github pull request from their branch to apply to the `develop` branch.  Use the [pull request template](/.github/pull_request_template.md) and fill out its sections appropriately when creating the Pull request.


 
