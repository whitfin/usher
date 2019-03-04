use std::fmt::Debug;

/// Matching trait to enable generic route matching algorithms.
///
/// This trait backs the main tree, enabling custom segment matching based
/// on the needs of the end developer. In many cases it's wasteful to check
/// for things like RegEx, especially when all routes will only be static
/// (as an example).
pub trait RoutingMatcher: Debug {
    /// Retrieves a potential capture from a segment.
    fn capture<'a>(&self, _segment: &'a str) -> Option<&'a str> {
        None
    }

    /// Determines whether an incoming segment is a match for a base segment.
    fn is_match(&self, segment: &str) -> bool;
}

/// Parsing trait to enable conversion from literals into matchers.
///
/// This is used to run through a cascading parsing flow to enable custom
/// matchers being implemented. This trait enables all segment matching to
/// be determined at creation time to avoid any costs at routing time.
pub trait SegmentParser: Debug {
    /// Attempts to parse a `RoutingMatcher` out of a segment.
    fn parse(&self, segment: &str) -> Option<Box<RoutingMatcher>>;
}

/// Segment parser to generate static route matchers.
#[derive(Debug)]
pub struct StaticSegmentParser;

/// `SegmentParser` implementation for the static matcher.
impl SegmentParser for StaticSegmentParser {
    /// Parses out a static matcher from a segment literal.
    ///
    /// Note that although this returns a result, it will never fail
    /// as every string literal can be treated as a static matcher.
    fn parse(&self, segment: &str) -> Option<Box<RoutingMatcher>> {
        Some(Box::new(StaticMatcher {
            inner: segment.to_owned(),
        }))
    }
}

/// Static path segment matcher.
///
/// This struct is constructed via the `StaticSegmentParser` and compares
/// incoming segments directly against the internal static `String` segment.
#[derive(Debug)]
pub struct StaticMatcher {
    inner: String,
}

impl RoutingMatcher for StaticMatcher {
    /// Compares an incoming segment against a literal base segment.
    fn is_match(&self, segment: &str) -> bool {
        &self.inner == segment
    }
}

/// Blanket implementation of `SegmentParser` for pure functions.
impl<F> SegmentParser for F
where
    F: Fn(&str) -> Option<Box<RoutingMatcher>> + Debug,
{
    /// Attempts to parse a `RoutingMatcher` out of a segment.
    fn parse(&self, segment: &str) -> Option<Box<RoutingMatcher>> {
        self(segment)
    }
}
