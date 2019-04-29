# Usher
[![Build Status](https://img.shields.io/travis/whitfin/usher.svg)](https://travis-ci.org/whitfin/usher)
[![Crates.io](https://img.shields.io/crates/v/usher.svg)](https://crates.io/crates/usher)
[![Coverage Status](https://img.shields.io/coveralls/github/whitfin/usher.svg)](https://coveralls.io/github/whitfin/usher)

Usher provides an easy way to construct parameterized routing trees in Rust.

The nodes of these trees is naturally generic, allowing Usher to lend itself
to a wide variety of use cases. Matching and parameterization rules are defined
by the developer using a simple set of traits, allowing for customization in
the routing algorithm itself. This provides easy support for various contexts
in which routing may be used.

This project was born of a personal need for something small to sit on top of
[Hyper](https://hyper.rs/), without having to work with a whole framework. Over
time it became clear that it provides utility outside of the HTTP realm, and so
the API was adapted to become more generic. As such, Usher provides several
"extensions" based on certain domains which essentially provide sugar over a
typical router. These extensions are all off by default, but can easily be set
as enabled via Cargo features.

Prior to v1.0 you can expect the API to receive some changes, although I will
do my best to keep this to a minimum to reduce any churn involved. One choice
that is perhaps going to change is the API around using non-filesystem based
pathing. Other than that expect changes as optimizations (and the likely API
refactoring associated with them) still need to be fully investigated.

### Getting Started

Usher is available on [crates.io](https://crates.io/crates/usher). The easiest
way to use it is to add an entry to your `Cargo.toml` defining the dependency:

```toml
[dependencies]
usher = "0.1"
```

If you require any of the Usher extensions, you can opt into them by setting the
feature flags in your dependency configuration:

```toml
usher = { version = "0.1", features = ["web"] }
```

You can find the available extensions in the documentation.

### Basic Usage

The construction of a tree is quite simple, depending on what your desired outcome
is. To construct a very basic/static tree, you can simply insert the routes you
care about:

```rust
use usher::prelude::*;

fn main() {
    // First we construct our `Router` using a set of parsers. Fortunately for
    // this example, Usher includes the `StaticParser` which uses basic string
    // matching to determine whether a segment in the path matches.
    let mut router: Router<String> = Router::new(vec![
        Box::new(StaticParser),
    ]);

    // Then we insert our routes; in this case we're going to store the numbers
    // 1, 2 and 3, against their equivalent name in typed English (with a "/"
    // as a prefix, as Usher expects filesystem-like paths (for now)).
    router.insert("/one", "1".to_string());
    router.insert("/two", "2".to_string());
    router.insert("/three", "3".to_string());

    // Finally we'll just do a lookup on each path, as well as the a path which
    // doesn't match ("/"), just to demonstrate what the return types look like.
    for path in vec!["/", "/one", "/two", "/three"] {
        println!("{}: {:?}", path, router.lookup(path));
    }
}
```

This will route exactly as it looks; matching each static segment provided against
the tree and retrieving the value associated with the path. The return type of the
`lookup(path)` function is `Option<(&T, Vec<(&str, (usize, usize)>)>`, with `&T`
referring to the generic value provided (`"1"`, etc), and the `Vec` including a set
of any parameters found during routing. In the case of no parameters, this vector
will be empty (as is the case above).

For usage based around extensions (such as HTTP), please see the documentation for
the module containing it - or visit the examples directory for actual usage.

### Advanced Usage

Of course, for some use cases you need to be able to control more than statically
matching against the path segments. In a web framework, you might allow for some
path segments which match regardless and simply capture their value (i.e. `:id`).
In order to allow this type of usage, there are two traits available in Usher; the
`Parser` and `Matcher` traits. These two traits can be implemented to describe how
to match against specific segments in an incoming path.

The `Matcher` trait is used to determine if an incoming path segment matches a
configured path segment. It's also responsible for pulling out any capture that
is associated with the incoming segment. The `Parser` trait is used to calculate
which `Matcher` type should be used on a configured path segment. At a glance it
might seem that these two traits could be combined but the difference is that the
`Parser` trait operates at router creation time, whereas the `Matcher` trait exists
for execution when matching against a created router.

To demonstrate these traits, we can use the `:id` example of a typical web framework.
The concept of this syntax is that it should match any value provided to the tree.
If my router was configured with the path `/:id`, it would match incoming paths of
`/123` and `/abc` (but not `/`). This would provide a captured value `id` which holds
the value `123` or `abc`.

#### Matcher

This pattern is pretty simple to implement using the two traits we defined above.
First of all we must construct our `Matcher` type (technically you might write the
`Parser` first, but it's easier to explain in this order). Fortunately, the rules
here are very simple.

```rust
/// A `Matcher` type used to match against dynamic segments.
///
/// The internal value here is the name of the path parameter (based on the
/// example talked through above, this would be the _owned_ `String` of `"id"`).
pub struct DynamicMatcher {
    inner: String
}

impl Matcher for DynamicMatcher {
    /// Determines if there is a capture for the incoming segment.
    ///
    /// In the pattern we described above the entire value becomes the capture,
    /// so we return a tuple of `("id", (start, end))` to represent the capture.
    fn capture(&self, segment: &str) -> Option<(&str, (usize, usize))> {
        Some((&self.inner, (0, segment.len())))
    }

    /// Determines if this matcher matches the incoming segment.
    ///
    /// Because the segment is dynamic and matches any value, this is able to
    /// always return `true` without even looking at the incoming segment.
    fn is_match(&self, _segment: &str) -> bool {
        true
    }
}
```

This implementation is fairly trivial and should be quite self-explanatory; the
matcher matches anything so `is_match/1` will always return true. We always want
to capture the segment, so that's returned from `capture/1`. A couple of things
to mention about captures;

- An implementation of `capture/1` is option, as it will default to `None`.
- The `capture/1` implementation is only called if `is_match/1` resolved to `true`.
- The tuple structure used for captures is necessary as we need some way to know
  the name of the captures at runtime. The names cannot be stored in the router
  itself as there may be use cases where the capture name is actually a function
  of the incoming path segment (not in this case specifically, of course).

#### Parser

Now that we have our `Matcher` type, we need to construct a `Parser` type in
order to associate the configured segments with the correct `Matcher`. This is
pretty trivial in our case, because pretty much the only rule we have is that
the segment must be of the pattern `:.+`, which we can roughly translate to
`starts_with(":")` for example purposes. As such, a `Parser` type might look
like this:

```rust
/// A `Parser` type used to parse out `DynamicMatcher` values.
pub struct DynamicParser;

impl Parser for DynamicParser {
    /// Attempts to parse a segment into a corresponding `Matcher`.
    ///
    /// As a dynamic segment is determined by the pattern `:.+`, we check the first
    /// character of the segment. If the segment is not `:` we are unable to parse
    /// and so return a `None` value.
    ///
    /// If it does start with a `:`, we construct a `DynamicMatcher` and pass the
    /// parameter name through as it's used when capturing values.
    fn parse(&self, segment: &str) -> Option<Box<Matcher>> {
        if &segment[0..1] != ":" {
            return None;
        }

        let field = &segment[1..];
        let matcher = DynamicMatcher {
            inner: field.to_owned()
        };

        Some(Box::new(matcher))
    }
}
```

One of the nice things about splitting the traits is that you can switch up the
syntax easily. Although both `DynamicMatcher` and `DynamicParser` are included
in Usher, you might want to use a different syntax. One other example of syntax
for parameters (I think in the Java realm) is `{id}`. To accomodate this case,
you only have to write a new `Parser` implementation; the existing `Matcher`
struct already works!

```rust
/// A customer `Parser` type used to parse out `DynamicMatcher` values.
pub struct CustomDynamicParser;

impl Parser for CustomDynamicParser {
    /// Attempts to parse a segment into a corresponding `Matcher`.
    ///
    /// This will match segments based on `{id}` syntax, rather than `:id`. We have
    /// to check the end characters, and pass back the something in the middle!
    fn parse(&self, segment: &str) -> Option<Box<Matcher>> {
        // has to start with "{"
        if &segment[0..1] != "{" {
            return None;
        }

        // has to end with "}"
        if &segment[(len - 1)..] != "}" {
            return None;
        }

        // so 1..(len - 1) trim the brackets
        let field = &segment[1..(len - 1)];
        let matcher = DynamicMatcher::new(field);

        // wrap it up!
        Some(Box::new(matcher))
    }
}
```

Of course, this also makes it trivial to match _either_ of the two forms shown
above. You can attach both parsers to your tree at startup, and it will allow
for both `:id` and `{id}`. This flexibility can be definitely be useful when
writing more involved frameworks, using Usher as the underlying routing layer.

#### Configuration

Now we have our types, we have to actually configure them in a router in order
for them to take effect. This is done at router initialization time, and you've
already seen an example of this in the basic example where we provide the basic
`StaticParser` type. Much like this example, we pass our parser in directly:

```rust
let mut router: Router<String> = Router::new(vec![
    Box::new(DynamicParser),
    Box::new(StaticParser),
]);
```

Using this definition, our new `Parser` will be used to determine if we can parse
dynamic segments from the path. Below is a demonstration of a simple path which
makes use of both matcher types (`S` dictates a static segment, and `D` dictates
a dynamic segment):

```
/api/user/:id
  ^   ^    ^
  |   |    |
  S   S    D
```

Please note that the order the parsers are provided is very important; you should
place the most "specific" parsers first as they are tested in order. If you placed
`StaticParser` first in the list above, then nothing would ever continue through to
the `DynamicParser` as every segment satisfies the `StaticParser` requirements.
