use partial_eq_dyn::{AsAny, DynPartialEq};
use partial_eq_dyn_derive::{AsAny, DynPartialEq, PartialEqDyn};

trait TestTrait: core::fmt::Debug + AsAny + DynPartialEq {}

#[derive(AsAny, DynPartialEq, PartialEq, Debug)]
enum TestTraitImplementor {
    Variant1(i32),
    Variant2(i32),
}

impl TestTrait for TestTraitImplementor {}

#[derive(PartialEqDyn, Debug)]
enum TestEnum {
    Variant1(i32, Box<i32>, Box<dyn TestTrait>),
    Variant2(i32, Box<i32>, Box<dyn TestTrait>),
}

#[test]
fn derive_partial_eq_dyn_on_unnamed_enum_and_use_it() {
    let first = TestEnum::Variant1(
        1,
        Box::<i32>::new(2),
        Box::new(TestTraitImplementor::Variant1(3)),
    );
    let second = TestEnum::Variant1(
        1,
        Box::<i32>::new(2),
        Box::new(TestTraitImplementor::Variant1(3)),
    );
    assert_eq!(first, second);
    let first = TestEnum::Variant1(
        1,
        Box::<i32>::new(2),
        Box::new(TestTraitImplementor::Variant2(3)),
    );
    let second = TestEnum::Variant1(
        1,
        Box::<i32>::new(2),
        Box::new(TestTraitImplementor::Variant2(3)),
    );
    assert_eq!(first, second);
    let other = TestEnum::Variant1(
        2,
        Box::<i32>::new(2),
        Box::new(TestTraitImplementor::Variant2(3)),
    );
    assert_ne!(first, other);
    let other = TestEnum::Variant1(
        1,
        Box::<i32>::new(2),
        Box::new(TestTraitImplementor::Variant2(4)),
    );
    assert_ne!(first, other);
    let other = TestEnum::Variant2(
        1,
        Box::<i32>::new(2),
        Box::new(TestTraitImplementor::Variant2(3)),
    );
    assert_ne!(first, other);
    let other = TestEnum::Variant1(
        1,
        Box::<i32>::new(2),
        Box::new(TestTraitImplementor::Variant1(3)),
    );
    assert_ne!(first, other);
}
