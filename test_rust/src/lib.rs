extern crate test_rust_macro;

use internal::test_outcome::TestAssertionFailure;

#[macro_use]
pub mod assertions;
pub mod description;
pub mod internal;
pub mod matcher;
pub mod matcher_support;
pub mod matchers;

pub mod prelude {
    pub use super::matcher::{Matcher, MatcherBase};
    pub use super::matchers::*;
    // Assert macros
    pub use super::verify_that;
}

pub type Result<T> = std::result::Result<T, TestAssertionFailure>;
