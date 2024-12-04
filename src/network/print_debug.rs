extern crate alloc;

#[macro_export]
macro_rules! print_debug {
    ($fmt:expr) => {
        #[cfg(feature = "print_debug")]
        $crate::network::specific::print(format!($fmt))
    };
    ($fmt:expr, $($args:tt)*) => {
        #[cfg(feature = "print_debug")]
        $crate::network::specific::print(format!($fmt, $($args)*))
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
