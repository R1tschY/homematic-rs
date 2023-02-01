use std::borrow::Cow;

use clap::Subcommand;
use serde::Serialize;
use xrs_xmlrpc::value::Value;

use homematic_rs::ServiceMessage;

use crate::message::list::ListServiceMessagesCommand;

pub mod list;

#[derive(Serialize)]
pub struct OutputServiceMessage<'a> {
    address: Cow<'a, str>,
    id: Cow<'a, str>,
    value: Value<'a>,
}

impl<'a> From<ServiceMessage<'a>> for OutputServiceMessage<'a> {
    fn from(value: ServiceMessage<'a>) -> Self {
        Self {
            address: value.0,
            id: value.1,
            value: value.2,
        }
    }
}

#[derive(Subcommand)]
pub enum MessageCommand {
    /// Lists all devices and channels
    List(ListServiceMessagesCommand),
}
