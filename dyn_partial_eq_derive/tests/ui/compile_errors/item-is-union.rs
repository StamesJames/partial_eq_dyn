use dyn_partial_eq_derive::PartialEqDyn;

#[derive(PartialEqDyn)]
union SomeUnion {
    first: i32,
    second: i8,
}

fn main() {}
