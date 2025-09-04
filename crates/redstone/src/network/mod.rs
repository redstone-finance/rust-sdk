pub mod as_str;
pub mod error;

use alloc::string::String;

/// Environment in which the code executes.
pub trait Environment {
    /// Environment specific print function.
    fn print<F: FnOnce() -> String>(print_content: F);
}

/// Default and standard implementation of the `Environmet` trait.
/// Uses println in std environment otherwise does nothing
pub struct StdEnv;

impl Environment for StdEnv {
    fn print<F: FnOnce() -> String>(_print_content: F) {
        #[cfg(feature = "std")]
        println!("{}", _print_content())
    }
}

/// Dummy implementation, does nothing.
pub struct Dummy;

impl Environment for Dummy {
    fn print<F: FnOnce() -> String>(_print_content: F) {}
}
