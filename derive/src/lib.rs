use proc_macro::TokenStream;

mod table;

#[proc_macro_derive(Table)]
pub fn derive_table(input: TokenStream) -> TokenStream {
    table::impl_derive(input)
}
