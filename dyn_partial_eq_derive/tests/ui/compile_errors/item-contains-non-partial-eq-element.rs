use dyn_partial_eq_derive::PartialEqDyn;

struct NoPartialEq;

#[derive(PartialEqDyn)]
struct Struct(i32, NoPartialEq);

fn main() {}
