# Mission: 实现 base 编程语言

## Why
我希望尝试自己实现一些有趣的编程语言，并通过亲手构建语言来真正理解它们是如何工作的。

当前目标语言本身的状态、边界与近期演化方向，记录在 [LANGUAGE.md](LANGUAGE.md) 中；本文件负责描述学习与项目推进的 mission，而不是完整承载语言规格。

## Success looks like
- 已独立实现一个最小但完整可运行的语言闭环，并在此基础上继续稳定扩展
- 能解释并亲手实现词法分析、语法分析、AST、求值/执行这几个核心环节，并逐步把结构整理得更清晰
- 以“小步扩展 + 及时整理”的方式持续增强语言能力，并在扩展过程中逐步做出更明确的语言设计选择
- 让目标语言沿着可自举的方向逐步演化，并进一步为 LLVM 后端探索积累基础

## Phase Plan

语言遵循动态、表达式优先、模式匹配优先的路线。Phase 按 gate/exit criteria 划分，每个 Phase 的边界是"下一批语言特性可以在不重写前一层契约的情况下加入"。

### Phase 0：Frontend closed loop ✓
**目标**：源码能稳定 tokenize + parse 成 Program/AST。
**退出标志**：scanner、parser、AST 的错误边界清楚；测试可覆盖合法和非法输入。
**状态**：已完成。

### Phase 1：Executable core ✓
**目标**：一段完整程序可走完 scanner → parser → AST → evaluator。
**已具备**：Integer/Boolean/String/Fn 四种值、算术/比较/match/函数字面量与调用表达、表达式语句、builtin 调用；`print` 已降级为 builtin；模块已拆分；warning 清零（仅保留 2 个防护性 `_ =>`）。
**状态**：已完成。

### Phase 2：Self-hosting prerequisites（当前）
**目标**：补齐自举所需的最小语言能力集。
**内容**：数据结构（list/vector）、递归、block/scope、文件 I/O、错误处理、script mode。
**退出标志**：可写 20–50 行脚本读取文件、操作数据、作递归计算。

### Phase 3：Minimal self-hosting
**目标**：用本语言写 scanner/parser/evaluator 子集，编译/解释自身。
**退出标志**：Gen0 运行源码得到 Gen1；Gen1 运行同一源码得到一致输出。

### Phase 4：Type system experiment
**目标**：以 optional/static-analysis pass 形式试验显式类型注解 + monomorphic checker。
**不做**：HM 推导、泛型、trait、subtyping。

## Constraints
- 目前学过一点词法/语法分析
- 希望教学风格理论和实践并重
- 当前阶段采用“小步扩展 + 及时整理”的推进方式，而不是先堆大量功能再统一重构
- 当前扩展应优先选择既能增强语言表达能力、又能帮助语言逐步形成更清晰设计、并为长期自举路径积累核心结构的内容
- 当前结构路线：scanner → parser → AST → evaluator，各模块已拆分到独立文件
- 语言路线：动态、表达式优先、模式匹配优先、语句系统最小化（`let` + 表达式语句）

## Out of scope
- 现在先不追求工业级优化器、JIT 或完整编译器工具链
- 现在先不追求设计一门大而全的通用编程语言
- Phase 4 之前不做完整类型系统（HM 推导、泛型、trait 等）
