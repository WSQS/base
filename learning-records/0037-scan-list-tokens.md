# Scan list tokens

用户已经为 list 数据结构完成了 scanner 层的最小推进：`Token::LBracket` 和 `Token::RBracket` 已加入，scanner 现在能将方括号识别为独立 token。这意味着后续 parser 构建 list 字面量 AST 时，词法入口已经就绪，逗号早已支持，`[1, 2, 3]` 的完整 token 序列已可被扫描。
