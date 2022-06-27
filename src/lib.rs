//! Parameterized routing for generic resources in Rust.
#![doc(html_root_url = "https://docs.rs/usher/0.2.1")]

// exposed modules
pub mod capture;
pub mod matcher;
pub mod node;
pub mod parser;
pub mod router;

// lift extensions
mod extensions;
pub use extensions::*;

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
    pub use super::matcher::Matcher;
    pub use super::parser::{DynamicParser, Parser, StaticParser};
    pub use super::router::Router;
}
