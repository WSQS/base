# Parse function expression

用户已经把函数从 scanner 层继续推进到了 parser / AST 层：语言现在拥有了独立的 `Expr::Fn { params, body }` 节点，并通过测试验证了匿名函数字面量 `fn(x){x}` 已能被正确解析为函数表达式 AST。函数字面量作为一种 prefix 结构（和 `match` 类似），由 `Token::Fn` 触发后在 `parse_primary()` 中依次消费参数列表和 body 表达式。这意味着后续进入函数调用和 evaluator 时，可以建立在已经稳定的函数 AST 入口之上继续推进。
