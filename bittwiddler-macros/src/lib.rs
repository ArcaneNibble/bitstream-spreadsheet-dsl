use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn bittwiddler_hierarchy_level(_attr: TokenStream, input: TokenStream) -> TokenStream {
    bittwiddler_dsl::macros::bittwiddler_hierarchy_level(input.into()).into()
}

#[proc_macro_attribute]
pub fn bittwiddler_properties(_attr: TokenStream, input: TokenStream) -> TokenStream {
    bittwiddler_dsl::macros::bittwiddler_properties(input.into()).into()
}