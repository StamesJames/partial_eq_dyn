use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn parse(input: TokenStream) -> DeriveInput {
    syn::parse2(input).expect("The Item is supposed to be a Struct or Enum")
}

pub fn impl_dyn_partial_eq(ast: &syn::DeriveInput) -> TokenStream {
    let item_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics partial_eq_dyn::DynPartialEq for #item_name #ty_generics #where_clause {
            fn dyn_eq(&self, other: &dyn std::any::Any) -> bool {
                other
                    .downcast_ref::<#item_name>()
                    .map_or(false, |other| self == other)
            }
        }
    }
}
