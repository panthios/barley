# `barley-std`

This crate contains the standard library for the barley workflow engine. It provides a set of common actions that can be used in any script.

## Usage

```rust
use barley_runtime::prelude::*;
use barley_std::thread::Sleep;
use std::time::Duration;


#[tokio::main]
async fn main() -> Result<()> {
  let sleep_1 = Sleep::new(Duration::from_secs(1));
  let sleep_2 = Sleep::new(Duration::from_secs(2));

  RuntimeBuilder::new()
    .add_action(sleep_1)
    .add_action(sleep_2)
    .build()
    .run()
    .await
}
```