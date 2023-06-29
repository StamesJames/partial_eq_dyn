use partial_eq_dyn::{AsAny, DynPartialEq};
use partial_eq_dyn_derive::{AsAny, DynPartialEq, PartialEqDyn};

trait TestTrait: core::fmt::Debug + AsAny + DynPartialEq {}

#[derive(Debug, AsAny, DynPartialEq, PartialEqDyn)]
enum TestEnum {
    Some {
        field1: i32,
        field2: Box<i32>,
        field3: Box<dyn TestTrait>,
    },
    None,
}

impl TestTrait for TestEnum {}

#[test]
fn derive_partial_eq_dyn_on_named_struct_self_reference_and_use_it() {
    let first = TestEnum::Some {
        field1: 1,
        field2: Box::<i32>::new(2),
        field3: Box::new(TestEnum::Some {
            field1: 1,
            field2: Box::<i32>::new(2),
            field3: Box::new(TestEnum::None),
        }),
    };
    let second = TestEnum::Some {
        field1: 1,
        field2: Box::<i32>::new(2),
        field3: Box::new(TestEnum::Some {
            field1: 1,
            field2: Box::<i32>::new(2),
            field3: Box::new(TestEnum::None),
        }),
    };
    assert_eq!(first, second);
    let other1 = TestEnum::Some {
        field1: 2,
        field2: Box::<i32>::new(2),
        field3: Box::new(TestEnum::Some {
            field1: 1,
            field2: Box::<i32>::new(2),
            field3: Box::new(TestEnum::None),
        }),
    };
    assert_ne!(first, other1);
    let other2 = TestEnum::Some {
        field1: 1,
        field2: Box::<i32>::new(2),
        field3: Box::new(TestEnum::Some {
            field1: 2,
            field2: Box::<i32>::new(2),
            field3: Box::new(TestEnum::None),
        }),
    };
    assert_ne!(first, other2);
}
