use http::Method;
use usher::http::HttpRouter;
use usher::prelude::*;

fn main() {
    // Just like in a normal Router, we provide our parsers at startup.
    let mut router: HttpRouter<()> =
        HttpRouter::new(vec![Box::new(DynamicParser), Box::new(StaticParser)]);

    // Then we insert some HTTP routes (note the HTTP method being used as
    // the method name for insertion).
    router.get("/", ());
    router.get("/status", ());
    router.get("/api/v1/user", ());
    router.post("/api/v1/user", ());
    router.put("/api/v1/user/:id", ());

    // Fetch some HTTP handlers based on the method/path combinations. If
    // the path matches, but the method does not, no handler will be found.
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
