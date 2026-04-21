# Scenario Recommendations

## Purpose

Record a research-backed recommendation for which reusable multi-role scenarios this repository can support with the current role set and which scenario should be implemented first.

This note translates public delivery and orchestration patterns into a pragmatic scenario roadmap for this repository.

## Recommendation Summary

The first scenario in the default SDLC chain should now be `Project Discovery`.

That is the highest-leverage upstream scenario because it closes the earlier discovery gap explicitly and creates a reusable bridge between raw project signals and the validated discovery package that downstream planning expects.

## Recommended Initial Scenario Set

### 1. Project Discovery

Implement this as the explicit upstream discovery scenario.

Core roles:

- `Business Analyst`
- optional `Product Owner`

Why:

- This closes the remaining upstream SDLC gap before `Project Planning`.
- It turns kickoff signals, rough stakeholder asks, and release-driven follow-up needs into validated discovery instead of leaving discovery as an implied role-only precondition.
- It fits the current repository especially well because the Business Analyst package already has a strong kickoff -> process-analysis -> requirements-handoff chain.

Expected job:

- clarify the business problem and scope boundaries
- capture process understanding and operational gaps
- verify requirements and confirmation posture
- hand a validated discovery package into planning
- optionally add early product-readiness framing when needed before planning starts

### 2. Project Planning

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

### 3. Sprint Preparation / Sprint Readiness

Core roles:

- `Product Owner`
- `Solution Architect` or `Technical Planner`
- optional `QA / Verification Lead`

Why:

- This is a plausible planning-adjacent scenario for teams that need work to become slice-ready before implementation begins.
- It fits the current roles without requiring a dedicated Scrum Master or Delivery Manager package.
- It should be treated as a cadence-specific planning variant, not as part of the default core SDLC chain while `Delivery Slice Planning` is the main slice-shaping scenario.

Expected job:

- refine current priority direction
- clarify acceptance-oriented guardrails
- make slice boundaries and dependencies explicit
- identify verification-sensitive readiness conditions

### 4. Delivery Slice Planning

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

### 5. Implementation and Release Prep

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

### 6. Secure Delivery / Secure Feature Lifecycle

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
- This scenario has now been implemented in [`secure-delivery-feature-lifecycle.definition.md`](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.definition.md), [`secure-delivery-feature-lifecycle.integration-map.md`](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.integration-map.md), [`secure-delivery-feature-lifecycle.review.md`](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.review.md), and [`secure-delivery-feature-lifecycle.handoff.md`](C:/Users/ericw/Projects/orpheum/scenarios/secure-delivery-feature-lifecycle.handoff.md).

Expected job:

- carry security/compliance framing across planning and delivery
- keep controls, evidence, and approval-sensitive decisions explicit
- preserve secure-delivery constraints through release posture

### 7. Review Remediation Loop

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

### 8. Verification And Release Gate

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

This scenario now has a reusable package in `scenarios/verification-and-release-gate.*` and should be used when the remaining work is evidence review, trust-boundary framing, and final downstream gating rather than implementation, slice shaping, or review remediation.

### 9. Release Feedback To Reprioritization

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

Status note:

- this scenario is now implemented as a reusable package in `scenarios/`

### 10. AI-Sensitive Feature Delivery

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

The current default core SDLC chain is:

1. `Project Discovery`
2. `Project Planning`
3. `Delivery Slice Planning`
4. `Implementation and Release Prep`
5. `Review Remediation Loop` when bounded remediation is needed
6. `Verification And Release Gate` when a distinct final downstream gate is needed
7. `Release Feedback To Reprioritization`

Variants and specializations:

1. `Sprint Preparation / Sprint Readiness`
2. `Secure Delivery / Secure Feature Lifecycle`
3. `AI-Sensitive Feature Delivery`

## Why Project Discovery Should Be First

This recommendation is supported both by the current repository state and by public delivery patterns.

From the current repo:

- the Business Analyst package already has a strong kickoff -> process-analysis -> requirements-handoff chain
- the repository had an explicit upstream gap because validated discovery was only a role-covered precondition
- the scenario creates a reusable boundary between "we have a project signal" and "we have validated discovery ready for planning"

From public delivery patterns:

- Scrum distinguishes product direction and backlog preparation from execution work and treats Sprint Planning as a collaborative planning activity rather than a coding step
- Agile planning guidance commonly separates backlog refinement, planning, and implementation readiness
- secure delivery guidance such as SDL-style models distinguishes requirements, design, planning, implementation, verification, and release as separate concerns
- release and review guidance treats implementation, review, validation, and release handling as distinct downstream stages rather than one blended activity

The common pattern is consistent:

- upstream clarification and validation
- product, architecture, and planning
- implementation and review
- verification and release handling

The current repository now covers that chain with implemented scenarios, with `Project Discovery` establishing the validated discovery package that the rest of the SDLC can trust.

That makes `Project Discovery` the most natural first scenario in the default reusable chain for the current repository.

## Limitation

The current role set is stronger for phase-oriented and quality-gated delivery scenarios than for ceremony-oriented Scrum scenarios such as Daily Scrum or Retrospective because the repository does not currently include a Scrum Master or Delivery Manager style role.

## Sources

- [Scrum Guide](https://scrumguides.org/scrum-guide)
- [Atlassian: Backlog refinement](https://www.atlassian.com/agile/scrum/backlog-refinement)
- [Atlassian: Product release](https://www.atlassian.com/agile/product-management/product-release)
- [GitHub Docs: About pull request reviews](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/reviewing-changes-in-pull-requests/about-pull-request-reviews)
- [GitHub Docs: Review deployments](https://docs.github.com/en/actions/how-tos/deploy/configure-and-manage-deployments/review-deployments)
- [Microsoft Security Development Lifecycle](https://learn.microsoft.com/en-us/compliance/assurance/assurance-microsoft-security-development-lifecycle)
