use proc_macro::TokenStream;

mod object;

#[proc_macro_derive(Object)]
pub fn derive_value(input: TokenStream) -> TokenStream {
    object::derive(input)
}

#[proc_macro_attribute]
pub fn object(args: TokenStream, input: TokenStream) -> TokenStream {
    object::attribute(args, input)
}
