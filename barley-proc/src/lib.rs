extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::{proc_macro_error, abort};
use quote::quote;
use syn::{self, Fields, FieldsNamed, Ident};


#[proc_macro_error]
#[proc_macro_attribute]
pub fn barley_action(_attr: TokenStream, input: TokenStream) -> TokenStream {
  let mut ast = syn::parse_macro_input!(input as syn::ItemStruct);

  match &mut ast.fields {
    Fields::Named(named) => {
      process_named_fields(named);
    },
    Fields::Unnamed(_) => {
      abort!(ast.ident, "Barley actions cannot have unnamed fields");
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
  }

  let new_field = syn::Field {
    attrs: vec![],
    vis: syn::Visibility::Inherited,
    mutability: syn::FieldMutability::None,
    ident: Some(Ident::new("__barley_deps", Span::call_site())),
    colon_token: None,
    ty: syn::parse_quote!(Vec<std::sync::Arc<dyn barley_runtime::Action>>)
  };

  fields.named.push(new_field);
}