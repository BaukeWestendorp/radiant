use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Data, DataEnum, DeriveInput, Variant};

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants.into_iter().collect::<Vec<_>>(),
        _ => {
            return syn::Error::new(input.span(), "A Value can only be an enum")
                .to_compile_error()
                .into()
        }
    };

    let impl_froms = gen_impl_froms(&variants);

    let extracted = quote! {
        #(#impl_froms)*
    };

    extracted.into()
}

fn gen_impl_froms(variants: &[Variant]) -> Vec<TokenStream> {
    variants
        .iter()
        .map(|variant| {
            let inner_type = match &variant.fields {
                syn::Fields::Unnamed(fields) => {
                    if fields.unnamed.len() != 1 {
                        return syn::Error::new(
                            fields.span(),
                            "Only one field is supported in a variant",
                        )
                        .to_compile_error();
                    }

                    fields.unnamed.first().unwrap().ty.clone()
                }
                _ => {
                    return syn::Error::new(variant.span(), "Only unnamed fields are supported")
                        .to_compile_error()
                }
            };

            let variant_ident = &variant.ident;

            quote! {
                impl From<#inner_type> for Value {
                    fn from(value: #inner_type) -> Self {
                        Value::#variant_ident(value)
                    }
                }

                impl TryFrom<Value> for #inner_type {
                    type Error = FlowError;

                    fn try_from(value: Value) -> Result<Self, Self::Error> {
                        match value {
                            Value::#variant_ident(value) => Ok(value),
                            _ => Err(FlowError::CastFailed),
                        }
                    }
                }
            }
        })
        .collect()
}
