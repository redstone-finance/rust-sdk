pub mod as_str;
pub mod error;
pub mod print_debug;
use alloc::string::String;

// Todo: extend with logging capabilities etc.

/// Environment in which the code executes.
pub trait Environment {
    /// Environment specific print function.
    fn print<F: FnOnce() -> String>(print_content: F);
}

/// Default and standard implementation of the `Environmet` trait.
/// Uses panic and println macros in implementation of trait function.
pub struct StdEnv;

#[cfg(feature = "std")]
impl Environment for StdEnv {
    fn print<F: FnOnce() -> String>(print_content: F) {
        println!("{}", print_content())
    }
}
