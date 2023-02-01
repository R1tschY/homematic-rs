use std::collections::HashMap;
use std::time::Duration;

use bitflags::bitflags;
use serde::{Deserialize, Deserializer, Serialize};
use xrs_xmlrpc::client::reqwest::XmlRpcClient;
use xrs_xmlrpc::value::Value;
use xrs_xmlrpc::XmlRpcError;

pub(crate) mod de;
mod device;
mod param;
mod service_message;

use crate::device::InstallMode;
pub use crate::service_message::ServiceMessage;
pub use device::{ChannelDirection, DeviceDeleteFlags, DeviceDescription, RxMode};
pub use param::{
    BoolParameterDescription, EnumParameterDescription, FloatParameterDescription,
    IntegerParameterDescription, ParameterDescription, Paramset, ParamsetDescription, SpecialF32,
    SpecialI32, StringParameterDescription,
};

pub const BIDCOS_WIRED_PORT: u16 = 2001;
pub const BIDCOS_RF_PORT: u16 = 2010;

pub struct HomeMaticClient {
    xmlrpc: XmlRpcClient,
}

/*
    "abortDeleteDevice",
    "activateLinkParamset",
    "addDevice",
    "addLink",
    "addVirtualDeviceInstance",
    "changeKey",
    "clearConfigCache",
    "deleteDevice",
    "deleteVolatileMetadata",
    "determine_parameter",
    "exit",
    "getAllMetadata",
    "getDeviceDescription",
    "getInstallMode",
    "getKeyMismatchDevice",
    "getLinkInfo",
    "getLinkPeers",
    "getLinks",
    "getMetadata",
    "getParamset",
    "getParamsetDescription",
    "getParamsetId",
    "getServiceMessages",
    "getValue",
    "getVersion",
    "getVolatileMetadata",
    "hasVolatileMetadata",
    "init",
    "listBidcosInterfaces",
    "listDevices",
    "listReplaceableDevices",
    "listTeams",
    "logLevel",
    "ping",
    "putParamset",
    "refreshDeployedDeviceFirmwareList",
    "removeLink",
    "replaceDevice",
    "reportValueUsage",
    "restoreConfigToDevice",
    "rssiInfo",
    "setBidcosInterface",
    "set_install_mode",
    "setInterfaceClock",
    "setLinkInfo",
    "setMetadata",
    "setRFLGWInfoLED",
    "setTeam",
    "setTempKey",
    "setValue",
    "setVolatileMetadata",
    "updateFirmware",
]
 */

impl HomeMaticClient {
    pub fn new(xmlrpc: XmlRpcClient) -> Self {
        Self { xmlrpc }
    }

    pub async fn list_devices(
        &self,
        buf: &mut String,
    ) -> Result<Vec<DeviceDescription>, XmlRpcError> {
        self.xmlrpc.call("listDevices", &(), buf).await
    }

    pub async fn get_device_description(
        &self,
        address: &str,
        buf: &mut String,
    ) -> Result<DeviceDescription, XmlRpcError> {
        self.xmlrpc
            .call("getDeviceDescription", &(address,), buf)
            .await
    }

    pub async fn get_paramset_description(
        &self,
        address: &str,
        paramset_type: &str,
        buf: &mut String,
    ) -> Result<ParamsetDescription, XmlRpcError> {
        self.xmlrpc
            .call("getParamsetDescription", &(address, paramset_type), buf)
            .await
    }

    pub async fn get_paramset_id(
        &self,
        address: &str,
        type_: &str,
        buf: &mut String,
    ) -> Result<String, XmlRpcError> {
        self.xmlrpc
            .call("getParamsetId", &(address, type_), buf)
            .await
    }

    pub async fn get_paramset<'a>(
        &self,
        address: &str,
        paramset_type: &str,
        buf: &'a mut String,
    ) -> Result<Paramset<'a>, XmlRpcError> {
        self.xmlrpc
            .call("getParamset", &(address, paramset_type), buf)
            .await
    }

    pub async fn put_paramset(
        &self,
        address: &str,
        paramset_type: &str,
        set: Paramset<'_>,
        buf: &mut String,
    ) -> Result<(), XmlRpcError> {
        self.xmlrpc
            .call("putParamset", &(address, paramset_type, set), buf)
            .await
    }

    pub async fn get_value<'a>(
        &self,
        address: &str,
        value_key: &str,
        buf: &'a mut String,
    ) -> Result<Value<'a>, XmlRpcError> {
        self.xmlrpc
            .call("getValue", &(address, value_key), buf)
            .await
    }

    pub async fn set_value(
        &self,
        address: &str,
        value_key: &str,
        value: Value<'_>,
        buf: &mut String,
    ) -> Result<(), XmlRpcError> {
        self.xmlrpc
            .call("setValue", &(address, value_key, value), buf)
            .await
    }

    pub async fn determine_parameter(
        &self,
        address: &str,
        paramset_key: &str,
        parameter_id: &str,
        buf: &mut String,
    ) -> Result<(), XmlRpcError> {
        self.xmlrpc
            .call(
                "determineParameter",
                &(address, paramset_key, parameter_id),
                buf,
            )
            .await
    }

    pub async fn delete_device(
        &self,
        address: &str,
        flags: DeviceDeleteFlags,
        buf: &mut String,
    ) -> Result<(), XmlRpcError> {
        self.xmlrpc
            .call("deleteDevice", &(address, flags.bits()), buf)
            .await
    }

    pub async fn abort_delete_device(
        &self,
        address: &str,
        buf: &mut String,
    ) -> Result<(), XmlRpcError> {
        self.xmlrpc
            .call("abortDeleteDevice", &(address,), buf)
            .await
    }

    pub async fn set_install_mode(&self, on: bool, buf: &mut String) -> Result<(), XmlRpcError> {
        self.xmlrpc.call("setInstallMode", &(on,), buf).await
    }

    pub async fn set_install_mode_with_timeout(
        &self,
        on: bool,
        time: Duration,
        mode: InstallMode,
        buf: &mut String,
    ) -> Result<(), XmlRpcError> {
        self.xmlrpc
            .call("setInstallMode", &(on, time.as_secs(), mode as i32), buf)
            .await
    }

    pub async fn set_install_mode_for_address(
        &self,
        on: bool,
        time: Duration,
        address: &str,
        buf: &mut String,
    ) -> Result<(), XmlRpcError> {
        self.xmlrpc
            .call("setInstallMode", &(on, time.as_secs(), address), buf)
            .await
    }

    pub async fn get_install_mode(&self, buf: &mut String) -> Result<Duration, XmlRpcError> {
        let remaining: u64 = self.xmlrpc.call("getInstallMode", &(), buf).await?;
        Ok(Duration::from_secs(remaining))
    }

    pub async fn get_key_missmatch_device(
        &self,
        reset: bool,
        buf: &mut String,
    ) -> Result<String, XmlRpcError> {
        self.xmlrpc
            .call("getKeyMissmatchDevice", &(reset,), buf)
            .await
    }

    pub async fn set_temp_key(
        &self,
        passphrase: &str,
        buf: &mut String,
    ) -> Result<(), XmlRpcError> {
        self.xmlrpc.call("setTempKey", &(passphrase,), buf).await
    }

    pub async fn get_service_messages<'a>(
        &self,
        buf: &'a mut String,
    ) -> Result<Vec<ServiceMessage<'a>>, XmlRpcError> {
        self.xmlrpc.call("getServiceMessages", &(), buf).await
    }
}
