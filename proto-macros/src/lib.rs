use proc_macro::TokenStream;

mod bolt_struct;

#[proc_macro_attribute]
pub fn bolt_struct_derive(attr: TokenStream, item: TokenStream) -> TokenStream {
    bolt_struct::generate_struct(attr.into(), item.into()).into()
}
