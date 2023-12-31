use partial_eq_dyn::{AsAny, DynPartialEq};
use partial_eq_dyn_derive::{AsAny, DynPartialEq, PartialEqDyn};

trait TestTrait: core::fmt::Debug + AsAny + DynPartialEq {}

#[derive(AsAny, DynPartialEq, PartialEq, Debug)]
struct TestTraitImplementor(i32);

impl TestTrait for TestTraitImplementor {}

#[derive(PartialEqDyn, Debug)]
struct TestStruct(i32, Box<i32>, Box<dyn TestTrait>);

#[test]
fn derive_partial_eq_dyn_on_unnamed_struct_and_use_it() {
    let first = TestStruct(1, Box::<i32>::new(2), Box::new(TestTraitImplementor(3)));
    let second = TestStruct(1, Box::<i32>::new(2), Box::new(TestTraitImplementor(3)));
    assert_eq!(first, second);
    let other1 = TestStruct(2, Box::<i32>::new(2), Box::new(TestTraitImplementor(3)));
    assert_ne!(first, other1);
    let other2 = TestStruct(1, Box::<i32>::new(2), Box::new(TestTraitImplementor(4)));
    assert_ne!(first, other2);
}
