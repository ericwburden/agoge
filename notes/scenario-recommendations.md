# Scenario Recommendations

## Purpose

Record a research-backed recommendation for which reusable multi-role scenarios this repository can support with the current role set and which scenario should be implemented first.

This note translates public delivery and orchestration patterns into a pragmatic scenario roadmap for this repository.

## Recommendation Summary

The first scenario to implement should be `Project Planning`.

That is the highest-leverage first scenario because it uses a strong portion of the existing role set, has clear public precedent, and creates a reusable bridge between validated discovery and execution-oriented roles.

## Recommended Initial Scenario Set

### 1. Project Planning

Implement this first.

Core roles:

- `Business Analyst`
- `Product Owner`
- `Solution Architect`
- `Technical Planner`
- optional `Security / Compliance Specialist`

Why:

- It is the clearest flagship scenario for the current package set.
- It turns discovery, product direction, architecture, and technical planning into one explicit pre-implementation phase.
- It fits the current repository especially well because these upstream planning roles are already strongly defined.

Expected job:

- clarify validated needs and scope
- establish product direction and priority posture
- turn direction into a reviewed solution shape
- turn reviewed architecture into an execution-ready planning package
- optionally bring in security/compliance when the planning phase must carry explicit control or obligation framing

### 2. Sprint Preparation / Sprint Readiness

Core roles:

- `Product Owner`
- `Solution Architect` or `Technical Planner`
- optional `QA / Verification Lead`

Why:

- This is a plausible planning-adjacent scenario for teams that need work to become slice-ready before implementation begins.
- It fits the current roles without requiring a dedicated Scrum Master or Delivery Manager package.

Expected job:

- refine current priority direction
- clarify acceptance-oriented guardrails
- make slice boundaries and dependencies explicit
- identify verification-sensitive readiness conditions

### 3. Delivery Slice Planning

Core roles:

- `Product Owner`
- `Solution Architect`
- `Technical Planner`
- optional `QA / Verification Lead`
- optional `Security / Compliance Specialist`

Why:

- This is the clearest bridge between broader project planning and bounded downstream delivery.
- It turns a reviewed project or initiative plan into one honest implementation-sized slice instead of forcing downstream roles to infer scope boundaries for themselves.
- It fits the current repository well because Product Owner, Solution Architect, and Technical Planner already cover the core slice-shaping concerns.

Expected job:

- choose the next bounded delivery slice from the broader plan
- make in-scope versus out-of-scope boundaries explicit
- preserve architecture, dependency, verification, and optional security/compliance constraints that still shape the slice
- hand one slice-sized package downstream into implementation-oriented work

### 4. Implementation and Release Prep

Core roles:

- `Implementation Engineer`
- `Code Reviewer`
- `QA / Verification Lead`
- `Release / Handoff Manager`

Why:

- This is one of the cleanest downstream scenarios in the current package set.
- It captures implementation, review, verification, and release packaging for one bounded delivery slice or release candidate as a reusable quality-gated sequence.
- The name makes it explicit that implementation is included while still keeping actual deployment out of scope.

Expected job:

- implement one bounded delivery slice against reviewed upstream direction
- review independently
- verify explicitly
- package that slice-sized candidate for release or adoption handling

### 5. Secure Delivery / Secure Feature Lifecycle

Core roles:

- `Business Analyst` or `Product Owner`
- `Solution Architect`
- `Security / Compliance Specialist`
- `Technical Planner`
- `Implementation Engineer`
- `QA / Verification Lead`
- `Release / Handoff Manager`

Why:

- The current repository already has a distinct security/compliance role, which makes secure delivery a natural scenario rather than an afterthought.
- This scenario is especially plausible for higher-risk or AI-sensitive work.

Expected job:

- carry security/compliance framing across planning and delivery
- keep controls, evidence, and approval-sensitive decisions explicit
- preserve secure-delivery constraints through release posture

### 6. Review Remediation Loop

Core roles:

- `Code Reviewer`
- `Implementation Engineer`
- optional `QA / Verification Lead`

