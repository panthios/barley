# `barley-interface`

This crate provides a basic command-line interface for the barley workflow engine. It provides runtime progress and error reporting.

This crate does not provide any interactive features, as that would defeat the purpose of Barley entirely. It is simply a wrapper around the base `Context` that provides more information at runtime.

## Usage

```rust
use barley_interface::Interface;
use barley_runtime::*;
use barley_utils::time::{Sleep, Duration};

#[tokio::main]
async fn main() -> Result<()> {
  let mut interface = Interface::new();
  let sleep = Sleep::new(Duration::from_secs(1));

  interface.add_action(sleep).await;

  interface.run().await
}