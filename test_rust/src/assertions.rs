#[macro_export]
macro_rules! verify_that {
    // general case:
    ($actual:expr, $expected:expr $(,)?) => {{
        use $crate::assertions::internal::Subject as _;
        $actual.check($expected, stringify!($actual))
    }};
}

pub mod internal {
    use crate::internal::test_outcome::TestAssertionFailure;
    use crate::matcher::{Matcher, MatcherResult, create_assertion_failure};
    use std::fmt::Debug;

    pub trait Subject: Copy + Debug {
        fn check(
            self,
            expected: impl Matcher<Self>,
            actual_expr: &'static str,
        ) -> Result<(), TestAssertionFailure> {
            match expected.matches(self) {
                MatcherResult::Match => Ok(()),
                MatcherResult::NoMatch => {
                    Err(create_assertion_failure(&expected, self, actual_expr))
                }
            }
        }
    }

    impl<T: Copy + Debug> Subject for T {}
}
