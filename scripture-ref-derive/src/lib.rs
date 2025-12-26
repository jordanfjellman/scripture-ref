mod book_enum;
mod book_variant;
mod parse;

#[proc_macro_derive(Book, attributes(chapters, verses))]
pub fn derive_book(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    book_enum::BookEnumData::from_derive_input(&input)
        .map(|data| data.generate())
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
