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