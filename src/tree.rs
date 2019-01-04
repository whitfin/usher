use http::Method;

use super::matcher::{RoutingMatcher, StaticMatcher};
use super::node::RoutingNode;

/// Represents a tree used to route requests.
///
/// The tree itself represents nothing more than builder methods and
/// stores the matchers used to compare against segments as they're
/// routed. The root node is simply a recursive `RoutingNode` struct.
#[derive(Debug)]
pub struct RoutingTree<T> {
    root: RoutingNode<T>,
    matchers: Vec<Box<RoutingMatcher>>,
}

impl<T> RoutingTree<T> {
    /// Creates a new `RoutingTree` with default matchers.
    pub fn new() -> Self {
        Self::new_with_matchers(vec![
            // default literal matcher
            Box::new(StaticMatcher),
        ])
    }

    /// Creates a new `RoutingTree` with provided matchers.
    pub fn new_with_matchers(matchers: Vec<Box<RoutingMatcher>>) -> Self {
        Self {
            matchers,
            root: RoutingNode::new("/".to_owned()),
        }
    }

    /// Inserts a route/handler pair for the provided path and method.
    pub fn insert(&mut self, method: Method, path: &str, t: T) {
        let mut current = &mut self.root;

        for segment in path.split('/').filter(|s| *s != "") {
            let child = current
                .children()
                .iter()
                .find(|child| child.segment() == segment);

            if child.is_none() {
                current.add_child(RoutingNode::new(segment.to_owned()));
            }

            current = current
                .children_mut()
                .iter_mut()
                .find(|child| child.segment() == segment)
                .unwrap();
        }

        current.add_handler(&method, t);
    }

    /// Attempts to route a method/path combination to a handler in the tree.
    pub fn route(&self, method: &Method, path: &str) -> Option<&T> {
        let mut current = &self.root;

        'segment: for segment in path.split('/').filter(|s| *s != "") {
            for child in current.children() {
                for matcher in &self.matchers {
                    if matcher.is_match(&child.segment(), segment) {
                        current = child;
                        continue 'segment;
                    }
                }
            }
        }

        current.handler(method)
    }

    /// Shrinks this tree to the minimal amount of memory possible.
    pub fn shrink(&mut self) {
        self.root.shrink();
    }
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
