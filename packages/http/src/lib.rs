use barley_runtime::prelude::*;
use async_trait::async_trait;


#[derive(Default)]
pub struct HttpGet {
    url: Input<String>
}

impl HttpGet {
    pub fn new<S>(url: S) -> Self
    where
        S: Into<Input<String>>
    {
        Self {
            url: url.into()
        }
    }
}

#[async_trait]
impl Action for HttpGet {
    async fn probe(&self, _runtime: Runtime) -> Result<Probe, Error> {
        Ok(Probe {
            can_rollback: false,
            needs_run: true
        })
    }

    async fn run(&self, runtime: Runtime, op: Operation) -> Result<Option<Output>, Error> {
        if op == Operation::Rollback {
            return Err(Error::OperationNotSupported);
        }

        let url = match self.url {
            Input::Static(ref s) => s.clone(),
            Input::Dynamic(ref d) => {
                let out = runtime.get_output(d.clone()).await
                    .ok_or(Error::NoActionReturn)?;

                if let Output::String(s) = out {
                    s
                } else {
                    return Err(Error::WrongOutputType);
                }
            }
        };

        let resp = ureq::get(&url)
            .call()
            .map_err(|e| Error::ActionFailed(
                format!("Failed to GET {}", url),
                e.to_string()
            ))?;

        let body = resp.into_string()
            .map_err(|e| Error::ActionFailed(
                format!("Failed to read response body from {}", url),
                e.to_string()
            ))?;
        
        Ok(Some(Output::String(body)))
    }

    fn display_name(&self) -> String {
        "GET <url>".to_string()
    }
}
