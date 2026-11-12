use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Fields, FieldsUnnamed, Generics, Ident, parse_macro_input};

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => {
            return syn::Error::new_spanned(name, "Encode can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let mut generics = input.generics.clone();

    for field in fields.iter() {
        let field_type = &field.ty;

        generics
            .make_where_clause()
            .predicates
            .push(syn::parse_quote!(
                #field_type: ::djson::Encode
            ));
    }

    let expanded = match fields {
        Fields::Named(fields_named) => encode_named_fields(name, fields_named, &generics),
        Fields::Unnamed(fields_unnamed) => encode_unnamed_fields(name, fields_unnamed, &generics),
        Fields::Unit => encode_unit_struct(name, &generics),
    };

    TokenStream::from(expanded)
}

#[allow(unused)]
#[proc_macro_derive(Decode)]
pub fn derive_decode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        syn::Data::Struct(data) => &data.fields,
        _ => {
            return syn::Error::new_spanned(name, "Decode can only be derived for structs")
                .to_compile_error()
                .into();
        }
    };

    let expanded = match fields {
        syn::Fields::Named(fields_named) => {
            decode_named_fields(name, fields_named, &input.generics)
        }
        syn::Fields::Unnamed(fields_unnamed) => {
            decode_unnamed_fields(name, fields_unnamed, &input.generics)
        }
        syn::Fields::Unit => decode_unit_struct(name, &input.generics),
    };

    return TokenStream::from(expanded);
}

fn encode_named_fields(
    name: &Ident,
    fields_named: &syn::FieldsNamed,
    generics: &Generics,
) -> TokenStream {
    let field_names = fields_named
        .named
        .iter()
        .map(|field| field.ident.as_ref().unwrap());

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::djson::Encode
            for #name #ty_generics #where_clause
        {
            fn encode(&self) -> ::djson::JsonValue {
                let mut object = ::djson::Map::new();

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

fn encode_unnamed_fields(
    name: &Ident,
    fields_unnamed: &FieldsUnnamed,
    generics: &Generics,
) -> TokenStream {
    let indexes = (0..fields_unnamed.unnamed.len()).map(syn::Index::from);

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::djson::Encode
            for #name #ty_generics #where_clause
        {
            fn encode(&self) -> ::djson::JsonValue {
                let mut list = Vec::new();

                #(
                    list.push(
                        ::djson::Encode::encode(&self.#indexes),
                    );
                )*

                ::djson::JsonValue::Array(list)
            }
        }
    }
    .into()
}

fn encode_unit_struct(name: &Ident, generics: &Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::djson::Encode
            for #name #ty_generics #where_clause
        {
            fn encode(&self) -> ::djson::JsonValue {
                ::djson::JsonValue::Object(
                    ::djson::Map::new()
                )
            }
        }
    }
    .into()
}

fn decode_named_fields(
    name: &Ident,
    fields: &syn::FieldsNamed,
    generics: &syn::Generics,
) -> TokenStream {
    let field_decoders = fields.named.iter().map(|field| {
        let field_name = field.ident.as_ref().unwrap();
        let field_type = &field.ty;
        let key = field_name.to_string();

        quote! {
            #field_name: {
                let value = object
                    .get(#key)
                    .ok_or(::djson::JsonFormatError::MissingField)?;

                <#field_type as ::djson::Decode>::decode(value)?
            }
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::djson::Decode
            for #name #ty_generics #where_clause
        {
            fn decode(
                json: &::djson::JsonValue
            ) -> ::djson::Result<Self> {
                let object = match json {
                    ::djson::JsonValue::Object(object) => object,
                    _ => return Err(::djson::JsonFormatError::InvalidObject),
                };

                Ok(Self {
                    #(#field_decoders),*
                })
            }
        }
    }
    .into()
}

fn decode_unnamed_fields(
    name: &Ident,
    fields: &FieldsUnnamed,
    generics: &syn::Generics,
) -> TokenStream {
    let field_decoders = fields.unnamed.iter().map(|field| {
        let field_type = &field.ty;

        quote! {
            <#field_type as ::djson::Decode>::decode(
                values.next()
                    .ok_or(::djson::JsonFormatError::MissingField)?
            )?
        }
    });

    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::djson::Decode
            for #name #ty_generics #where_clause
        {
            fn decode(
                json: &::djson::JsonValue
            ) -> ::djson::Result<Self> {
                let mut values = match json {
                    ::djson::JsonValue::Array(values) => values.into_iter(),
                    _ => return Err(::djson::JsonFormatError::InvalidArray),
                };

                let result = Self(
                    #(#field_decoders),*
                );

                if values.next().is_some() {
                    return Err(::djson::JsonFormatError::InvalidArray);
                }

                Ok(result)
            }
        }
    }.into()
}

fn decode_unit_struct(name: &Ident, generics: &syn::Generics) -> TokenStream {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics ::djson::Decode
            for #name #ty_generics #where_clause
        {
            fn decode(
                json: &::djson::JsonValue
            ) -> ::djson::Result<Self> {
                match json {
                    ::djson::JsonValue::Object(_) => Ok(Self),
                    _ => Err(::djson::JsonFormatError::InvalidObject),
                }
            }
        }
    }
    .into()
}
