use bitflags::bitflags;
use serde::{Deserialize, Deserializer};
use xrs_xmlrpc::value;

bitflags! {
    /// Or-connection of flags for UI display.
    struct DeviceFlags: i32 {
        /// This object should be visible to the end user.
        const Visible = 0x1;
        /// This object is only used internally and is not visible to the end user.
        const Internal = 0x2;
        /// This object cannot be deleted.
        const DontDelete = 0x8;
    }
}

fn deserialize_device_flags<'de, D>(deserializer: D) -> Result<DeviceFlags, D::Error>
where
    D: Deserializer<'de>,
{
    let value: i32 = Deserialize::deserialize(deserializer)?;
    Ok(DeviceFlags::from_bits_truncate(value))
}

/// Specifies the direction (transmit or receive) of this channel in a direct link.
#[derive(Deserialize, Debug)]
pub enum ChannelDirection {
    /// Channel does not support direct linking
    None = 0,
    Sender = 1,
    Receiver = 2,
}

bitflags! {
    /// Or-connection of flags representing the receive mode of the device.
    struct InternalRxMode: i32 {
        /// The device is permanently on receive.
        const ALWAYS = 0x1;
        /// The device operates in wake on radio mode.
        const BURST = 0x2;
        /// The device can be reached after pressing the configuration key.
        const CONFIG = 0x4;
        /// The device can be woken up after a direct communication with the control panel.
        const WAKEUP = 0x8;
        /// The device supports lazy configuration.
        ///
        /// The device can be configured after a normal operation
        /// (e.g. keystroke of a remote control).
        const LAZY_CONFIG = 0x10;
    }
}

fn deserialize_rx_mode<'de, D>(deserializer: D) -> Result<Option<RxMode>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i32> = Deserialize::deserialize(deserializer)?;
    Ok(value.map(InternalRxMode::from_bits_truncate).map(RxMode))
}

#[derive(Debug)]
pub struct RxMode(InternalRxMode);

impl RxMode {
    pub fn always(&self) -> bool {
        self.0.contains(InternalRxMode::ALWAYS)
    }

    pub fn burst(&self) -> bool {
        self.0.contains(InternalRxMode::BURST)
    }

    pub fn config(&self) -> bool {
        self.0.contains(InternalRxMode::CONFIG)
    }

    pub fn wakeup(&self) -> bool {
        self.0.contains(InternalRxMode::WAKEUP)
    }

    pub fn lazy_config(&self) -> bool {
        self.0.contains(InternalRxMode::LAZY_CONFIG)
    }
}

#[derive(Deserialize, Debug)]
pub struct DeviceDescription {
    /// Type of device
    #[serde(rename = "TYPE")]
    pub ty: String,

    /// Address of channel or device
    #[serde(rename = "ADDRESS")]
    pub address: String,

    /// Radio address of device
    ///
    /// Only for devices.
    #[serde(rename = "RF_ADDRESS")]
    pub rf_address: Option<i32>,

    /// Addresses of the child channels.
    #[serde(rename = "CHILDREN", default)]
    pub children: Vec<String>,

    /// Address of parent device
    ///
    /// Empty on devices.
    #[serde(rename = "PARENT", default)]
    pub parent: String,

    /// Type (short name) of the parent device.
    ///
    /// Only for devices.
    #[serde(rename = "PARENT_TYPE")]
    pub parent_type: Option<String>,

    /// Specifies the channel number.
    ///
    /// Only for devices.
    #[serde(rename = "INDEX")]
    pub index: Option<i32>,

