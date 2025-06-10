use crate::description::Description;
use crate::matcher::{Matcher, MatcherBase, MatcherResult};
use crate::matcher_support::edit_distance;
use crate::matcher_support::summarize_diff::create_diff;
use std::fmt::Debug;

pub fn eq<T>(expected: T) -> EqMatcher<T> {
    EqMatcher { expected }
}

/// A matcher which matches a value equal to `expected`.
///
/// See [`eq`].
#[derive(MatcherBase)]
pub struct EqMatcher<T> {
    pub(crate) expected: T,
}

impl<T: Debug, A: Debug + Copy + PartialEq<T>> Matcher<A> for EqMatcher<T> {
    fn matches(&self, actual: A) -> MatcherResult {
        (actual == self.expected).into()
    }

    fn describe(&self, matcher_result: MatcherResult) -> Description {
        match matcher_result {
            MatcherResult::Match => format!("is equal to {:?}", self.expected).into(),
            MatcherResult::NoMatch => format!("isn't equal to {:?}", self.expected).into(),
        }
    }

    fn explain_match(&self, actual: A) -> Description {
        let expected_debug = format!("{:#?}", self.expected);
        let actual_debug = format!("{actual:#?}");
        let description = Matcher::<A>::describe(self, self.matches(actual));

        let diff = if is_multiline_string_debug(&actual_debug)
            && is_multiline_string_debug(&expected_debug)
        {
            create_diff(
                // The two calls below return None if and only if the strings expected_debug
                // respectively actual_debug are not enclosed in ". The calls to
                // is_multiline_string_debug above ensure that they are. So the calls cannot
                // actually return None and unwrap() should not panic.
                &to_display_output(&actual_debug).unwrap(),
                &to_display_output(&expected_debug).unwrap(),
                edit_distance::Mode::Exact,
            )
        } else {
            create_diff(&actual_debug, &expected_debug, edit_distance::Mode::Exact)
        };

        if diff.is_empty() {
            format!("which {description}").into()
        } else {
            format!("which {description}\n\n{diff}").into()
        }
    }
}

fn is_multiline_string_debug(string: &str) -> bool {
    string.starts_with('"')
        && string.ends_with('"')
        && !string.contains('\n')
        && string.contains("\\n")
}

fn to_display_output(string: &str) -> Option<String> {
    Some(
        string
            .strip_prefix('"')?
            .strip_suffix('"')?
            .split("\\n")
            .collect::<Vec<_>>()
            .join("\n"),
    )
}

#[cfg(test)]
mod tests {
    use crate::Result;
    use crate::prelude::*;

    #[test]
    fn eq_matches_string_reference_with_string_reference() -> Result<()> {
        verify_that!("A string", eq("A string"))
    }

    #[test]
    fn eq_matches_owned_string_with_string_reference() -> Result<()> {
        let value = "A string".to_string();
        verify_that!(value, eq("A string"))
    }

    #[test]
    fn eq_matches_owned_string_reference_with_string_reference() -> Result<()> {
        let value = "A string".to_string();
        verify_that!(&value, eq("A string"))
    }

    #[test]
    fn eq_matches_i32_with_i32() -> Result<()> {
        verify_that!(123, eq(123))
    }
}
