pub mod parser {
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
}
