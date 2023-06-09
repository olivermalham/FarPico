extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, DataStruct, Fields};

#[proc_macro_derive(HAL)]
pub fn hal_derive_proc_macro(input: TokenStream) -> TokenStream {
    // 1. Use syn to parse the input tokens into a syntax tree.
    // 2. Generate new tokens based on the syntax tree. This is additive to the `enum` or
    //    `struct` that is annotated (it doesn't replace them).
    // 3. Return the generated tokens.

    let DeriveInput {
        ident: struct_name_ident,
        data,
        ..
    } = parse_macro_input!(input as DeriveInput); // Same as: syn::parse(input).unwrap();

    let fields = match data {
        Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) => fields.named,
        _ => panic!("this derive macro only works on structs with named fields"),
    };

    println!("Fields: {:?}", fields.clone());

    let dispatch_calls = fields.clone().into_iter().map(|f| {

        let field_name = f.ident;

        quote! {
            "#field_name" => self.#field_name.dispatch(action, parameter_json)?
            }
        }
    );

    let refresh_calls = fields.into_iter().map(|f| {

        let field_name = f.ident;

        quote! { self.#field_name.refresh()? }
        }
    );

    quote! {
        impl HalFuncs for #struct_name_ident {
            fn to_json(&self) -> String {
                serde_json::to_string(self).ok().unwrap()
            }

            fn dispatch(&mut self, target: &str, action: &str, parameter_json: &str) -> Result <(), String> {
                println!("Action Request {}.{} - {}", target, action, parameter_json);
                match target {
                    #(#dispatch_calls,)*
                    _ => ()
                };
                return Ok(());
            }

            fn refresh(&mut self) -> Result <(), String> {
                println!("Refresh Request!");

                #(#refresh_calls;)*

                return Ok(());
            }
        }
    }.into()
}
