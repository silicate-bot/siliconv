//! Procedural macros for interacting with Siliconv replays.

use proc_macro::TokenStream;

mod meta;

#[proc_macro_derive(Meta, attributes(meta))]
/// Derives the Meta trait for a struct.
pub fn derive_meta_macro(item: TokenStream) -> TokenStream {
    meta::derive_meta(item)
}