    /// Indicates whether secured transmission is enabled for the channel.
    #[serde(
        rename = "AES_ACTIVE",
        deserialize_with = "crate::de::deserialize_bool_option"
    )]
    pub aes_active: Option<bool>,

    /// List of the names of the existing parameter sets.
    #[serde(rename = "PARAMSETS")]
    pub paramsets: Vec<String>,

    /// Firmware version of the device.
    ///
    /// Only for devices. Optional.
    #[serde(rename = "FIRMWARE")]
    pub firmware: Option<String>,

    /// Firmware version available for a firmware update.
    ///
    /// Only for devices. Optional.
    #[serde(rename = "AVAILABLE_FIRMWARE")]
    pub available_firmware: Option<String>,

    /// Indicates whether the firmware of a device is updateable.
    ///
    /// Only for devices. Optional.
    #[serde(rename = "UPDATABLE")]
    pub updatable: Option<bool>,

    /// Version of the device or channel description.
    #[serde(rename = "VERSION")]
    pub version: Option<i32>,

    /// Or linking of flags for UI display.
    #[serde(rename = "FLAGS", deserialize_with = "deserialize_device_flags")]
    flags: DeviceFlags,

    /// List of roles that the channel can acquire as a sender in a link.
    ///
    /// A role is e.g. "SWITCH" for a channel that can send switching commands.
    ///
    /// Only for channels.
    #[serde(
        rename = "LINK_SOURCE_ROLES",
        deserialize_with = "crate::de::role_list"
    )]
    pub link_source_roles: Option<Vec<String>>,

    /// List of roles that the channel can acquire as a receiver in a link.
    ///
    /// A role is e.g. "SWITCH" for a channel that can receive switching commands.
    ///
    /// Only for channels.
    #[serde(
        rename = "LINK_TARGET_ROLES",
        deserialize_with = "crate::de::role_list"
    )]
    pub link_target_roles: Option<Vec<String>>,

    /// Specifies the direction (transmit or receive) of this channel in a direct link.
    ///
    /// Only for channels.
    #[serde(rename = "DIRECTION")]
    pub direction: Option<ChannelDirection>,

    /// The address of the other channel belonging to the group is specified here.
    ///
    /// Only available for grouped channels (key pairs).
    /// Optional.
    #[serde(rename = "GROUP")]
    pub group: Option<String>,

    /// Specifies the address of the virtual team channel.
    ///
    /// Only available for channels with a team (e.g. smoke detectors).
    /// Optional.
    #[serde(rename = "TEAM")]
    pub team: Option<String>,

    /// Selection of a team on the UI.
    ///
    /// A channel to be assigned to a team and the virtual team channel (the team) must have the
    /// same value here.
    ///
    /// Only available for channels with a team (e.g. smoke detectors) and for virtual team channels.
    /// Optional.
    #[serde(rename = "TEAM_TAG")]
    pub team_tag: Option<String>,

    /// Addresses of the channels assigned to the team.
    ///
    /// Only for channels that represent a team.
    /// Optional.
    #[serde(rename = "TEAM_CHANNELS")]
    pub team_channels: Option<Vec<String>>,

    /// Serial number of the interface assigned to the device.
    ///
    /// Only on BidCos-RF.
    /// Optional.
    #[serde(rename = "INTERFACE")]
    pub interface: Option<String>,

    /// Is true if the interface assignment of the device is automatically adapted to the reception
    /// conditions.
    ///
    /// Only on BidCos-RF.
    /// Optional.
    #[serde(
        rename = "ROAMING",
        deserialize_with = "crate::de::deserialize_bool_option"
    )]
    pub roaming: Option<bool>,

    /// Or-connection of flags representing the receive mode of the device.
    ///
    /// Only on BidCos-RF.
    /// Only for devices.
    /// Optional.
    #[serde(rename = "RX_MODE", deserialize_with = "deserialize_rx_mode")]
    pub rx_mode: Option<RxMode>,
}

impl DeviceDescription {
    /// This object should be visible to the end user.
    pub fn is_visible(&self) -> bool {
        self.flags.contains(DeviceFlags::Visible)
    }

    /// This object is only used internally and is not visible to the end user.
    pub fn is_internal(&self) -> bool {
        self.flags.contains(DeviceFlags::Internal)
    }

    /// This object is deletable.
    pub fn is_deletable(&self) -> bool {
        !self.flags.contains(DeviceFlags::DontDelete)
    }
}

bitflags! {
    /// Or-connection of flags when deleting devices.
    pub struct DeviceDeleteFlags: i32 {
        /// The device is reset to the factory state before deletion.
        const RESET = 0x1;
        /// The device is also deleted when it is not accessible.
        const FORCE = 0x2;
        ///If the device is not accessible, it will be deleted at the next opportunity.
        const DEFER = 0x4;
    }
}

#[derive(Debug)]
pub enum InstallMode {
    Normal = 1,
    Reset = 2,
}
