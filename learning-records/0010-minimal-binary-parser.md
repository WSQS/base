# Learning Record 0010: minimal binary parser

## Context
用户已完成 Lesson 0010，主题是让 parser 的表达式能力从单个 `Number / Ident` 扩展到最小二元表达式结构。

## What was learned
- 用户已让 `parse_expr()` 能读取左侧原子表达式、识别一个二元运算符，并继续读取右侧原子表达式。
- 用户已能够构造 `Expr::Binary` 节点，而不再只返回单个 `Number` 或 `Ident`。
- 用户已把最小 binary expression 解析接入 `let` 与 `print` 语句解析流程中。
- 用户已通过测试验证 `let x = 1 + 2;` 能产出带 `Expr::Binary` 的 AST 结构。

## Why it matters
这标志着 parser 已经不只是“语句骨架解析器”，而开始真正构造表达式树。到这一步为止，Phase 0 的 parser 已经能够支撑更像真实语言的最小程序结构。

## Current boundary
- 当前 parser 已能处理最小 binary expression，如 `1 + 2`、`x + 3`。
- 当前表达式解析仍然是最小版本，还没有正式处理完整优先级体系或更复杂的分组规则。
- 当前 `scan` 的结构性技术债仍被保留，但不阻塞 parser 阶段继续推进。

## Implications for next lessons
- 下一课可以继续细化表达式解析，例如开始处理更系统的优先级分层或最小括号分组。
- 也可以开始讨论 evaluator 的最小执行路径，因为 AST 结构已经越来越接近可运行闭环。
- 后续课程仍应维持：先扩展一小步 parser 能力，再用测试锁定行为，再做适度 review。
