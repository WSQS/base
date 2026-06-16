# Scan match-related tokens

用户已经把未来最小 `match` 表达式所需的词法符号先限制在 scanner 层单独推进：`match`、`_`、`=>`、花括号和逗号现在已经能被识别，并通过独立测试验证了面向完整示意写法的 token 序列。这意味着后续进入 `match` 的 parser 外壳时，可以建立在已经稳定的词法入口之上，而不必再把 scanner、parser 和 evaluator 混在同一课里推进。