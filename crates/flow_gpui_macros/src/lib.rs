use proc_macro::TokenStream;
use syn::parse_macro_input;

mod node_kind;

#[proc_macro_derive(
    NodeKind,
    attributes(node_kind, input, computed_output, constant_output, meta)
)]
pub fn node_kind_derive(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as syn::DeriveInput);
    node_kind::derive(derive_input).unwrap().into()
}
