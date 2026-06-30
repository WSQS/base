# Parse list literals

用户已经把 list 从 scanner 层推进到了 parser / AST 层：`Expr::List(Vec<Expr>)` 已加入，parser 在 `parse_primary()` 中通过 `Token::LBracket` 分支实现了 list 字面量解析，支持空 list 和任意类型元素，并通过测试验证了 `[1, 2, 3]` 已能被正确构造为 list 字面量 AST。这意味着后续进入 evaluator / runtime value 时，list 的语法入口已经就绪。
