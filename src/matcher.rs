//! Matchers used to compare against incoming segments.
//!
//! Values of type `Matcher` are stored inside a tree and used to match
//! against incoming segments in order to walk through the tree correctly.

/// Matching trait to enable generic route matching algorithms.
///
/// This trait backs the main tree, enabling custom segment matching based
/// on the needs of the end developer. In many cases it's wasteful to check
/// for things like RegEx, especially when all routes will only be static
/// (as an example).
pub trait Matcher {
    /// Retrieves a potential capture from a segment.
    fn capture<'a>(&self, _segment: &'a str) -> Option<(&str, &'a str)> {
        None
    }

    /// Determines whether an incoming segment is a match for a base segment.
    fn is_match(&self, segment: &str) -> bool;
}

/// Blanket implementation of `Matcher` for pure functions.
///
/// Pure functions are assumed to not have a capture group, as there's no
/// way to directly name them at this point (unless derived from the input).
impl<F> Matcher for F
where
    F: Fn(&str) -> bool,
{
    /// Determines whether an incoming segment is a match for a base segment.
    fn is_match(&self, segment: &str) -> bool {
        self(segment)
    }
}

/// Static path segment matcher.
///
/// This struct is constructed via the `StaticParser` and compares ncoming
/// segments directly against the internal static `String` segment.
pub struct StaticMatcher {
    inner: String,
}

impl StaticMatcher {
    /// Constructs a new `StaticMatcher` from a segment.
    pub fn new<S: Into<String>>(s: S) -> Self {
        Self { inner: s.into() }
    }
}

impl Matcher for StaticMatcher {
    /// Compares an incoming segment against a literal base segment.
    fn is_match(&self, segment: &str) -> bool {
        &self.inner == segment
    }
}
