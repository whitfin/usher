//! Extension implementations based on top of a `Router`.
//!
//! These structures are applicable in different circumstances, depending on
//! what the developer is trying to do. Each extension is disabled by default
//! and opt-in via build features. Whether these extensions live in this crate
//! in future is yet to be determined; so be prepared for the possibility that
//! this module disappear at some point in future (prior to v1.0).
//!
//! See the documentation for each extension to find the feature necessary to
//! include the module. In general the name of the extension will match the
//! feature, but this isn't always possible due to some Cargo behaviour.
#[cfg(feature = "web")]
pub mod http;
