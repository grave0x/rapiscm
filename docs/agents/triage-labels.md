# Triage Labels

The five canonical triage states used by the `triage` skill when processing incoming issues.

| Role | Label | Meaning |
|------|-------|---------|
| Needs triage | `needs-triage` | Maintainer must evaluate — default state for new issues |
| Needs info | `needs-info` | Waiting on reporter for clarification or reproduction steps |
| Ready for agent | `ready-for-agent` | Fully specified — an AFK agent can pick this up with zero human context |
| Ready for human | `ready-for-human` | Needs a human to implement (complex, design-heavy, or agent-unfriendly) |
| Wontfix | `wontfix` | Will not be actioned — closed with explanation |

## How labels are stored

Since the issue tracker is local markdown (`.scratch/`), labels are stored as
YAML frontmatter in each issue file:

```yaml
---
labels: [needs-triage]
---
```

The `triage` skill reads/writes this frontmatter. For repos with GitHub remotes,
labels are also applied via `gh issue edit --add-label`.
