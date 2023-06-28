use dyn_partial_eq::DynPartialEq;
use dyn_partial_eq_derive::PartialEqDyn;

trait NonAsAny: DynPartialEq {}

#[derive(PartialEqDyn)]
struct TestStruct(Box<dyn NonAsAny>);

fn main() {}
