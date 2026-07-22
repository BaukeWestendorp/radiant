use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Error, Fields, parse_macro_input};

#[proc_macro_derive(Input)]
pub fn derive_input(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let Data::Enum(data) = &input.data else {
        return Error::new_spanned(name, "Input can only be derived for enums")
            .to_compile_error()
            .into();
    };

    let mut variants_list = Vec::new();
    let mut match_arms = Vec::new();

    for variant in &data.variants {
        if !matches!(variant.fields, Fields::Unit) {
            return Error::new_spanned(
                variant,
                "Input can only be derived for enums with unit variants",
            )
            .to_compile_error()
            .into();
        }

        let var_ident = &variant.ident;
        let var_name_str = var_ident.to_string();

        variants_list.push(quote! {
            #name::#var_ident
        });

        match_arms.push(quote! {
            #name::#var_ident => #var_name_str.to_string()
        });
    }

    let expanded = quote! {
        impl ::rd_ui::DropdownValue for #name {
            fn variants() -> Vec<Self>
            where
                Self: Sized
            {
                vec![
                    #( #variants_list, )*
                ]
            }

            fn label(&self) -> String {
                match self {
                    #( #match_arms, )*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
