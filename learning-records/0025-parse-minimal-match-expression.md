# Parse minimal match expression

用户已经把最小 `match` 表达式从词法层继续推进到了 parser / AST 层：语言现在拥有了独立的 `Pattern`、`MatchArm` 和 `Expr::Match` 结构，并通过测试验证了 `match subject { pattern => expr, ... }` 的外壳已经能被正确构造成树。这意味着后续进入 `match` 的 evaluator 执行语义时，可以建立在已经分清“表达式”和“模式”职责的 AST 结构之上继续推进。