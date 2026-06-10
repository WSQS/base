# Learning Record 0009: minimal parser

## Context
用户已完成 Lesson 0009，主题是让 parser 第一次真正消费 token，并先解析 Phase 0 的最小语句骨架。

## What was learned
- 用户已让 `parse()` 从“手工返回固定 AST”转变为“根据 token 类型分派语句解析”。
- 用户已实现最小 `print` 语句解析：`print expr;`。
- 用户已实现最小 `let` 语句解析：`let name = expr;`，其中当前表达式子解析仍限制在最简单的 Number / Ident。
- 用户已开始在测试中验证 parser 产出的 `Stmt` 结构，而不再只验证 token 序列。

## Why it matters
这标志着项目正式从 lexer 阶段进入 parser 阶段。虽然当前 parser 还很小，但它已经具备“消费 token -> 产出 AST”的真实工作模式。

## Current boundary
- 当前 parser 已能处理最简单的 `let x = 1;` 和 `print x;`。
- 当前 parser 还没有完成二元表达式解析，因此不能把 `let x = 1 + 2;` 视作已完整支持。
- 已明确有一个测试边界问题被有意延后处理，不阻塞继续推进课程。

## Implications for next lessons
- 下一课应继续扩展 parser，而不是回到 lexer。
- 最自然的下一步是让表达式解析从单个 Number / Ident，扩展到最小二元表达式结构。
- 之后再逐步接入 evaluator，完成 Phase 0 闭环。
