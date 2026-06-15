# Introduce boolean literals

用户已经把布尔值从“只能由比较表达式间接产生”的状态，推进成了源语言中的一级字面量：`true` / `false` 现在能被 scanner 识别、被 parser 解析、被 evaluator 求值，并进入环境与输出。这意味着语言的值模型在源语言层面终于变得对称，后续再引入最小 `match` 时，布尔分支将不再依赖比较表达式作为唯一入口。