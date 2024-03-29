use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};
use usher::capture::find_capture;
use usher::http::HttpRouter;
use usher::prelude::*;

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// This signature is borrowed from the Hyper "echo" example codebase.
type BoxFut = Pin<Box<dyn Future<Output = Result<Response<Body>, hyper::Error>> + Send>>;

/// Represents a boxed function which receives a request/params and returns a response future.
type Callee = Box<dyn Fn(Request<Body>, Vec<(&str, (usize, usize))>) -> BoxFut + Send + Sync>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Create our address to bind to, localhost:3000
    let addr = ([127, 0, 0, 1], 3000).into();

    // Just like in a normal Router, we provide our parsers at startup.
    let mut router: HttpRouter<Callee> =
        HttpRouter::new(vec![Box::new(DynamicParser), Box::new(StaticParser)]);

    // This will echo the provided name back to the caller.
    router.get(
        "/:name",
        Box::new(|req, params| {
            let path = req.uri().path();
            let name = find_capture(path, &params, "name").unwrap();

            let body = format!("Hello, {}!\n", name).into();
            let resp = Response::new(body);

            Box::pin(async move { Ok(resp) })
        }),
    );

    // Wrap inside an Arc to avoid large clones.
    let router = Arc::new(router);

    // Construct our Hyper server.
    let server = make_service_fn(move |_conn| {
        // We need a "clone" of the router.
        let router = router.clone();

        async {
            // Construct a Hyper service from a function which turns a request
            // into an asynchronous response (which comes from echo()).
            let server = service_fn(move |req: Request<Body>| {
                // We need a "clone" of the router.
                let router = router.clone();

                async move {
                    // First we need to extract the method and path to use for the
                    // actual handler lookup as it uses a combination of both values.
                    let method = req.method();
                    let path = req.uri().path();

                    // Then we delegate to a hander when possible.
                    match router.handler(method, path) {
                        // In this case, invoke the handler and pass through the
                        // request instance and the captures associated with it.
                        //
                        // In this case we pass through the captures as they're
                        // generated by default, but this might be where you wish
                        // to turn them into something like a `HashMap` for access.
                        Some((handler, captures)) => handler(req, captures).await,

                        // If no handler matches, we generate a 404 response to
                        // state so back to the caller. This happens when either
                        // the HTTP method or the path is wrong during matching.
                        None => {
                            let mut response = Response::new(Body::empty());
                            *response.status_mut() = StatusCode::NOT_FOUND;
                            Ok(response)
                        }
                    }
                }
            });

            // pass back the server value
            Ok::<_, hyper::Error>(server)
        }
    });

    // Log the port we're listening on so we don't forget!
    println!("Listening on http://{}", addr);

    // Initialze the actual service.
    Server::bind(&addr).serve(server).await?;

    Ok(())
}
