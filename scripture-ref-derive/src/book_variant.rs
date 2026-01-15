use std::collections::BTreeMap;

use syn::{Ident, Lit, Meta, Variant};

use crate::parse::{parse_array_from_string, parse_u8_array_from_string, parse_u8_from_string};

#[derive(Debug)]
pub struct BookVariantData {
    pub name: Ident,
    pub num_chapters: Option<u8>,
    pub max_verses_per_chapter: Vec<u8>,
    pub series: Option<String>,
    pub abbreviations: Vec<String>,
    canonical_name: Option<String>,
}

impl BookVariantData {
    pub fn from_variant(variant: &Variant) -> syn::Result<Self> {
        let name = variant.ident.clone();
        let mut num_chapters = None;
        let mut max_verses_per_chapter = Vec::new();
        let mut series = None;
        let mut canonical_name = None;
        let mut abbreviations = Vec::new();

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
            } else if ident == "canonical_name" {
                canonical_name = Some(string_value);
            } else if ident == "abbreviations" {
                abbreviations = parse_array_from_string(&string_value)?;
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
            abbreviations,
            canonical_name,
        })
    }

    pub fn canonical_name(&self) -> String {
        self.canonical_name
            .as_ref()
            .map(|name| name.to_string())
            .unwrap_or_else(|| self.name.to_string())
    }
}

#[derive(Debug)]
pub(crate) struct BookVariantMapping {
    pub(crate) variant_name: String,
    pub(crate) name: String,
}

pub(crate) type BookVariantLookupTable = BTreeMap<(char, usize), Vec<BookVariantMapping>>;

pub(crate) fn build_book_variant_lookup_table(
    variants: &Vec<BookVariantData>,
) -> BookVariantLookupTable {
    let mut table: BTreeMap<(char, usize), Vec<BookVariantMapping>> = BTreeMap::new();
    let mut variant_names: Vec<(String, Vec<String>)> = variants
        .iter()
        .map(|v| {
            let variant_name = v.name.to_string();
            (
                variant_name,
                v.abbreviations
                    .clone()
                    .into_iter()
                    .map(|a| a.to_ascii_lowercase())
                    .collect(),
            )
        })
        .collect();

    for variant in variants {
        let variant_name = variant.name.to_string();
        variant_names.push((
            variant_name,
            vec![variant.canonical_name().to_ascii_lowercase()],
        ));
    }

    for (variant_name, names) in variant_names {
        for name in names {
            if let Some(first_char) = name.chars().next() {
                let key = (first_char, name.len());
                table.entry(key).or_default().push(BookVariantMapping {
                    variant_name: variant_name.clone(),
                    name,
                });
            }
        }
    }

    table
}
