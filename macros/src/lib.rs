use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput, Field};
#[proc_macro_derive(Getter)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let st_name = input.ident;
    let fields = match input.data {
        syn::Data::Struct(value) => match value.fields {
            syn::Fields::Named(name) => name.named,
            _ => panic!("the struct fields should have name"),
        },
        _ => panic!("Error it should be struct"),
    };
    let getter = getter(fields);
    let expand = quote!(
        impl #st_name {
            #(#getter)*
        }
    );
    expand.into()
}

fn getter(fields: Punctuated<Field, Comma>) -> impl Iterator<Item = proc_macro2::TokenStream> {
    fields.into_iter().map(|field| {
        let getter_fn_name = field.ident;
        let field_ty = field.ty;
        quote!(
            pub fn #getter_fn_name(&self) -> &#field_ty{
                &self.#getter_fn_name
            }
        )
    })
}
