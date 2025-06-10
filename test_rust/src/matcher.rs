pub use test_rust_macro::MatcherBase;

use crate::description::Description;
use crate::internal::test_outcome::TestAssertionFailure;
use std::fmt::Debug;

/// The result of applying a [`Matcher`] on an actual value.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MatcherResult {
    /// The actual value matches according to the [`Matcher`] definition.
    Match,
    /// The actual value does not match according to the [`Matcher`] definition.
    NoMatch,
}

impl MatcherResult {
    /// Returns `true` if `self` is [`MatcherResult::Match`], otherwise
    /// `false`.
    pub fn is_match(self) -> bool {
        matches!(self, MatcherResult::Match)
    }

    /// Returns `true` if `self` is [`MatcherResult::NoMatch`], otherwise
    /// `false`.
    pub fn is_no_match(self) -> bool {
        matches!(self, MatcherResult::NoMatch)
    }
}

impl From<bool> for MatcherResult {
    fn from(b: bool) -> Self {
        if b {
            MatcherResult::Match
        } else {
            MatcherResult::NoMatch
        }
    }
}

impl From<MatcherResult> for bool {
    fn from(matcher_result: MatcherResult) -> Self {
        matcher_result.is_match()
    }
}

pub trait MatcherBase {}

pub trait Matcher<ActualT: Debug + Copy>: MatcherBase {
    fn matches(&self, actual: ActualT) -> MatcherResult;
    fn describe(&self, matcher_result: MatcherResult) -> Description;
    fn explain_match(&self, actual: ActualT) -> Description {
        format!("which {}", self.describe(self.matches(actual))).into()
    }
}

/// Any actual value whose debug length is greater than this value will be
/// pretty-printed. Otherwise, it will have normal debug output formatting.
const PRETTY_PRINT_LENGTH_THRESHOLD: usize = 60;

/// Constructs a [`TestAssertionFailure`] reporting that the given `matcher`
/// does not match the value `actual`.
///
/// The parameter `actual_expr` contains the expression which was evaluated to
/// obtain `actual`.
#[track_caller]
pub(crate) fn create_assertion_failure<T: Debug + Copy>(
    matcher: &impl Matcher<T>,
    actual: T,
    actual_expr: &'static str,
) -> TestAssertionFailure {
    let actual_formatted = format!("{actual:?}");
    let actual_formatted = if actual_formatted.len() > PRETTY_PRINT_LENGTH_THRESHOLD {
        format!("{actual:#?}")
    } else {
        actual_formatted
    };
    TestAssertionFailure::create(format!(
        "\
Value of: {actual_expr}
Expected: {}
Actual: {actual_formatted},
{}",
        matcher.describe(MatcherResult::Match),
        matcher.explain_match(actual).indent(),
    ))
}
