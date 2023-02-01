use crate::param::OutputParameterDescription;
use crate::Format;
use clap::Args;
use homematic_rs::HomeMaticClient;
use std::collections::HashMap;
use std::error::Error;

#[derive(Args)]
pub struct ListParamsCommand {
    /// device or channel address
    address: String,
    /// paramset type
    paramset_type: String,
}

impl ListParamsCommand {
    pub async fn exec(
        &self,
        client: &HomeMaticClient,
        format: Format,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let mut buf = String::new();
        let paramset: HashMap<String, OutputParameterDescription> = client
            .get_paramset_description(&self.address, &self.paramset_type, &mut buf)
            .await?
            .into_iter()
            .map(|(key, sm)| (key, sm.into()))
            .collect();

        serde_json::to_writer_pretty(std::io::stdout(), &paramset)?;

        Ok(())
    }
}
