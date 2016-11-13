use util::*;

#[test]
fn get_values_out_of_options() {
    let x: Option<&str> = Some("hello");
    test(__, x.unwrap());
}

#[test]
#[should_panic]
fn extracting_values_from_options_can_be_risky() {
    // change something about the below lines of code to cause a panic
    let x: Option<bool> = Some(true);
    x.unwrap();
}

#[test]
fn we_can_safely_check_whether_an_option_contains_something() {
    let x = Some(28);
    let y: Option<String> = None;
    test(__, x.is_some());
    test(__, x.is_none());
    test(__, y.is_some());
    test(__, y.is_none());
}

#[test]
fn we_can_also_safely_extract_values_from_an_option_if_you_provide_a_default() {
    let x = None;
    test(__, x.unwrap_or(""));
}
