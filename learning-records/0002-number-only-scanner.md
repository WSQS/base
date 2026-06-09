# Learning Record 0002: number-only scanner

## Context
用户已完成 Lesson 0002，主题是亲手实现只支持数字和空白的最小扫描器。

## What was learned
- 用户已能使用 Rust 的 `enum` 表示 token，并用 `Vec<Token>` 收集扫描结果。
- 用户已实现从左到右扫描字符串，读取连续数字片段，并输出 `NUMBER(...)` token。
- 用户已处理空白分隔，并能在输入结束后补交最后一个尚未输出的数字 token。

## Why it matters
这标志着用户已经从“知道 token 是什么”进入“亲手实现最小 lexer 行为”。后续扩展新 token 时，核心工作将是增量修改这个扫描流程，而不是重新理解 lexer 的基本职责。

## Implications for next lessons
- 下一课最合适只增加一种新 token，例如 `+`。
- 新课应继续保持实践驱动：在现有 NUMBER-only scanner 基础上，增量扩展输入/输出行为。
- 下一技能应聚焦：在保留 NUMBER 扫描逻辑的前提下，识别单字符 token。
