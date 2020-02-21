pub mod parser {
    use usher::matcher::*;
    use usher::parser::*;

    #[test]
    fn static_parsing() {
        assert!(StaticParser.parse("anything").is_some());
    }

    #[test]
    fn dynamic_parsing() {
        assert!(DynamicParser.parse("nah").is_none());
        assert!(DynamicParser.parse(":id").is_some());
    }

    #[test]
    fn closure_parsing() {
        assert!(create_static_matcher.parse("anything").is_some());
    }

    fn create_static_matcher(input: &str) -> Option<Box<dyn Matcher>> {
        Some(Box::new(StaticMatcher::new(input)))
    }
}
