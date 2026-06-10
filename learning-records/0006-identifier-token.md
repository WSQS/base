# Learning Record 0006: identifier token

## Context
用户已完成 Lesson 0006，主题是让扫描器第一次识别标识符 `Ident(String)`。

## What was learned
- 用户已让扫描器从只识别数字和符号，扩展到可以识别由连续字母/字母数字组成的名字 token。
- 用户已理解 `Ident(String)` 与 `Number(i64)` 在所有权上的差异，并在实现中使用 `clone()` 解决缓冲区复用问题。
- 用户已学会在测试中用 `matches!` + guard 的形式检查 `Ident("foo")` 这类带字符串内容的 token。
- 用户已通过测试验证名字与括号、运算符、等号、分号混排时的输出顺序。

## Why it matters
这标志着扫描器第一次进入“名字语言”阶段。到这一步为止，用户的 lexer 已经能够切出非常接近真实语句结构的 token 流，为下一步区分关键字和标识符做好准备。

## Implications for next lessons
- 下一课最自然进入关键字区分，例如把 `let`、`print` 从普通 `Ident` 中识别出来。
- 也可以在课后 review 阶段继续整理当前 `flush` + `value: String` 的组织方式，为后续 lexer 增长减轻结构负担。
- 后续课程应继续保持：先通过输入/输出和测试验证新 token 行为，再做温和重构。
