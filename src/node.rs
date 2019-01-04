use http::Method;

use std::collections::HashMap;

/// Represents a node inside the main tree.
///
/// Nodes contain their segment literal, any handlers associated with
/// the current leaf, and any child nodes associated with this segment.
/// This structure is recursive to form a tree of nodes.
#[derive(Debug)]
pub struct RoutingNode<T> {
    segment: String,
    handlers: HashMap<Method, T>,
    children: Vec<RoutingNode<T>>,
}

impl<T> RoutingNode<T> {
    /// Constructs a new `RoutingNode` from a literal.
    pub(crate) fn new(segment: String) -> Self {
        Self {
            segment,
            handlers: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Registers a child node inside this node.
    pub(crate) fn add_child(&mut self, child: RoutingNode<T>) {
        self.children.push(child);
    }

    /// Registers a handler for a given method inside this node.
    pub(crate) fn add_handler(&mut self, method: &Method, t: T) {
        self.handlers.insert(method.to_owned(), t);
    }

    /// Retrieves a reference to the children of this node.
    pub(crate) fn children(&self) -> &[RoutingNode<T>] {
        &self.children
    }

    /// Retrieves a mutable reference to the children of this node.
    pub(crate) fn children_mut(&mut self) -> &mut [RoutingNode<T>] {
        &mut self.children
    }

    /// Retrieves a potential handler associated with the provided method.
    pub(crate) fn handler(&self, method: &Method) -> Option<&T> {
        self.handlers.get(method)
    }

    /// Retrieves the segment literal for this node.
    pub(crate) fn segment(&self) -> &str {
        &self.segment
    }

    /// Shrinks this node to the minimal amount of memory possible.
    pub(crate) fn shrink(&mut self) {
        self.children.shrink_to_fit();
        self.handlers.shrink_to_fit();
        for child in &mut self.children {
            child.shrink();
        }
    }
}
