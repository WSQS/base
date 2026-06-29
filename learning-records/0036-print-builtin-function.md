# Print as builtin function

用户已从 evaluator 层将 `print` 从特殊语句彻底转换为 builtin 函数：`print_builtin` 已实现（按值类型格式化输出并返回原值），`eval_program()` 在初始环境中注入了 `"print" → Value::BuiltinFn(print_builtin)`，`Stmt::Expr` 已从自动打印简化为纯求值。这意味着 `print` 现在与 `add` 等其他 builtin 运行时地位完全相同——都是全局环境中的函数值，语言的语句系统定型为 `let` 声明 + 表达式语句。
