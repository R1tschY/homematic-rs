use crate::param::OutputParameterDescription;
use crate::Format;
use clap::Args;
use homematic_rs::{HomeMaticClient, Paramset};
use std::collections::HashMap;
use std::error::Error;

#[derive(Args)]
pub struct GetParamCommand {
    /// device or channel address
    address: String,
    /// paramset type
    paramset_type: String,
}

impl GetParamCommand {
    pub async fn exec(
        &self,
        client: &HomeMaticClient,
        format: Format,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let mut buf = String::new();
        let paramset: Paramset = client
            .get_paramset(&self.address, &self.paramset_type, &mut buf)
            .await?;

        serde_json::to_writer_pretty(std::io::stdout(), &paramset)?;

        Ok(())
    }
}
