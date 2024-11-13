use darling::FromVariant;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, Variant};

#[derive(Debug, FromVariant)]
#[darling(attributes(node_category))]
struct Meta {
    name: Option<String>,
}

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let variants = match &input.data {
        Data::Enum(data) => data.variants.clone().into_iter().collect::<Vec<_>>(),
        _ => panic!("Only enums are supported"),
    };

    let name = &input.ident;
    let impl_node_category = gen_impl_node_category(name, &variants);
    let impl_display = gen_impl_display(name, &variants);

    Ok(quote! {
        #impl_node_category
        #impl_display
    })
}

fn gen_impl_node_category(name: &Ident, variants: &[Variant]) -> TokenStream {
    let vars = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            #name::#variant_name
        }
    });

    quote! {
        impl flow::gpui::NodeCategory for #name {
            fn all() -> impl Iterator<Item = Self> {
                vec![#(#vars),*].into_iter()
            }
        }
    }
}

fn gen_impl_display(name: &Ident, variants: &[Variant]) -> TokenStream {
    let cases = variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let meta = Meta::from_variant(variant).ok();
        let name_value = match meta.and_then(|meta| meta.name) {
            Some(name) => name,
            None => variant_name.to_string(),
        };

        quote! {
            #name::#variant_name => #name_value,
        }
    });

    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let str = match self {
                    #(#cases)*
                }
                .to_string();
                write!(f, "{}", str)
            }
        }
    }
}
