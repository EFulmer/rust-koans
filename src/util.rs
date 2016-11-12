use std::any::Any;
use std::fmt::Debug;

pub const __: Option<&'static ()> = Some(&());

pub fn test_eq<T: Any, U: Any + Debug + PartialEq>(actual: T, expected: U) {
    let actual_any = &actual as &Any;

    if actual_any.is::<Option<&()>>() {
         panic!("-=> FILL ME IN! <=-");
    }

    match actual_any.downcast_ref::<U>() {
        Some(x) => assert_eq!(*x, expected),
        None => panic!("Need to either fill in the '__' with another expression, or there's a type
                       error here"),
    }
}

pub fn test_true<T: Any + Debug + PartialEq>(v: T) {
    let v = &v as &Any;

    if !( v.is::<bool>() ) {
         panic!("-=> FILL ME IN! <=-");
    }

    match v.downcast_ref::<bool>() {
        Some(x) => assert!(*x),
        None => panic!("Need to either fill in the '__' with another expression, or there's a type
                       error here"),
    }
}
