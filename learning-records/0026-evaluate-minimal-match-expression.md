# Evaluate minimal match expression

用户已完成 `match` 表达式从 parser 到 evaluator 的推进：`eval_expr` 中新增了 `Expr::Match` 求值分支，采用 `arms.iter().find()` 组合子风格实现自上而下的首次匹配优先求值。subject 只求值一次，使用引用与每个 arm 的 pattern 比较，三种模式（`Number`、`Boolean`、`Wildcard`）均正确工作，`_` 作为兜底。全部 25 个测试通过。这意味着语言第一次拥有了运行时的控制流选择能力。
