use std::{
    borrow::Cow,
    fmt::{Display, Formatter, Result as FmtResult}
};

/// Java中的文档注释
#[derive(Debug, PartialEq)]
pub struct DocumentationComment<'a>(Cow<'a, str>);

impl<'a> Display for DocumentationComment<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "/**{}*/\n", self.0)
    }
}

impl<'a> From<&'a str> for DocumentationComment<'a> {
    fn from(value: &'a str) -> Self {
        Self (value.into())
    }
}

impl<'a> From<String> for DocumentationComment<'a> {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}