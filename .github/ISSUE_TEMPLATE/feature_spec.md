---
title: [Feature Name]: New Feature Spec.
labels: feature-spec, planning
assignees: @[Planner Agent]
---

## 🚀 Feature Overview
*Briefly describe the problem this feature solves and its ultimate goal.*

## ✨ Detailed Plan (The "How")
### 1. Functional Requirements
- [ ] Requirement 1: Detail what must happen functionally.
- [ ] Requirement 2: Specify edge cases and necessary constraints.

### 2. Technical Design Decisions
*Describe the architectural changes, API endpoints affected, or models that need updating.*

### 3. Implementation Steps (For PR Agent)
This section serves as a step-by-step guide for the implementation agent to follow when creating the Pull Request.  Ensure there's some level of detail for the developer agent to implement the feature accordingly.  Leave highly technical details out, as the developer agent is more than capable of figuring those out.
1. *Step 1:* e.g., Update `packages/api/src/users/routes.rs` with new endpoints.
2. *Step 2:* e.g., Create migration scripts in `packages/migration/`.
3. *Step 3:* e.g., Write unit tests for the new logic in `packages/entity/`.

### 4. Risk Assessment
1. [ ] Determine how much of the current code base this change will modify.  If it is just additional there is minimal risk.  If it involves modifying many functions/files across many packages/crates it is high risk.  Rate the level of change/risk of introducing regressions/bugs on a scale of 1-100 where 1 is low and 100 is high risk.  Call out any specific risks worth mentioning.

## ✅ Review Checklist (For Review Agent)
- [ ] Does this change introduce any regressions?
- [ ] Are all necessary tests written and passing?
- [ ] Is documentation updated accordingly?
---