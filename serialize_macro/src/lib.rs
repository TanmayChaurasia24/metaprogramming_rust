use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields}; // use to parse Rust syntax tree into Abstract syntax tree

#[proc_macro_derive(SerializeNumberStruct)]
pub fn serialize_number_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap(); // convert macro input to rust syntax tree
    let name = &ast.ident; // extract the name of the struct

    let serial_fields = match &ast.data {
        Data::Struct(data_struct) => {
            // make sure that we are working with structs only
            match &data_struct.fields {
                // this makes sure that inside the struct, there are name fields
                Fields::Named(fields) => {
                    let field_serial = fields.name.iter().map(|field| {
                        // this extracts every field
                        let field_name = &field.ident;
                        quote! {
                            result.extend_from_slice(&self.#field_name.to_be_bytes()); // coverts every field into bytes
                        }
                    });

                    quote! {
                        #(#field_serial)*
                    }
                }

                _ => panic!("only named fields are supperted"),
            }
        }

        _ => panic!("only structs are supported"),
    };

    let generated = quote! {
        impl Serialize for #name {
            fn serialize(&self) -> Vec<u8> {
                let mut result = Vec::new();
                #serial_fields
                result
            }
        }
    };

    generated.into();
}

#[proc_macro_derive(DeserializeNumberStruct)]
pub fn deserialize_number_struct(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap(); // macro input to syntax tree
    let name = &ast.ident;

    let (deserialize_fields, field_assignments, total_size) = match &ast.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let mut offset: usize = 0;
                let mut field_deserial = Vec::new();
                let mut field_assignments = Vec::new();

                for field in &fields.named {
                    let field_name = &field.ident;
                    let feild_size = 4;
                    let start_offset = offset;
                    let end_offset = offset + feild_size;

                    field_deserial.push(quote! {
                            let #field_name = {
                                let bytes: [u8; 4] = base[#start_offset..#end_offset].try_into().map_err(|_| Error)?;
                                i32::from_be_bytes(bytes)
                            };
                        });

                    field_assignments.push(quote! {
                        #field_name
                    });

                    offset += feild_size;
                }

                (field_deserial, field_assignments, offset);
            }

            _ => panic!("only named fields are supported"),
        },

        _ => panic!("only structs are supported"),
    };

    // Generate full impl block for Deserialize
    let generated = quote! {
        impl Deserialize for #name {
            fn deserialize(base: &[u8]) -> Result<Self, Error> {
                // Check that enough bytes were passed
                if base.len() < #total_size {
                    return Err(Error);
                }

                // Field deserialization code
                #(#deserialize_fields)*

                // Construct struct using deserialized values
                Ok(#name {
                    #(#field_assignments,)*
                })
            }
        }
    };

    generated.into()
}
