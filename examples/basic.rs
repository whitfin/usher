use usher::prelude::*;

fn main() {
    // First we construct our `Router` using a set of parsers. Fortunately for
    // this example, Usher includes the `StaticParser` which uses basic string
    // matching to determine whether a segment in the path matches.
    let mut router: Router<String> = Router::new(vec![Box::new(StaticParser)]);

    // Then we insert our routes; in this case we're going to store the numbers
    // 1, 2 and 3, against their equivalent name in typed English (with a "/"
    // as a prefix, as Usher expects filesystem-like paths (for now)).
    router.insert("/one", "1".to_string());
    router.insert("/two", "2".to_string());
    router.insert("/three", "3".to_string());

    // Finally we'll just do a lookup on each path, as well as the a path which
    // doesn't match ("/"), just to demonstrate what the return types look like.
    for path in &["/", "/one", "/two", "/three"] {
        println!("{}: {:?}", path, router.lookup(path));
    }
}
