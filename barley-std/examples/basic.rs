use barley_runtime::prelude::*;
use barley_std::thread::Sleep;
use std::time::Duration;



#[tokio::main]
async fn main() {
    RuntimeBuilder::new()
        .add_action(Sleep::new(Duration::from_secs(1)).into())
        .add_action(Sleep::new(Duration::from_secs(2)).into())
        .add_action(Sleep::new(Duration::from_secs(3)).into())
        .build()
        .run()
        .await
        .unwrap();
}