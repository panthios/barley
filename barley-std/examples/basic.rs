use barley_runtime::prelude::*;
use barley_std::thread::Sleep;
use std::time::Duration;



#[tokio::main]
async fn main() -> Result<()> {
    let mut runtime = Runtime::new();

    let sleep_1 = Sleep::new(Duration::from_secs(1));
    let sleep_2 = Sleep::new(Duration::from_secs(2));

    let sleep1 = runtime.add_action(sleep_1).await;
    let mut sleep2 = runtime.add_action(sleep_2).await;

    sleep2.requires(sleep1);
    
    runtime.update_action(sleep2).await;

    runtime.run().await
}