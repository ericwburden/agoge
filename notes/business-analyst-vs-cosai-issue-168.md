# Business Analyst Comparison: `agoge` vs. CoSAI Issue #168

This note compares [`agents/business-analyst.md`](D:/Projects/agoge/agents/business-analyst.md) with [CoSAI issue #168](https://github.com/cosai-oasis/secure-ai-tooling/issues/168) through a role design lens.

## Overall Assessment

`agoge` currently defines a Business Analyst as an early-stage discovery and requirements role for software and system projects. CoSAI issue #168 defines an AI Business Analyst as a business-layer governance and lifecycle oversight persona for AI systems. The two overlap in requirements, process mapping, gap analysis, and business-to-technical translation, but they are not duplicates. `agoge` is stronger for initial project setup and discovery; the CoSAI persona is stronger for AI-specific governance, oversight, and post-deployment accountability.

## Comparison

- **Purpose**: `agoge` is optimized for kickoff, discovery, and requirements clarification before solutioning. CoSAI is optimized for aligning AI systems with business capabilities, governance needs, and oversight across the lifecycle.
- **Core responsibilities**: Both cover requirements gathering, process mapping, gap analysis, stakeholder alignment, and translation between business and technical work. CoSAI extends further into AI augmentation opportunity identification, prompt/tool/guardrail refinement, acceptance validation of agent outputs, KPI and drift monitoring, and trustworthiness documentation.
- **Boundaries**: `agoge` explicitly excludes delivery management, implementation ownership, and detailed technical design. CoSAI also excludes building or training models, but it pushes the role deeper into lifecycle oversight, control definition, and operational accountability than `agoge` currently does.
- **Artifacts**: `agoge` defaults to business objectives, stakeholder lists, as-is/to-be process summaries, verified requirements, assumptions, and risks. CoSAI expects a broader governance artifact set including capability maps, process models, traceability, oversight rules, business-alignment controls, and ongoing feedback artifacts.
- **Operating posture**: `agoge` is a structured interviewer optimized for discovery quality and requirement clarity. CoSAI is a business accountability persona optimized for AI system alignment, human oversight, trustworthiness, and continuous validation.

## Worth Adopting

- Add stronger language around **requirements traceability** from business objective to agent requirement.
- Add explicit **human oversight considerations** for cases where the subject system includes autonomous or agentic behavior.
- Add optional guidance for **acceptance validation of agent outputs** when the project is AI-enabled.
- Sharpen the distinction between **business objectives** and **AI or agent constraints**, so solution-specific controls are recorded separately from the underlying business need.

## Do Not Import Yet

- Do not turn the `agoge` Business Analyst into a general-purpose **AI governance persona**; that would blur the current kickoff/discovery focus.
- Do not import **KPI monitoring, drift tracking, and ongoing operational oversight** as default responsibilities; those belong to a later lifecycle role or a separate AI-specific variant.
- Do not import **control and risk taxonomy** as core BA output yet; that would overfit the template to governance-heavy AI programs and reduce reuse across general software and system projects.
- Do not import **backlog prioritization or initiative grooming** as a default responsibility; that starts to blend the role with product ownership.

## Recommendation

Keep the current `agoge` template as the baseline Business Analyst role for project kickoff and discovery. Treat CoSAI issue #168 as a useful adjacent pattern for a future AI-specific extension, likely a separate `AI Business Analyst` or governance-oriented variant rather than a direct replacement for the current template.
