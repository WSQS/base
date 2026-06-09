# Learning Record 0003: single-char token extension

## Context
用户已完成 Lesson 0003，主题是在 NUMBER-only scanner 基础上增量加入 `+` 运算符 token。

## What was learned
- 用户已掌握增量扩展 token 的标准模式：enum 加变体 → 扫描循环加字符匹配分支 → 先结束当前数字再推入新 token。
- 用户已验证 `Plus` 与数字混排的多种情况，包括无空格输入。
- 用户已引入单元测试来验证扫描器输出。
- 用户已意识到：随着单字符 token 增多，`' '` 和运算符分支之间的"结束当前数字"逻辑开始出现重复。

## Why it matters
增量扩展 token 是 lexer 实现贯穿始终的核心技能。这节课证明了用户不是在写一次性的数字扫描器，而是在逐步搭一个可扩展的词法分析框架。

## Implications for next lessons
- 下一课可以继续按相同模式加入 `-`、`*`、`/` 等单字符运算符，进一步暴露重复逻辑，为后续结构调整做铺垫。
- 也可以选择加入第 1 个关键字 `let` 或 `print`，引入标识符与关键字区分。
- 下一课之前建议先完成 `&String` → `&str` 以及测试封装优化。
