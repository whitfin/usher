pub mod capture {
    use usher::capture::*;

    #[test]
    fn finding_captures() {
        let path = "/api/v1/user/123";
        let captures = vec![("vsn", (5, 7)), ("type", (8, 12)), ("id", (13, 16))];

        let id = find_capture(path, &captures, "id");
        assert_eq!(id, Some("123"));

        let object = find_capture(path, &captures, "type");
        assert_eq!(object, Some("user"));

        let version = find_capture(path, &captures, "vsn");
        assert_eq!(version, Some("v1"));

        let missing = find_capture(path, &captures, "missing");
        assert_eq!(missing, None);
    }
}
