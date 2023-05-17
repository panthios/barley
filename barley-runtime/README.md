# `barley-runtime`

This crate provides the runtime for the barley workflow engine. It is responsible for coordinating the execution of the workflow and managing its state.

`barley-runtime` is not stable. Many features and APIs are still in development. The crate is available for testing, but should not be used in production.

## Usage

```rust
use barley_runtime::*;
use barley_utils::time::{Sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
  let mut ctx = Context::new();
  let sleep = Sleep::new(Duration::from_secs(1));

  ctx.add_action(sleep);

  ctx.run().await
}
```