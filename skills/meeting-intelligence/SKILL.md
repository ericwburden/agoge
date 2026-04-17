---
name: meeting-intelligence
description: Prepare discovery meetings, decision meetings, and project kickoff materials from local Markdown context; use when gathering local notes, prior artifacts, and research into agendas, pre-reads, and meeting prep outputs without depending on Notion.
metadata:
  short-description: Prep meetings from local Markdown context
---

# Meeting Intelligence

Prepare meetings by gathering local context, tailoring agendas or pre-reads, and enriching them with relevant research or artifact references.

For this repository's Business Analyst workflows, this is the preferred default meeting-prep path. Treat Notion-based variants as reference implementations unless the task explicitly requires Notion.

## Quick start
1) Confirm meeting goal, attendees, duration, and desired decisions or outcomes.
2) Gather context from local Markdown files, prior notes, artifacts, and relevant repository documents.
3) Pick the right meeting shape: status, decision, planning, retrospective, one-on-one, or discovery.
4) Draft the agenda or pre-read in local Markdown with linked sources and explicit owners or timeboxes.
5) Enrich with concise research or historical context as needed and finalize the meeting output.

## Workflow

### 1) Gather inputs
- Ask for meeting objective, desired outcomes or decisions, attendees, duration, date or timing, and prior materials.
- Read relevant local files first: notes, prior meeting outputs, artifacts, specs, decision records, or project docs.
- Capture blockers, open questions, and risks before drafting the meeting structure.

### 2) Choose the meeting shape
- Status/update → short context plus agenda and blockers.
- Decision/approval → decision framing, options, risks, and required outcome.
- Planning or project kickoff → goals, scope, assumptions, and discussion prompts.
- Retrospective or feedback → observations, prompts, and improvement areas.
- Discovery → context, open questions, stakeholder concerns, and information gaps.

### 3) Build the agenda or pre-read
- Create a local Markdown output with:
  - meeting purpose
  - expected decisions or outcomes
  - context or linked references
  - agenda items with owner or facilitator and timebox when useful
  - risks, blockers, or open questions
  - required prep for attendees

### 4) Enrich with research or artifact context
- Pull concise context from local artifacts or external research when useful.
- Separate known facts from hypotheses or opinions.
- Keep cited or linked references visible so future readers can trace the basis for the meeting.

### 5) Finalize
- Ensure the meeting output is usable without chat context.
- Verify names, goals, and decision asks are consistent.
- If the meeting is part of Business Analyst work, align the output to the Business Analyst artifacts and separate business objectives from proposed solutions.

## Notes

- This is a local-Markdown adaptation of a Notion-based meeting-prep pattern.
- Prefer repo-local artifacts and notes as the default source of truth.
- Use with `meeting-notes-and-actions` when turning the completed meeting back into discovery outputs.
