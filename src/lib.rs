use clap::Parser;

#[derive(Parser)]
/// Basics of Derive API.
pub struct Args {
    /// Explanation of the first argument.
    pub one: String,
}
