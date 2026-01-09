use quote::quote;
use syn::{Data, DeriveInput, Error, Ident};

use crate::book_variant::BookVariantData;

pub struct BookEnumData {
    pub name: Ident,
    pub variants: Vec<BookVariantData>,
}

impl BookEnumData {
    pub fn from_derive_input(input: &DeriveInput) -> syn::Result<Self> {
        let Data::Enum(data_enum) = &input.data else {
            return Err(Error::new_spanned(
                input,
                "'Book' can only be derived for enums",
            ));
        };

        let variants = data_enum
            .variants
            .iter()
            .map(BookVariantData::from_variant)
            .collect::<syn::Result<Vec<_>>>()?;

        Ok(Self {
            name: input.ident.clone(),
            variants,
        })
    }
}

impl BookEnumData {
    pub fn generate(&self) -> proc_macro2::TokenStream {
        let book_impl = self.generate_book_impl();
        let book_series_enum = self.generate_book_series_enum();
        let book_series_impl = self.generate_book_series_impl();

        quote! {
            #book_impl
            #book_series_enum
            #book_series_impl
        }
    }

    fn generate_book_impl(&self) -> proc_macro2::TokenStream {
        let enum_name = &self.name;
        let chapters_arms = self.generate_chapters_arms();
        let verses_arms = self.generate_verses_arms();
        let canonical_name_arms = self.generate_canonical_name_arms();

        quote! {
            impl #enum_name {
                /// Returns the number of chapters in the book.
                pub fn chapter_count(&self) -> u8 {
                    match self {
                        #(#chapters_arms)*
                    }
                }

                /// Returns the canonical name of the book.
                ///
                /// For example, the canonical name of Genesis is "Genesis".
                /// The canonical name of 1 Kings is "Kings".
                /// The canonical name of 2 Timothy is "Timothy".
                pub fn canonical_name(&self) -> &'static str {
                    match self {
                        #(#canonical_name_arms)*
                    }
                }

                /// Returns the maximum potential number of verses for a given chapter in book.
                /// Does not include exceptions in base texts or translations.
                pub fn max_verse_count_by_chapter(&self) -> &'static [u8] {
                    match self {
                        #(#verses_arms)*
                    }
                }

                pub fn max_verses_in_chapter(&self, chapter: u8) -> Result<u8, String> {
                    let verses = self.max_verse_count_by_chapter();
                    if chapter < 1 || chapter > verses.len() as u8 {
                        Err(format!("{:?} does not have chapter {}", self, chapter))
                    } else {
                        Ok(verses[chapter as usize - 1])
                    }

                }

                /// Returns the maximum number of verses for a book. Does not include potential
                /// exceptions in base texts or translations.
                pub fn verse_count(&self) -> u8 {
                    self.max_verse_count_by_chapter().iter().sum()
                }
            }
        }
    }

    fn generate_canonical_name_arms(&self) -> Vec<proc_macro2::TokenStream> {
        let enum_name = &self.name;
        self.variants
            .iter()
            .map(|v| {
                let variant_name = &v.name;
                let canonical_name = &v
                    .series
                    .as_ref()
                    .map(|name| quote! { #name })
                    .unwrap_or_else(|| {
                        quote! { stringify!(#variant_name) }
                    });
                quote! {
                    #enum_name::#variant_name => #canonical_name,
                }
            })
            .collect()
    }

    fn generate_chapters_arms(&self) -> Vec<proc_macro2::TokenStream> {
        let enum_name = &self.name;
        self.variants
            .iter()
            .flat_map(|v| {
                v.num_chapters.map(|num| {
                    let variant_name = &v.name;
                    quote! {
                        #enum_name::#variant_name => #num,
                    }
                })
            })
            .collect()
    }

    fn generate_verses_arms(&self) -> Vec<proc_macro2::TokenStream> {
        let enum_name = &self.name;
        self.variants
            .iter()
            .filter(|v| !v.max_verses_per_chapter.is_empty())
            .map(|v| {
                let variant_name = &v.name;
                let verses = &v.max_verses_per_chapter;
                // TODO: use array length  based on chapters, but not until performance testing can
                // verify improvements
                quote! { #enum_name::#variant_name => &[#(#verses),*], }
            })
            .collect()
    }

    // books without a series default to the enum name
    // books with a series default to the series name
    fn generate_book_series_enum(&self) -> proc_macro2::TokenStream {
        let series: std::collections::HashSet<String> = self
            .variants
            .iter()
            .map(|v| {
                let variant_name = v.name.to_string();
                v.series.as_ref().unwrap_or(&variant_name).to_owned()
            })
            .collect();
        let series_arms: Vec<syn::Ident> = series
            .iter()
            .map(|series| syn::Ident::new(series, proc_macro2::Span::call_site()))
            .collect();
        quote! {
            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            enum BookSeries {
                #(#series_arms,)*
            }
        }
    }

    fn generate_book_series_impl(&self) -> proc_macro2::TokenStream {
        let enum_name = &self.name;
        let book_series = self.variants.iter().map(|v| {
            let fallback = v.name.to_string();
            let variant_name = &v
                .series
                .as_ref()
                .map(|series| quote! { #series })
                .unwrap_or_else(|| quote! { stringify!(#fallback) });
            quote! {
                #enum_name::#variant_name => BookSeries::#variant_name,
            }
        });
        quote! {
            impl BookSeries {
                pub fn from_book(book: &Book) -> Self {
                    match book {
                        #(#book_series)*
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    #[test]
    fn test_from_derive_input() {
        let tokens = quote! {
            enum Book {
                #[chapters = "3"]
                #[verses = "31, 25, 24"]
                Alpha = 1,
                #[chapters = "2"]
                #[canonical_name = "The Book of Beta"]
                Beta,
            }
        };

        let data = BookEnumData::from_derive_input(&syn::parse2(tokens).unwrap()).unwrap();

        assert_eq!(data.name, "Book");
        assert_eq!(data.variants.len(), 2);

        let first_variant = &data.variants[0];
        assert_eq!(first_variant.name, "Alpha");
        assert_eq!(first_variant.num_chapters, Some(3));
        assert_eq!(first_variant.max_verses_per_chapter, vec![31, 25, 24]);

        let second_variant = &data.variants[1];
        assert_eq!(second_variant.name, "Beta");
        assert_eq!(second_variant.num_chapters, Some(2));
        assert!(second_variant.max_verses_per_chapter.is_empty());
        assert_eq!(second_variant.series, Some("The Book of Beta".to_string()));
    }

    #[test]
    fn test_rejects_struct() {
        let input: DeriveInput = syn::parse_quote! {
            struct NotAnEnum {
                field: u8,
            }
        };
        let result = BookEnumData::from_derive_input(&input);
        assert!(result.is_err());
    }
}
