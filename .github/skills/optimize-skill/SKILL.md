---
name: optimize-skill
description: Optimize an existing GitHub Copilot skill based on specific problems the user points out, using available repository and conversation context to make the skill clearer, tighter, and easier to invoke correctly. Use when the user reports issues with a skill's triggers, scope, structure, wording, behavior, or supporting files and wants a targeted revision.
---

# Optimize Skill

## Quick start

1. Read the target skill's `SKILL.md` and any directly referenced local files.
2. Start from the concrete problems the user identified, not from a generic rewrite goal.
3. Use available context such as nearby skills, repository purpose, current workflow, recent conversation state, and any earlier use of the target skill in this session to infer what the skill should optimize for.
4. Before editing, classify the reported problem: is it caused by the target skill itself, repository-local workflow, current session instructions, or higher-level agent behavior? Only encode the first category into the target skill unless the user explicitly asks for a scope change.
5. Edit only what is needed to fix the reported problems without changing the intended capability unless the user asks for a scope change.
6. Keep the result concise, specific, and easy for an agent to invoke correctly.

## Context to use

Use as much relevant context as is available:
- the user's explicitly reported problems or dissatisfaction
- earlier turns where the target skill was used, discussed, or produced unsatisfactory output
- the current repository's purpose and conventions
- neighboring skills that may overlap or compete for invocation
- referenced files, examples, helper scripts, and supporting documents
- recent conversation context that clarifies intended behavior or priorities

Do not optimize in a vacuum. If the user says a skill is too broad, too vague, not triggering, or producing the wrong kind of output, treat that as the primary optimization target. If the target skill was used earlier in the session, use that actual behavior as evidence for what should change.

Do not bake temporary workflow habits, repository-specific collaboration patterns, or session-only instructions into a reusable skill unless the user explicitly wants that scope change.

## Workflow

### 1. Inspect the skill

Check:
- what exact problem the user wants fixed
- whether the problem belongs to the target skill itself or to surrounding workflow/instructions
- whether the current session already contains an example of the target skill being invoked or producing output
- frontmatter fields such as `name`, `description`, and optional flags
- whether the description clearly states capability and concrete triggers
- whether the main instructions are too broad, vague, repetitive, or overly long
- whether advanced content should move into a separate reference file
- whether examples and checklists are concrete enough to guide behavior
- whether the skill fits this repository's actual workflow and goals
- whether another local skill already covers part of the same job more clearly

If the user gave multiple problems, rank them by impact on invocation accuracy and output quality.

If there is prior in-session evidence, prefer observed failure modes over hypothetical ones.

If the failure comes from surrounding workflow rather than the skill itself, either leave the skill unchanged or make only the minimal clarification needed to prevent future confusion.

### 2. Optimize the description

Ensure the description:
- addresses the failure mode implied by the user's complaint
- says what the skill does in the first sentence
- says `Use when ...` in the second sentence
- includes concrete trigger words, contexts, file types, or workflows
- distinguishes the skill from nearby or overlapping skills
- stays within frontmatter limits and avoids filler

### 3. Optimize the body

Prefer this structure:
- short purpose statement
- quick start or minimal workflow
- step-by-step workflow for non-trivial tasks
- optional advanced section with links to local reference files

While editing:
- preserve alignment with the user's stated problem
- preserve the boundary between target skill content and external workflow
- remove repetition
- replace vague language with explicit actions
- keep checklists actionable
- keep terminology consistent
- avoid speculative or time-sensitive guidance
- prefer targeted fixes over full rewrites when a small change solves the issue

### 4. Split only when necessary

Create supporting files only if clearly needed, such as when:
- `SKILL.md` becomes too long or mixes multiple concerns
- detailed examples would distract from the default workflow
- reusable review criteria deserve their own file

Keep references one level deep, for example `[REFERENCE.md](REFERENCE.md)`.

## Optimization checklist

Before finishing, verify:
- [ ] The reported user problem was directly addressed
- [ ] Relevant repository or conversation context was used
- [ ] Earlier in-session usage of the target skill was incorporated when available
- [ ] Repository workflow and session-only instructions were not accidentally encoded as target skill behavior
- [ ] Description is specific and trigger-based
- [ ] The skill's scope is clear
- [ ] Instructions are concise and actionable
- [ ] Repetition has been removed
- [ ] Structure supports fast invocation
- [ ] Supporting files exist only if necessary
- [ ] Existing capability was preserved unless the user requested scope changes

## Output expectations

When reporting back:
- summarize which reported problems were optimized
- mention what context influenced the revision
- mention whether earlier in-session target skill behavior influenced the revision
- mention any new supporting files
- call out any remaining ambiguity in the skill's intended scope