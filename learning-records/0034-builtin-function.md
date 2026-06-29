# Builtin function

用户已经将 builtin 函数基础设施引入了 evaluator 层：`Value::BuiltinFn(fn(Vec<Value>) -> Value)` 已加入运行时值类型，`Expr::Call` 处理中通过 match 分派在用户定义函数和 builtin 之间区分调用路径，并通过测试验证了 `add(1, 2)` 返回 3。这意味着语言第一次拥有了用 Rust 原生实现的内置函数能力，后续每加一个 builtin 都是同样的路径：实现 Rust 函数 → 注入到初始环境。
