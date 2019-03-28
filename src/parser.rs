//! Parsers used to create `Matcher` values from segments.
//!
//! Values of type `Parser` are required to construct `Matcher` values
//! at tree creation time, to specify priority order when routing an
//! incoming set of segments. A parser can also be a pure function which
//! can derive a potential `Matcher` from an input segment directly.
use crate::matcher::{Matcher, StaticMatcher};

/// Parsing trait to enable conversion from literals into matchers.
///
/// This is used to run through a cascading parsing flow to enable custom
/// matchers being implemented. This trait enables all segment matching to
/// be determined at creation time to avoid any costs at routing time.
pub trait Parser {
    /// Attempts to parse a `Matcher` out of a segment.
    fn parse(&self, segment: &str) -> Option<Box<Matcher>>;
}

/// Blanket implementation of `Parser` for pure functions.
impl<F> Parser for F
where
    F: Fn(&str) -> Option<Box<Matcher>>,
{
    /// Attempts to parse a `Matcher` out of a segment.
    fn parse(&self, segment: &str) -> Option<Box<Matcher>> {
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
    fn parse(&self, segment: &str) -> Option<Box<Matcher>> {
        Some(Box::new(StaticMatcher::new(segment.to_owned())))
    }
}
