use std::any::Any;
use std::fmt::Debug;

pub const __: Option<&'static ()> = None;

pub fn test<T: Any + Debug, U: Any + Debug + PartialEq>(actual: T, expected: U) {
    let actual_any = &actual as &Any;

    match actual_any.downcast_ref::<U>() {
        Some(x) => assert_eq!(*x, expected),
        None => panic!("fill me in!"),
    }
}
