use std::{
    borrow::Cow,
    fmt::{Result, Write},
};

/// Number of space used to indent lines when no alignment is required.
pub(crate) const INDENTATION_SIZE: usize = 2;

#[derive(Debug, Default)]
pub(crate) struct List(Vec<Block>, Decoration);

impl List {
    /// Render this instance using the formatter `f`.
    ///
    /// Indent each line of output by `indentation` spaces.
    pub(crate) fn render(&self, f: &mut dyn Write, indentation: usize) -> Result {
        self.render_with_prefix(f, indentation, "".into())
    }

    /// Append a new [`Block`] containing `literal`.
    ///
    /// The input `literal` is split into lines so that each line will be
    /// indented correctly.
    pub(crate) fn push_literal(&mut self, literal: Cow<'static, str>) {
        self.0.push(literal.into());
    }

    fn render_with_prefix(
        &self,
        f: &mut dyn Write,
        indentation: usize,
        prefix: Cow<'static, str>,
    ) -> Result {
        if self.0.is_empty() {
            return Ok(());
        }

        let enumeration_padding = self.enumeration_padding();

        self.0[0].render(
            f,
            indentation,
            self.full_prefix(0, enumeration_padding, &prefix).into(),
        )?;
        for (index, block) in self.0[1..].iter().enumerate() {
            writeln!(f)?;
            block.render(
                f,
                indentation + prefix.len(),
                self.prefix(index + 1, enumeration_padding),
            )?;
        }
        Ok(())
    }

    fn full_prefix(&self, index: usize, enumeration_padding: usize, prior_prefix: &str) -> String {
        format!("{prior_prefix}{}", self.prefix(index, enumeration_padding))
    }

    fn prefix(&self, _: usize, _: usize) -> Cow<'static, str> {
        match self.1 {
            Decoration::None => "".into(),
        }
    }

    fn enumeration_padding(&self) -> usize {
        match self.1 {
            Decoration::None => 0,
        }
    }
}

#[derive(Debug)]
enum Block {
    /// A block of text.
    ///
    /// Each constituent [`Fragment`] contains one line of text. The lines are
    /// indented uniformly to the current indentation of this block when
    /// rendered.
    Literal(Vec<Fragment>),
}

impl Block {
    fn render(&self, f: &mut dyn Write, indentation: usize, prefix: Cow<'static, str>) -> Result {
        match self {
            Self::Literal(fragments) => {
                if fragments.is_empty() {
                    return Ok(());
                }

                write!(f, "{:indentation$}{prefix}", "")?;
                fragments[0].render(f)?;
                let block_indentation = indentation + prefix.as_ref().len();
                for fragment in &fragments[1..] {
                    writeln!(f)?;
                    write!(f, "{:block_indentation$}", "")?;
                    fragment.render(f)?;
                }
                Ok(())
            }
        }
    }
}

impl From<String> for Block {
    fn from(value: String) -> Self {
        Block::Literal(
            value
                .lines()
                .map(|v| Fragment(v.to_string().into()))
                .collect(),
        )
    }
}

impl From<&'static str> for Block {
    fn from(value: &'static str) -> Self {
        Block::Literal(value.lines().map(|v| Fragment(v.into())).collect())
    }
}

impl From<Cow<'static, str>> for Block {
    fn from(value: Cow<'static, str>) -> Self {
        match value {
            Cow::Borrowed(value) => value.into(),
            Cow::Owned(value) => value.into(),
        }
    }
}

/// A string representing one line of a description or match explanation.
#[derive(Debug)]
struct Fragment(Cow<'static, str>);

impl Fragment {
    fn render(&self, f: &mut dyn Write) -> Result {
        write!(f, "{}", self.0)
    }
}

/// The decoration which appears on [`Block`] of a [`List`] when rendered.
#[derive(Debug, Default)]
enum Decoration {
    /// No decoration on each [`Block`]. The default.
    #[default]
    None,
}
