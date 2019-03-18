//! Routing abstractions based on custom rulesets.
//!
//! Although the behaviour implemented in this module is quite minimal, it
//! should be applicable in many use cases due to the generic typing and
//! tries to avoid tying itself to any particular mental model (except for
//! a generic tree structure). The structures in this module can be used
//! directly, but would typically provide more value as the underlying
//! routing for more domain oriented structures.
use crate::matcher::Matcher;
use crate::node::Node;
use crate::parser::Parser;

/// Routing structure providing routing for generic types.
///
/// A `Router` is constructed from a set of `Parser` values, which are used to
/// construct the shape of the structure internally. During construction of a
/// `Router`, these parsers are used to turn path segments into `Matcher` types
/// which are used in routing to calculate the traversal of the tree.
///
/// The order of the provided `Parser` values is important as it defines the order
/// they're checked against a path segment. If a parser matching any segment is
/// placed first, it will always match and short circuit before checking any other
/// provided parsers. Always put the "strictest" parsers first in the vector.
pub struct Router<T> {
    root: Node<T>,
    parsers: Vec<Box<Parser>>,
}

impl<T> Router<T> {
    /// Creates a new `Router`, using the provided matchers.
    pub fn new(mut parsers: Vec<Box<Parser>>) -> Self {
        parsers.shrink_to_fit();

        let parsed = parse_segment(&parsers, "/");
        let parsed = parsed.expect("unparsed segment");

        Self {
            parsers,
            root: Node::new(parsed),
        }
    }

    /// Inserts a route/handler pair for the provided path and method.
    ///
    /// Internally this is pretty similar to `update`, except that it guarantees
    /// that the provided value `t` is stored as the leaf value. If the leaf already
    /// contains a value, it will be overwritten. If this is not desired, you can
    /// likely implement the insertion easily via `update` instead.
    #[inline(always)]
    pub fn insert(&mut self, path: &str, t: T) {
        self.update(path, |_| t)
    }

    /// Attempts to route a method/path combination.
    ///
    /// This function will also capture any parameters involved in routing, into a
    /// `Vec` which is returned inside the containing `Option`. Each capture consists
    /// of a name and value, to help identify the matched parameter. Whilst this is
    /// easily determined as the vector is ordered, it's helpful for those who wish
    /// to turn captures into a map-like structure afterward.
    ///
    /// If a route does not require any parameters, this vector is still returned but
    /// is empty. This isn't a big deal; a `Vec` will only allocate memory when you
    /// first push something into it in most cases, so the performance hit is minimal.
    pub fn lookup<'a>(&self, path: &'a str) -> Option<(&T, Vec<(&str, &'a str)>)> {
        let mut current = None;
        let mut captures = Vec::new();

        for segment in path.split('/').filter(|s| *s != "") {
            current = current
                .unwrap_or(&self.root)
                .children()
                .iter()
                .find(|child| child.matcher().is_match(segment));

            if current.is_none() {
                break;
            }

            if let Some(capture) = current.unwrap().matcher().capture(segment) {
                captures.push(capture);
            }
        }

        current
            .and_then(|node| node.value())
            .map(|handler| (handler, captures))
    }

    /// Updates a leaf node inside a `Router`.
    ///
    /// If the node does not currently exist, it will be built out and populated
    /// with the result of the update function (which can be used to generate a
    /// value for first insertion).
    pub fn update<F>(&mut self, path: &str, f: F)
    where
        F: FnOnce(Option<T>) -> T,
    {
        let mut current = &mut self.root;

        for segment in path.split('/').filter(|s| *s != "") {
            let child = current
                .children()
                .iter()
                .find(|child| child.matcher().is_match(segment));

            if child.is_none() {
                let parsed = parse_segment(&self.parsers, segment);
                let parsed = parsed.expect("unparsed segment");
                let router = Node::new(parsed);

                current.add_child(router);
            }

            current = current
                .children_mut()
                .iter_mut()
                .find(|child| child.matcher().is_match(segment))
                .unwrap();
        }

        current.update(f);
    }
}

/// Attempts to parse a `Matcher` based on the provided segment literal.
///
/// All provided parsers will be tested (in order) against the input segment to enable
/// passing the most "specific" parsers earlier in the chain. In the case a `Matcher`
/// is found, this function will short circuit and pass the first matcher back to the caller.
fn parse_segment(parsers: &[Box<Parser>], segment: &str) -> Option<Box<Matcher>> {
    parsers.iter().find_map(|parser| parser.parse(segment))
}
