use proc_macro2::Ident;
use syn::{self, ItemImpl, ImplItemFn, FnArg, Stmt};
use proc_macro_error::{emit_error, abort_if_dirty};


pub fn replace_funcs(ast: &mut ItemImpl) {
  let trait_ = ast.trait_.clone().unwrap();

  let mut check = None;
  let mut perform = None;
  let mut rollback = None;

  for item in ast.items.iter_mut() {
    if let syn::ImplItem::Fn(fn_) = item {
      if fn_.sig.ident == "check" {
        check = Some(fn_);
      } else if fn_.sig.ident == "perform" {
        perform = Some(fn_);
      } else if fn_.sig.ident == "rollback" {
        rollback = Some(fn_);
      }
    }
  }

  if check.is_none() {
    emit_error!(trait_.1, "Barley actions must implement the `check` method");
  }

  if perform.is_none() {
    emit_error!(trait_.1, "Barley actions must implement the `perform` method");
  }

  if rollback.is_none() {
    emit_error!(trait_.1, "Barley actions must implement the `rollback` method");
  }

  abort_if_dirty();

  let check = check.unwrap();
  let perform = perform.unwrap();
  let rollback = rollback.unwrap();

  revise_check(check);
  revise_perform(perform);
  revise_rollback(rollback);
}

fn revise_check(func: &mut ImplItemFn) {
  let sig = func.sig.clone();
  let ctx_name = check_ctx(func);

  let mut stmts = func.block.stmts.clone();

  if stmts.is_empty() {
    emit_error!(
      sig.ident, "The `check` method cannot be empty";
      help = "If this action is always run, return `Ok(false)`"
    );
    return;
  }

  let before_custom: Stmt = syn::parse_quote! {
    {
      let __barley_deps = self.__barley_deps.clone();
      for dep in __barley_deps.iter() {
        if !dep.check(#ctx_name).await? {
          return Ok(false);
        }
      }
    }
  };

  stmts.insert(0, before_custom);
  func.block.stmts = stmts;
}

fn revise_perform(func: &mut ImplItemFn) {
  let sig = func.sig.clone();
  let ctx_name = check_ctx(func);

  let mut stmts = func.block.stmts.clone();

  if stmts.is_empty() {
    emit_error!(
      sig.ident, "The `perform` method cannot be empty";
      help = "If this action is not implemented, return `Ok(())`"
    );
    return;
  }

  let before_custom: Stmt = syn::parse_quote! {
    {
      let __barley_deps = self.__barley_deps.clone();
      for dep in __barley_deps.iter() {
        if !dep.check(#ctx_name).await? {
          dep.perform(#ctx_name).await?;
        }
      }
    }
  };

  stmts.insert(0, before_custom);
  func.block.stmts = stmts;
}

fn revise_rollback(func: &ImplItemFn) {
  let sig = func.sig.clone();
  let _ctx_name = check_ctx(func);

  let stmts = func.block.stmts.iter().collect::<Vec<_>>();

  if stmts.is_empty() {
    emit_error!(
      sig.ident, "The `rollback` method cannot be empty";
      help = "If this action is not implemented, return `Ok(())`"
    );
    return;
  }
}

fn check_ctx(func: &ImplItemFn) -> Option<Ident> {
  let sig = func.sig.clone();
  let args = sig.inputs.iter().collect::<Vec<_>>();

  let ctx = args[1];
  let ctx_name = match ctx {
    FnArg::Typed(pat) => {
      match &*pat.pat {
        syn::Pat::Ident(ident) => { ident.ident.clone() },
        _ => {
          emit_error!(
            sig.ident, "Invalid context parameter name";
            help = "Naming the context parameter `ctx` is recommended"
          );
          return None;
        }
      }
    },
    FnArg::Receiver(_) => {
      emit_error!(
        sig.ident, "The context parameter cannot be `self`";
        help = "Naming the context parameter `ctx` is recommended"
      );
      return None;
    }
  };

  Some(ctx_name)
}

pub fn add_funcs(ast: &mut ItemImpl) {
  let check_deps: ImplItemFn = syn::parse_quote! {
    async fn check_deps(&self, ctx: &mut barley_runtime::Context) -> Result<bool> {
      let __barley_deps = self.__barley_deps.clone();

      for dep in __barley_deps.iter() {
        if !dep.check(ctx).await? {
          return Ok(false);
        }
      }

      Ok(true)
    }
  };

  let id: ImplItemFn = syn::parse_quote! {
    fn id(&self) -> barley_runtime::Id {
      self.__barley_id
    }
  };

  let add_dep: ImplItemFn = syn::parse_quote! {
    fn add_dep(&mut self, action: std::sync::Arc<dyn barley_runtime::Action>) {
      self.__barley_deps.push(action);
    }
  };

  ast.items.push(syn::ImplItem::Fn(check_deps));
  ast.items.push(syn::ImplItem::Fn(id));
  ast.items.push(syn::ImplItem::Fn(add_dep));
}