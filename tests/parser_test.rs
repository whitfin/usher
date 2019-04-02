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
        let parser = |input: &str| -> Option<Box<Matcher>> {
            // just generate a static match directly
            Some(Box::new(StaticMatcher::new(input)))
        };
        assert!(parser.parse("anything").is_some());
    }
}
