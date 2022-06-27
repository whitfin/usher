pub mod matcher {
    use usher::matcher::*;

    #[test]
    fn static_matching() {
        let matcher = StaticMatcher::new("value");

        assert!(matcher.is_match("value"));
        assert!(!matcher.is_match("not-value"));

        assert_eq!(matcher.capture("value"), None);
        assert_eq!(matcher.capture("not-value"), None);
    }

    #[test]
    fn dynamic_matching() {
        let matcher = DynamicMatcher::new("field");

        assert!(matcher.is_match("value"));
        assert!(matcher.is_match("not-value"));

        assert_eq!(matcher.capture("value"), Some(("field", (0, 5))));
        assert_eq!(matcher.capture("not-value"), Some(("field", (0, 9))));
    }

    #[test]
    fn closure_matching() {
        let matcher = |input: &str| input == "value";

        assert!(matcher.is_match("value"));
        assert!(!matcher.is_match("not-value"));

        assert_eq!(matcher.capture("value"), None);
        assert_eq!(matcher.capture("not-value"), None);
    }
}
