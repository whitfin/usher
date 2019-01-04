//! Generic request routing for web services in Rust.
#![doc(html_root_url = "https://docs.rs/usher/0.1.0")]

// exposed modules
pub mod matcher;
pub mod node;
pub mod tree;

// prelude module
pub mod prelude {
    //! A "prelude" for crates using the `usher` crate.
    //!
    //! This prelude contains the required imports for almost all use cases, to
    //! avoid having to include modules and structures directly:
    //!
    //! ```rust
    //! use usher::prelude::*;
    //! ```
    //!
    //! The prelude may grow over time, but it is unlikely to shrink.
    pub use super::matcher::{RoutingMatcher, StaticMatcher};
    pub use super::tree::RoutingTree;
}
