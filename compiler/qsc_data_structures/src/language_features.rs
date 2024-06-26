// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use bitflags::bitflags;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Copy)]
pub struct LanguageFeatures(u8);

bitflags! {
    impl LanguageFeatures: u8 {
        const V2PreviewSyntax = 0b1;
        const PreviewQirGen = 0b10;
    }
}

impl LanguageFeatures {
    pub fn merge(&mut self, other: impl Into<LanguageFeatures>) {
        self.0 |= other.into().0;
    }
}

impl Default for LanguageFeatures {
    fn default() -> Self {
        LanguageFeatures::empty()
    }
}

impl<I> FromIterator<I> for LanguageFeatures
where
    I: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        iter.into_iter().fold(LanguageFeatures::empty(), |acc, x| {
            acc | match x.as_ref() {
                "v2-preview-syntax" => LanguageFeatures::V2PreviewSyntax,
                "preview-qir-gen" => LanguageFeatures::PreviewQirGen,
                _ => LanguageFeatures::empty(),
            }
        })
    }
}
