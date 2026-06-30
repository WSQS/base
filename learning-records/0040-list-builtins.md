# List operation builtins

用户已经为 list 添加了四个操作 builtin：`len`、`head`、`tail`、`push`，全部通过 `get_builtin_env()` 统一注入初始环境，并通过测试验证了能对运行时 list 值进行基本操作。这意味着 list 现在不仅是静态字面量容器，还可以在语言内部被构造、拆分、查询长度——覆盖了自举阶段对数据结构增删查的基本需求。
