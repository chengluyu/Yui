use crate::field::Fields;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{Data, DeriveInput, Error, Ident};

pub struct Attribute {
    ident: Ident,
    path: String,
    fields: Fields,
}

impl Attribute {
    pub fn from_ast(input: &DeriveInput) -> Result<Self, Error> {
        match &input.data {
            Data::Struct(data_struct) => {
                let path = input.ident.to_string();

                Ok(Attribute {
                    ident: input.ident.clone(),
                    path,
                    fields: Fields::from_ast(&data_struct.fields)?,
                })
            }
            _ => Err(Error::new_spanned(&input, "Attribute must be a struct")),
        }
    }

    pub fn get_implement(&self) -> TokenStream {
        let name = self.ident.clone();
        let from_attributes_args = self
            .fields
            .parse_attributes_args_token_stream(format_ident!("input"), name.clone());
        let from_meta = self.fields.parse_meta_token_stream(name.clone());
        let path = self.path.clone();
        let to_token_temp_value = self.fields.get_to_token_temp_value_token_stream();
        let to_token = self.fields.get_to_token_token_stream(name.clone());

        quote! {
            impl yui::AttributeStructure for #name {
                fn get_path() -> yui::Symbol {
                    yui::Symbol::new(#path)
                }

                fn from_meta(
                    input: &syn::Meta
                ) -> Result<Self, syn::Error>
                where
                    Self: std::marker::Sized {
                    #from_meta
                }

                fn from_attribute_args(input: syn::AttributeArgs) -> Result<Self, syn::Error>
                where
                    Self: std::marker::Sized {
                    #from_attributes_args
                }
            }

            impl syn::parse_macro_input::ParseMacroInput for #name {
                fn parse(input: syn::parse::ParseStream) -> Result<Self, syn::Error> {
                    let attribute_args = syn::AttributeArgs::parse(input)?;
                    use yui::AttributeStructure;
                    Self::from_attribute_args(attribute_args)
                }
            }

            impl quote::ToTokens for #name {
                 fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                    use quote::TokenStreamExt;
                    use yui::AttributeStructure;
                    #(#to_token_temp_value;)*
                    tokens.append(proc_macro2::Group::new(
                        proc_macro2::Delimiter::Brace,
                        quote::quote! {
                            #to_token
                        },
                    ))
                 }
            }
        }
    }
}
