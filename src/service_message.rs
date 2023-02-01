use crate::StringParameterDescription;
use serde::Deserialize;
use std::borrow::Cow;
use xrs_xmlrpc::value::Value;

/// Service message
#[derive(Deserialize, Debug)]
pub struct ServiceMessage<'a>(
    #[serde(borrow = "'a")] pub Cow<'a, str>,
    #[serde(borrow = "'a")] pub Cow<'a, str>,
    #[serde(borrow = "'a")] pub Value<'a>,
);

impl<'a> ServiceMessage<'a> {
    /// Address (serial number) of the channel that generated the service message.
    pub fn address(&self) -> &str {
        self.0.as_ref()
    }

    /// ID of the service message (CONFIG_PENDING, UNREACH, etc.)
    pub fn id(&self) -> &str {
        self.1.as_ref()
    }

    /// Value of the Service message
    pub fn value(&self) -> &Value<'a> {
        &self.2
    }
}
