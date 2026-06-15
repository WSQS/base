# 实现自己的编程语言 Resources

## Knowledge

### Foundations: interpreters, compilers, and mental models

- [Book: _Crafting Interpreters_ — Robert Nystrom](https://craftinginterpreters.com/)
  高质量、可直接上手的解释器实现教材。Use for: 从零实现词法分析、语法分析、AST、解释执行，以及设计最小语言原型。
- [Book: _Essentials of Compilation_ — Jeremy G. Siek](https://mitpress.mit.edu/9780262047760/essentials-of-compilation/)
  强调按特性逐步增长编译器：从整数和变量起步，再引入布尔、控制流、堆数据、垃圾回收、函数和词法作用域。Use for: 设计“Phase 0 -> Phase 1 -> Phase 2”这类渐进式语言/编译器路线。
- [Book: _Compiler Construction_ — Niklaus Wirth](https://people.inf.ethz.ch/wirth/CompilerConstruction/CompilerConstruction1.pdf)
  通过 Oberon-0 展示一个“小但具有代表性”的教学语言如何支撑编译器实现。Use for: 参考“一个未来可扩展到编译器本体的小语言，通常至少包含哪些基础元素”。
- [Book: _Writing Interpreters in Rust: a Guide_](https://rust-hosted-langs.github.io/book/introduction.html)
  以 Rust 为宿主语言，讨论解释器/虚拟机实现中会遇到的内存管理、抽象边界和工程问题。Use for: 把当前 Rust 实现继续扩展到更完整的语言系统。
- [Book: _Structure and Interpretation of Computer Programs_](https://sarabander.github.io/sicp/)
  经典教材，帮助理解语言设计、求值模型与抽象。Use for: 建立“语言为什么这样工作”的深层心智模型。

### Language design principles and small-language thinking
- [Talk/Essay: “Growing a Language” — Guy L. Steele Jr.](https://www.cs.virginia.edu/~evans/cs655/readings/steele.pdf)
  从语言如何“长出来”的角度讨论核心能力、可组合抽象和扩展路径。Use for: 判断当前阶段是该加入一个硬编码语法特性，还是先建立一个能承载后续扩展的核心机制。
- [Paper: “Hints on Programming Language Design” — C. A. R. Hoare](https://flint.cs.yale.edu/cs428/doc/HintsPL.pdf)
  经典语言设计原则文章，强调简单性、清晰性、可理解性和设计约束。Use for: 在加入赋值、控制流、作用域等能力前，检查该特性是否让语言概念更清楚，还是只是增加表面功能。
- [Paper: “On the Design of Programming Languages” — Niklaus Wirth](https://people.csail.mit.edu/feser/pld-s23/Wirth_Design.pdf)
  Wirth 关于语言设计原则的文章，适合和 Oberon-0、Pascal 这类“小而完整”的语言一起看。Use for: 校准“简单”不是功能贫乏，而是概念透明、边界明确、结构一致。
- [Paper: “The Next 700 Programming Languages” — Peter J. Landin](https://www.cs.cmu.edu/~crary/819-f09/Landin66.pdf)
  讨论如何用统一的语义核心承载大量表面语言差异。Use for: 区分“语言表面语法”与“核心求值模型”，避免每加一个语法糖都污染解释器/编译器核心。
- [Report: _Revised5 Report on the Algorithmic Language Scheme_](https://people.eecs.berkeley.edu/~bh/61a-pages/Volume2/r5rs.pdf)
  Scheme 的小核心设计报告，展示 primitive forms、derived forms、词法作用域、尾调用等如何组织成清晰语言。Use for: 参考“哪些形式必须进核心，哪些可以作为派生形式或库形式晚一点加入”。
- [Book: _Programming Languages: Application and Interpretation_ — Shriram Krishnamurthi](https://cs.brown.edu/courses/cs173/2012/book/)
  用解释器逐步解释变量、作用域、状态、函数、递归等语言概念，并反复讨论设计选择。Use for: 在当前小语言扩展表达式、语句、赋值和作用域时，把“怎么实现”转化为“语义到底是什么”。
- [Book: _Essentials of Programming Languages_ — Friedman, Wand, Haynes](https://mitpress.mit.edu/9780262062794/essentials-of-programming-languages/)
  通过一系列小语言解释器逐步加入变量、作用域、状态、控制等能力。Use for: 参考“每加入一个语言机制，都要同步更新语法、环境/存储模型和求值规则”的设计纪律。
- [Guide: “Creating Languages” — The Racket Guide](https://docs.racket-lang.org/guide/languages.html)
  Racket 官方文档中关于创建语言的章节，强调语言可以作为可组合、可限制、可扩展的抽象边界。Use for: 思考当前语言未来是否需要宏、子语言、模块或 DSL 边界，而不是只把语言当作固定语法集合。
- [Book/Guide: _Beautiful Racket_ — Matthew Butterick](https://beautifulracket.com/)
  以 Racket 为工具讲 language-oriented programming，适合从设计小语言的角度看“为什么要做一门语言”。Use for: 训练“先定义语言边界和使用场景，再决定语法和实现”的思维方式。
- [Book: _Compilers: Principles, Techniques, and Tools_](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)
  编译原理经典参考。Use for: 需要系统查阅词法、语法、语义分析与代码生成概念时。

### Incremental growth, staging, and educational compiler structure
- [Project: _chibicc_ — Rui Ueyama](https://github.com/rui314/chibicc)
  通过大量小提交把一个极小 C 编译器逐步长成可自举实现。Use for: 学习“真实语言如何沿着增量提交一路成长到自举”的工程节奏。
- [Project: _CC500_](https://github.com/8l/cc500)
  极小 self-hosting C 编译器案例。Use for: 理解“真正能自举”的语言/编译器最少会涉及哪些工程能力，如字符数组、符号表、I/O 与少量运行时支持。
- [Project: _M2-Planet_](https://github.com/oriansj/M2-Planet/blob/master/HACKING)
  bootstrappable builds 语境下的极小 C 编译器。Use for: 研究自举不仅是语言特性问题，还涉及构建阶段、信任链和受限宿主环境。
- [Site: _Bootstrappable Builds_](https://www.bootstrappable.org/)
  关注从极小可信种子逐步构建完整软件栈的项目与实践集合。Use for: 把“将来可自举”拆成更具体的问题：种子语言、阶段链、可复现构建、信任边界和最小依赖。
- [Article: “Finding the Bottom Turtle”](https://blog.dave.tf/post/finding-bottom-turtle/)
  解释 stage0、Mes、M2-Planet 等项目如何从很小的可验证起点逐步搭建编译链。Use for: 理解自举路径不是一次跳到 self-hosting，而是设计一条能逐步减少外部依赖的阶梯。
- [Paper: “Reflections on Trusting Trust” — Ken Thompson](https://www.cs.cmu.edu/~rdriley/487/papers/Thompson_1984_ReflectionsonTrustingTrust.pdf)
  经典 Turing Award lecture，说明编译器自举和信任链之间的安全问题。Use for: 防止把 self-hosting 神话化；在长期目标中区分“能编译自己”和“构建链可信”。
- [Paper: “Countering Trusting Trust through Diverse Double-Compiling” — David A. Wheeler](https://dwheeler.com/trusting-trust/)
  讨论通过 diverse double-compiling 检查编译器二进制与源码是否对应。Use for: 后期研究自举可信性时参考；当前阶段只需理解它提醒我们保留清晰、可替换的构建阶段。
- [Article: “How I wrote a self-hosting C compiler in 40 days”](https://www.sigbus.info/how-i-wrote-a-self-hosting-c-compiler-in-40-days)
  从工程过程角度解释一个编译器如何从很小的子集逐渐长到能编译自己。Use for: 参考“先完成小闭环，再逐步吞回自己”的实现节奏。
- [Article: “Bootstrapping (compilers)” — Wikipedia](https://en.wikipedia.org/wiki/Bootstrapping_%28compilers%29)
  总览自举、交叉编译、阶段编译等基本概念。Use for: 建立“自举是分阶段过程，而不是某个静态特性清单”的总框架。
- [Article: “Bootstrapping and self-hosting” — Tom Mewett](https://tmewett.com/bootstrapping-metacompiling/)
  面向实践者解释 bootstrapping、self-hosting、metacompiling 等概念之间的区别。Use for: 在路线规划时避免混淆“用别的语言写初版编译器”“目标语言能写编译器”和“完整构建链可自举”。
- [Project: _Make a Lisp_ / _mal_](https://github.com/kanaka/mal)
  用 11 个可测试步骤逐步实现一个 Lisp，最后能运行 mal 版 mal。Use for: 参考从 reader、eval、环境、函数、宏、文件加载到 self-hosting 的小步扩展路线。
- [Paper: “An Incremental Approach to Compiler Construction” — Abdulaziz Ghuloum](https://scheme2006.cs.uchicago.edu/11-ghuloum.pdf)
  展示如何从能编译极小表达式开始，每一步都保持可运行结果。Use for: 设计当前 workspace 的阶段验收标准：每次扩展一个语言能力，同时保持端到端闭环。
- [Paper/Method: “A Nanopass Infrastructure for Compiler Education” — Sarkar, Waddell, Dybvig](https://stanleymiracle.github.io/blogs/compiler/docs/extra/nanopass-infrastructure.pdf)
  把编译器拆成许多小 pass，每个 pass 都有明确的输入/输出语言。Use for: 防止语言增长后 parser、AST、语义分析和执行逻辑混成一团；尤其适合规划中间表示和特性降级。
- [Course Notes: “IMP and Operational Semantics” — UT Austin CS 345H](https://www.cs.utexas.edu/~bornholt/courses/cs345h-24sp/lectures/4-operational/)
  用小命令式语言 IMP 讲 operational semantics、状态和控制流。Use for: 在加入赋值、条件、循环前，先写清楚语句如何改变状态、表达式是否有副作用、控制流如何终止或继续。

### Bootstrapping and self-hosting path
- [Project: _MicroJack_](https://courses.cs.washington.edu/courses/cse390b/20sp/projects/microjack.html)
  一个更小的教学语言/项目子集，保留变量、赋值、简单运算、条件、循环和 I/O。Use for: 观察“介于最小闭环与编译器可表达能力之间”的课程化能力边界。
- [Course: _Nand2Tetris Project 9 / Jack_](https://www.nand2tetris.org/project09)
  展示一个简单但真实的高级语言及其配套小系统如何作为教学落点。Use for: 参考“小语言要长成更完整系统时，哪些能力会很快变得必要”。
- [Project Collection: _PL Zoo_](https://github.com/andrejbauer/plzoo)
  收集多个小语言实现，适合横向比较特性选择如何影响解释器/编译器结构。Use for: 做语言特性取舍时参考不同 prior art。

### Expression parsing techniques
- [Article: “Parsing Expressions by Precedence Climbing” — Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing)
  清晰解释表达式解析。Use for: 学习如何实现运算符优先级和结合性。
- [Article: “Pratt Parsers: Expression Parsing Made Easy” — Bob Nystrom](https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/)
  Pratt 解析的经典入门文章。Use for: 设计可扩展表达式语法时。

### Pattern matching, structured control flow, and data-directed design
- [Manual: “Patterns” — OCaml Manual](https://ocaml.org/manual/patterns.html)
  系统化介绍 literal、variable、constructor、tuple、record、list、or-pattern 等模式语法。Use for: 设计独立的 `Pattern` AST，并理解“模式不是表达式”的语言分层。
- [Docs: “Basic Data Types and Pattern Matching” — OCaml](https://ocaml.org/docs/basic-data-types)
  展示 primitive value、list、record、user-defined variant 如何与模式匹配协同工作。Use for: 规划这门语言未来如何从整数/布尔扩展到更适合结构分派的数据模型。
- [Report: “Haskell 2010 Report — Case Expressions and Pattern Matching”](https://www.haskell.org/onlinereport/haskell2010/haskellch3.html)
  用 `case` 表达式描述模式匹配核心语义，并展示其他匹配构造如何围绕它建立。Use for: 思考是否把未来的 `match` 作为核心控制流表达式，并让其他分支形式围绕它展开。
- [Docs: “Pattern Matching” — Erlang](https://www.erlang.org/doc/system/patterns.html)
  展示变量绑定、通配符与模式匹配如何成为语言运行机制的一部分。Use for: 研究“模式匹配优先”的语言如何把匹配深入到赋值、函数子句和消息处理等结构中。
- [Docs: “Functions” — Erlang](https://www.erlang.org/doc/system/ref_man_functions.html)
  展示函数子句按顺序尝试模式与 guard 的语义。Use for: 参考未来如果语言支持多子句函数，如何把它们统一降低为匹配内核。
- [Guide: “Case Expressions” — Gleam Tour](https://tour.gleam.run/flow-control/case-expressions/)
  直接把 `case` 视为最常见控制流形式之一，突出“按数据形状分派”。Use for: 参考“模式匹配优先、`if` 只是补充”的语言设计气质。
- [Guide: “Custom Types” — Elm Guide](https://guide.elm-lang.org/types/custom_types)
  强调先用 custom type 描述状态，再通过模式匹配消费这些状态。Use for: 观察“先建模可能状态，再写控制流”的设计路径。
- [Guide: “Pattern Matching” — Elm Guide](https://guide.elm-lang.org/types/pattern_matching)
  展示模式匹配如何成为处理 `Maybe`、`Result` 和自定义 variant 的默认方式。Use for: 研究当语言开始支持结构化值时，控制流如何自然转向模式分派。
- [Book Chapter: “Enums and Pattern Matching” — The Rust Book](https://doc.rust-lang.org/book/ch06-00-enums.html)
  展示 `enum`、`match`、穷尽性和 `if let` 的关系。Use for: 对比“模式匹配为主、`if` 为辅”的工程化折中设计。
- [Paper: “Warnings for Pattern Matching” — Luc Maranget](https://moscova.inria.fr/~maranget/papers/warn/warn.pdf)
  模式匹配穷尽性检查与无用分支检测的经典资料。Use for: 未来实现 `_`、constructor pattern、冗余分支警告和非穷尽匹配诊断时参考。
- [Paper: “Compiling Pattern Matching to Good Decision Trees” — Luc Maranget](https://moscova.inria.fr/~maranget/papers/ml05e-maranget.pdf)
  讲解如何把高层模式匹配编译成更高效的决策树。Use for: 当前阶段可作为中长期参考，帮助理解 `match` 最终如何从解释器结构走向更系统的实现。
- [Paper: “Views: A Way for Pattern Matching to Cohabit with Data Abstraction” — Philip Wadler](https://dl.acm.org/doi/10.1145/41625.41653)
  讨论模式匹配与数据抽象之间的张力。Use for: 在未来加入模块、抽象类型或隐藏内部表示时，提前理解模式匹配可能带来的设计压力。
- [Book: _The Implementation of Functional Programming Languages_ — Simon Peyton Jones](https://www.microsoft.com/en-us/research/wp-content/uploads/1987/01/slpj-book-1987-small.pdf)
  包含将高层模式定义转换为较小 `case` 内核的经典实现思路。Use for: 如果这门语言以后走向更完整的匹配系统，可参考如何把模式匹配降低为更核心的执行语义。

## Wisdom (Communities)

### Practice-oriented language design communities

- [r/ProgrammingLanguages](https://www.reddit.com/r/ProgrammingLanguages/)
  活跃且相关度高的社区。Use for: 看别人如何设计小语言、获得实现思路、了解常见坑。
- [LangDev Stack Exchange](https://langdev.stackexchange.com/)
  适合讨论语言设计与实现细节。Use for: 当你遇到具体的语法、语义或实现取舍问题时提问或检索。
- [Lambda the Ultimate](https://lambda-the-ultimate.org/)
  老牌 programming languages weblog，偏语言设计、语义、类型系统和 PL 研究讨论。Use for: 查找语言设计思想、历史讨论和高质量争议；适合在做较大设计取舍前扩展视野。

### Research communities and conference entry points
- [ACM SIGPLAN / PLDI](https://www.sigplan.org/Conferences/PLDI/)
  Programming Language Design and Implementation 方向的核心会议入口。Use for: 当某个设计问题进入“这个特性如何影响实现、优化、运行时或工具链”层面时查找更正式的研究参考。
- [ACM SIGPLAN / POPL](https://www.sigplan.org/Conferences/POPL/)
  Principles of Programming Languages 方向的核心会议入口。Use for: 当需要更严谨地理解语义、类型系统、程序等价、效果系统等概念时查找研究脉络。

### Design process and evolution references
- [Rust RFCs](https://github.com/rust-lang/rfcs)
  Rust 语言和生态重大变化的 RFC 记录。Use for: 学习成熟语言如何描述 motivation、rationale、drawbacks、alternatives 和 unresolved questions，并把它迁移成项目内 DESIGN_NOTES 模板。
- [Swift Evolution](https://swift.org/swift-evolution/)
  Swift 语言、标准库和包管理器演化提案集合。Use for: 观察真实语言如何把语法特性、源兼容性、迁移路径和未来方向写进同一份设计提案。
- [PEP 1: PEP Purpose and Guidelines](https://peps.python.org/pep-0001/)
  Python Enhancement Proposal 的目的和写作规范。Use for: 借鉴“每个语言特性都要写清楚问题、规范和理由”的轻量流程，避免小语言阶段随手加功能。

### Self-hosting and toolchain design discussions
- [Discussion: “What are the benefits to self-hosting compilers?” — LangDev Stack Exchange](https://langdev.stackexchange.com/questions/912/what-are-the-benefits-to-self-hosting-compilers)
  适合看到社区如何讨论 self-hosting 的真实收益与代价，而不是把它神化成默认终点。Use for: 在长期路线设计时校准“为什么追求自举”。
- [Discussion: “What’s the smallest subset of language features you need to bootstrap its compiler?” — Stack Overflow](https://stackoverflow.com/questions/47356651/whats-the-smallest-subset-of-language-features-you-need-to-bootstrap-its-compil)
  展示“理论下界”和“工程可行性”之间的差异。Use for: 防止把图灵完备误当成可维护自举语言的设计标准。
- [Compiler Explorer community resources](https://godbolt.org/)
  虽然主要面向编译结果观察，但能帮助建立语言到目标表示的直觉。Use for: 后续接触编译型实现时扩展视野。

## Gaps

- 还缺少更偏中文、适合快速复习的编程语言设计 / 解释器 / 编译器资料
- 还缺少直接对比“Phase 0 最小闭环 / Phase 1 算法能力 / Phase 2 编译器可表达能力 / Phase 3 自举候选子集”的中文整理资料
- 还缺少本项目自己的 `DESIGN.md` / `PHASES.md` / `BOOTSTRAP.md`，用于把这些外部资源转化为当前目标语言的阶段性决策记录
- 还缺少“下一步扩展语法能力”的本地评估清单，例如：是否增加表达力、是否明确语义、是否保持最小闭环、是否为未来自举积累结构
