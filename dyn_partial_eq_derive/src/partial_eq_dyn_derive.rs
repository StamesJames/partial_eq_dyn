use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    self,
    punctuated::Punctuated,
    token::Comma,
    DataStruct, FieldsNamed, FieldsUnnamed, Ident, Path,
    Type::{self, TraitObject},
    TypePath, Variant,
};

pub fn impl_partial_eq_dyn(ast: &syn::DeriveInput) -> TokenStream {
    let item_ident = &ast.ident;
    let item_data = &ast.data;
    let gen_field_comps = match item_data {
        syn::Data::Struct(data) => gen_part_eq_struct_fields(data),
        syn::Data::Enum(data) => gen_part_eq_enum_variants(&data.variants),
        syn::Data::Union(_) =>
            panic!(
                "Since standart PartialEq can't be derived for Unions we also don't derive a dynamic PartialEq"
            ),
    };
    let gen_part_eq = quote! {
        impl PartialEq for #item_ident {
            fn eq(&self, other: &Self) -> bool {
                #gen_field_comps
            }
        }
    };
    gen_part_eq.into()
}

fn gen_part_eq_struct_fields(data: &DataStruct) -> quote::__private::TokenStream {
    match &data.fields {
        syn::Fields::Named(fields) => gen_part_eq_struct_named_fields(fields),
        syn::Fields::Unnamed(fields) => gen_part_eq_struct_unnamed_fields(fields),
        syn::Fields::Unit => quote! { true },
    }
}

fn gen_part_eq_struct_named_fields(fields: &FieldsNamed) -> quote::__private::TokenStream {
    let (fields_named_left, fields_named_right, fields_named_comps) = make_named_fields(fields);
    quote!(match (self, other) {(Self{#fields_named_left}, Self{#fields_named_right}) => #fields_named_comps })
}
fn gen_part_eq_struct_unnamed_fields(fields: &FieldsUnnamed) -> quote::__private::TokenStream {
    let (fields_unnamed_left, fields_unnamed_right, fields_unnamed_comps) =
        make_unnamed_fields(fields);
    quote!(match (self, other) { (Self(#fields_unnamed_left), Self(#fields_unnamed_right)) => #fields_unnamed_comps })
}

fn gen_part_eq_enum_variants(
    varaints: &Punctuated<Variant, Comma>,
) -> quote::__private::TokenStream {
    let mut gen_match_arms = quote!();
    for variant in varaints {
        let variant_name = &variant.ident;
        match &variant.fields {
            syn::Fields::Named(fields) => {
                let (fields_named_left, fields_named_right, fields_named_comps) =
                    make_named_fields(fields);
                gen_match_arms = quote! {
                ( Self::#variant_name{#fields_named_left}, Self::#variant_name{#fields_named_right} ) => #fields_named_comps,
                 #gen_match_arms};
            }
            syn::Fields::Unnamed(fields) => {
                let (fields_unnamed_left, fields_unnamed_right, fields_unnamed_comps) =
                    make_unnamed_fields(fields);
                gen_match_arms = quote! {
                    (Self::#variant_name(#fields_unnamed_left), Self::#variant_name(#fields_unnamed_right)) => #fields_unnamed_comps ,
                    #gen_match_arms
                };
            }
            syn::Fields::Unit => {
                gen_match_arms =
                    quote!((Self::#variant_name , Self::#variant_name) => true , #gen_match_arms);
            }
        }
    }
    return quote! {
        match (self, other) {
            #gen_match_arms
            _ => false
        }
    };
}

fn make_comp_for_type(
    left_name: &Ident,
    right_name: &Ident,
    ty: &Type,
) -> quote::__private::TokenStream {
    match ty {
        TraitObject(_) => quote!(#left_name.any_eq(#right_name.as_any())),
        Type::Reference(ty) => make_comp_for_type(left_name, right_name, &*ty.elem),
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            if segments.iter().any(|segment| match &segment.arguments {
                syn::PathArguments::AngleBracketed(args) => args.args.iter().any(|arg| match arg {
                    syn::GenericArgument::Type(ty) => match ty {
                        TraitObject(_) => true,
                        _ => false,
                    },
                    _ => false,
                }),
                _ => false,
            }) {
                quote!(#left_name.any_eq(#right_name.as_any()))
            } else {
                quote!(#left_name == #right_name)
            }
        }
        _ => quote!(#left_name == #right_name),
    }
    // quote!(#left_name.any_eq(#right_name.as_any()))
}

fn make_named_fields(
    fields: &FieldsNamed,
) -> (
    quote::__private::TokenStream,
    quote::__private::TokenStream,
    quote::__private::TokenStream,
) {
    let mut fields_named_left = quote!();
    let mut fields_named_right = quote!();
    let mut fields_named_comps = quote!(true);

    for field in &fields.named {
        let field_name = field.ident.as_ref().unwrap();
        let left_name = format_ident!("l_{}", field_name);
        let right_name = format_ident!("r_{}", field_name);
        fields_named_left = quote! { #field_name : #left_name, #fields_named_left };
        fields_named_right = quote! { #field_name : #right_name, #fields_named_right };
        let new_comp = make_comp_for_type(&left_name, &right_name, &field.ty);
        fields_named_comps = quote!(#new_comp && #fields_named_comps);
    }
    (fields_named_left, fields_named_right, fields_named_comps)
}

fn make_unnamed_fields(
    fields: &FieldsUnnamed,
) -> (
    quote::__private::TokenStream,
    quote::__private::TokenStream,
    quote::__private::TokenStream,
) {
    let mut fields_unnamed_left = quote!();
    let mut fields_unnamed_right = quote!();
    let mut fields_unnamed_comps = quote!(true);
    for (n, field) in (&fields.unnamed).iter().enumerate() {
        let left_name = format_ident!("l_{}", n);
        let right_name = format_ident!("r_{}", n);
        fields_unnamed_left = quote! { #left_name , #fields_unnamed_left };
        fields_unnamed_right = quote! { #right_name , #fields_unnamed_right };
        let new_comp = make_comp_for_type(&left_name, &right_name, &field.ty);
        fields_unnamed_comps = quote!(#new_comp && #fields_unnamed_comps);
    }
    (
        fields_unnamed_left,
        fields_unnamed_right,
        fields_unnamed_comps,
    )
}
