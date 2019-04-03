pub mod router {
    use usher::prelude::*;

    #[test]
    fn basic_routing() {
        let mut router: Router<usize> = Router::new(vec![Box::new(StaticParser)]);

        router.insert("/1", 1);
        router.insert("/2", 2);
        router.insert("/3", 3);

        let n1 = router.lookup("/1");
        let n2 = router.lookup("/2");
        let n3 = router.lookup("/3");
        let n4 = router.lookup("/4");

        assert_eq!(n1, Some((&1, vec![])));
        assert_eq!(n2, Some((&2, vec![])));
        assert_eq!(n3, Some((&3, vec![])));
        assert_eq!(n4, None);
    }

    #[test]
    fn nested_routing() {
        let mut router: Router<usize> = Router::new(vec![Box::new(StaticParser)]);

        router.insert("/number/1", 1);
        router.insert("/number/2", 2);
        router.insert("/number/3", 3);

        let n1 = router.lookup("/number/1");
        let n2 = router.lookup("/number/2");
        let n3 = router.lookup("/number/3");
        let n4 = router.lookup("/number/4");

        assert_eq!(n1, Some((&1, vec![])));
        assert_eq!(n2, Some((&2, vec![])));
        assert_eq!(n3, Some((&3, vec![])));
        assert_eq!(n4, None);
    }

    #[test]
    fn captured_routing() {
        let mut router: Router<()> =
            Router::new(vec![Box::new(DynamicParser), Box::new(StaticParser)]);

        router.insert("/:id", ());

        let n1 = router.lookup("/1");
        let n2 = router.lookup("/1/1");
        let n3 = router.lookup("/");

        assert_eq!(n1, Some((&(), vec![("id", (1, 2))])));
        assert_eq!(n2, None);
        assert_eq!(n3, None);
    }
}
