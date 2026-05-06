# List Projects

Returns all projects with id, name, slugId, and associated team keys.

Write the following to a file and run `./linear-skill query --query-file <path>`:

```graphql
{ projects { nodes { id name slugId teams { nodes { key } } } } }
```
