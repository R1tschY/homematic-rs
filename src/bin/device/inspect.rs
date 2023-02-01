use crate::Format;
use clap::Args;
use homematic_rs::{ChannelDirection, DeviceDescription, HomeMaticClient, RxMode};
use serde::Serialize;
use std::error::Error;

#[derive(Args)]
pub struct InspectDeviceCommand {
    /// Device or channel address
    device: String,
}

/// Specifies the direction (transmit or receive) of this channel in a direct link.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OutputChannelDirection {
    None,
    Sender,
    Receiver,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputRxMode {
    pub always: bool,
    pub burst: bool,
    pub config: bool,
    pub wakeup: bool,
    pub lazy_config: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputDeviceDescription {
    /// Type of device
    #[serde(rename = "type")]
    pub ty: String,

    /// Address of channel or device
    pub address: String,

    /// Radio address of device
    ///
    /// Only for devices.
    pub rf_address: Option<i32>,

    /// Addresses of the child channels.
    pub children: Vec<String>,

    /// Address of parent device
    ///
    /// Empty on devices.
    pub parent: Option<String>,

    /// Type (short name) of the parent device.
    ///
    /// Only for devices.
    pub parent_type: Option<String>,

    /// Specifies the channel number.
    ///
    /// Only for devices.
    pub index: Option<i32>,

    /// Indicates whether secured transmission is enabled for the channel.
    pub aes: bool,

    /// List of the names of the existing parameter sets.
    pub param_sets: Vec<String>,

    /// Firmware version of the device.
    ///
    /// Only for devices. Optional.
    pub firmware: Option<String>,

    /// Firmware version available for a firmware update.
    ///
    /// Only for devices. Optional.
    pub available_firmware: Option<String>,

    /// Indicates whether the firmware of a device is updateable.
    ///
    /// Only for devices. Optional.
    pub updatable: bool,

    /// Version of the device or channel description.
    pub version: Option<i32>,

    /// This object should be visible to the end user.
    pub visible: bool,

    /// This object is only used internally and is not visible to the end user.
    pub internal: bool,

    /// This object is deletable.
    pub deletable: bool,

    /// List of roles that the channel can acquire as a sender in a link.
    ///
    /// A role is e.g. "SWITCH" for a channel that can send switching commands.
    ///
    /// Only for channels.
    pub link_source_roles: Option<Vec<String>>,

    /// List of roles that the channel can acquire as a receiver in a link.
    ///
    /// A role is e.g. "SWITCH" for a channel that can receive switching commands.
    ///
    /// Only for channels.
    pub link_target_roles: Option<Vec<String>>,

    /// Specifies the direction (transmit or receive) of this channel in a direct link.
    ///
    /// Only for channels.
    pub direction: Option<OutputChannelDirection>,

    /// The address of the other channel belonging to the group is specified here.
    ///
    /// Only available for grouped channels (key pairs).
    /// Optional.
    pub group: Option<String>,

    /// Specifies the address of the virtual team channel.
    ///
    /// Only available for channels with a team (e.g. smoke detectors).
    /// Optional.
    pub team: Option<String>,

    /// Selection of a team on the UI.
    ///
    /// A channel to be assigned to a team and the virtual team channel (the team) must have the
    /// same value here.
    ///
    /// Only available for channels with a team (e.g. smoke detectors) and for virtual team channels.
    /// Optional.
    pub team_tag: Option<String>,

    /// Addresses of the channels assigned to the team.
    ///
    /// Only for channels that represent a team.
    /// Optional.
    pub team_channels: Option<Vec<String>>,

    /// Serial number of the interface assigned to the device.
    ///
    /// Only on BidCos-RF.
    /// Optional.
    pub interface: Option<String>,

    /// Is true if the interface assignment of the device is automatically adapted to the reception
    /// conditions.
    ///
    /// Only on BidCos-RF.
    /// Optional.
    pub roaming: Option<bool>,

    /// Or-connection of flags representing the receive mode of the device.
    ///
    /// Only on BidCos-RF.
    /// Only for devices.
    /// Optional.
    pub rx_mode: Option<OutputRxMode>,
}

impl From<DeviceDescription> for OutputDeviceDescription {
    fn from(value: DeviceDescription) -> Self {
        Self {
            visible: value.is_visible(),
            internal: value.is_internal(),
            deletable: value.is_deletable(),
            ty: value.ty,
            address: value.address,
            rf_address: value.rf_address,
            children: value.children,
            parent: if value.parent.is_empty() {
                None
            } else {
                Some(value.parent)
            },
            parent_type: value.parent_type,
            index: value.index,
            aes: value.aes_active.unwrap_or_default(),
            param_sets: value.paramsets,
            firmware: value.firmware,
            available_firmware: value.available_firmware,
            updatable: value.updatable.unwrap_or_default(),
            version: value.version,
            link_source_roles: value.link_source_roles,
            link_target_roles: value.link_target_roles,
            direction: match value.direction {
                None => None,
                Some(ChannelDirection::None) => Some(OutputChannelDirection::None),
                Some(ChannelDirection::Sender) => Some(OutputChannelDirection::Sender),
                Some(ChannelDirection::Receiver) => Some(OutputChannelDirection::Receiver),
            },
            group: value.group,
            team: value.team,
            team_tag: value.team_tag,
            team_channels: value.team_channels,
            interface: value.interface,
            roaming: value.roaming,
            rx_mode: value.rx_mode.map(|mode| OutputRxMode {
                always: mode.always(),
                burst: mode.burst(),
                config: mode.config(),
                wakeup: mode.wakeup(),
                lazy_config: mode.lazy_config(),
            }),
        }
    }
}

impl InspectDeviceCommand {
    pub async fn exec(
        &self,
        client: &HomeMaticClient,
        format: Format,
    ) -> Result<(), Box<dyn Error + 'static>> {
        let mut buf = String::new();
        let device: OutputDeviceDescription = client
            .get_device_description(&self.device, &mut buf)
            .await?
            .into();

        serde_json::to_writer_pretty(std::io::stdout(), &device)?;

        Ok(())
    }
}
