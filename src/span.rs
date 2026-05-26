use std::{cmp::Ordering, fmt, hash::Hash, ops::Range};

/// A span of source code, represented as a range of byte offsets.
///
/// Spans are cheap to copy as they only contain byte offsets.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    /// Create a new span from byte offsets.
    pub fn new(start: usize, end: usize) -> Self {
        Span { start, end }
    }

    /// Create a span that covers two spans.
    pub fn join(self, other: impl std::borrow::Borrow<Span>) -> Span {
        let other = other.borrow();
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Get the start byte offset.
    pub fn start(&self) -> usize {
        self.start
    }

    /// Get the end byte offset.
    pub fn end(&self) -> usize {
        self.end
    }

    /// Get the byte range.
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Get the source text covered by this span.
    pub fn source_text(self, full_source: &str) -> &str {
        let start = self.start.min(full_source.len());
        let end = self.end.min(full_source.len());
        &full_source[start..end]
    }

    /// Get the (line, column) of the start position (1-indexed).
    pub fn start_line_col(self, source: &str) -> (usize, usize) {
        let offset = self.start.min(source.len());
        let mut line = 1;
        let mut col = 1;
        for (i, ch) in source.char_indices() {
            if i >= offset {
                break;
            }
            if ch == '\n' {
                line += 1;
                col = 1;
            } else {
                col += 1;
            }
        }
        (line, col)
    }

    /// Create a call site span (no source tracking).
    pub fn call_site() -> Self {
        Span { start: 0, end: 0 }
    }
}

impl PartialOrd for Span {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Span {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.start.cmp(&other.start) {
            Ordering::Equal => self.end.cmp(&other.end),
            ord => ord,
        }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Span({}..{})", self.start, self.end)
    }
}

impl Default for Span {
    fn default() -> Self {
        Self::call_site()
    }
}
