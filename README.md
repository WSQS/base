# base

A small, dynamically-typed interpreted language built from scratch in Rust.

## What this is

base is a minimal scripting language implemented over ~40 incremental lessons — each one scoped to a single implementation phase (scanner, parser, or evaluator). It started as "parse an integer and print it" and has grown into a language with first-class functions, pattern matching, list operations, and a builtin function mechanism.

**Current capabilities:**

- Five runtime value types: integers, booleans, strings, functions, and lists
- Expressions: arithmetic, comparison, match expressions, function literals, function calls, list literals
- Statements: `let` bindings and expression statements
- Builtins: `print`, `len`, `head`, `tail`, `push`
- Module structure: `src/{scan,parse,ast,eval,value,util}.rs`

**What it looks like:**

```text
let double = fn(x) { x + x };
let nums = [1, 2, 3];
print(len(nums));          -- 3
print(double(head(nums))); -- 2

let result = match 2 {
    1 => "one",
    2 => "two",
    _ => "other",
};
print(result);             -- two
```

## Phase plan

| Phase | Goal | Status |
|---|---|---|
| Phase 0 | Frontend closed loop (scanner + parser + AST) | ✓ |
| Phase 1 | Executable core (expressions, functions, builtins) | ✓ |
| Phase 2 | Self-hosting prerequisites (data structures, recursion, scope, I/O) | In progress |
| Phase 3 | Minimal self-hosting | |
| Phase 4 | Type system experiment | |

## Teaching workspace

This repository doubles as a teaching workspace — every language feature was built through a structured lesson. See `lessons/` for the full lesson history, `reference/` for quick-reference checklists, and `learning-records/` for what was learned at each step.

## Building and running

```bash
cargo build
cargo test
echo 'print("hello");' | cargo run
```

Requires Rust nightly (edition 2024).
