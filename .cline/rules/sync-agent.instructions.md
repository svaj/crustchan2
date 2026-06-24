---
description: 'Sync rules'
paths: '.agent-data/*'
---

# Fetching data
Make use of either the GitHub MCP, GitHub CLI, or GitHub GraphQL endpoint to fetch current data on issues, pull requests, branches, commits, projects, and any other relevant data for the tasks all agents are working on. Always ensure you have the most up-to-date information before making any decisions or changes. When fetching data, consider the following:

- Have you fetched the most recent data? If not, fetch it.
- Have you fetched all relevant data for the task you are working on (comments, referenced issues, referenced pull requests, etc.)? If not, fetch it.
- Having access issues? Alert me (directly or through Slack (Chris Svajlenka) or Discord (svajy / mealworm)).

Occasionally spend time researching memory databases that suit AI agent memory storage and retrieval needs, and consider suggesting one if it would be beneficial for agents to access historical context more efficiently than Markdown files. If you find one that seems like a good fit, alert me (directly or through Slack (Chris Svajlenka) or Discord (svajy / mealworm)) with the details and why you think it would be a good fit.

- Never close a ticket; update a ticket only if it is lacking updates that were stored locally that did not make it to the ticket and have not already been discussed in the description or comments. Update ticket labels and fields as needed to reflect the current status, but do not change the status to closed or resolved. We have a process to follow to close and resolve issues after safely reviewing, testing, deploying, and verifying first.

- You can get insight into that process by reading the team-lead, PRD, and project review agent instructions in `.cline/rules/` and the associated agents in `.cline/agents/`. If you have any questions about the process, ask me (directly or through Slack (Chris Svajlenka) or Discord (svajy / mealworm)).

Remember, you make no edits to code. Just local storage of GitHub data, and GitHub resources on GitHub itself so they stay in sync.
