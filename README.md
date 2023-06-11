# Barley

Barley is a simple and lightweight scripting framework. Using Rust's safety guarantees and powerful type system, Barley provides the relational power of Makefiles with the compile-time speed of native languages.

## Features

- **Simple**: Barley is designed with safety and simplicity in mind. It is easy to learn, and provides an intuitive interface for writing scripts at scale.
- **Fast**: All Barley scripts are compiled to native machine code. This makes Barley scripts extremely fast, and allows them to be used in performance-critical applications. This can make scripts harder to distribute at scale, but the relatively small compile times ease this burden.
- **Concurrent**: Barley will run actions in parallel whenever possible.
- **Extensible**: Barley uses dynamic traits under the hood. This allows commands to easily depend on one another.

## Examples

### Writing a script

```rust
use barley_runtime::*;
use barley_std::thread::Sleep;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), ActionError> {
  let wait_1s: ActionObject = Sleep::new(Duration::from_secs(1)).into();
  let mut wait_2s: ActionObject = Sleep::new(Duration::from_secs(2)).into();

  wait_2s.requires(wait_1s.clone());

  RuntimeBuilder::new()
    .add_action(wait_1s)
    .add_action(wait_2s)
    .build()
    .run()
    .await
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
  async fn probe(&self, _runtime: Runtime) -> Result<Probe, ActionError> {
    Ok(Probe {
      needs_run: true,
      can_rollback: false
    })
  }

  async fn run(&self, _runtime: Runtime, op: Operation) -> Result<Option<ActionOutput>, ActionError> {
    println!("{}", self.message);
    Ok(None)
  }

  fn display_name(&self) -> String {
    "".to_string()
  }
}
```