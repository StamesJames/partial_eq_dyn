use partial_eq_dyn_derive::PartialEqDyn;

struct NoPartialEq;

#[derive(PartialEqDyn)]
struct Struct(i32, NoPartialEq);

fn main() {}
