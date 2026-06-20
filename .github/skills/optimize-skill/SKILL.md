---
name: optimize-skill
description: Diagnose and improve an existing GitHub Copilot skill or its surrounding workspace guidance based on concrete problems, using repository and conversation context to clarify the real issue before editing. Use when the user reports that a skill is behaving incorrectly, is confusing, has the wrong scope, or may need changes in either the skill or related workspace instruction files.
---

# Optimize Skill

## Quick start

1. Read the target skill's `SKILL.md` and any directly referenced local files.
2. Start from the concrete problems the user identified, not from a generic rewrite goal.
3. Use available context such as nearby skills, repository purpose, current workflow, recent conversation state, and any earlier use of the target skill in this session to infer what the skill should optimize for.
4. Before editing, clarify the actual problem: confirm whether the issue is really in the target skill, in repository-local workflow, in current session instructions, or in higher-level agent behavior.
5. If the user asks to "record", "persist", or "land" a reflection, first classify the information type before choosing a file: skill workflow guidance belongs in the relevant skill or instruction file; project collaboration preferences belong in repository notes; project goals and constraints belong in mission/design files.
6. If the problem is still ambiguous, discuss it with the user first instead of editing immediately.
7. Edit only what is needed to fix the confirmed problem without changing the intended capability unless the user asks for a scope change.

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
- what kind of information is being changed: skill behavior, project goal or design constraint, collaboration preference, or repository content/workflow guidance
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

If the problem statement is underspecified, ask focused questions or summarize the candidate interpretations before making edits.

### 2. Decide what should change

Choose the smallest correct target for the fix:
- the target skill's frontmatter
- the target skill body
- a referenced local file
- another workspace instruction/configuration file
- no file yet; clarify with the user first

When the user asks to persist a reflection, do not choose the destination file based only on importance. Choose it based on file responsibility.

Do not default to editing `description` and body if the problem is really elsewhere.

### 3. Optimize the description

Only revise the description when the confirmed problem involves invocation, trigger accuracy, or scope confusion.

Ensure the description:
- addresses the failure mode implied by the user's complaint
- says what the skill does in the first sentence
- says `Use when ...` in the second sentence
- includes concrete trigger words, contexts, file types, or workflows
- distinguishes the skill from nearby or overlapping skills
- stays within frontmatter limits and avoids filler

### 4. Optimize the body or related files

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

If the confirmed fix belongs outside the skill file, update that file instead and explain why the skill itself was not the right edit target.

### 5. Split only when necessary

Create supporting files only if clearly needed, such as when:
- `SKILL.md` becomes too long or mixes multiple concerns
- detailed examples would distract from the default workflow
- reusable review criteria deserve their own file

Keep references one level deep, for example `[REFERENCE.md](REFERENCE.md)`.

## Optimization checklist

Before finishing, verify:
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
- mention whether you first clarified or reframed the problem before editing
- mention what context influenced the revision
- mention whether earlier in-session target skill behavior influenced the revision
- mention whether the fix landed in the skill file or another workspace file, and why
- mention any new supporting files
- call out any remaining ambiguity in the skill's intended scope