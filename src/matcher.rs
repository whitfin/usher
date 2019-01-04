use std::fmt::Debug;

/// Matching trait to enable generic route matching algorithms.
///
/// This trait backs the main tree, enabling custom segment matching based
/// on the needs of the end developer. In many cases it's wasteful to check
/// for things like RegEx, especially when all routes will only be static
/// (as an example).
pub trait RoutingMatcher: Debug + Send {
    /// Determines whether an incoming segment is a match for a base segment.
    fn is_match(&self, base: &str, incoming: &str) -> bool;
}

/// Static path segment matcher.
#[derive(Debug)]
pub struct StaticMatcher;
impl RoutingMatcher for StaticMatcher {
    /// Compares the incoming segment against a literal base segment.
    fn is_match(&self, base: &str, incoming: &str) -> bool {
        base == incoming
    }
}
