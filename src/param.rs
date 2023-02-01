use serde::Deserialize;
use std::collections::HashMap;
use xrs_xmlrpc::value::Value;

pub type ParamsetDescription = HashMap<String, ParameterDescription>;

pub type Paramset<'a> = HashMap<String, Value<'a>>;

#[derive(Deserialize)]
#[serde(tag = "TYPE")]
pub enum ParameterDescription {
    #[serde(rename = "FLOAT")]
    Float(FloatParameterDescription),

    #[serde(rename = "INTEGER")]
    Integer(IntegerParameterDescription),

    #[serde(rename = "BOOL")]
    Bool(BoolParameterDescription),

    #[serde(rename = "ENUM")]
    Enum(EnumParameterDescription),

    #[serde(rename = "STRING")]
    String(StringParameterDescription),

    #[serde(rename = "ACTION")]
    Action(BoolParameterDescription),
}

#[derive(Deserialize)]
pub struct SpecialF32 {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "VALUE")]
    pub value: f32,
}

#[derive(Deserialize)]
pub struct SpecialI32 {
    #[serde(rename = "ID")]
    pub id: String,

    #[serde(rename = "VALUE")]
    pub value: i32,
}

#[derive(Deserialize)]
pub struct FloatParameterDescription {
    // Bitfield: 1=Read, 2=Write, 4=Event
    #[serde(rename = "OPERATIONS")]
    pub operations: i32,

    // Bitfield: 0x01 : Visible-Flag., 0x02 : Internal-Flag, 0x04 : Transform-Flag, 0x08 : Service-Flag, 0x10 : Sticky-Flag
    #[serde(rename = "FLAGS")]
    pub flags: i32,

    #[serde(rename = "DEFAULT")]
    pub default: f32,

    #[serde(rename = "MIN")]
    pub min: f32,

    #[serde(rename = "MAX")]
    pub max: f32,

    #[serde(rename = "UNIT")]
    pub unit: Option<String>,

    #[serde(rename = "TAB_ORDER")]
    pub tab_order: Option<i32>,

    #[serde(rename = "CONTROL")]
    pub control: Option<String>,

    #[serde(rename = "SPECIAL")]
    pub special: Option<Vec<SpecialF32>>,
}

#[derive(Deserialize)]
pub struct IntegerParameterDescription {
    // Bitfield: 1=Read, 2=Write, 4=Event
    #[serde(rename = "OPERATIONS")]
    pub operations: i32,

    // Bitfield: 0x01 : Visible-Flag., 0x02 : Internal-Flag, 0x04 : Transform-Flag, 0x08 : Service-Flag, 0x10 : Sticky-Flag
    #[serde(rename = "FLAGS")]
    pub flags: i32,

    #[serde(rename = "DEFAULT")]
    pub default: i32,

    #[serde(rename = "MIN")]
    pub min: i32,

    #[serde(rename = "MAX")]
    pub max: i32,

    #[serde(rename = "UNIT")]
    pub unit: Option<String>,

    #[serde(rename = "TAB_ORDER")]
    pub tab_order: Option<i32>,

    #[serde(rename = "CONTROL")]
    pub control: Option<String>,

    #[serde(rename = "SPECIAL")]
    pub special: Option<Vec<SpecialI32>>,
}

#[derive(Deserialize)]
pub struct BoolParameterDescription {
    // Bitfield: 1=Read, 2=Write, 4=Event
    #[serde(rename = "OPERATIONS")]
    pub operations: i32,

    // Bitfield: 0x01 : Visible-Flag., 0x02 : Internal-Flag, 0x04 : Transform-Flag, 0x08 : Service-Flag, 0x10 : Sticky-Flag
    #[serde(rename = "FLAGS")]
    pub flags: i32,

    #[serde(rename = "DEFAULT")]
    pub default: bool,

    #[serde(rename = "MIN")]
    pub min: bool,

    #[serde(rename = "MAX")]
    pub max: bool,

    #[serde(rename = "UNIT")]
    pub unit: Option<String>,

    #[serde(rename = "TAB_ORDER")]
    pub tab_order: Option<i32>,

    #[serde(rename = "CONTROL")]
    pub control: Option<String>,
}

#[derive(Deserialize)]
pub struct EnumParameterDescription {
    // Bitfield: 1=Read, 2=Write, 4=Event
    #[serde(rename = "OPERATIONS")]
    pub operations: u8,

    // Bitfield: 0x01 : Visible-Flag., 0x02 : Internal-Flag, 0x04 : Transform-Flag, 0x08 : Service-Flag, 0x10 : Sticky-Flag
    #[serde(rename = "FLAGS")]
    pub flags: u8,

    #[serde(rename = "DEFAULT")]
    pub default: String,

    #[serde(rename = "MIN")]
    pub min: String,

    #[serde(rename = "MAX")]
    pub max: String,

    #[serde(rename = "UNIT")]
    pub unit: Option<String>,

    #[serde(rename = "TAB_ORDER")]
    pub tab_order: Option<i32>,

    #[serde(rename = "CONTROL")]
    pub control: Option<String>,

    #[serde(rename = "VALUE_LIST")]
    pub values: Vec<String>,
}

#[derive(Deserialize)]
pub struct StringParameterDescription {
    // Bitfield: 1=Read, 2=Write, 4=Event
    #[serde(rename = "OPERATIONS")]
    pub operations: i32,

    // Bitfield: 0x01 : Visible-Flag., 0x02 : Internal-Flag, 0x04 : Transform-Flag, 0x08 : Service-Flag, 0x10 : Sticky-Flag
    #[serde(rename = "FLAGS")]
    pub flags: i32,

    #[serde(rename = "DEFAULT")]
    pub default: String,

    #[serde(rename = "MIN")]
    pub min: String,

    #[serde(rename = "MAX")]
    pub max: String,

    #[serde(rename = "UNIT")]
    pub unit: Option<String>,

    #[serde(rename = "TAB_ORDER")]
    pub tab_order: Option<i32>,

    #[serde(rename = "CONTROL")]
    pub control: Option<String>,
}
