# Evaluate and print string literals

用户已经把字符串从 scanner 和 parser 层推进到了 evaluator / runtime 层：`Value` 中新增了 `String(String)` 变体，`eval_expr(Expr::String(s))` 已返回对应运行时值，`print` 已支持输出字符串。通过 `let "..."` 存储和 `print "..."` 输出验证了完整链路。这意味着字符串作为第三种值类型已正式进入运行时，scan → parse → eval → output 全通。scanner 和 parser / AST 在这节课中未做任何改动，严格保持了单层推进。
