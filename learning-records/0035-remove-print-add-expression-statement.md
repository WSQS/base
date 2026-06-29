# Remove print statement, add expression statement

用户已从 scanner、AST 和 parser 三个层面完成了 `print` 语句的移除和表达式语句的引入：scanner 不再识别 `print` 为关键字（现为普通 `Ident`），`Stmt::Print` 已被 `Stmt::Expr { expr }` 替代，parser 的 `_ =>` 兜底分支实现了表达式语句解析。所有测试迁移至 `print(x)` 函数调用形式并适配了 `Expr::Call` 层级断言。这意味着语言的语句系统已简化为 `let` 声明 + 表达式语句两种，`print` 作为普通标识符等待下一课注入为 builtin 函数。
