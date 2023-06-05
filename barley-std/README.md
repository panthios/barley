# `barley-std`

This crate contains the standard library for the barley workflow engine. It provides a set of common actions that can be used in any script.

## Usage

```rust
use barley_runtime::prelude::*;
use barley_std::thread::Sleep;
use std::time::Duration;


#[tokio::main]
async fn main() -> Result<()> {
  let mut interface = Interface::new();

  let wait_1s = Sleep::new(Duration::from_secs(1));
  let wait_2s = Sleep::new(Duration::from_secs(2));

  let wait_1s = interface.add_action(wait_1s);
  let mut wait_2s = interface.add_action(wait_2s);

  wait_2s.requires(wait_1s);

  interface.run().await
}
```