# Solution Architect Skill Sourcing

## Purpose

Capture which local skills already support the [`Solution Architect`](D:/Projects/agoge/roles/solution-architect.md) role, which new repo-native skills were needed, and which external skill patterns are worth adapting rather than copying directly.

## Local Skill Support

### Useful As-Is

- [`meeting-notes-and-actions`](D:/Projects/agoge/skills/meeting-notes-and-actions/SKILL.md)
  - Useful for normalizing architecture workshop notes, discovery follow-ups, and design-session transcripts before architecture work begins.
- [`content-research-writer`](D:/Projects/agoge/skills/content-research-writer/SKILL.md)
  - Useful when architecture direction depends on external standards, patterns, or technology comparisons that need explicit sourcing.

### Useful With Existing Local-Markdown Positioning

- [`research-documentation`](D:/Projects/agoge/skills/research-documentation/SKILL.md)
  - Useful for synthesizing local technical constraints, design notes, and reference material before architecture design or review.
- [`spec-to-implementation`](D:/Projects/agoge/skills/spec-to-implementation/SKILL.md)
  - Useful only after architecture is mature and handed downstream. It is not a core Solution Architect skill.

## New Repo-Native Skills Added

- [`architecture-design`](D:/Projects/agoge/skills/architecture-design/SKILL.md)
  - Added because the role needed a dedicated default path for shaping boundaries, components, flows, interfaces, and decision records.
- [`architecture-review`](D:/Projects/agoge/skills/architecture-review/SKILL.md)
  - Added because the role needed a durable architecture review stage and artifact rather than a transient pass/fail judgment.
- [`architecture-handoff-packaging`](D:/Projects/agoge/skills/architecture-handoff-packaging/SKILL.md)
  - Added because the role needed a dedicated downstream packaging step distinct from design and review.

## External Skill Patterns Reviewed

### `aws-solution-architect`

Source:

- [AWS Solution Architect skill listing](https://agent-skills.md/skills/alirezarezvani/claude-skills/aws-solution-architect)

Useful pattern:

- start from explicit workload, scale, budget, team, and compliance constraints
- evaluate solution direction in terms of architecture drivers rather than tool enthusiasm
- keep operational concerns such as cost, reliability, and deployment implications visible

Adaptation decision:

- adapt the requirement-and-constraint framing
- do not import the AWS- or IaC-specific assumptions into this repo's general Solution Architect role

### `python-architecture-review`

Sources:

- [python-architecture-review skill listing](https://playbooks.com/skills/rknall/claude-skills/python-architecture-review)
- [rknall/claude-skills repository](https://github.com/rknall/claude-skills)

Useful pattern:

- review architecture with a structured checklist
- make scalability, security, deployment, database, and API concerns explicit review dimensions
- produce actionable review feedback rather than generic approval

Adaptation decision:

- adapt the checklist and review-readiness posture
- do not import the Python-backend specialization into this repo's repo-neutral role package

### `architecture-review`

Source:

- [architecture-review skill listing](https://claude-plugins.dev/skills/%40codenamev/ai-software-architect/architecture-review)

Useful pattern:

- create a durable review document rather than relying on a transient discussion
- summarize findings, recommendations, and follow-up work explicitly
- treat review as a stage in the lifecycle, not an informal aside

Adaptation decision:

- adapt the durable review-artifact idea
- do not import the multi-persona team framing or tool-specific storage assumptions

### `developer-kit`

Source:

- [giuseppe-trisciuoglio/developer-kit](https://github.com/giuseppe-trisciuoglio/developer-kit)

Useful pattern:

- architecture is treated as a first-class domain with dedicated review and design capabilities
- role-specific or domain-specific architecture tooling is separated from generic coding help

Adaptation decision:

- adapt the principle that architecture work deserves dedicated capabilities
- do not import stack-specific plugin complexity into this repo's lean role package

## Implementation Decision

For this repository, the right move is:

- keep the Solution Architect role repo-neutral
- use existing local synthesis and note-normalization skills where they already fit
- add dedicated architecture skills for design, review, and handoff
- treat Product Owner direction as an explicit optional upstream input when product posture materially shapes architecture, rather than adding a separate scenario-only architecture skill
- embed interface and contract coverage into the architecture artifacts and checks rather than adding a separate `integration-contracts` artifact or skill in v1

## Follow-Up Consideration

If future Solution Architect work becomes heavily integration-centric, the next likely addition would be a dedicated interface-framing or integration-contracts skill. For now, that concern is handled inside the design, review, and handoff stages.

The Project Planning hardening pass confirmed that no new Solution Architect skill was needed for scenario participation. The gap was upstream input clarity, not missing architecture method.
