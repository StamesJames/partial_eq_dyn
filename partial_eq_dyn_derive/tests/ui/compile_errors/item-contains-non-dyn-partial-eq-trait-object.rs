use partial_eq_dyn::AsAny;
use partial_eq_dyn_derive::PartialEqDyn;

trait TestTrait: AsAny {}

#[derive(PartialEqDyn)]
struct TestStruct(Box<dyn TestTrait>);

fn main() {}
