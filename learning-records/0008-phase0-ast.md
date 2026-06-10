# Learning Record 0008: phase0 ast

## Context
用户已完成 Lesson 0008，主题是先定义 Phase 0 的最小 AST，为 parser 实现建立明确输出目标。

## What was learned
- 用户已定义 Program / Stmt / Expr 三层结构，并把 Phase 0 的语句边界明确为 `Let` 与 `Print`。
- 用户已理解递归表达式节点为什么需要 `Box<Expr>` 这样的间接层，而不能直接在 enum 变体中内嵌自身。
- 用户已能用 AST 结构表达 `let x = 1 + 2; print x;` 这样的最小程序。
- 用户已在测试中开始检查 AST 结构，而不再只验证 token 序列。

## Why it matters
这标志着学习重点从“源码如何切 token”正式转向“token 最终要组织成什么结构”。一旦 AST 定义清楚，parser 的任务就从模糊的“处理 token”变成明确的“产出 Program / Stmt / Expr 结构”。

## Implications for next lessons
- 下一课应正式进入 parser 实现，而不是继续停留在 AST 设计层。
- parser 的第一批目标仍应围绕 Phase 0 两类语句：`let name = expr;` 与 `print expr;`。
- 进入 parser 课后，测试应逐步从词法层断言，扩展到对 Program / Stmt / Expr 结构的断言。
