# Learning Record 0007: keyword token

## Context
用户已完成 Lesson 0007，主题是在现有标识符扫描器基础上，把 `let` / `print` 从普通 `Ident` 中区分出来。

## What was learned
- 用户已掌握关键字识别的正确时机：先把整个名字读完整，再判断它是否是保留字。
- 用户已实现 `Let` 和 `Print` token，并保持普通标识符仍然输出为 `Ident(String)`。
- 用户已通过测试验证关键字和普通标识符可以在同一输入中正确区分。
- 当前 lexer 已经具备支撑 Phase 0 最小语句形态 `let name = expr;` 和 `print expr;` 的词法能力。

## Why it matters
这一步标志着 Phase 0 的 lexer 收尾基本完成。接下来 parser 不再需要猜测 `Ident("let")` 是否是关键字，而可以直接根据 token 类型进入语句解析。

## Implications for next lessons
- 下一课应正式进入 parser 阶段，而不是继续扩展 lexer。
- parser 的第一批目标应直接支持两类语句：`let name = expr;` 与 `print expr;`。
- 进入 parser 前，可以先对当前 lexer 做一次适度 review，特别是检查关键字识别与 `flush` 逻辑的结构质量。
