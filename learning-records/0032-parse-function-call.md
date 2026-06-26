# Parse function call

用户已经把函数调用从语法层推进到了 parser / AST 层：`Expr::Call { func, args }` 已加入，parser 在 `parse_primary()` 末尾通过 postfix 检查 `LParen` 实现了 `expr(args)` 的解析，并通过测试验证了调用表达式 AST 结构。`func` 可以是任意表达式而不仅限于标识符。这意味着函数字面量（第 31 课）和函数调用（本课）现在都已进入 AST，后续进入 evaluator 时可以在已稳定的调用语法入口上继续推进。
