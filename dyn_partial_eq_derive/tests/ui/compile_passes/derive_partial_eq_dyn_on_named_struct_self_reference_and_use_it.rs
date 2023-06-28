use dyn_partial_eq::{AsAny, DynPartialEq};
use dyn_partial_eq_derive::{AsAny, DynPartialEq, PartialEqDyn};

trait TestTrait: core::fmt::Debug + AsAny + DynPartialEq {}

#[derive(AsAny, DynPartialEq, PartialEqDyn, Debug)]
struct TestStruct {
    field1: i32,
    field2: Box<i32>,
    field3: Option<dyn TestTrait>,
}

impl TestTrait for TestStruct {}

fn main() {
    let first = TestStruct {
        field1: 1,
        field2: Box::<i32>::new(2),
        field3: Some(TestStruct {
            field1: 1,
            field2: Box::<i32>::new(2),
            field3: Box::new(None),
        }),
    };
    let second = TestStruct {
        field1: 1,
        field2: Box::<i32>::new(2),
        field3: Some(TestStruct {
            field1: 1,
            field2: Box::<i32>::new(2),
            field3: Box::new(None),
        }),
    };
    assert_eq!(first, second);
    let other1 = TestStruct {
        field1: 2,
        field2: Box::<i32>::new(2),
        field3: Some(TestStruct {
            field1: 1,
            field2: Box::<i32>::new(2),
            field3: Box::new(None),
        }),
    };
    assert_ne!(first, other1);
    let other2 = TestStruct {
        field1: 1,
        field2: Box::<i32>::new(2),
        field3: Some(TestStruct {
            field1: 2,
            field2: Box::<i32>::new(2),
            field3: Box::new(None),
        }),
    };
    assert_ne!(first, other2);
}
