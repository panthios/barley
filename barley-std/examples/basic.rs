use barley_runtime::prelude::*;
use barley_std::time::Sleep;
use std::time::Duration;



#[tokio::main]
async fn main() {
    let secs_1: ActionObject = Sleep::new(Duration::from_secs(1)).into();
    let mut secs_2: ActionObject = Sleep::new(Duration::from_secs(2)).into();

    secs_2.requires(secs_1.clone());

    RuntimeBuilder::new()
        .add_action(secs_1)
        .add_action(secs_2)
        .build()
        .run()
        .await
        .unwrap();
}