use http::Method;

use super::matcher::{RoutingMatcher, SegmentParser, StaticSegmentParser};
use super::node::RoutingNode;

/// Represents a tree used to route requests.
///
/// The tree itself represents nothing more than builder methods and
/// stores the matchers used to compare against segments as they're
/// routed. The root node is simply a recursive `RoutingNode` struct.
#[derive(Debug)]
pub struct RoutingTree<T> {
    root: RoutingNode<T>,
    parsers: Vec<Box<SegmentParser>>,
}

impl<T> RoutingTree<T> {
    /// Creates a new `RoutingTree` with default matchers.
    pub fn new() -> Self {
        Self::with_parsers(vec![Box::new(StaticSegmentParser)])
    }

    /// Creates a new `RoutingTree` with provided matchers.
    pub fn with_parsers(parsers: Vec<Box<SegmentParser>>) -> Self {
        let parsed = parse_segment(&parsers, "/");
        let parsed = parsed.expect("unparsed segment");

        Self {
            parsers,
            root: RoutingNode::new(parsed),
        }
    }

    /// Inserts a route/handler pair for the provided path and method.
    pub fn insert(&mut self, method: Method, path: &str, t: T) {
        let mut current = &mut self.root;

        for segment in path.split('/').filter(|s| *s != "") {
            let child = current
                .children()
                .iter()
                .find(|child| child.matcher().is_match(segment));

            if child.is_none() {
                let parsed = parse_segment(&self.parsers, segment);
                let parsed = parsed.expect("unparsed segment");
                let router = RoutingNode::new(parsed);

                current.add_child(router);
            }

            current = current
                .children_mut()
                .iter_mut()
                .find(|child| child.matcher().is_match(segment))
                .unwrap();
        }

        current.add_handler(&method, t);
    }

    /// Attempts to route a method/path combination to a handler in the tree.
    pub fn route(&self, method: &Method, path: &str) -> Option<&T> {
        let mut current = None;

        'segment: for segment in path.split('/').filter(|s| *s != "") {
            for child in current.unwrap_or(&self.root).children() {
                if child.matcher().is_match(segment) {
                    current = Some(child);
                    continue 'segment;
                }
            }
        }

        current.and_then(|node| node.handler(method))
    }

    /// Shrinks this tree to the minimal amount of memory possible.
    pub fn shrink(&mut self) {
        self.root.shrink();
    }
}

/// Attempts to parse a `RoutingMatcher` based on the provided segment literal.
///
/// All provided parsers will be tested (in order) against the input segment to enable
/// passing the most "specific" parsers earlier in the chain. In the case a `RoutingMatcher`
/// is found, this function will short circuit and pass the first matcher back to the caller.
fn parse_segment(parsers: &[Box<SegmentParser>], segment: &str) -> Option<Box<RoutingMatcher>> {
    parsers.iter().find_map(|parser| parser.parse(segment))
}

/// Delegates a HTTP method to the `route` method in a tree.
macro_rules! http_delegate {
    ($name:ident, $method:expr) => {
        #[inline]
        pub fn $name(&mut self, path: &str, t: T) {
            self.insert($method, path, t)
        }
    };
}

// Automatic HTTP method delegates.
impl<T> RoutingTree<T> {
    http_delegate!(connect, Method::CONNECT);
    http_delegate!(delete, Method::DELETE);
    http_delegate!(get, Method::GET);
    http_delegate!(head, Method::HEAD);
    http_delegate!(options, Method::OPTIONS);
    http_delegate!(patch, Method::PATCH);
    http_delegate!(post, Method::POST);
    http_delegate!(put, Method::PUT);
    http_delegate!(trace, Method::TRACE);
}
