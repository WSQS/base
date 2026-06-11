# Minimal evaluator

用户已经完成最小 evaluator：能够对 `Expr::Number` 与最小 `Expr::Binary` 递归求值，并通过 `main -> parse -> eval_program` 跑通 `print expr;` 的执行闭环。这意味着 Phase 0 已经第一次具备“输入源码并得到运行输出”的真实能力，为下一步引入最小变量环境做好了准备。