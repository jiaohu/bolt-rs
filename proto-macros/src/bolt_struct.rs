use proc_macro2::TokenStream;
// use


pub(crate) fn generate_struct(attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = item.clone();
    let structure = syn::parse2::<syn::ItemStruct>(item).expect("generate storage error");
    eprintln!("{:?}", structure.fields.len());

    a
}