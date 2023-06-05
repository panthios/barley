# Barley

Barley is still in early development. It is not yet ready for use. This README describes the intended functionality of Barley, but does not reflect the current state of the project.

Barley is a simple and lightweight scripting framework. Using Rust's safety guarantees and powerful type system, Barley provides the relational power of Makefiles with the compile-time speed of native languages.

## Features

- **Simple**: Barley is designed with safety and simplicity in mind. It is easy to learn, and provides an intuitive interface for writing scripts at scale.
- **Fast**: All Barley scripts are compiled to native machine code. This makes Barley scripts extremely fast, and allows them to be used in performance-critical applications. This can make scripts harder to distribute at scale, but the relatively small compile times ease this burden.
- **Concurrent**: Barley will run actions in parallel whenever possible.
- **Extensible**: Barley uses dynamic traits under the hood. This allows commands to easily depend on one another. Procedural macros are also provided to allow for easy creation of new commands.

## Examples

### Writing a script

```rust
use barley_interface::Interface;
use barley_runtime::*;
use barley_std::thread::Sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
  let interface = Interface::new();

  let wait_1s = Sleep::new(Duration::from_secs(1));
  let wait_2s = Sleep::new(Duration::from_secs(2));

  let wait_1s = interface.add_action(wait_1s).await;
  let mut wait_2s = interface.add_action(wait_2s).await;

  wait_2s.requires(wait_1s);

  interface.update_action(wait_2s).await;

  interface.run().await
}
```

### Writing a command

```rust
use barley_runtime::prelude::*;
use async_trait::async_trait;

pub struct Print {
  message: String
}

#[async_trait]
impl Action for Print {
  async fn check(&self, ctx: Arc<RwLock<Context>>) -> Result<bool> {
    Ok(false)
  }

  async fn perform(&self, ctx: Arc<RwLock<Context>>) -> Result<Option<ActionOutput>> {
    println!("{}", self.message);
    Ok(None)
  }

  async fn rollback(&self, ctx: Arc<RwLock<Context>>) -> Result<()> {
    Ok(())
  }
}
```