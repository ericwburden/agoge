# SDLC Role Recommendations

## Purpose

Capture a research-backed recommendation for which reusable SDLC roles this repository should define next after the [`Business Analyst`](D:/Projects/agoge/roles/business-analyst.md) and [`Role Builder`](D:/Projects/agoge/roles/role-builder.md) packages.

This note emphasizes patterns observed in other agentic systems and then translates those patterns into a pragmatic role roadmap for this repository.

## Recommendation Summary

The next role to build should be `Solution Architect`.

That is the highest-leverage next step because it sits directly after Business Analyst work in the SDLC chain:

- Business Analyst clarifies the business problem, process needs, and verified requirements.
- Solution Architect should turn that validated problem space into a solution shape:
  - system boundaries
  - component responsibilities
  - integration points
  - data and control flow
  - architectural constraints
  - technical risks
  - decision framing for downstream planning and implementation

This is the next major ambiguity-reduction step before execution planning or coding.

## Recommended Role Order

### 1. Solution Architect

Build this next.

Why:

- It is the cleanest downstream consumer of the current Business Analyst artifact chain.
- It turns validated requirements into system design without collapsing into implementation.
- It is strongly supported by role patterns in agentic software-company systems.

Expected job:

- consume BA outputs
- define solution structure and boundaries
- identify architectural options and tradeoffs
- make technical constraints explicit
- produce architecture-ready handoff artifacts for planning and build roles

### 2. Technical Planner

Build this after Solution Architect.

Why:

- Once architecture exists, the next ambiguity is execution structure.
- This role should convert architecture plus requirements into workstreams, sequencing, dependencies, and implementation slices.
- It should remain distinct from the architect role so design and execution planning do not blur.

Expected job:

- turn approved solution direction into an execution-ready plan
- break work into technically coherent slices
- identify dependencies, spikes, and sequencing constraints
- prepare implementation roles without redefining requirements or architecture

### 3. QA / Verification Lead

Build this before or alongside implementation-heavy roles.

Why:

- Agentic systems often under-specify verification relative to generation.
- This repo already values checks and traceability, so a dedicated verification role would strengthen the SDLC materially.
- This role would close the loop between requirements, architecture, implementation, and evidence of correctness.

Expected job:

- define verification strategy
- connect test evidence back to requirements and architecture
- identify validation gaps
- decide whether outputs are actually ready for downstream release or handoff

### 4. Implementation Engineer

Build this after architecture and planning are explicit.

Why:

- The execution role is important, but lower leverage than architect/planner/verification if the upstream structure is still forming.
- This role should be narrow and disciplined rather than acting as a catch-all software agent.

Expected job:

- implement against approved requirements, architecture, and plan
- surface execution blockers
- avoid silently redefining requirements or system design

### 5. Code Reviewer

Build this as a distinct role, not as a responsibility folded into implementation.

Why:

- Review is a separate quality function with a different mindset.
- This repo already treats review as a first-class pattern.
- A dedicated reviewer role would improve repeatability of risk-finding and change validation.

Expected job:

- review implementation outputs for defects, regressions, and missing validation
- check conformance to architecture and requirements
- keep findings distinct from implementation ownership

### 6. Release / Handoff Manager

Build this after the core design/build/verify chain is in place.

Why:

- End-to-end SDLC reuse eventually needs a clean release and downstream handoff stage.
- This becomes more valuable after upstream role packages exist.

Expected job:

- package validated outputs for release or adoption
- capture operational caveats, rollout constraints, and final downstream instructions
- avoid redefining requirements, architecture, or implementation details

## Why Solution Architect Should Be Next

This recommendation is supported both by the current repo state and by external agentic-system patterns.

From the current repo:

- The Business Analyst role already produces requirements and process-grounded handoff artifacts.
- The next missing stage is the role that consumes those outputs and defines the solution structure.
- Without an architect role, planning and implementation roles would be forced to make design decisions implicitly.

From other agentic systems:

