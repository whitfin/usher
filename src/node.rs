//! Nodes to represent the internal structure of a router.
use super::matcher::Matcher;

/// Node structure to represent the internal structure of a router.
///
/// A router is simply a `Node` which doesn't have any parent nodes,
/// which allows for the recursive structure of the tree. Each
/// `Node` can have a value of the generic type, which is the value
/// returned when routing occurs.
///
/// Every `Node` also has an associated `Matcher` which is used
/// to test for compatibility when routing (because not every node
/// is applicable on a given segment order). This `Matcher` is
/// automatically provided to the `Node` at creation time and is
/// calculated by the routing system.
///
/// Lastly, a `Node` can have child instances to represent the
/// recursive structure of a router. These children are stored in
/// a `Vec` as there's currently no logical way to index them into
/// a more suitable structure. If a `Node` has no children, the
/// containing vector does not require any memory allocation. Any
/// memory will be allocated lazily, and should remain minimal in
/// most standard cases (as it depends on the allocator in use).
pub struct Node<T> {
    value: Option<T>,
    matcher: Box<Matcher>,
    children: Vec<Node<T>>,
}

impl<T> Node<T> {
    /// Constructs a new `Node` from a literal.
    pub(crate) fn new(matcher: Box<Matcher>) -> Self {
        Self {
            matcher,
            value: None,
            children: Vec::new(),
        }
    }

    /// Registers a child node inside this node.
    pub(crate) fn add_child(&mut self, child: Node<T>) {
        self.children.reserve_exact(1);
        self.children.push(child);
    }

    /// Retrieves a reference to the children of this node.
    pub(crate) fn children(&self) -> &[Node<T>] {
        &self.children
    }

    /// Retrieves a mutable reference to the children of this node.
    pub(crate) fn children_mut(&mut self) -> &mut [Node<T>] {
        &mut self.children
    }

    /// Retrieves the matching struct for this node.
    pub(crate) fn matcher(&self) -> &dyn Matcher {
        &*self.matcher
    }

    /// Updates the inner value of this routing
    pub(crate) fn update<F>(&mut self, f: F)
    where
        F: FnOnce(Option<T>) -> T,
    {
        let t = f(self.value.take());
        self.value.replace(t);
    }

    /// Retrieves a potential handler associated with the provided method.
    pub(crate) fn value(&self) -> Option<&T> {
        self.value.as_ref()
    }
}
