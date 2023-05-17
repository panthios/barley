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
use syn::{self, Fields, FieldsNamed, Ident, ItemImpl, ImplItem, Item};

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
  let trait_ = ast.trait_.clone().unwrap();

  let mut check = None;
  let mut perform = None;
  let mut rollback = None;

  let mut check_index = None;
  let mut perform_index = None;
  let mut rollback_index = None;

  for (index, item) in ast.items.iter().enumerate() {
    if let ImplItem::Fn(fn_) = item {
      if fn_.sig.ident == "check" {
        check = Some(fn_);
        check_index = Some(index);
      } else if fn_.sig.ident == "perform" {
        perform = Some(fn_);
        perform_index = Some(index);
      } else if fn_.sig.ident == "rollback" {
        rollback = Some(fn_);
        rollback_index = Some(index);
      }
    }
  }

  if check.is_none() {
    abort!(trait_.1, "Barley actions must implement the `check` method");
  }

  if perform.is_none() {
    abort!(trait_.1, "Barley actions must implement the `perform` method");
  }

  if rollback.is_none() {
    abort!(trait_.1, "Barley actions must implement the `rollback` method");
  }

  let check = check.unwrap();
  let perform = perform.unwrap();
  let rollback = rollback.unwrap();

  let check_body = check.block.clone();
  let perform_body = perform.block.clone();
  let rollback_body = rollback.block.clone();

  let check = quote! {
    async fn check(&self, ctx: &mut barley_runtime::Context) -> barley_runtime::Result<bool> {
      if !self.check_deps(ctx).await? {
        return Ok(false);
      }

      #check_body
    }
  };

  let perform = quote! {
    async fn perform(&self, ctx: &mut barley_runtime::Context) -> barley_runtime::Result<()> {
      let deps = self.__barley_deps.clone();

      for dep in deps {
        if !dep.check(ctx).await? {
          dep.perform(ctx).await?;
        }
      }

      #perform_body
    }
  };

  let rollback = quote! {
    async fn rollback(&self, ctx: &mut barley_runtime::Context) -> barley_runtime::Result<()> {
      let deps = self.__barley_deps.clone();

      for dep in deps {
        if dep.check(ctx).await? {
          dep.rollback(ctx).await?;
        }
      }

      #rollback_body
    }
  };

  let check_deps = quote! {
    async fn check_deps(&self, ctx: &mut barley_runtime::Context) -> barley_runtime::Result<bool> {
      for dep in self.__barley_deps.clone() {
        if !dep.check(ctx).await? {
          return Ok(false);
        }
      }

      Ok(true)
    }
  };

  let id = quote! {
    fn id(&self) -> barley_runtime::Id {
      self.__barley_id
    }
  };

  let add_dep = quote! {
    fn add_dep(&mut self, action: std::sync::Arc<dyn barley_runtime::Action>) {
      self.__barley_deps.push(action);
    }
  };

  // Sort the indices in reverse order so that we can remove them without
  // affecting the indices of the remaining items.
  let mut indices = vec![check_index, perform_index, rollback_index];
  indices.sort_by(|a, b| b.cmp(a));

  for index in indices {
    ast.items.remove(index.unwrap());
  }

  ast.items.push(syn::parse_quote!(#check));
  ast.items.push(syn::parse_quote!(#perform));
  ast.items.push(syn::parse_quote!(#check_deps));
  ast.items.push(syn::parse_quote!(#id));
  ast.items.push(syn::parse_quote!(#rollback));
  ast.items.push(syn::parse_quote!(#add_dep));

  let output = quote! {
    #ast
  };

  output.into()
}