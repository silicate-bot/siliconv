use darling::FromAttributes;
use proc_macro::TokenStream;
use quote::quote;

#[derive(FromAttributes)]
#[darling(attributes(meta))]
struct MetaOptions {
    #[darling(rename = "rename")]
    renamed: Option<String>,
    default: Option<syn::Expr>,
}

fn from_each_named_field(
    transform: impl Fn(&syn::Field) -> proc_macro2::TokenStream,
    data_struct: &syn::DataStruct,
) -> Vec<proc_macro2::TokenStream> {
    let mut fields = Vec::new();
    if let syn::Fields::Named(fields_named) = &data_struct.fields {
        for field in &fields_named.named {
            fields.push(transform(field));
        }
    }

    fields
}

pub fn derive_meta(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    let fields = if let syn::Data::Struct(data_struct) = &input.data {
        from_each_named_field(
            |field| {
                let meta = MetaOptions::from_attributes(&field.attrs).unwrap();

                let ident = field.ident.as_ref().unwrap();
                let name = meta.renamed.unwrap_or(ident.to_string());

                quote! {
                    map.insert(
                        #name.to_string(),
                        siliconv_core::meta::MetaField::new(|| self.#ident.encode()),
                    );
                }
            },
            data_struct,
        )
    } else {
        Vec::new()
    };

    let from_fields = if let syn::Data::Struct(data_struct) = &input.data {
        from_each_named_field(
            |field| {
                let meta = MetaOptions::from_attributes(&field.attrs).unwrap();
                let default = meta
                    .default
                    .map(|expr| quote! { #expr })
                    .unwrap_or(quote! { Default::default() });

                let ident = field.ident.as_ref().unwrap();
                let name = meta.renamed.unwrap_or(ident.to_string());

                quote! {
                    #ident: fields
                        .get(#name)
                        .and_then(|f| f.decode::<_>(f.encode()))
                        .unwrap_or(#default),
                }
            },
            data_struct,
        )
    } else {
        Vec::new()
    };

    let ident = &input.ident;
    let output = quote! {
        impl siliconv_core::meta::Meta for #ident {
            fn fields(&self) -> std::collections::HashMap<String, siliconv_core::meta::MetaField<'_>> {
                use siliconv_core::meta::MetaEncodable;
                let mut map = std::collections::HashMap::new();
                #(#fields)*
                map
            }

            fn from_fields(fields: std::collections::HashMap<String, siliconv_core::meta::MetaField<'_>>) -> Self {
                use siliconv_core::meta::{MetaEncodable, MetaDecodable};
                Self {
                    #(#from_fields)*
                }
            }
        }
    };

    output.into()
}
