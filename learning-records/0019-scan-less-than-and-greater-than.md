# Scan less-than and greater-than

用户已经按单一实现层推进的节奏，把比较运算符扩展先限制在 scanner 层：`Token` 新增了 `Less` 与 `Greater`，`scan()` 已能识别 `<` 和 `>`，并用独立测试验证了 token 序列。这意味着后续引入比较表达式时，可以在不回头重做词法层的前提下，继续单独推进 parser 与 evaluator。