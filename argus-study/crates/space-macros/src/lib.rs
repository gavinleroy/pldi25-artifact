use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct};

#[proc_macro_derive(Collectible)]
pub fn collectible_derive(
  input: TokenStream,
) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let ast =
    parse_macro_input!(input as ItemStruct);

  // Build the impl
  let name = &ast.ident;
  TokenStream::from(quote! {
      impl space::prelude::Collectible for #name {}
  })
}

#[proc_macro_derive(Rock)]
pub fn rock_derive(
  input: TokenStream,
) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let ast =
    parse_macro_input!(input as ItemStruct);

  // Build the impl
  let name = &ast.ident;
  TokenStream::from(quote! {
      impl space::prelude::Rock for #name {}
  })
}

#[proc_macro_derive(Debris)]
pub fn debris_derive(
  input: TokenStream,
) -> TokenStream {
  // Parse the input tokens into a syntax tree
  let ast =
    parse_macro_input!(input as ItemStruct);

  // Build the impl
  let name = &ast.ident;
  TokenStream::from(quote! {
      impl space::prelude::Debris for #name {}
  })
}
