use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(Mineral)]
pub fn mineral_derive(
  input: TokenStream,
) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let ast =
    parse_macro_input!(input as ItemStruct);

  let name = &ast.ident;
  TokenStream::from(quote! {
      impl Mineral for #name {}
  })
}
