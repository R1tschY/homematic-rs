use crate::message::OutputServiceMessage;
use crate::Format;
use clap::Args;
use homematic_rs::{ChannelDirection, DeviceDescription, HomeMaticClient, RxMode};
use serde::Serialize;
use std::error::Error;

#[derive(Args)]
pub struct ListServiceMessagesCommand {}

impl ListServiceMessagesCommand {
    pub async fn exec(
        &self,
        client: &HomeMaticClient,
        format: Format,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let mut buf = String::new();
        let messages: Vec<OutputServiceMessage> = client
            .get_service_messages(&mut buf)
            .await?
            .into_iter()
            .map(|sm| sm.into())
            .collect();

        serde_json::to_writer_pretty(std::io::stdout(), &messages)?;

        Ok(())
    }
}
