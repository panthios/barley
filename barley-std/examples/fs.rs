use barley_runtime::prelude::*;
use barley_std::fs::{WriteFile, ReadFile, DeleteFile};


#[tokio::main]
async fn main() -> Result<(), ActionError> {
    let write: ActionObject = WriteFile::new_static("foo.txt", "Hello, world!").into();
    let mut read: ActionObject = ReadFile::new("foo.txt").into();
    let mut delete: ActionObject = DeleteFile::new("foo.txt").into();

    read.requires(write.clone());
    delete.requires(read.clone());

    RuntimeBuilder::new()
        .add_action(write)
        .add_action(read)
        .add_action(delete)
        .build()
        .run()
        .await
}