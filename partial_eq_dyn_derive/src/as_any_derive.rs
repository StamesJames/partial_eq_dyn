use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn parse(input: TokenStream) -> DeriveInput {
    syn::parse2(input).expect("The Item is supposed to be a Struct or Enum")
}

pub fn impl_as_any(ast: &syn::DeriveInput) -> TokenStream {
    let item_name = &ast.ident;
    quote! {
        impl  partial_eq_dyn::AsAny for #item_name {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
}
