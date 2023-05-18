#![deny(missing_docs)]

//! `barley-proc`
//! 
//! This crate should not be used directly. It is used by the `barley` workflow
//! engine to easily create new [`Action`]s.
//! 
//! [`Action`]: https://docs.rs/barley-runtime/latest/barley_runtime/trait.Action.html

extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{proc_macro_error, abort};
use quote::quote;
use syn::{self, Fields, FieldsNamed, Ident, ItemImpl, Item};

mod utils;
mod assert;

/// Apply the required features to an [`Action`] struct.
/// 
/// This method should be applied to BOTH the struct definition and the
/// [`Action`] trait implementation. It will add the required fields and
/// methods to the struct implementation.
/// 
/// [`Action`]: https://docs.rs/barley-runtime/latest/barley_runtime/trait.Action.html
#[proc_macro_error]
#[proc_macro_attribute]
pub fn barley_action(_attr: TokenStream, input: TokenStream) -> TokenStream {
  let ast = syn::parse_macro_input!(input as Item);

  match ast {
    Item::Struct(struct_) => {
      barley_action_struct(struct_)
    },
    Item::Impl(impl_) => {
      barley_action_impl(impl_)
    },
    _ => {
      abort!(ast, "Barley actions must be structs or Action impls");
    }
  }
}

fn barley_action_struct(mut ast: syn::ItemStruct) -> TokenStream {
  match &mut ast.fields {
    Fields::Named(named) => {
      process_named_fields(named);
    },
    Fields::Unnamed(_) => {
      abort!(ast.ident, "Barley actions cannot have unnamed fields at this time");
    },
    Fields::Unit => {
      abort!(ast.ident, "Barley actions must have at least one field");
    }
  }

  let output = quote! {
    #ast
  };

  output.into()
}

fn process_named_fields(fields: &mut FieldsNamed) {
  for field in fields.named.clone() {
    let ident = field.ident.unwrap();

    if ident == "__barley_deps" {
      abort!(ident, "Barley actions cannot have a field named `__barley_deps`");
    }

    if ident == "__barley_id" {
      abort!(ident, "Barley actions cannot have a field named `__barley_id`");
    }
  }

  let deps = syn::Field {
    attrs: vec![],
    vis: syn::Visibility::Inherited,
    mutability: syn::FieldMutability::None,
    ident: Some(Ident::new("__barley_deps", Span::call_site())),
    colon_token: None,
    ty: syn::parse_quote!(Vec<std::sync::Arc<dyn barley_runtime::Action>>)
  };

  let id = syn::Field {
    attrs: vec![],
    vis: syn::Visibility::Inherited,
    mutability: syn::FieldMutability::None,
    ident: Some(Ident::new("__barley_id", Span::call_site())),
    colon_token: None,
    ty: syn::parse_quote!(barley_runtime::Id)
  };

  fields.named.push(deps);
  fields.named.push(id);
}

fn barley_action_impl(mut ast: ItemImpl) -> TokenStream {
  utils::replace_funcs(&mut ast);
  utils::add_funcs(&mut ast);

  let output = quote! {
    #ast
  };

  output.into()
}