#[macro_export]
macro_rules! not_cancelled_println {
    ($cancel:expr) => {
        if !$cancel.is_cancelled() {
            println!();
        }
    };

    ($cancel:expr, $msg:literal) => {
        if !$cancel.is_cancelled() {
            println!($msg);
        }
    };

    ($cancel:expr, $fmt:literal, $($arg:tt)*) => {
        if !$cancel.is_cancelled() {
            println!($fmt, $($arg)*);
        }
    };
}

#[macro_export]
macro_rules! concat_arrays {
    ($($arr:expr),+ $(,)?) => {{
        let mut vec = Vec::new();
        $(vec.extend_from_slice(&$arr);)+
        vec
    }};
}
