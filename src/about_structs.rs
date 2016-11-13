const __: Option<&'static str> = Some("fill me in");

#[test]
fn we_can_access_tuple_structs_like_tuples() {
    // a tuple struct with an int in the first position and boolean in the second
    struct NumberAndBool(i32, bool);
    let tuple_struct = NumberAndBool(10, true);

    assert_eq!(__, 10);
    assert_eq!(__, true);
}

#[test]
fn struct_initialization() {
    struct Musician {
        name: &'static str,
    };

    let x = Musician { name: "KID...KID ROCK!" };

    assert_eq!(__, x.name);
}


