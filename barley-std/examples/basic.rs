use barley_runtime::prelude::*;
use barley_interface::Interface;
use barley_std::thread::Sleep;
use std::time::Duration;



#[tokio::main]
async fn main() -> Result<()> {
    let interface = Interface::new();

    let sleep_1 = Sleep::new(Duration::from_secs(1));
    let sleep_2 = Sleep::new(Duration::from_secs(2));

    let sleep1 = interface.add_action(sleep_1).await;
    let mut sleep2 = interface.add_action(sleep_2).await;

    sleep2.requires(sleep1);

    interface.update_action(sleep2).await;

    interface.run().await
}