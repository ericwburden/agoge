# AGENTS

## Purpose

This repository is an experimentation space for Codex instructions, skills, workflows, and related SDLC patterns. Changes should favor reusable structure, explicit reasoning, and practical validation over ad hoc experimentation that leaves little durable value behind.

## Working Principles

- Preserve the repository as a training and integration environment.
- Prefer small, reviewable changes with clear intent.
- Document new conventions close to where they are introduced.
- Optimize for reusable workflows rather than project-specific hacks.
- Treat experiments as candidates for standardization, comparison, or rejection.

## Expected Repository Direction

Over time, this project should accumulate:

- reusable instruction patterns
- workflow templates
- guidance for evaluating AI-assisted delivery practices
- examples of successful and unsuccessful approaches
- lightweight scaffolding that supports repeatable experimentation

## Agent Guidance

- Read existing documentation before introducing new structure.
- When adding a new pattern, explain the problem it solves and the tradeoffs.
- Keep terminology consistent across files and directories.
- If a proposed change introduces process overhead, justify it.
- Prefer documentation and automation that reduce ambiguity for future agents.

## Change Standard

A good change in this repository should usually do at least one of the following:

- make an AI workflow more repeatable
- clarify an instruction or operating convention
- improve evaluation of workflow quality or outcomes
- reduce ambiguity for future contributors or agents

## Current Limitation

Git initialization is still pending because `git` is not currently available in the shell environment for this workspace.
