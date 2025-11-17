#[macro_export]
macro_rules! score_write {
    ($dst:expr, $($arg:tt)*) => {
        write($dst, mw_log_format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! score_writeln {
    ($dst:expr $(,)?) => {
        $crate::score_write!($dst, "\n")
    };
    ($dst:expr, $($arg:tt)*) => {
        write($dst, mw_log_format_args_nl!($($arg)*))
    };
}