- `MetaGPT` explicitly places an architect between requirement work and engineering work. Its README describes a software-company workflow with product managers, architects, project managers, and engineers, and its architect role is defined around producing system design output rather than code implementation. Sources:
  - [MetaGPT README](https://raw.githubusercontent.com/FoundationAgents/MetaGPT/main/README.md)
  - [MetaGPT architect role](https://raw.githubusercontent.com/FoundationAgents/MetaGPT/main/metagpt/roles/architect.py)
  - [MetaGPT product manager role](https://raw.githubusercontent.com/FoundationAgents/MetaGPT/main/metagpt/roles/product_manager.py)
  - [MetaGPT QA engineer role](https://raw.githubusercontent.com/FoundationAgents/MetaGPT/main/metagpt/roles/qa_engineer.py)
- `ChatDev` models software development as a virtual company with differentiated roles such as CEO, CPO, CTO, programmer, reviewer, and tester. The CTO role is effectively the nearest analogue to an architecture/design authority. Source:
  - [ChatDev repository](https://github.com/OpenBMB/ChatDev)

The common pattern is consistent:

- requirements-oriented role first
- architecture/design role next
- execution roles after that
- verification/review roles as separate quality functions

## Patterns Observed In Other Agentic Systems

### Pattern 1: Strong role specialization improves orchestration

`MetaGPT` and `ChatDev` both model software work as differentiated roles rather than one general agent. Their role sets are not identical, but both separate requirement work, design work, implementation, and verification.

Implication for this repo:

- keep roles narrow
- do not build one oversized "software engineer" role that absorbs planning, design, verification, and release

### Pattern 2: Roles need explicit routing descriptions, not just rich internal prompts

`CrewAI` makes `role`, `goal`, and `backstory` required agent attributes, and `AutoGen` explicitly argues that short agent descriptions improve orchestration because the orchestrator needs to know when to call on each role.

Sources:

- [CrewAI agents](https://docs.crewai.com/en/concepts/agents)
- [AutoGen: All About Agent Descriptions](https://microsoft.github.io/autogen/0.2/blog/2023/12/29/AgentDescriptions/)

Implication for this repo:

- each role should keep a crisp job-to-be-done
- workflows should make routing conditions explicit
- role packages should emphasize when the role should be used, not only how it should behave internally

### Pattern 3: Human-in-the-loop checkpoints remain important in multi-agent workflows

Microsoft's `spec-to-agents` sample uses specialized agents, a coordinator, structured routing, and explicit human feedback/approval points.

Source:

- [microsoft/spec-to-agents](https://github.com/microsoft/spec-to-agents)

Implication for this repo:

- roles should be allowed to stop and request confirmation at meaningful uncertainty boundaries
- architect, planner, and release-oriented roles should likely include explicit approval or decision checkpoints

### Pattern 4: Structured outputs are central to role handoffs

Across these systems, agents are not only differentiated by persona but by output responsibility:

- PRD or research
- design or architecture
- task/plan decomposition
- code
- tests
- review or approval

Implication for this repo:

- each new role should be built from artifact responsibilities first
- workflows and checks should be derived from those artifacts
- role packages should favor explicit handoff artifacts over implicit chat-state transitions

## Recommended Next Role Definitions

These are the highest-value concrete role candidates for this repo after Business Analyst.

### Solution Architect

Primary downstream input:

- BA business objectives
- BA process analysis
- BA requirements specification
- BA requirements handoff

Primary outputs likely needed:

- solution architecture brief
- system context and boundary definition
- integration and dependency map
- architectural decision record or tradeoff summary
- architecture handoff to planning and implementation

### Technical Planner

Primary downstream input:

- architecture outputs
- verified requirements

Primary outputs likely needed:

- implementation strategy
- workstream map
- sequencing and dependency plan
- spike and risk treatment plan
- implementation handoff

### QA / Verification Lead

Primary downstream input:

- requirements
- architecture
- implementation or test artifacts

Primary outputs likely needed:

- verification strategy
- requirement-to-test traceability
- evidence review
- release-readiness or verification handoff

## Roles To Defer For Now

These roles are useful, but should not come before Solution Architect.

### Product Owner

Useful later, but likely overlaps too much with Business Analyst until prioritization and backlog governance become first-class in this repo.

### Delivery Manager / Project Manager

Useful later, but lower leverage right now than architecture, planning, and verification.

### Security / Compliance Specialist

Important in some contexts, but not the next universal role unless this repository shifts toward regulated or high-assurance delivery as a primary goal.

## Practical Recommendation For This Repository

The next implementation sequence should be:

1. `Solution Architect`
2. `Technical Planner`
3. `QA / Verification Lead`
4. `Implementation Engineer`
5. `Code Reviewer`
6. `Release / Handoff Manager`

This sequence follows the strongest recurring pattern observed in agentic software systems:

- requirement discovery
- architecture and design
- execution planning
- implementation
- verification and review
- downstream release or handoff

## Sources

- [MetaGPT README](https://raw.githubusercontent.com/FoundationAgents/MetaGPT/main/README.md)
- [MetaGPT architect role](https://raw.githubusercontent.com/FoundationAgents/MetaGPT/main/metagpt/roles/architect.py)
- [MetaGPT product manager role](https://raw.githubusercontent.com/FoundationAgents/MetaGPT/main/metagpt/roles/product_manager.py)
- [MetaGPT QA engineer role](https://raw.githubusercontent.com/FoundationAgents/MetaGPT/main/metagpt/roles/qa_engineer.py)
- [ChatDev repository](https://github.com/OpenBMB/ChatDev)
- [CrewAI agents](https://docs.crewai.com/en/concepts/agents)
- [AutoGen: Agent and Multi-Agent Applications](https://microsoft.github.io/autogen/stable/user-guide/core-user-guide/core-concepts/agent-and-multi-agent-application.html)
- [AutoGen: All About Agent Descriptions](https://microsoft.github.io/autogen/0.2/blog/2023/12/29/AgentDescriptions/)
- [Microsoft spec-to-agents sample](https://github.com/microsoft/spec-to-agents)
