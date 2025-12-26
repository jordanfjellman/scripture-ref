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
                "Book can only be derived for enums",
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
        let enum_name = &self.name;
        #[allow(unused_variables)]
        let chapters_arms = self.generate_chapters_arms();
        #[allow(unused_variables)]
        let verses_arms = self.generate_verses_arms();

        quote! {
            impl #enum_name {
                /// Returns the number of chapters in the book.
                pub fn chapter_count(&self) -> u8 {
                    match self {
                        #(#chapters_arms)*
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
                        Err(format!("Chapter {} is out of range for {:?}", chapter, self))
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
