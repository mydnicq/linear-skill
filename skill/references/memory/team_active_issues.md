# Team Active Issues

Returns all non-Done/Canceled issues for a given team.

Write the following to a file and run `./linear-skill query --query-file <path>`:

```graphql
query { team(id: "TEAM_ID") { issues(filter: { state: { name: { nin: ["Done", "Canceled", "Cancelled"] } } }) { nodes { identifier title state { name } assignee { name } priority createdAt updatedAt } } } }
```

Replace `TEAM_ID` with the actual team UUID. Get team IDs from the `teams` query.
