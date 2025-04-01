use proc_macro::TokenStream;

mod value;

#[proc_macro_derive(Value, attributes(value))]
pub fn derive_value(input: TokenStream) -> TokenStream {
    value::derive(input)
}
