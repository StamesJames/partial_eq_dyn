use proc_macro::TokenStream;
use quote::quote;

pub fn impl_dyn_partial_eq(ast: &syn::DeriveInput) -> TokenStream {
    let item_name = &ast.ident;
    quote! {
        impl AnyEq for #item_name {
            fn any_eq(&self, other: &dyn Any) -> bool {
                other
                    .downcast_ref::<#item_name>()
                    .map_or(false, |other| self == other)
            }
        }
    }
    .into()
}
