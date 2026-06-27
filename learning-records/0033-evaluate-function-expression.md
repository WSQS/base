# Evaluate function expression

用户已经把函数从 parser/AST 层继续推进到了 evaluator 层：`Value::Fn` 已加入运行时值类型，`eval_expr(Expr::Fn)` 返回函数值而不立即执行 body，`eval_expr(Expr::Call)` 实现了完整的调用链路（求值 func 和 args → 环境克隆 → 参数绑定 → 在局部环境中求值 body）。通过测试验证了 `let f = fn(x){x+1}; f(5)` 返回 6。这意味着语言第一次拥有了用户可定义的抽象能力，函数从扫描到解析到求值的全链路已打通。scanner、parser 和 AST 结构在这一课中未做任何改动。
