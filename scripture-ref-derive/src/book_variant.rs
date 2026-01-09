use syn::{Ident, Lit, Meta, Variant};

use crate::parse::{parse_u8_array_from_string, parse_u8_from_string};

pub struct BookVariantData {
    pub name: Ident,
    pub num_chapters: Option<u8>,
    pub max_verses_per_chapter: Vec<u8>,
    pub series: Option<String>,
}

impl BookVariantData {
    pub fn from_variant(variant: &Variant) -> syn::Result<Self> {
        let name = variant.ident.clone();
        let mut num_chapters = None;
        let mut max_verses_per_chapter = Vec::new();
        let mut series = None;

        for attr in &variant.attrs {
            let Meta::NameValue(meta) = &attr.meta else {
                continue;
            };

            let Some(ident) = meta.path.get_ident() else {
                continue;
            };

            let syn::Expr::Lit(expr_lit) = &meta.value else {
                continue;
            };

            let Lit::Str(lit_str) = &expr_lit.lit else {
                continue;
            };

            let string_value = lit_str.value();

            if ident == "chapters" {
                num_chapters = Some(parse_u8_from_string(&string_value)?);
            } else if ident == "verses" {
                max_verses_per_chapter = parse_u8_array_from_string(&string_value)?;
            } else if ident == "series" {
                series = Some(string_value);
            }
        }

        if let Some(num_chapters) = num_chapters
            && num_chapters as usize != max_verses_per_chapter.len()
        {
            return Err(syn::Error::new_spanned(
                &variant,
                "Number of chapters does not match number of verses",
            ));
        }

        Ok(Self {
            name,
            num_chapters,
            max_verses_per_chapter,
            series,
        })
    }
}
