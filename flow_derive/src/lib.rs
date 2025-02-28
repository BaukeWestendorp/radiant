use proc_macro::TokenStream;

mod node_kind;
mod value;

#[proc_macro_derive(
    NodeKind,
    attributes(node_kind, node, input, computed_output, constant_output, processor)
)]
pub fn derive_node_kind(input: TokenStream) -> TokenStream {
    node_kind::derive(input)
}

#[proc_macro_derive(Value, attributes(value))]
pub fn derive_value(input: TokenStream) -> TokenStream {
    value::derive(input)
}
