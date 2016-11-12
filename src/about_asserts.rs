use util::*;

#[test]
fn test_assert_truth() {
    assert!(false); // this should be true
}

#[test]
fn test_assert_with_message() {
    assert!(false, "This should be true -- please fix this");
}

#[test]
fn test_fill_in_values() {
    test_eq(2, __);
}

#[test]
fn test_assert_equality() {
    let expected_value = 2;
    let actual_value = 0;
    assert!(expected_value == actual_value);
}

#[test]
fn test_a_better_way_of_asserting_equality() {
    let expected_value = 2;
    let actual_value = __;
    
    test_eq(expected_value, actual_value);
}

#[test]
fn test_that_test_eq_works_the_same_way_as_test_macro() {
    assert!(false);
}
