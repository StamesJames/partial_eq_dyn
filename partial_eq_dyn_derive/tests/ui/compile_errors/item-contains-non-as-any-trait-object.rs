use partial_eq_dyn::DynPartialEq;
use partial_eq_dyn_derive::PartialEqDyn;

trait NonAsAny: DynPartialEq {}

#[derive(PartialEqDyn)]
struct TestStruct(Box<dyn NonAsAny>);

fn main() {}
