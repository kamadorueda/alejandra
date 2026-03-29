# Vision Command Center

This branch is Alejandra's planning and coordination hub.
Agents pick work items from here and execute them on feature branches.

## Structure

```
vision/
├── README.md              ← You are here. Index and conventions.
├── product/               ← Product-level docs (strategy, positioning, research)
│   └── strategy.md        ← Overall vision & differentiation strategy
├── plans/                 ← Implementation plans (one per feature/fix)
│   └── NNN-slug.md        ← Numbered, self-contained, ready for an agent to execute
└── backlog/
    └── work-items.md      ← Prioritized queue of work items for agents
```

## How It Works

### Product Docs (`product/`)

High-level thinking that informs *what* to build and *why*.
These are living documents — updated as the landscape changes.

- `strategy.md` — Market positioning, differentiation, adoption strategy
- Future: `research/`, `rfcs/`, `competitive-analysis/` as needed

### Implementation Plans (`plans/`)

Each plan is a self-contained blueprint an agent can pick up and execute.

**Naming**: `NNN-slug.md` where NNN is a zero-padded sequence number.

**Required sections in every plan**:

```markdown
# Title

## Status
One of: `draft` | `ready` | `in-progress` | `done` | `blocked`

## Priority
One of: `P0-critical` | `P1-high` | `P2-medium` | `P3-low`

## Context
Why this work matters. Link to issues, product strategy, or user pain.

## Scope
What's in and what's out.

## Implementation
Step-by-step instructions an agent can follow autonomously.
Include file paths, function names, and concrete code changes.

## Branch
Target branch name for implementation (e.g., `feat/skip-formatting`).

## Verification
How to confirm the work is correct (tests, manual checks, CI).

## Dependencies
Other plans that must complete first (by number), or `none`.
```

### Backlog (`backlog/work-items.md`)

The prioritized queue. Agents scan this to find their next task.
Each item links to a plan in `plans/` once a plan exists.

## Conventions

### For humans adding plans

1. Write the plan in `plans/NNN-slug.md`
2. Add an entry to `backlog/work-items.md`
3. Set status to `draft` until the plan is ready for execution
4. Push to the `vision` branch

### For agents picking up work

1. Read `backlog/work-items.md` to find the highest-priority `ready` item
2. Read the linked plan in `plans/`
3. Create the feature branch specified in the plan
4. Execute the plan
5. Push the feature branch
6. Update the plan status to `done` and push to `vision`

### Priority Definitions

| Priority | Meaning | Guidance |
|---|---|---|
| **P0-critical** | Blocking users or eroding trust | Do immediately. Skip the queue. |
| **P1-high** | Key differentiator or top-requested feature | Next up after any P0s. |
| **P2-medium** | Improves product, not urgent | Work on when P0/P1 queue is empty. |
| **P3-low** | Nice to have, speculative, or low-impact | Opportunistic. |

### Numbering

- Plans are numbered sequentially: 001, 002, 003...
- Numbers are permanent — never reused, even if a plan is abandoned
- Gaps are fine (e.g., 001, 002, 005 if 003/004 were abandoned)
