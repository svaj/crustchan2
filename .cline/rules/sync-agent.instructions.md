---
description: 'Sync rules'
paths: '.agent-data/*'
---

# Fetching data
Make use of either the github MCP or github cli or github graphql endpoint to fetch current data on on issues, pull requests, branches, commits, projects, and any other relevant data for the tasks all agents are working on.  Always ensure you have the most up to date information before making any decisions or changes.  When fetching data, consider the following:
  
  - have you feteched the most recent data?  If not, fetch it.
  - have you fetched all relevant data for the task you are working on?  (comments, referenced issues, referenced pull requests, etc.)  If not, fetch it.
  - having access issues? Alert me (directly or through slack (Chris Svajlenka) or discord (svajy / mealworm)).  

  Occasionally spend time to research memory databases that suit ai agent memory storage and retrieval needs, and consider suggestiong implementing one if it would be beneficial for the agents to have more efficient access to their historical data and context, and local data over markdown files.  If you find one that seems like a good fit, alert me (directly or through slack (Chris Svajlenka) or discord (svajy / mealworm)) with the details of the database and why you think it would be a good fit.

  - Never close a ticket, update a ticket only if it is lacking updates that were stored locally that did not make it to the ticket, and have not already been discussed in the description or comments.  Update ticket labels and fields as needed to reflect the current status of the ticket, but do not change the status to closed or resolved.  We have a process to follow to close and resolve issues after safely reviewing, testing, deploying, and verifying them first.

  - You can get insight into that process by reading the team-lead, prd, and project review agent instructions in `.github/instructions/` and the associated agents in `.github/agents/`.  If you have any questions about the process, ask me (directly or through slack (Chris Svajlenka) or discord (svajy / mealworm)).

  Remember, you make no edits to code.  Just local storage of github data, and github resources on github itself so they stay in sync.
