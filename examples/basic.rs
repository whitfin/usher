use http::Method;
use usher::prelude::*;

#[derive(Debug)]
pub struct DynamicSegment;
impl RoutingMatcher for DynamicSegment {
    fn capture<'a>(&self, segment: &'a str) -> Option<&'a str> {
        Some(segment)
    }

    fn is_match(&self, _segment: &str) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct DynamicSegmentParser;
impl SegmentParser for DynamicSegmentParser {
    fn parse(&self, segment: &str) -> Option<Box<RoutingMatcher>> {
        if &segment[0..1] == ":" {
            return Some(Box::new(DynamicSegment));
        }
        None
    }
}

fn main() {
    let mut tree: RoutingTree<()> = RoutingTree::with_parsers(vec![
        Box::new(DynamicSegmentParser),
        Box::new(StaticSegmentParser),
    ]);

    tree.insert(Method::GET, "/", ());
    tree.insert(Method::GET, "/status", ());
    tree.insert(Method::GET, "/api/v1/user", ());
    tree.insert(Method::POST, "/api/v1/user", ());
    tree.insert(Method::PUT, "/api/v1/user/:id", ());
    tree.shrink();

    println!("GET /: {:?}", tree.route(&Method::GET, "/"));
    println!("GET /status: {:?}", tree.route(&Method::GET, "/status"));
    println!("GET /api: {:?}", tree.route(&Method::GET, "/api"));
    println!("GET /api/v1: {:?}", tree.route(&Method::GET, "/api/v1"));
    println!(
        "GET /api/v1/user: {:?}",
        tree.route(&Method::GET, "/api/v1/user")
    );
    println!(
        "PUT /api/v1/user: {:?}",
        tree.route(&Method::PUT, "/api/v1/user")
    );
    println!(
        "POST /api/v1/user: {:?}",
        tree.route(&Method::POST, "/api/v1/user")
    );
    println!(
        "GET /api/v1/user/steve: {:?}",
        tree.route(&Method::GET, "/api/v1/user/steve")
    );
    println!(
        "PUT /api/v1/user/steve: {:?}",
        tree.route(&Method::PUT, "/api/v1/user/steve")
    );
}
