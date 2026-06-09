# Learning Record 0004: more single-char tokens

## Context
用户已完成 Lesson 0004，主题是在现有 `Number` + `Plus` 扫描器基础上，继续加入 `Minus`、`Star`、`Slash` 三种单字符运算符。

## What was learned
- 用户已能按增量模式持续扩展单字符 token：新增 enum 变体、添加扫描分支、保证在推入运算符前先结束当前 Number。
- 用户已补充单元测试，验证 `Minus`、`Star`、`Slash` 与数字混排时的输出顺序。
- 用户已经主动意识到重复逻辑问题，并开始尝试抽出 `flush` 逻辑进行收敛。
- 用户在这一过程中接触到了 Rust 闭包捕获与借用问题，并开始理解为什么有些重复逻辑适合抽成函数/方法，而不是长期闭包。

## Why it matters
这说明用户已经不只是“会加 token”，而是开始进入实现结构优化阶段。接下来 lexer 的学习会逐渐从“增加功能”转向“在增加功能的同时保持结构可扩展”。

## Implications for next lessons
- 下一课很适合进入另一类 token：括号、等号、分号，或者正式进入标识符/关键字。
- 在进入新 token 类别之前，值得先对当前 scanner 做一次小范围整理，把 `flush` 逻辑调整为更稳妥的 Rust 结构。
- 后续课程可以逐步引入“从重复逻辑中提炼局部抽象”的实践节奏。
