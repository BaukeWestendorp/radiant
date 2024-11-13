use darling::{FromDeriveInput, FromVariant};
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, DeriveInput, Expr, Fields, Ident, Type, Variant};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(value))]
struct Attrs {
    graph_definition: Type,
    data_type: Ident,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(meta))]
struct VariantMeta {
    default_value: Expr,
}

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let attrs = Attrs::from_derive_input(&input)?;

    let variants = match &input.data {
        Data::Enum(data) => data.variants.clone().into_iter().collect::<Vec<_>>(),
        Data::Struct(data) => {
            return Err(syn::Error::new(
                data.struct_token.span(),
                "Only enums are supported",
            ))
        }
        Data::Union(data) => {
            return Err(syn::Error::new(
                data.union_token.span(),
                "Only enums are supported",
            ))
        }
    };

    let decl = gen_decl(&input, &attrs, &variants)?;
    let impl_data_type = gen_impl_data_type(&attrs, &variants);
    let impl_try_froms = gen_impl_try_froms(&variants, &attrs.graph_definition);

    Ok(quote! {
        #decl

        #impl_data_type

        #impl_try_froms
    })
}

fn gen_decl(input: &DeriveInput, attrs: &Attrs, variants: &[Variant]) -> syn::Result<TokenStream> {
    let vis = &input.vis;
    let data_type = &attrs.data_type;

    let variant_idents = variants.iter().map(|variant| &variant.ident);

    Ok(quote! {
        #[derive(Clone, serde::Serialize, serde::Deserialize)]
        #vis enum #data_type {
            #(#variant_idents)*,
        }
    })
}

fn gen_impl_data_type(attrs: &Attrs, variants: &[Variant]) -> TokenStream {
    let graph_def = &attrs.graph_definition;
    let data_type = &attrs.data_type;

    let default_cases = variants.iter().map(|variant| {
        let meta = VariantMeta::from_variant(variant).expect("#[meta(...) is required on variant");

        let var = &variant.ident;
        let default_value = &meta.default_value;

        quote! {
            Self::#var => __GraphValue::#var(#default_value)
        }
    });

    quote! {
        impl flow::DataType<#graph_def> for #data_type {
            fn default_value(&self) -> <#graph_def as flow::GraphDefinition>::Value {
                type __GraphValue = <#graph_def as flow::GraphDefinition>::Value;

                match self {
                    #(#default_cases)*,
                }
            }
        }
    }
}

fn gen_impl_try_froms(variants: &[Variant], graph_def: &Type) -> TokenStream {
    fn gen_impl_try_from_for_type(
        variant_ident: &Ident,
        ty: &Type,
        graph_def: &Type,
    ) -> TokenStream {
        quote! {
            impl TryFrom<<#graph_def as flow::GraphDefinition>::Value> for #ty {
                type Error = flow::GraphError;

                fn try_from(value: <#graph_def as flow::GraphDefinition>::Value) -> Result<Self, Self::Error> {

                    type __GraphValue = <#graph_def as flow::GraphDefinition>::Value;

                    match value {
                        __GraphValue::#variant_ident(value) => Ok(value),
                        _ => Err(flow::GraphError::CastFailed),
                    }
                }
            }
        }
    }

    let impls = variants.iter().map(|variant| match &variant.fields {
        Fields::Unnamed(field) => {
            let ty = field.unnamed.first().unwrap().ty.clone();
            gen_impl_try_from_for_type(&variant.ident, &ty, graph_def)
        }
        _ => panic!("Only unnamed fields are supported"),
    });

    quote! {
        #(#impls)*
    }
}
