use darling::{FromDeriveInput, FromVariant};
use quote::quote;
use syn::{
    Data, DataEnum, DeriveInput, Ident, Path, Variant, parse_macro_input, spanned::Spanned as _,
};

#[derive(FromDeriveInput)]
#[darling(supports(enum_any), attributes(value))]
struct Attrs {
    graph_def: Path,
    data_type: Ident,
}

#[derive(FromVariant)]
#[darling(attributes(value))]
struct VariantMeta {
    color: u32,
}

pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let attrs: Attrs = match Attrs::from_derive_input(&input) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors().into();
        }
    };

    let variants = match input.data {
        Data::Enum(DataEnum { variants, .. }) => variants.into_iter().collect::<Vec<_>>(),
        _ => {
            return syn::Error::new(input.span(), "Only enums are supported")
                .to_compile_error()
                .into();
        }
    };

    let impl_value = gen_impl_value(&attrs.graph_def, &input.ident, &variants);
    let impl_value_froms = gen_impl_froms(&input.ident, &variants);
    let data_type = gen_data_type(&attrs.data_type, &variants);
    let data_type_impl = gen_data_type_impl(&attrs.graph_def, &attrs.data_type, &variants);

    let extracted = quote! {
        #impl_value
        #(#impl_value_froms)*

        #data_type
        #data_type_impl
    };

    extracted.into()
}

fn gen_impl_value(
    graph_def: &Path,
    value: &Ident,
    variants: &[Variant],
) -> proc_macro2::TokenStream {
    let variant_idents = variants.iter().map(|variant| &variant.ident);

    quote! {
        impl flow::Value<#graph_def> for #value {
            fn data_type(&self) -> <#graph_def as flow::GraphDef>::DataType {
                match self {
                    #(Self::#variant_idents(_) => <#graph_def as flow::GraphDef>::DataType::#variant_idents),*
                }
            }

            fn cast_to(&self, to: &<#graph_def as flow::GraphDef>::DataType) -> Option<Value> {
                match (self, to) {
                    _ => todo!(),
                }
            }
        }
    }
}

fn gen_impl_froms(value: &Ident, variants: &[Variant]) -> Vec<proc_macro2::TokenStream> {
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
                        .to_compile_error();
                }
            };

            let variant_ident = &variant.ident;

            quote! {
                impl From<#inner_type> for #value {
                    fn from(value: #inner_type) -> Self {
                        #value::#variant_ident(value)
                    }
                }

                impl TryFrom<#value> for #inner_type {
                    type Error = flow::FlowError;

                    fn try_from(value: #value) -> Result<Self, Self::Error> {
                        match value {
                            #value::#variant_ident(value) => Ok(value),
                            _ => Err(Self::Error::CastFailed)
                        }
                    }
                }
            }
        })
        .collect()
}

fn gen_data_type(name: &Ident, variants: &[Variant]) -> proc_macro2::TokenStream {
    let variant_idents = variants.iter().map(|variant| &variant.ident);

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum #name {
            #(#variant_idents),*
        }
    }
}

fn gen_data_type_impl(
    graph_def: &Path,
    data_type: &Ident,
    variants: &[Variant],
) -> proc_macro2::TokenStream {
    let variant_idents = variants.iter().map(|variant| &variant.ident);

    let color_arms = variants.iter().map(|variant| {
        let VariantMeta { color, .. } =
            VariantMeta::from_variant(&variant).expect("Failed to parse variant metadata");
        let variant_ident = &variant.ident;

        quote! {
            Self::#variant_ident => gpui::rgb(#color).into(),
        }
    });

    quote! {
        impl flow::DataType<#graph_def> for #data_type {
            fn default_value(&self) -> <#graph_def as flow::GraphDef>::Value {
                match self {
                    #(Self::#variant_idents => <#graph_def as flow::GraphDef>::Value::#variant_idents(Default::default())),*
                }
            }

            fn color(&self) -> gpui::Hsla {
                match self {
                    #(#color_arms)*
                }
            }
        }
    }
}
