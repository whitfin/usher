//! Types and traits related to parameter capturing.
//!
//! Currently this module is a placeholder for new traits coming later; the
//! only things stored in this module at this point are simple type aliases
//! (which are extremely likely to change in future, so don't rely on them).

/// Basic type alias for a captured value pair.
pub type Capture<'a> = (&'a str, (usize, usize));

/// Alias type for a set of multiple `Capture` values.
pub type Captures<'a> = Vec<Capture<'a>>;
pub type CapturesRef<'a> = &'a [Capture<'a>];

/// Retrieves a potential captured value from a parameter set by name.
///
/// This function uses the provided path and captures to locate a value set against
/// the provided name. If multiple values exist, only the first value will be found.
///
/// This function will panic if the bounds provided are invalid for the provided path,
/// although this should never happen in reality unless you're mocking captures.
#[inline]
pub fn find_capture<'a, 'p>(path: &'p str, capt: CapturesRef<'a>, name: &str) -> Option<&'p str> {
    capt.iter()
        .find(|(n, _)| *n == name)
        .map(|(_, bounds)| &path[(bounds.0)..(bounds.1)])
}
