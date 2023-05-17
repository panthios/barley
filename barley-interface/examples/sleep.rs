use barley_interface::*;
use barley_runtime::*;
use barley_utils::time::{Duration, Sleep};


#[tokio::main]
async fn main() -> Result<()> {
    let mut interface = Interface::new();

    interface.add_action(Sleep::new(Duration::from_secs(1)));
    interface.add_action(Sleep::new(Duration::from_secs(2)));
    interface.add_action(Sleep::new(Duration::from_secs(3)));

    interface.run().await
}