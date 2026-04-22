---
id: implementation-evidence
kind: artifact
title: Implementation Evidence
version: 1
summary: Capture the local validation activities, environments, observed results,
  and evidence limits associated with an implementation slice.
template: true
default_output_path: docs/planning/implementation-evidence.md
checks:
- implementation-evidence
- implementation-traceability
- implementation-engineer-boundary
---

# Implementation Evidence

## Purpose

Capture the local validation activities, environments, observed results, and evidence limits associated with an implementation slice.

Use this artifact to preserve implementation-facing evidence without forcing downstream roles to reconstruct local validation from terminal output, ad hoc notes, or incomplete recollection.

This checked-in file is the canonical template definition. Create an instantiated project copy before filling it out for live Implementation Engineer work.

## Completion Guidance

This artifact is complete when a downstream reader can understand what validation was attempted, what environments and revisions it applied to, what the observed results were, what was skipped or failed, and where the evidence is too weak to support strong claims.

## Related Checks

- Primary: [`implementation-evidence.check.md`](D:/Projects/orpheum/checks/implementation-evidence.check.md)
- Cross-cutting: [`implementation-traceability.check.md`](D:/Projects/orpheum/checks/implementation-traceability.check.md)
- Cross-cutting: [`implementation-engineer-boundary.check.md`](D:/Projects/orpheum/checks/implementation-engineer-boundary.check.md)

## Evidence Scope Summary

Summarize what implementation slice this evidence package applies to and what kinds of validation it includes.

## Revision And Environment Provenance

Capture the revision, branch, build context, runtime, environment, fixtures, datasets, configuration state, or other provenance details needed to interpret the evidence honestly.

## Validation Activities

List the local validation activities that were attempted, such as builds, unit tests, integration checks, manual walkthroughs, local runs, browser checks, or migration dry runs.

For each activity, capture:

- what was attempted
- the exact command, procedure, or manual path used when that level of detail materially affects reproducibility
- what scope or component it covered
- what environment or setup it depended on
- whether it completed, failed, was partial, or was skipped

## Observed Results

Summarize the main outcomes of the validation activities.

Keep clean passes, mixed results, failures, and inconclusive outcomes separate.

## Known Failures And Skipped Checks

List failed checks, unavailable checks, intentionally skipped validation, and any reasons they were not resolved in this implementation pass.

## Manual Verification Notes

Capture any manual observations, UX checks, exploratory testing notes, or operator observations that materially affect downstream understanding.

## Logs, Artifacts, And Supporting References

Reference any logs, screenshots, run outputs, fixtures, or supporting artifacts that materially strengthen or clarify the evidence package.

## Evidence Gaps And Confidence Limits

Identify where the evidence is partial, stale, environment-bound, non-representative, or otherwise too weak for stronger downstream conclusions.

## Revalidation Watchouts

List the changes or conditions that would invalidate this evidence package or materially weaken its usefulness downstream.
