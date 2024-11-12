use proc_macro::TokenStream;
use syn::parse_macro_input;

mod node_category;
mod node_kind;
mod value;

#[proc_macro_derive(
    NodeKind,
    attributes(node_kind, input, computed_output, constant_output, meta)
)]
pub fn node_kind_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    node_kind::derive(derive_input).unwrap().into()
}

#[proc_macro_derive(NodeCategory, attributes(node_category))]
pub fn node_category_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    node_category::derive(derive_input).unwrap().into()
}

#[proc_macro_derive(Value, attributes(value, meta))]
pub fn value_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    value::derive(derive_input).unwrap().into()
}
