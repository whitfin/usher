use http::Method;
use usher::http::HttpRouter;
use usher::prelude::*;

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
    /// so we return a tuple of `("id", <segment>)` to represent the capture.
    fn capture<'a>(&self, segment: &'a str) -> Option<(&str, &'a str)> {
        Some((&self.inner, segment))
    }

    /// Determines if this matcher matches the incoming segment.
    ///
    /// Because the segment is dynamic and matches any value, this is able to
    /// always return `true` without even looking at the incoming segment.
    fn is_match(&self, _segment: &str) -> bool {
        true
    }
}

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

fn main() {
    let mut router: HttpRouter<()> = HttpRouter::new(vec![
        Box::new(DynamicParser),
        Box::new(StaticParser),
    ]);

    router.get("/", ());
    router.get("/status", ());
    router.get("/api/v1/user", ());
    router.post("/api/v1/user", ());
    router.put("/api/v1/user/:id", ());

    println!("GET /: {:?}", router.handler(&Method::GET, "/"));
    println!("GET /status: {:?}", router.handler(&Method::GET, "/status"));
    println!("GET /api: {:?}", router.handler(&Method::GET, "/api"));
    println!("GET /api/v1: {:?}", router.handler(&Method::GET, "/api/v1"));
    println!(
        "GET /api/v1/user: {:?}",
        router.handler(&Method::GET, "/api/v1/user")
    );
    println!(
        "PUT /api/v1/user: {:?}",
        router.handler(&Method::PUT, "/api/v1/user")
    );
    println!(
        "POST /api/v1/user: {:?}",
        router.handler(&Method::POST, "/api/v1/user")
    );
    println!(
        "GET /api/v1/user/steve: {:?}",
        router.handler(&Method::GET, "/api/v1/user/steve")
    );
    println!(
        "PUT /api/v1/user/steve: {:?}",
        router.handler(&Method::PUT, "/api/v1/user/steve")
    );
}
