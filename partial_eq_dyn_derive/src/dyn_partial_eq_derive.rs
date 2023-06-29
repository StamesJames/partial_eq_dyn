use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn parse(input: TokenStream) -> DeriveInput {
    syn::parse2(input).expect("The Item is supposed to be a Struct or Enum")
}

pub fn impl_dyn_partial_eq(ast: &syn::DeriveInput) -> TokenStream {
    let item_name = &ast.ident;
    quote! {
        impl partial_eq_dyn::DynPartialEq for #item_name {
            fn dyn_eq(&self, other: &dyn std::any::Any) -> bool {
                other
                    .downcast_ref::<#item_name>()
                    .map_or(false, |other| self == other)
            }
        }
    }
}
