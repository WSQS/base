# Parse comparison expressions

用户已经把比较运算符从 scanner 层继续推进到了 parser 层：比较 token 现在能进入 `Expr::Binary`，并且通过独立测试验证了比较层优先级低于乘除与加减。这意味着后续如果要真正计算比较结果，evaluator 可以建立在已经稳定的比较表达式树之上，而不必再回头重构表达式解析层次。