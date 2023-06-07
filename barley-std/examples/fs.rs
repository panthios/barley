use barley_runtime::prelude::*;
use barley_std::fs::{WriteFile, ReadFile};


#[tokio::main]
async fn main() -> Result<()> {
    let write: ActionObject = WriteFile::new_static("foo.txt", "Hello, world!").into();
    let mut read: ActionObject = ReadFile::new("foo.txt").into();

    read.requires(write.clone());

    RuntimeBuilder::new()
        .add_action(write)
        .add_action(read)
        .build()
        .run()
        .await
}