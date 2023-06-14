use proc_macro::TokenStream;
use quote::quote;

pub fn impl_as_any(ast: &syn::DeriveInput) -> TokenStream {
    let item_name = &ast.ident;
    quote! {
        impl AsAny for #item_name {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
    }
    .into()
}
