# Disclaimer

This is the first crate I published so I am new to making things production ready. Therefore use this crate with caution and feedback is welcome.

# dyn_partial_eq_derive

To implement PartialEq on types with trait object fields you can use the derive macro PartialEqDyn. The implementation needs the traits that are present as trait objects to have AsAny and DynPartialEq as supertraits. For those traits there also exist derive macros AsAny and DynPartialEq.
Here an Example:
```
use dyn_partial_eq::{AsAny, DynPartialEq};
use dyn_partial_eq_derive::{AsAny, DynPartialEq, PartialEqDyn};

trait TestTrait: core::fmt::Debug + AsAny + DynPartialEq {}

#[derive(AsAny, DynPartialEq, PartialEq)]
struct TestTraitImplementor(i32);

impl TestTrait for TestTraitImplementor {}

#[derive(PartialEqDyn)]
struct TestStruct {
    field1: i32,
    field2: Box<i32>,
    field3: Box<dyn TestTrait>,
}
```

Or if the type implements the trait itself:

```
use dyn_partial_eq::{AsAny, DynPartialEq};
use dyn_partial_eq_derive::{AsAny, DynPartialEq, PartialEqDyn};

trait TestTrait: core::fmt::Debug + AsAny + DynPartialEq {}

#[derive(AsAny, DynPartialEq, PartialEqDyn)]
struct TestStruct {
    field1: i32,
    field2: Box<i32>,
    field3: Option<dyn TestTrait>,
}

impl TestTrait for TestStruct {}
```