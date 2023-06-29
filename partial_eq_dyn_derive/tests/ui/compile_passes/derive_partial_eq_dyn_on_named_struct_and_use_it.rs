use partial_eq_dyn::{AsAny, DynPartialEq};
use partial_eq_dyn_derive::{AsAny, DynPartialEq, PartialEqDyn};

trait TestTrait: core::fmt::Debug + AsAny + DynPartialEq {}

#[derive(AsAny, DynPartialEq, PartialEq, Debug)]
struct TestTraitImplementor(i32);

impl TestTrait for TestTraitImplementor {}

#[derive(PartialEqDyn, Debug)]
struct TestStruct {
    field1: i32,
    field2: Box<i32>,
    field3: Box<dyn TestTrait>,
}

fn main() {
    let first = TestStruct {
        field1: 1,
        field2: Box::<i32>::new(2),
        field3: Box::new(TestTraitImplementor(3)),
    };
    let second = TestStruct {
        field1: 1,
        field2: Box::<i32>::new(2),
        field3: Box::new(TestTraitImplementor(3)),
    };
    assert_eq!(first, second);
    let other1 = TestStruct {
        field1: 2,
        field2: Box::<i32>::new(2),
        field3: Box::new(TestTraitImplementor(3)),
    };
    assert_ne!(first, other1);
    let other2 = TestStruct {
        field1: 1,
        field2: Box::<i32>::new(2),
        field3: Box::new(TestTraitImplementor(4)),
    };
    assert_ne!(first, other2);
}
