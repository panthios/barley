# `barley-proc`

This crate provides the procedural macros for the barley workflow engine.

All functions from `barley-proc` are re-exported with `barley-runtime`. Since the runtime is essential anyway, this crate should not be imported directly. Use the `barley-runtime` crate instead.

## Usage

```rust
use barley_runtime::*;
use async_trait::async_trait;

#[barley_action]
#[derive(Default)]
struct Print {
  message: String
}

impl Print {
  fn new(message: String) -> Self {
    // `Default` is required to set the internal
    // barley state.
    Self { message, ..Default::default() }
  }
}

#[barley_action]
#[async_trait]
impl Action for Print {
  async fn check(&self, ctx: &mut Context) -> Result<bool> {
    Ok(false)
  }

  async fn perform(&mut self, ctx: &mut Context) -> Result<()> {
    println!("{}", self.message);
    Ok(())
  }

  async fn rollback(&mut self, ctx: &mut Context) -> Result<()> {
    Ok(())
  }
}
```