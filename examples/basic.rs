use http::Method;
use usher::prelude::*;

#[derive(Debug)]
pub struct ParamMatcher;
impl RoutingMatcher for ParamMatcher {
    fn is_match(&self, base: &str, incoming: &str) -> bool {
        &base[0..1] == ":" && incoming.len() > 0
    }
}

fn main() {
    let mut tree: RoutingTree<()> = RoutingTree::new_with_matchers(vec![
        // static pathing
        Box::new(StaticMatcher),
        // parameterized pathing
        Box::new(ParamMatcher),
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
