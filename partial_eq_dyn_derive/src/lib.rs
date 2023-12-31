//!# Disclaimer

//!This is the first crate I published so I am new to making things production ready. Therefore use this crate with caution and feedback is welcome.
//! # partial_eq_dyn_derive
//!
//! To implement PartialEq on types with trait object fields you can use the derive macro PartialEqDyn. The implementation needs the traits that are present as trait objects to have AsAny and DynPartialEq as supertraits. For those traits there also exist derive macros AsAny and DynPartialEq.
//!Here an Example:
//!```
//!use partial_eq_dyn::{AsAny, DynPartialEq};
//!use partial_eq_dyn_derive::{AsAny, DynPartialEq, PartialEqDyn};
//!
//!trait TestTrait:AsAny + DynPartialEq {}
//!
//!#[derive(AsAny, DynPartialEq, PartialEq)]
//!struct TestTraitImplementor(i32);
//!
//!impl TestTrait for TestTraitImplementor {}
//!
//!#[derive(PartialEqDyn)]
//!struct TestStruct {
//!    field1: i32,
//!    field2: Box<i32>,
//!    field3: Box<dyn TestTrait>,
//!}
//!```
//!
//!Or if the type implements the trait itself:
//!
//!```
//!use partial_eq_dyn::{AsAny, DynPartialEq};
//!use partial_eq_dyn_derive::{AsAny, DynPartialEq, PartialEqDyn};
//!trait TestTrait: AsAny + DynPartialEq {}
//!
//!#[derive(AsAny, DynPartialEq, PartialEqDyn)]
//!enum TestEnum {
//!    Some {
//!        field1: i32,
//!        field2: Box<i32>,
//!        field3: Box<dyn TestTrait>,
//!    },
//!    None,
//!}
//!
//!impl TestTrait for TestEnum {}
//!```

use proc_macro::TokenStream;
mod as_any_derive;
mod dyn_partial_eq_derive;
mod partial_eq_dyn_derive;

/// This derives the AsAny trait in its most easy form of an identity function
#[proc_macro_derive(AsAny)]
pub fn as_any_derive(input: TokenStream) -> TokenStream {
    let ast = as_any_derive::parse(input.into());
    as_any_derive::impl_as_any(&ast).into()
}

/// This derives the DynPartialEq trait by simple downcasting the Any Object to the given type and calling the regular PartialEq comparison. Therefor your Type has to implement PartialEq
#[proc_macro_derive(DynPartialEq)]
pub fn dyn_partial_eq_derive(input: TokenStream) -> TokenStream {
    let ast = dyn_partial_eq_derive::parse(input.into());
    dyn_partial_eq_derive::impl_dyn_partial_eq(&ast).into()
}

/// This derives PartialEq but manages trait Objects by calling their dyn_eq methods and casting. To work all trait Objects have to be bounded by DynPartialEq and AsAny.
#[proc_macro_derive(PartialEqDyn)]
pub fn partial_eq_any_eq_derive(input: TokenStream) -> TokenStream {
    let ast = partial_eq_dyn_derive::parse(input.into());
    partial_eq_dyn_derive::impl_partial_eq_dyn(&ast).into()
}

#[cfg(test)]
mod test {
    use crate::{as_any_derive, dyn_partial_eq_derive, partial_eq_dyn_derive};
    use quote::quote;
    #[test]
    fn parse_named_struct() {
        as_any_derive::parse(quote! {
            struct NamedStruct {
                field1: i32,
                field2: String,
                field3: Box<i32>,
            }
        });
        dyn_partial_eq_derive::parse(quote! {
            struct NamedStruct {
                field1: i32,
                field2: String,
                field3: Box<i32>,
            }
        });
        partial_eq_dyn_derive::parse(quote! {
            struct NamedStruct {
                field1: i32,
                field2: String,
                field3: Box<i32>,
            }
        });
    }
    #[test]
    fn parse_unnamed_struct() {
        as_any_derive::parse(quote! {
            struct UnnamedStruct (
                i32,
                String,
                Box<i32>,
            );
        });
        dyn_partial_eq_derive::parse(quote! {
            struct UnnamedStruct (
                i32,
                String,
                Box<i32>,
            );
        });
        partial_eq_dyn_derive::parse(quote! {
            struct UnnamedStruct (
                i32,
                String,
                Box<i32>,
            );
        });
    }
    #[test]
    fn parse_named_enum() {
        as_any_derive::parse(quote! {
            enum NamedEnum {
                First{first:i32},
                Second{sec_first:String,
                sec_second:Box<i32>,},
            }
        });
        dyn_partial_eq_derive::parse(quote! {
            enum NamedEnum {
                First{first:i32},
                Second{sec_first:String,
                sec_second:Box<i32>,},
            }
        });
        partial_eq_dyn_derive::parse(quote! {
            enum NamedEnum {
                First{first:i32},
                Second{sec_first:String,
                sec_second:Box<i32>,},
            }
        });
    }
    #[test]
    fn parse_unnamed_enum() {
        as_any_derive::parse(quote! {
            enum UnnamedEnum {
                First(i32),
                Second(String,
                Box<i32>,),
            }
        });
        dyn_partial_eq_derive::parse(quote! {
            enum UnnamedEnum {
                First(i32),
                Second(String,
                Box<i32>,),
            }
        });
        partial_eq_dyn_derive::parse(quote! {
            enum UnnamedEnum {
                First(i32),
                Second(String,
                Box<i32>,),
            }
        });
    }
    #[test]
    fn parse_mixed_named_enum() {
        as_any_derive::parse(quote! {
            enum MixedNamedEnum {
                First(i32,String),
                Second{first:String,
                second:Box<i32>,},
            }
        });
        dyn_partial_eq_derive::parse(quote! {
            enum MixedNamedEnum {
                First(i32,String),
                Second{first:String,
                second:Box<i32>,},
            }
        });
        partial_eq_dyn_derive::parse(quote! {
            enum MixedNamedEnum {
                First(i32,String),
                Second{first:String,
                second:Box<i32>,},
            }
        });
    }
}
