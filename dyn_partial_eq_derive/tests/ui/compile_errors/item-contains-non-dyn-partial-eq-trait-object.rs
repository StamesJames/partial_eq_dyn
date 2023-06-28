use dyn_partial_eq::AsAny;
use dyn_partial_eq_derive::PartialEqDyn;

trait TestTrait: AsAny {}

#[derive(PartialEqDyn)]
struct TestStruct(Box<dyn TestTrait>);

fn main() {}
