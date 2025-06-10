use crate::internal::description_renderer::{INDENTATION_SIZE, List};
use std::borrow::Cow;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Default)]
pub struct Description {
    elements: List,
    initial_indentation: usize,
}

impl Description {
    /// Indents the lines in elements of this description.
    pub fn indent(self) -> Self {
        Self {
            initial_indentation: INDENTATION_SIZE,
            ..self
        }
    }
}

impl Display for Description {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.elements.render(f, self.initial_indentation)
    }
}

impl<T: Into<Cow<'static, str>>> From<T> for Description {
    fn from(value: T) -> Self {
        let mut elements = List::default();
        elements.push_literal(value.into());
        Self {
            elements,
            ..Default::default()
        }
    }
}
