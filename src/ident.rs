use std::{
    fmt,
    hash::{Hash, Hasher},
};

use crate::span::Span;

/// A Java identifier.
///
/// Similar to `proc_macro2::Ident` in syn, but uses a `String` internally
/// since we are not in a procedural macro context.
#[derive(Clone)]
pub struct Ident {
    pub name: String,
    pub span: Span,
}

impl Ident {
    /// Create a new identifier.
    pub fn new(name: String, span: Span) -> Self {
        Ident { name, span }
    }

    /// Create a new identifier from a `&str`.
    pub fn new_str(name: &str, span: Span) -> Self {
        Ident {
            name: name.to_string(),
            span,
        }
    }

    /// Get the name of the identifier.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the span of the identifier.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Create a dummy identifier.
    pub fn dummy(name: &str) -> Self {
        Ident {
            name: name.to_string(),
            span: Span::call_site(),
        }
    }
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Ident {}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ident {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl fmt::Debug for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ident({:?})", self.name)
    }
}

impl fmt::Display for Ident {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        &self.name
    }
}

impl std::borrow::Borrow<str> for Ident {
    fn borrow(&self) -> &str {
        &self.name
    }
}

/// Construct an identifier from a string literal.
///
/// # Panics
/// Panics if the string is not a valid Java identifier.
#[macro_export]
macro_rules! format_ident {
    ($fmt:expr) => {
        $crate::Ident::new_str($fmt, $crate::span::Span::call_site())
    };
    ($fmt:expr, $($arg:tt)*) => {{
        let s = std::fmt::format(std::format_args!($fmt, $($arg)*));
        $crate::Ident::new(s, $crate::span::Span::call_site())
    }};
}

/// Check if a character can start a Java identifier.
pub fn is_java_identifier_start(ch: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_start(ch) || ch == '$' || ch == '_'
}

/// Check if a character can be part of a Java identifier.
pub fn is_java_identifier_continue(ch: char) -> bool {
    unicode_xid::UnicodeXID::is_xid_continue(ch) || ch == '$'
}

/// Validate that a string is a valid Java identifier.
#[allow(dead_code)]
pub fn is_valid_java_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let mut chars = s.chars();
    if !is_java_identifier_start(chars.next().unwrap()) {
        return false;
    }
    chars.all(is_java_identifier_continue)
}

/// Validate that a string is a valid Java type identifier
/// (not a contextual keyword like `record`, `sealed`, `var`, `yield`, `permits`).
#[allow(dead_code)]
pub fn is_valid_type_identifier(s: &str) -> bool {
    is_valid_java_identifier(s)
        && !matches!(s, "record" | "sealed" | "var" | "yield" | "permits" | "_")
}
