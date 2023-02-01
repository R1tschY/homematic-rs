use crate::device::inspect::InspectDeviceCommand;
use crate::device::list::ListDevicesCommand;
use clap::Subcommand;

pub mod inspect;
pub mod list;

#[derive(Subcommand)]
pub enum DeviceCommand {
    /// Lists all devices and channels
    List(ListDevicesCommand),
    /// Describe device or channel
    Inspect(InspectDeviceCommand),
}
