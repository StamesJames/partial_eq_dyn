use proc_macro::TokenStream;
mod as_any;
mod dyn_partial_eq_derive;
mod partial_eq_dyn_derive;

#[proc_macro_derive(AsAny)]
pub fn as_any_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    as_any::impl_as_any(&ast)
}
#[proc_macro_derive(DynPartialEq)]
pub fn any_eq_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    dyn_partial_eq_derive::impl_dyn_partial_eq(&ast)
}

#[proc_macro_derive(PartialEqDyn)]
pub fn partial_eq_any_eq_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    partial_eq_dyn_derive::impl_partial_eq_dyn(&ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
