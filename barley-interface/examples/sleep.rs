use barley_interface::*;
use barley_runtime::prelude::*;
use barley_utils::time::{Duration, Sleep};


#[tokio::main]
async fn main() -> Result<()> {
    let interface = Interface::new();

    interface.add_action(Sleep::new(Duration::from_secs(1))).await;
    interface.add_action(Sleep::new(Duration::from_secs(2))).await;
    interface.add_action(Sleep::new(Duration::from_secs(3))).await;

    interface.run().await
}