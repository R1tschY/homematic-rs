use crate::Format;
use clap::Args;
use comfy_table::Table;
use homematic_rs::{DeviceDescription, HomeMaticClient};
use log::warn;
use std::collections::HashMap;
use std::error::Error;

#[derive(Args)]
pub struct ListDevicesCommand {
    /// Show device channels
    #[arg(long)]
    channels: bool,
}

impl ListDevicesCommand {
    pub async fn exec(
        &self,
        client: &HomeMaticClient,
        format: Format,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let mut buf = String::new();
        let devices = client.list_devices(&mut buf).await?;

        let mut devs = HashMap::<&str, &DeviceDescription>::new();
        for dev in &devices {
            devs.insert(&dev.address, &dev);
        }

        match format {
            Format::Table => {
                let mut table = Table::new();
                table
                    .set_header(vec!["TYPE", "ADDRESS", "PARAMSETS"])
                    .load_preset(comfy_table::presets::NOTHING);

                for dev in &devices {
                    if dev.parent.is_empty() {
                        table.add_row(vec![&dev.ty, &dev.address, &dev.paramsets.join(", ")]);
                        if self.channels {
                            for channel in &dev.children {
                                if let Some(channel) = devs.get(&channel as &str) {
                                    table.add_row(vec![
                                        &format!("    {}", channel.ty),
                                        &channel.address,
                                        &channel.paramsets.join(", "),
                                    ]);
                                    // TODO: what about sub-channels?
                                } else if !channel.is_empty() {
                                    warn!("Unknown channel address: {}", channel);
                                }
                            }
                        }
                    }
                }

                println!("{table}");
            }
            Format::Json => {
                todo!()
            }
        }

        Ok(())
    }
}
