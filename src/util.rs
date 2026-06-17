#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        println!("[{}:{}] {}", file!(), line!(), format_args!($($arg)*))
    };
}