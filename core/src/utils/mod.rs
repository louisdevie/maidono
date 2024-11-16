mod action_path;
mod error;
mod location;
pub mod macros;
pub mod path;
mod report;

pub use action_path::{ActionPath, ActionPathPattern};
pub use error::{Error, ErrorPrinter, Result};
pub use location::Location;
pub(crate) use report::Report;

pub(crate) fn split_in_two(string: &str, c: char) -> (&str, Option<&str>) {
    let mut parts = string.splitn(2, c);
    (parts.next().unwrap(), parts.next())
}
