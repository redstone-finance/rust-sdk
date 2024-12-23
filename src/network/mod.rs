pub mod as_str;
pub mod error;
pub mod print_debug;

// Todo: extend with logging capabilities etc.

/// Environment in which the code executes.
pub trait Environment {
    /// Environment specific revert function. Expected to halt execution of the program.
    fn revert<F: FnOnce() -> String>(revert_msg: F);

    /// Environment specific print function.
    fn print<F: FnOnce() -> String>(print_content: F);
}

/// Default and standard implementation of the `Environmet` trait.
/// Uses panic and println macros in implementation of trait function.
pub struct StdEnv;

impl Environment for StdEnv {
    fn revert<F: FnOnce() -> String>(revert_msg: F) {
        panic!("{}", revert_msg());
    }

    fn print<F: FnOnce() -> String>(print_content: F) {
        println!("{}", print_content())
    }
}
