# Evaluate list literals

用户已经把 list 从 parser/AST 层推进到了 evaluator 层：`Value::List(Vec<Value>)` 已加入运行时值类型，`eval_expr(Expr::List)` 对每个元素递归求值后包装为运行时 list 值，并通过测试验证了 `[1, 2]` 已能被正确求值。这意味着语言拥有了第五种运行时值类型，list 全链路（scan → parse → eval → print）已贯通。下一步可继续推进 list 操作 builtin。
