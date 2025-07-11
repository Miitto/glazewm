use proc_macro2::TokenStream;
use quote::quote;

use super::{
  Key,
  attrs::{enums::EnumAttr, variant::VkValue},
};
use crate::Os;

/// Creates the match arms for the `into_vk` impl for the given key and Os.
fn to_match_arm(key: &Key, enum_attrs: &EnumAttr, os: Os) -> TokenStream {
  let ident = &key.ident;
  match &key.attrs {
    super::VariantAttr::Wildcard(other_variants) => {
      // If the key is a wildcard, we match it to the `Custom` variant.
      let (vk_val, prefix) = match os {
        Os::Windows => {
          (&other_variants.key_codes.win, &enum_attrs.win_enum)
        }
        Os::MacOS => {
          (&other_variants.key_codes.macos, &enum_attrs.macos_enum)
        }
        Os::Linux => {
          (&other_variants.key_codes.linux, &enum_attrs.linux_enum)
        }
      };
      let var = match vk_val {
        VkValue::Key(vk) => quote! {#prefix::#vk},
        _ => return quote! {},
      };
      quote! { Self::Custom(vk) => #var(vk) }
    }
    super::VariantAttr::Key(key_attrs) => {
      let (value, prefix) = match os {
        Os::Windows => (&key_attrs.key_codes.win, &enum_attrs.win_enum),
        Os::MacOS => (&key_attrs.key_codes.macos, &enum_attrs.macos_enum),
        Os::Linux => (&key_attrs.key_codes.linux, &enum_attrs.linux_enum),
      };

      // Output the match arms.
      match value {
        VkValue::Key(value) => {
          quote! { Self::#ident => #prefix::#value}
        }
        VkValue::Virt(value) => {
          quote! { Self::#ident => #prefix::#value}
        }
        _ => quote! {},
      }
    }
  }
}

/// Creates a `into_vk` implementation for the `Key` enum using the given
/// keys.
pub fn make_into_vk_impl(
  keys: &[Key],
  enum_attrs: &EnumAttr,
) -> TokenStream {
  let win_enum = &enum_attrs.win_enum;
  let mac_enum = &enum_attrs.macos_enum;
  let linux_enum = &enum_attrs.linux_enum;

  let win_arms = keys
    .iter()
    .map(|key| to_match_arm(key, enum_attrs, Os::Windows))
    .filter(|arm| !arm.is_empty());

  let mac_arms = keys
    .iter()
    .map(|key| to_match_arm(key, enum_attrs, Os::MacOS))
    .filter(|arm| !arm.is_empty());

  let linux_arms = keys
    .iter()
    .map(|key| to_match_arm(key, enum_attrs, Os::Linux))
    .filter(|arm| !arm.is_empty());

  quote! {
    #[cfg(target_os = "windows")]
    pub fn into_vk(self) -> #win_enum {
      // The comma is inside the brackes so that a trailing comma is generated for the last arm.
      match self {
        #(#win_arms,)*
        _ => { unreachable!("Key not found in Windows VK mapping"); }
      }
    }

    #[cfg(target_os = "macos")]
    pub fn into_vk(self) -> #mac_enum {
      match self {
        #(#mac_arms,)*
        _ => { unreachable!("Key not found in macOS VK mapping"); }
      }
    }

    #[cfg(target_os = "linux")]
    pub fn into_vk(self) -> #linux_enum {
      match self {
        #(#linux_arms,)*
        _ => { unreachable!("Key not found in linux VK mapping"); }
      }
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    pub fn into_vk(self) {
      compile_error!("`into_vk` is not supported on this OS at this time.");
    }
  }
}
