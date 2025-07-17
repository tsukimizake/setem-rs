use setem_rs::{generate_setters, process_elm_file, setup_parser};
use std::collections::HashSet;

fn extract_fields_from_file(file_path: &str) -> HashSet<String> {
    let mut parser = setup_parser();
    let mut identifiers = HashSet::new();
    process_elm_file(file_path, &mut parser, &mut identifiers);
    identifiers
}

fn assert_fields_match(actual: &HashSet<String>, expected: &[&str]) {
    let expected_set: HashSet<String> = expected.iter().map(|s| s.to_string()).collect();

    let missing: Vec<_> = expected_set.difference(actual).collect();
    let extra: Vec<_> = actual.difference(&expected_set).collect();

    if !missing.is_empty() {
        panic!("Missing fields: {:?}", missing);
    }
    if !extra.is_empty() {
        panic!("Extra fields: {:?}", extra);
    }
}

#[test]
fn test_record_def_and_expr() {
    let test_file = "tests/fixtures/RecordDefAndExpr.elm";
    let identifiers = extract_fields_from_file(test_file);

    let expected_fields = ["f1", "f2", "f3", "f3_f1", "f3_f2"];
    assert_fields_match(&identifiers, &expected_fields);

    // Also test that setters are generated correctly
    let setters = generate_setters(&identifiers, "s_");
    assert!(setters.contains("s_f1 : a -> { b | f1 : a } -> { b | f1 : a }"));
    assert!(setters.contains("s_f2 : a -> { b | f2 : a } -> { b | f2 : a }"));
    assert!(setters.contains("s_f3 : a -> { b | f3 : a } -> { b | f3 : a }"));
    assert!(setters.contains("s_f3_f1 : a -> { b | f3_f1 : a } -> { b | f3_f1 : a }"));
    assert!(setters.contains("s_f3_f2 : a -> { b | f3_f2 : a } -> { b | f3_f2 : a }"));
}

#[test]
fn test_custom_prefix() {
    let test_file = "tests/fixtures/RecordDefAndExpr.elm";
    let identifiers = extract_fields_from_file(test_file);

    let setters = generate_setters(&identifiers, "set_");
    assert!(setters.contains("set_f1 : a -> { b | f1 : a } -> { b | f1 : a }"));
    assert!(setters.contains("set_f2 : a -> { b | f2 : a } -> { b | f2 : a }"));
}