Why:

- Review and re-review is a common repeatable scenario with strong public precedent.
- It is a natural narrow scenario for hardening a candidate after findings are raised.

Expected job:

- convert findings into explicit remediation work
- repackage changed implementation evidence
- re-evaluate review posture

### 7. Verification And Release Gate

Core roles:

- `QA / Verification Lead`
- `Security / Compliance Specialist`
- `Release / Handoff Manager`

Why:

- This is a plausible narrower gate scenario for higher-risk or externally sensitive changes.
- It uses the current verification, security/compliance, and release packaging roles without requiring additional delivery-management structure.

Expected job:

- confirm verification posture
- preserve security/compliance conditions
- package the final release or adoption gate honestly

### 8. Release Feedback To Reprioritization

Core roles:

- `Release / Handoff Manager`
- `Product Owner`
- optional `Business Analyst`

Why:

- This gives the current role set a clean adaptation loop after release or adoption work.
- It supports learning-driven reprioritization without requiring new roles.

Expected job:

- turn release or adoption learnings into durable product inputs
- reprioritize or redirect scope
- route discovery gaps back upstream when needed

### 9. AI-Sensitive Feature Delivery

Core roles:

- `Business Analyst`
- `Product Owner`
- `Solution Architect`
- `Security / Compliance Specialist`
- `Technical Planner`
- `Implementation Engineer`
- `QA / Verification Lead`
- `Release / Handoff Manager`

Why:

- The repo is explicitly Allium-aware and already treats trust boundaries and human control points as important.
- This scenario is a plausible variant of secure delivery for autonomy-sensitive or behavior-sensitive work.

Expected job:

- keep human checkpoints explicit
- keep specification-sensitive behavior visible
- carry trust-boundary constraints through planning, implementation, verification, and release

## Recommended Order

1. `Project Planning`
2. `Delivery Slice Planning`
3. `Implementation and Release Prep`
4. `Sprint Preparation / Sprint Readiness`
5. `Secure Delivery / Secure Feature Lifecycle`
6. `Review Remediation Loop`
7. `Verification And Release Gate`
8. `Release Feedback To Reprioritization`
9. `AI-Sensitive Feature Delivery`

## Why Project Planning Should Be First

This recommendation is supported both by the current repository state and by public delivery patterns.

From the current repo:

- the upstream planning roles are already the strongest coherent chain in the repository
- the scenario can be implemented with minimal invention because the current role set already covers discovery, product direction, architecture, and planning
- it creates a reusable boundary between "we understand the work" and "we are ready to implement"

From public delivery patterns:

- Scrum distinguishes product direction and backlog preparation from execution work and treats Sprint Planning as a collaborative planning activity rather than a coding step
- Agile planning guidance commonly separates backlog refinement, planning, and implementation readiness
- secure delivery guidance such as SDL-style models distinguishes requirements, design, planning, implementation, verification, and release as separate concerns
- release and review guidance treats implementation, review, validation, and release handling as distinct downstream stages rather than one blended activity

The common pattern is consistent:

- upstream clarification and direction
- architecture and planning
- implementation and review
- verification and release handling

That makes `Project Planning` the most natural first reusable scenario for the current repository.

## Limitation

The current role set is stronger for phase-oriented and quality-gated delivery scenarios than for ceremony-oriented Scrum scenarios such as Daily Scrum or Retrospective because the repository does not currently include a Scrum Master or Delivery Manager style role.

## Sources

- [Scrum Guide](https://scrumguides.org/scrum-guide)
- [Atlassian: Backlog refinement](https://www.atlassian.com/agile/scrum/backlog-refinement)
- [Atlassian: Product release](https://www.atlassian.com/agile/product-management/product-release)
- [GitHub Docs: About pull request reviews](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/reviewing-changes-in-pull-requests/about-pull-request-reviews)
- [GitHub Docs: Review deployments](https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/review-deployments)
- [Microsoft Security Development Lifecycle](https://learn.microsoft.com/en-us/compliance/assurance/assurance-microsoft-security-development-lifecycle)
