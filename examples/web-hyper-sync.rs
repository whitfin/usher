use hyper::rt::{self, Future};
use hyper::service::service_fn_ok;
use hyper::{Body, Request, Response, Server, StatusCode};
use usher::http::HttpRouter;
use usher::prelude::*;

/// Represents a boxed function which receives a request/params and returns a response.
type Callee = Box<Fn(Request<Body>, Vec<(&str, &str)>) -> Response<Body> + Send + Sync>;

fn main() {
    // Create our address to bind to, localhost:3000
    let addr = ([127, 0, 0, 1], 3000).into();

    // Construct our Hyper server.
    let server = Server::bind(&addr)
        .serve(|| {
            // Just like in a normal Router, we provide our parsers at startup.
            let mut router: HttpRouter<Callee> =
                HttpRouter::new(vec![Box::new(DynamicParser), Box::new(StaticParser)]);

            // This will echo the provided name back to the caller.
            router.get(
                "/:name",
                Box::new(|_req, params| {
                    let param = params[0];
                    let value = param.1.to_string();

                    Response::new(format!("Hello, {}!\n", value).into())
                }),
            );

            // Construct a Hyper service from a function which turns a request
            // into an asynchronous response (which comes from echo()).
            service_fn_ok(move |req: Request<Body>| {
                // First we need to extract the method and path to use for the
                // actual handler lookup. For now the path needs to be owned,
                // although this will hopefully change in future when there are
                // some changes to the captures API. If you are going to own the
                // captures you get back (by storing them in a map, etc), then
                // you can avoid making the path owned at this point in time.
                let method = req.method();
                let path = req.uri().path().to_owned();

                // Then we delegate to a hander when possible.
                match router.handler(method, &path) {
                    // In this case, invoke the handler and pass through the
                    // request instance and the captures associated with it.
                    Some((handler, captures)) => handler(req, captures),

                    // If no handler matches, we generate a 404 response to
                    // state so back to the caller. This happens when either
                    // the HTTP method or the path is wrong during matching.
                    None => {
                        let mut response = Response::new(Body::empty());
                        *response.status_mut() = StatusCode::NOT_FOUND;
                        response
                    }
                }
            })
        })
        .map_err(|e| eprintln!("server error: {}", e));

    // Log the port we're listening on so we don't forget!
    println!("Listening on http://{}", addr);

    // Initialze the actual service.
    rt::run(server);
}
