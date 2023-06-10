use barley_runtime::prelude::*;
use barley_std::process::Command;


#[tokio::main]
async fn main() -> Result<(), ActionError> {
    let apt_update: ActionObject = Command::new("apt-get".to_string(), vec![
        "update".to_string().into()
    ]).into();

    let mut apt_install: ActionObject = Command::new("apt-get".to_string(), vec![
        "install".to_string().into(),
        "-y".to_string().into(),
        "curl".to_string().into()
    ]).into();

    apt_install.requires(apt_update.clone());

    RuntimeBuilder::new()
        .add_action(apt_update)
        .add_action(apt_install)
        .build()
        .perform()
        .await
}