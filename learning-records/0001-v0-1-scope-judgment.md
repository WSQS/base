# Learning Record 0001: v0.1 scope judgment

## Context
用户已完成 Lesson 0001，主题是为第一门编程语言原型判断合适的 v0.1 范围。

## What was learned
- 第一版语言应优先追求最小闭环：源码 -> token -> AST -> 执行 -> 输出。
- 用户已学习如何判断一个功能是否属于 v0.1：保留闭环必需功能，延后会显著增加复杂度的功能。
- 当前推荐的 v0.1 范围包括：数字、标识符、四则运算、括号、`let`、`print`。

## Why it matters
这为后续进入 token 设计、lexer、parser 和执行模型提供了稳定边界，避免第一步就把语言做得过大。

## Implications for next lessons
- 下一课应该直接建立在这个范围之上。
- 最合适的下一技能是：把 v0.1 范围转成明确的 token 设计，并能判断输入源码会被切成什么 token 序列。
