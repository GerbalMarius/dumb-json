use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;


#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => {
            return syn::Error::new_spanned(name, "Encode can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let expanded = match fields {
        syn::Fields::Named(fields_named) => encode_named_fields(name, fields_named),
        syn::Fields::Unnamed(fields_unamed) => encode_unamed_fields(name, fields_unamed),
        syn::Fields::Unit => encode_uninit_struct(name),
    };

    return TokenStream::from(expanded);
}

fn encode_named_fields(name: &syn::Ident, fields_named: &syn::FieldsNamed) -> TokenStream {
    let field_names = fields_named
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap());

    quote! {
            impl ::djson::Encode for #name {
                fn encode(&self) -> ::djson::JsonValue {
                    let mut object = ::std::collections::HashMap::new();

                    #(
                        object.insert(
                            stringify!(#field_names).to_string(),
                            ::djson::Encode::encode(&self.#field_names),
                        );
                    )*

                    ::djson::JsonValue::Object(object)
                }
            }
    }
    .into()
}

fn encode_unamed_fields(name: &syn::Ident, fields_unamed: &syn::FieldsUnnamed) -> TokenStream {
    let indexes = (0..fields_unamed.unnamed.len()).map(syn::Index::from);

    quote! {
        impl ::djson::Encode for #name {
            fn encode(&self) -> ::djson::JsonValue {
                let mut object = ::std::collections::HashMap::new();

                #(
                    object.insert(
                        stringify!(#indexes).to_string(),
                        ::djson::Encode::encode(&self.#indexes),
                    );
                )*

                ::djson::JsonValue::Object(object)
            }
        }
    }
    .into()
}

fn encode_uninit_struct(name: &syn::Ident) -> TokenStream {
    quote! {
            impl ::djson::Encode for #name {
                fn encode(&self) -> ::djson::JsonValue {
                    ::djson::JsonValue::Object(
                        ::std::collections::HashMap::new()
                    )
                }
            }
    }.into()
}
