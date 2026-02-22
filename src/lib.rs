use std::str::FromStr;

use convert_case::Casing;
use paste::paste;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use syn::spanned::Spanned;

struct RecordParams(syn::Type, syn::Ident, Option<syn::LitStr>);
impl syn::parse::Parse for RecordParams {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let value_type = input.parse()?;
    let _ = input.parse::<syn::Token![,]>();
    let ident = input.parse()?;
    let _ = input.parse::<syn::Token![,]>();
    let header = input.parse().ok();
    Ok(RecordParams(value_type, ident, header))
  }
}

const GENERIC_ERROR_MSG: &str = "Expected to be used with an enum";

#[proc_macro_attribute]
pub fn record(
  params: proc_macro::TokenStream,
  ast: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let RecordParams(value_type, ident, header) = syn::parse2(params.into()).expect("Invalid params");

  let ast_tokens: TokenStream = ast.clone().into();

  let mut ast_iter = ast_tokens.clone().into_iter();

  let enum_props_group = ast_iter
    .find_map(|next| {
      if let TokenTree::Group(group) = next {
        Some(group)
      } else {
        None
      }
    })
    .expect(GENERIC_ERROR_MSG);

  let struct_props: Vec<TokenStream> = enum_props_group
    .stream()
    .into_iter()
    .filter_map(|token| {
      if let TokenTree::Ident(ident) = token {
        let ident_str = ident.to_string();
        let snake = ident_str.to_case(convert_case::Case::Snake);
        let snake_ident = syn::Ident::new(&snake, ident_str.span());

        Some(quote! {
          pub #snake_ident: #value_type ,
        })
      } else {
        None
      }
    })
    .collect();

  let header_token = header
    .map(|token| TokenStream::from_str(&token.value()).expect(GENERIC_ERROR_MSG))
    .unwrap_or(TokenStream::new());

  let output = quote! {
    #ast_tokens
    #header_token
    struct #ident {
      #(#struct_props)*
    }
  };

  output.into()
}
