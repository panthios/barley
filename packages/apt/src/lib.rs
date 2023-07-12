use barley_runtime::prelude::*;
use async_trait::async_trait;
use tokio::process::Command;


#[derive(Debug, Default)]
pub struct AptUpdate;

impl AptUpdate {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl Action for AptUpdate {
    async fn probe(&self, _runtime: Runtime) -> Result<Probe, Error> {
        Ok(Probe {
            can_rollback: false,
            needs_run: true
        })
    }

    async fn run(&self, _runtime: Runtime, op: Operation) -> Result<Option<Output>, Error> {
        if op == Operation::Rollback {
            return Err(Error::OperationNotSupported);
        }

        let cmd = Command::new("apt-get")
            .arg("update")
            .output()
            .await
            .map_err(|e| Error::ActionFailed(
                "Failed to run `apt-get update`".to_string(),
                e.to_string()
            ))?;
        
        if !cmd.status.success() {
            return Err(Error::ActionFailed(
                "`apt-get update` returned an error".to_string(),
                String::from_utf8_lossy(&cmd.stderr).to_string()
            ))
        }

        Ok(None)
    }

    fn display_name(&self) -> String {
        "apt-get update".to_string()
    }
}

#[derive(Default)]
pub struct AptInstall {
    packages: Vec<Input<String>>
}

impl AptInstall {
    pub fn new<V>(packages: V) -> Self
    where
        V: IntoIterator<Item = Input<String>>
    {
        Self {
            packages: packages.into_iter().collect()
        }
    }

    async fn get_package_names(&self, runtime: Runtime) -> Result<Vec<String>, Error> {
        let mut names = Vec::new();

        for package in self.packages.iter() {
            match package {
                Input::Static(s) => names.push(s.clone()),
                Input::Dynamic(d) => {
                    let output = runtime.get_output(d.clone()).await
                        .ok_or(Error::NoActionReturn)?;

                    if let Output::String(s) = output {
                        names.push(s);
                    } else {
                        return Err(Error::WrongOutputType);
                    }
                }
            }
        }

        Ok(names)
    }
}

#[async_trait]
impl Action for AptInstall {
    async fn probe(&self, runtime: Runtime) -> Result<Probe, Error> {
        let names = self.get_package_names(runtime).await?;

        for name in names {
            let cmd = Command::new("dpkg")
                .arg("-s")
                .arg(&name)
                .output()
                .await
                .map_err(|e| Error::ActionFailed(
                    format!("Failed to run `dpkg -s {}`", name),
                    e.to_string()
                ))?;
            
            if !cmd.status.success() {
                return Ok(Probe {
                    can_rollback: false,
                    needs_run: true
                });
            }

            let output = String::from_utf8_lossy(&cmd.stdout);

            if !output.contains("Status: install ok installed") {
                return Ok(Probe {
                    can_rollback: false,
                    needs_run: true
                });
            }
        }

        Ok(Probe {
            can_rollback: false,
            needs_run: false
        })
    }

    async fn run(&self, runtime: Runtime, op: Operation) -> Result<Option<Output>, Error> {
        if op == Operation::Rollback {
            return Err(Error::OperationNotSupported);
        }

        let names = self.get_package_names(runtime).await?;

        let cmd = Command::new("apt-get")
            .arg("install")
            .arg("-y")
            .args(&names)
            .output()
            .await
            .map_err(|e| Error::ActionFailed(
                format!("Failed to run `apt-get install {}`", names.join(" ")),
                e.to_string()
            ))?;
        
        if !cmd.status.success() {
            return Err(Error::ActionFailed(
                "`apt-get install` returned an error".to_string(),
                String::from_utf8_lossy(&cmd.stderr).to_string()
            ))
        }

        Ok(None)
    }

    fn display_name(&self) -> String {
        "apt-get install <packages>".to_string()
    }
}
