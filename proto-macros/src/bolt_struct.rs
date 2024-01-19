use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, Fields, Generics, ItemConst, ItemStruct, Meta, parse_macro_input, parse_quote, WhereClause};
use proto_common::marker::MARKER_TINY_STRUCT_BASE;


fn get_struct_info(
    structure: ItemStruct,
) -> (Ident, Generics, Option<WhereClause>, Fields, u8) {
    let name = structure.ident;
    let type_args = structure.generics;
    let where_clause = type_args.where_clause.clone();
    let fields = structure.fields;

    let marker = match fields.len() {
        0..=15 => MARKER_TINY_STRUCT_BASE | fields.len() as u8,
        _ => panic!("struct has too many fields"),
    };

    (name, type_args, where_clause, fields, marker)
}

pub(crate) fn generate_struct(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let a = item.clone();
    let structure = syn::parse2::<ItemStruct>(item).expect("generate item struct error");

    let (name, type_args, where_clause, fields, marker) =
        get_struct_info(structure.clone());


    let field_names: Vec<Ident> = fields.into_iter().map(|f| f.ident.unwrap()).collect();

    let byte_var_names: Vec<Ident> = field_names
        .iter()
        .map(|name| format_ident!("{}_bytes", name))
        .collect();

    let byte_var_defs = byte_var_names.iter()
        .zip(field_names.iter())
        .map(|(var_name, field_name)| {
            quote!(let #var_name = crate::value::value::Value::from(self.#field_name).serialize()?;)
        });

    let deserialize_var_defs = field_names.iter().map(|name| {
        quote!(
            let (#name, remaining) = crate::value::value::Value::deserialize(bytes)?;
            bytes = remaining;
        )
    });

    let deserialize_fields = field_names
        .iter()
        .map(|name| quote!(#name: #name.try_into()?,));

    quote!(
        #structure

        impl #type_args crate::serialization::BoltValue for #name #type_args
        #where_clause
        {
            fn marker(&self) -> crate::result_type::SerializeResult<u8> {
                Ok(#marker)
            }

            fn serialize(self) -> crate::result_type::SerializeResult<::bytes::Bytes> {
                use ::bytes::BufMut;
                use crate::serialization::{BoltStructure, BoltValue};

                let marker = self.marker()?;
                let signature = self.signature();
                #(#byte_var_defs)*

                // Marker byte, signature byte, then the rest of the data
                let mut result_bytes_mut = ::bytes::BytesMut::with_capacity(
                    std::mem::size_of::<u8>() * 2 #(+ #byte_var_names.len())*
                );
                result_bytes_mut.put_u8(marker);
                result_bytes_mut.put_u8(signature);
                #(result_bytes_mut.put(#byte_var_names);)*
                Ok(result_bytes_mut.freeze())
            }

            fn deserialize<B>(mut bytes: B) -> crate::result_type::DeserializeResult<(Self, B)>
            where B: ::bytes::Buf + ::std::panic::UnwindSafe
            {
                #(#deserialize_var_defs)*
                Ok((Self { #(#deserialize_fields)* }, bytes))
            }
        }
    ).into()
}
