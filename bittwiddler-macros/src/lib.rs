use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bittwiddler_hierarchy_level(attr: TokenStream, input: TokenStream) -> TokenStream {
    bittwiddler_dsl::macros::bittwiddler_hierarchy_level(attr.into(), input.into()).into()
}

#[proc_macro_attribute]
pub fn bittwiddler_properties(attr: TokenStream, input: TokenStream) -> TokenStream {
    bittwiddler_dsl::macros::bittwiddler_properties(attr.into(), input.into()).into()
}
