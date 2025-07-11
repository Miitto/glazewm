use quote::{ToTokens, quote};
use syn::spanned::Spanned;

use crate::prelude::*;

pub fn try_to_enum_discriminant(
  input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  let input = syn::parse_macro_input!(input as syn::DeriveInput);

  let name = &input.ident;

  let enum_data = match input.data {
    syn::Data::Enum(data) => data,
    _ => panic!("`TryToDiscriminant` can only be derived for enums"),
  };

  let variants = match enum_data
    .variants
    .iter()
    .map(syn_varaint_to_variant)
    .collect::<syn::Result<Vec<Variant>>>()
  {
    Ok(variants) => variants,
    Err(err) => return err.to_compile_error().into(),
  };

  let repr = input
    .attrs
    .find_list_attrs("repr")
    .next()
    .map(|attr| {
      attr
        .parse_args::<syn::Type>()
        .map(|t| t.to_token_stream())
        .expect("Failed to parse repr attribute")
    })
    .unwrap_or_else(|| quote! {isize});

  let normal = variants
    .iter()
    .filter(|v| v.discriminant.is_some())
    .collect::<Vec<_>>();

  let other: Vec<_> = variants
    .iter()
    .filter(|v| v.discriminant.is_none())
    .collect();
  if other.len() > 1 {
    let errors = other.iter().map(|v| {
      ToError::error(
        &v.name,
        "Only one variant can be marked as `#[other]`",
      )
      .to_compile_error()
    });

    return quote! {#(#errors)*}.into();
  }

  let variants = normal.iter().map(|variant| {
    let var = &variant.name;
    let discriminant = &variant.discriminant;
    quote! { #discriminant => Ok(#name::#var) }
  });

  if let Some(other) = other.first() {
    let var = &other.name;
    let variants = normal.iter().map(|variant| {
      let var = &variant.name;
      let discriminant = &variant.discriminant;
      quote! { #discriminant => #name::#var }
    });
    quote! {
      impl From<#repr> for #name {
        fn from(value: #repr) -> Self {
          match value {
            #(#variants),*,
            _ => #name::#var(value)
          }
        }
      }
    }
    .into()
  } else {
    quote! {
      impl TryFrom<#repr> for #name {
        type Error = &'static str;

        fn try_from(value: #repr) -> Result<Self, Self::Error> {
          match value {
            #(#variants),*,
            _ => Err("Failed to convert value to enum discriminant")
          }
        }
      }
    }
    .into()
  }
}

pub enum Discriminant {
  Literal(syn::LitInt),
  Path(syn::Path),
}

impl syn::parse::Parse for Discriminant {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let lookahead = input.lookahead1();

    if lookahead.peek(syn::LitInt) {
      let lit_int: syn::LitInt = input.parse()?;
      Ok(Discriminant::Literal(lit_int))
    } else if lookahead.peek(syn::Ident) {
      let path: syn::Path = input.parse()?;
      Ok(Discriminant::Path(path))
    } else {
      Err(lookahead.error())
    }
  }
}

impl quote::ToTokens for Discriminant {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      Discriminant::Literal(lit_int) => lit_int.to_tokens(tokens),
      Discriminant::Path(path) => path.to_tokens(tokens),
    }
  }
}

struct Variant {
  pub name: syn::Ident,
  pub discriminant: Option<Discriminant>,
}

fn syn_varaint_to_variant(variant: &syn::Variant) -> syn::Result<Variant> {
  let is_other = variant
    .attrs
    .iter()
    .any(|attr| attr.path().is_ident("other"));

  let name = variant.ident.clone();

  if is_other {
    if variant.fields.len() != 1 {
      return Err(ToError::error(
        &variant,
        "The `#[other]` variant must have exactly one field",
      ));
    }

    return Ok(Variant {
      name,
      discriminant: None,
    });
  }

  let discriminant = match &variant.discriminant {
    Some((_, expr)) => match expr {
      syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Int(lit_int),
        ..
      }) => Some(Discriminant::Literal(lit_int.clone())),
      syn::Expr::Path(syn::ExprPath { path, .. }) => {
        Some(Discriminant::Path(path.clone()))
      }
      _ => {
        return Err(ToError::error(
          &expr,
          format!(
            "Expected a literal integer or constant as a discriminant, found, {}",
            expr.to_token_stream()
          ),
        ));
      }
    },
    None => {
      return Err(ToError::error(
        &variant,
        "Enum variant must have a discriminant",
      ));
    }
  };

  Ok(Variant { name, discriminant })
}
