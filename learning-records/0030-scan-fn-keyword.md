# Scan fn keyword

用户已经将 `fn` 关键字接入了 scanner：`Token::Fn` 已加入，scanner 的 flush 闭包和末尾收尾逻辑都支持 `"fn"` 分支，并通过测试验证了完整的函数字面量 token 序列。这意味着后续 parser 构建函数字面量 AST 时，词法入口已经就绪，而不必在解析阶段临时处理关键字识别。
