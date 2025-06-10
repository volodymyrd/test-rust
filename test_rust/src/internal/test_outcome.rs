use std::fmt::{Debug, Display, Error, Formatter};

/// A code location.
///
/// `std::panic::Location` does not provide a constructor, hence we cannot
/// construct a fake value.
///
/// **For internal use only. API stablility is not guaranteed!**
#[doc(hidden)]
#[derive(Clone)]
enum Location {
    Real(&'static std::panic::Location<'static>),
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::Real(l) => write!(f, "{l}"),
        }
    }
}

/// A report that a single test assertion failed.
#[derive(Clone)]
pub struct TestAssertionFailure {
    /// A human-readable formatted string describing the error.
    pub description: String,
    pub custom_message: Option<String>,
    location: Location,
}

impl TestAssertionFailure {
    /// Creates a new instance with the given `description`.
    pub fn create(description: String) -> Self {
        Self {
            description,
            custom_message: None,
            location: Location::Real(std::panic::Location::caller()),
        }
    }
}

impl Display for TestAssertionFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        writeln!(f, "{}", self.description)?;
        if let Some(custom_message) = &self.custom_message {
            writeln!(f, "{custom_message}")?;
        }
        writeln!(f, "  at {}", self.location)
    }
}

// The standard Rust test harness outputs the TestAssertionFailure with the
// Debug trait. We want the output to be formatted, so we use a custom Debug
// implementation which defers to Display.
impl Debug for TestAssertionFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(self, f)
    }
}
