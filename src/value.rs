#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Boolean(bool),
    String(String),
}