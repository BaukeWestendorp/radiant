use darling::FromVariant;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, Variant};

#[derive(Debug, FromVariant)]
#[darling(attributes(node_category))]
struct NodeCategoryVariantArgs {
    name: Option<String>,
}

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    let variants = match &input.data {
        Data::Enum(data) => data.variants.clone().into_iter().collect::<Vec<_>>(),
        _ => panic!("Only enums are supported"),
    };

    let name = &input.ident;
    let helper_types = gen_helper_types(name);
    let impl_node_category = gen_impl_node_category(name, &variants);
    let impl_display = gen_impl_display(name, &variants);

    let expansion = quote! {
        #helper_types
        #impl_node_category
        #impl_display
    };

    Ok(expansion)
}

fn gen_helper_types(name: &Ident) -> TokenStream {
    quote! {
        type __GraphNodeCategory = #name;
    }
}

fn gen_impl_node_category(name: &Ident, variants: &[Variant]) -> TokenStream {
    let vars = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        quote! {
            #name::#variant_name
        }
    });

    quote! {
        impl flow_gpui::NodeCategory for #name {
            fn all() -> impl Iterator<Item = Self> {
                vec![#(#vars),*].into_iter()
            }
        }
    }
}

fn gen_impl_display(name: &Ident, variants: &[Variant]) -> TokenStream {
    let cases = variants.iter().map(|variant| {
        let variant_name = &variant.ident;

        let meta = NodeCategoryVariantArgs::from_variant(&variant).ok();
        let name_value = match meta.map(|meta| meta.name).flatten() {
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
