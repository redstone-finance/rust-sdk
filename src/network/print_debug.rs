#[macro_export]
macro_rules! print_debug {
    ($fmt:expr) => {
        #[cfg(feature = "print_debug")]
        todo!("This will be fixed")
    };
    ($fmt:expr, $($args:tt)*) => {
        #[cfg(feature = "print_debug")]
        todo!("This will be fixed")
    };
}

#[macro_export]
macro_rules! print_and_panic {
    ($fmt:expr) => {{
        $crate::print_debug!($fmt);
        panic!($fmt)
    }};
    ($fmt:expr, $($args:tt)*) => {{
        $crate::print_debug!($fmt, $($args)*);
        panic!($fmt, $($args)*)
    }};
}
