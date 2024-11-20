use proc_macro::TokenStream;

mod node_kind;

#[proc_macro_derive(
    NodeKind,
    attributes(node_kind, node, input, computed_output, constant_output, processor)
)]
pub fn derive_node_kind(input: TokenStream) -> TokenStream {
    node_kind::derive(input)
}
