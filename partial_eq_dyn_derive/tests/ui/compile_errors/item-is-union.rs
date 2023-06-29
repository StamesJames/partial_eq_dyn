use partial_eq_dyn_derive::PartialEqDyn;

#[derive(PartialEqDyn)]
union SomeUnion {
    first: i32,
    second: i8,
}

fn main() {}
