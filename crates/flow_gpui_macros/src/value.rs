// quote! {
//     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
//     pub enum TestGraphValue {
//         Float(f32),
//     }

//     impl Value<TestGraphDefinition> for TestGraphValue {
//         fn try_cast_to(&self, target: &TestGraphDataType) -> Result<Self, GraphError> {
//             use TestGraphDataType as DT;

//             match (self, target) {
//                 (Self::Float(_), DT::Float) => Ok(self.clone()),
//             }
//         }
//     }

//     impl TryFrom<TestGraphValue> for f32 {
//         type Error = GraphError;

//         fn try_from(value: TestGraphValue) -> Result<Self, Self::Error> {
//             match value {
//                 TestGraphValue::Float(value) => Ok(value),
//             }
//         }
//     }

//     #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
//     pub enum TestGraphDataType {
//         Float,
//     }

//     impl DataType<TestGraphDefinition> for TestGraphDataType {
//         fn default_value(&self) -> TestGraphValue {
//             match self {
//                 Self::Float => TestGraphValue::Float(f32::default()),
//             }
//         }
//     }

//     impl VisualDataType for TestGraphDataType {
//         fn color(&self) -> Hsla {
//             match self {
//                 Self::Float => rgb(0xFF3C59).into(),
//             }
//         }
//     }
// }

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
    let impl_try_froms = gen_impl_try_froms(&variants);

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
    let graph_definition = &attrs.graph_definition;
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
        impl flow::DataType<#graph_definition> for #data_type {
            fn default_value(&self) -> __GraphValue {
                match self {
                    #(#default_cases)*,
                }
            }
        }
    }
}

fn gen_impl_try_froms(variants: &[Variant]) -> TokenStream {
    fn gen_impl_try_from_for_type(variant_ident: &Ident, ty: &Type) -> TokenStream {
        quote! {
            impl TryFrom<__GraphValue> for #ty {
                type Error = flow::GraphError;

                fn try_from(value: __GraphValue) -> Result<Self, Self::Error> {
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
            gen_impl_try_from_for_type(&variant.ident, &ty)
        }
        _ => panic!("Only unnamed fields are supported"),
    });

    quote! {
        #(#impls)*
    }
}
