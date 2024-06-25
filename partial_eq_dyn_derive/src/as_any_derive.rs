use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn parse(input: TokenStream) -> DeriveInput {
    syn::parse2(input).expect("The Item is supposed to be a Struct or Enum")
}

pub fn impl_as_any(ast: &syn::DeriveInput) -> TokenStream {
    let item_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    quote! {
        impl #impl_generics partial_eq_dyn::AsAny for #item_name #ty_generics #where_clause {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }
        }
    }
}
