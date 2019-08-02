//! Parsers used to create `Matcher` values from segments.
//!
//! Values of type `Parser` are required to construct `Matcher` values
//! at tree creation time, to specify priority order when routing an
//! incoming set of segments. A parser can also be a pure function which
//! can derive a potential `Matcher` from an input segment directly.
use crate::matcher::{DynamicMatcher, Matcher, StaticMatcher};

/// Parsing trait to enable conversion from literals into matchers.
///
/// This is used to run through a cascading parsing flow to enable custom
/// matchers being implemented. This trait enables all segment matching to
/// be determined at creation time to avoid any costs at routing time.
pub trait Parser: Send + Sync {
    /// Attempts to parse a `Matcher` out of a segment.
    fn parse(&self, segment: &str) -> Option<Box<dyn Matcher>>;
}

/// Blanket implementation of `Parser` for pure functions.
impl<F> Parser for F
where
    F: Fn(&str) -> Option<Box<dyn Matcher>> + Send + Sync,
{
    /// Attempts to parse a `Matcher` out of a segment.
    fn parse(&self, segment: &str) -> Option<Box<dyn Matcher>> {
        self(segment)
    }
}

/// Segment parser to generate static route matchers.
pub struct StaticParser;

/// `Parser` implementation for the static matcher.
impl Parser for StaticParser {
    /// Parses out a static matcher from a segment literal.
    ///
    /// Note that although this returns a result, it will never fail
    /// as every string literal can be treated as a static matcher.
    fn parse(&self, segment: &str) -> Option<Box<dyn Matcher>> {
        Some(Box::new(StaticMatcher::new(segment)))
    }
}

/// Segment parser to generate dynamic router matchers.
pub struct DynamicParser;

impl Parser for DynamicParser {
    /// Parses out a dynamic segment based on the `:.+` syntax.
    ///
    /// If you wish to use a custom syntax, you can construct a custom `Parser`
    /// implementation which constructs a `DynamicMatcher` instance.
    fn parse(&self, segment: &str) -> Option<Box<dyn Matcher>> {
        if &segment[0..1] != ":" || segment.len() == 1 {
            return None;
        }

        let field = &segment[1..];
        let matcher = DynamicMatcher::new(field);

        Some(Box::new(matcher))
    }
}
