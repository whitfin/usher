use http::Method;
use usher::ext::http::HttpRouter;
use usher::prelude::*;

#[derive(Debug)]
pub struct DynamicMatcher(String);
impl Matcher for DynamicMatcher {
    fn capture<'a>(&self, segment: &'a str) -> Option<(&str, &'a str)> {
        Some((&self.0, segment))
    }

    fn is_match(&self, _segment: &str) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct DynamicParser;
impl Parser for DynamicParser {
    fn parse(&self, segment: &str) -> Option<Box<Matcher>> {
        if &segment[0..1] != ":" {
            return None;
        }
        let name = (&segment[1..]).to_owned();
        Some(Box::new(DynamicMatcher(name)))
    }
}

fn main() {
    let mut router: HttpRouter<()> = HttpRouter::new(vec![
        // dynamic `:id` patterns
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
