use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
    self,
    punctuated::Punctuated,
    token::Comma,
    DataEnum, DataStruct, DeriveInput, FieldsNamed, FieldsUnnamed, Ident, Path,
    Type::{self, TraitObject},
    TypePath, Variant,
};

pub fn parse(input: TokenStream) -> DeriveInput {
    syn::parse2(input).expect("The Item is supposed to be a Struct or Enum")
}

pub fn impl_partial_eq_dyn(ast: &syn::DeriveInput) -> TokenStream {
    let item_ident = &ast.ident;
    let item_data = &ast.data;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let gen_field_comps = match item_data {
        syn::Data::Struct(data) => gen_part_eq_struct_data(data),
        syn::Data::Enum(data) => gen_part_eq_enum_data(data),
        syn::Data::Union(_) =>
            panic!(
                "Since standard PartialEq can't be derived for Unions we also don't derive a dynamic PartialEq"
            ),
    };
    let gen_part_eq = quote! {
        impl #impl_generics PartialEq for #item_ident #ty_generics #where_clause {
            fn eq(&self, other: &Self) -> bool {
                #gen_field_comps
            }
        }
    };
    gen_part_eq
}

fn gen_part_eq_struct_data(data: &DataStruct) -> proc_macro2::TokenStream {
    match &data.fields {
        syn::Fields::Named(fields) => gen_part_eq_struct_named_fields(fields),
        syn::Fields::Unnamed(fields) => gen_part_eq_struct_unnamed_fields(fields),
        syn::Fields::Unit => quote! { true },
    }
}

fn gen_part_eq_enum_data(data: &DataEnum) -> proc_macro2::TokenStream {
    let varaints: &Punctuated<Variant, Comma> = &data.variants;
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
    quote! {
        match (self, other) {
            #gen_match_arms
            _ => false
        }
    }
}

fn gen_part_eq_struct_named_fields(fields: &FieldsNamed) -> proc_macro2::TokenStream {
    let (fields_named_left, fields_named_right, fields_named_comps) = make_named_fields(fields);
    quote!(match (self, other) {(Self{#fields_named_left}, Self{#fields_named_right}) => #fields_named_comps })
}
fn gen_part_eq_struct_unnamed_fields(fields: &FieldsUnnamed) -> proc_macro2::TokenStream {
    let (fields_unnamed_left, fields_unnamed_right, fields_unnamed_comps) =
        make_unnamed_fields(fields);
    quote!(match (self, other) { (Self(#fields_unnamed_left), Self(#fields_unnamed_right)) => #fields_unnamed_comps })
}

fn make_named_fields(
    fields: &FieldsNamed,
) -> (
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
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
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
    proc_macro2::TokenStream,
) {
    let mut fields_unnamed_left = quote!();
    let mut fields_unnamed_right = quote!();
    let mut fields_unnamed_comps = quote!(true);
    for (n, field) in fields.unnamed.iter().rev().enumerate() {
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

fn make_comp_for_type(
    left_name: &Ident,
    right_name: &Ident,
    ty: &Type,
) -> proc_macro2::TokenStream {
    match ty {
        TraitObject(_) => quote!(#left_name.dyn_eq(#right_name.as_any())),
        Type::Reference(ty) => make_comp_for_type(left_name, right_name, &ty.elem),
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            match segments.iter().any(|segment| match &segment.arguments {
                syn::PathArguments::AngleBracketed(args) => args
                    .args
                    .iter()
                    .any(|arg| matches!(arg, syn::GenericArgument::Type(TraitObject(_)))),
                _ => false,
            }) {
                true => quote!(#left_name.dyn_eq(#right_name.as_any())),
                false => quote!(#left_name == #right_name),
            }
        }
        _ => quote!(#left_name == #right_name),
    }
}
