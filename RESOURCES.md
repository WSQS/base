# 实现自己的编程语言 Resources

## Knowledge

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
- [Book: _Compilers: Principles, Techniques, and Tools_](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)
  编译原理经典参考。Use for: 需要系统查阅词法、语法、语义分析与代码生成概念时。
- [Project: _chibicc_ — Rui Ueyama](https://github.com/rui314/chibicc)
  通过大量小提交把一个极小 C 编译器逐步长成可自举实现。Use for: 学习“真实语言如何沿着增量提交一路成长到自举”的工程节奏。
- [Project: _CC500_](https://github.com/8l/cc500)
  极小 self-hosting C 编译器案例。Use for: 理解“真正能自举”的语言/编译器最少会涉及哪些工程能力，如字符数组、符号表、I/O 与少量运行时支持。
- [Project: _M2-Planet_](https://github.com/oriansj/M2-Planet/blob/master/HACKING)
  bootstrappable builds 语境下的极小 C 编译器。Use for: 研究自举不仅是语言特性问题，还涉及构建阶段、信任链和受限宿主环境。
- [Article: “How I wrote a self-hosting C compiler in 40 days”](https://www.sigbus.info/how-i-wrote-a-self-hosting-c-compiler-in-40-days)
  从工程过程角度解释一个编译器如何从很小的子集逐渐长到能编译自己。Use for: 参考“先完成小闭环，再逐步吞回自己”的实现节奏。
- [Article: “Bootstrapping (compilers)” — Wikipedia](https://en.wikipedia.org/wiki/Bootstrapping_%28compilers%29)
  总览自举、交叉编译、阶段编译等基本概念。 Use for: 建立“自举是分阶段过程，而不是某个静态特性清单”的总框架。
- [Project: _MicroJack_](https://courses.cs.washington.edu/courses/cse390b/20sp/projects/microjack.html)
  一个更小的教学语言/项目子集，保留变量、赋值、简单运算、条件、循环和 I/O。Use for: 观察“介于最小闭环与编译器可表达能力之间”的课程化能力边界。
- [Course: _Nand2Tetris Project 9 / Jack_](https://www.nand2tetris.org/project09)
  展示一个简单但真实的高级语言及其配套小系统如何作为教学落点。Use for: 参考“小语言要长成更完整系统时，哪些能力会很快变得必要”。
- [Project Collection: _PL Zoo_](https://github.com/andrejbauer/plzoo)
  收集多个小语言实现，适合横向比较特性选择如何影响解释器/编译器结构。Use for: 做语言特性取舍时参考不同 prior art。
- [Article: “Parsing Expressions by Precedence Climbing” — Eli Bendersky](https://eli.thegreenplace.net/2012/08/02/parsing-expressions-by-precedence-climbing)
  清晰解释表达式解析。Use for: 学习如何实现运算符优先级和结合性。
- [Article: “Pratt Parsers: Expression Parsing Made Easy” — Bob Nystrom](https://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/)
  Pratt 解析的经典入门文章。Use for: 设计可扩展表达式语法时。

## Wisdom (Communities)

- [r/ProgrammingLanguages](https://www.reddit.com/r/ProgrammingLanguages/)
  活跃且相关度高的社区。Use for: 看别人如何设计小语言、获得实现思路、了解常见坑。
- [LangDev Stack Exchange](https://langdev.stackexchange.com/)
  适合讨论语言设计与实现细节。Use for: 当你遇到具体的语法、语义或实现取舍问题时提问或检索。
- [Discussion: “What are the benefits to self-hosting compilers?” — LangDev Stack Exchange](https://langdev.stackexchange.com/questions/912/what-are-the-benefits-to-self-hosting-compilers)
  适合看到社区如何讨论 self-hosting 的真实收益与代价，而不是把它神化成默认终点。Use for: 在长期路线设计时校准“为什么追求自举”。
- [Discussion: “What’s the smallest subset of language features you need to bootstrap its compiler?” — Stack Overflow](https://stackoverflow.com/questions/47356651/whats-the-smallest-subset-of-language-features-you-need-to-bootstrap-its-compil)
  展示“理论下界”和“工程可行性”之间的差异。Use for: 防止把图灵完备误当成可维护自举语言的设计标准。
- [Compiler Explorer community resources](https://godbolt.org/)
  虽然主要面向编译结果观察，但能帮助建立语言到目标表示的直觉。Use for: 后续接触编译型实现时扩展视野。

## Gaps

- 还缺少更偏中文、适合快速复习的编译原理/解释器资料
- 还缺少直接对比“Phase 0 最小闭环 / Phase 1 算法能力 / Phase 2 编译器可表达能力”的中文整理资料
- 还缺少更偏中文、适合快速复习的编译原理/解释器资料
