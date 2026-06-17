# Parse string literals

用户已经把字符串从 scanner 层继续推进到了 parser / AST 层：语言现在拥有了独立的 `Expr::String(String)` 节点，并通过测试验证了字符串 token 已能被正确解析为字符串字面量表达式。这意味着后续进入 evaluator / runtime value 时，可以建立在已经稳定的字符串语法入口之上继续推进，而不必把 scanner、parser 和 evaluator 混在同一课里完成。
