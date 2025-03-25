#[macro_export]
macro_rules! error {
    ($($arg:expr),+) => {{
        $crate::internal::call!($crate::Console, error, $($arg),+);
    }};
}
