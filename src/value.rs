use crate::ast::Expr;

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Boolean(bool),
    String(String),
    Fn { params: Vec<String>, body: Expr },
    BuiltinFn(fn(Vec<Value>) -> Value),
    List(Vec<Value>),
}
