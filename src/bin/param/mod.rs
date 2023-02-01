use crate::param::get::GetParamCommand;
use crate::param::list::ListParamsCommand;
use clap::Subcommand;
use homematic_rs::{
    BoolParameterDescription, EnumParameterDescription, FloatParameterDescription,
    IntegerParameterDescription, ParameterDescription, SpecialF32, SpecialI32,
    StringParameterDescription,
};
use serde::Serialize;

mod get;
mod list;

#[derive(Serialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum OutputParameterDescription {
    Float(OutputFloatParameterDescription),
    Integer(OutputIntegerParameterDescription),
    Bool(OutputBoolParameterDescription),
    Enum(OutputEnumParameterDescription),
    String(OutputStringParameterDescription),
    Action(OutputBoolParameterDescription),
}

impl From<ParameterDescription> for OutputParameterDescription {
    fn from(value: ParameterDescription) -> Self {
        match value {
            ParameterDescription::Float(pd) => OutputParameterDescription::Float(pd.into()),
            ParameterDescription::Integer(pd) => OutputParameterDescription::Integer(pd.into()),
            ParameterDescription::Bool(pd) => OutputParameterDescription::Bool(pd.into()),
            ParameterDescription::Enum(pd) => OutputParameterDescription::Enum(pd.into()),
            ParameterDescription::String(pd) => OutputParameterDescription::String(pd.into()),
            ParameterDescription::Action(pd) => OutputParameterDescription::Action(pd.into()),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputSpecialF32 {
    pub id: String,
    pub value: f32,
}

impl From<SpecialF32> for OutputSpecialF32 {
    fn from(value: SpecialF32) -> Self {
        Self {
            id: value.id,
            value: value.value,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputSpecialI32 {
    pub id: String,
    pub value: i32,
}

impl From<SpecialI32> for OutputSpecialI32 {
    fn from(value: SpecialI32) -> Self {
        Self {
            id: value.id,
            value: value.value,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputFloatParameterDescription {
    // Bitfield: 1=Read, 2=Write, 4=Event
    pub operations: i32,
    // Bitfield: 0x01 : Visible-Flag., 0x02 : Internal-Flag, 0x04 : Transform-Flag, 0x08 : Service-Flag, 0x10 : Sticky-Flag
    pub flags: i32,
    pub default: f32,
    pub min: f32,
    pub max: f32,
    pub unit: Option<String>,
    pub tab_order: Option<i32>,
    pub control: Option<String>,
    pub special: Option<Vec<OutputSpecialF32>>,
}

impl From<FloatParameterDescription> for OutputFloatParameterDescription {
    fn from(value: FloatParameterDescription) -> Self {
        Self {
            operations: value.operations,
            flags: value.flags,
            default: value.default,
            min: value.min,
            max: value.max,
            unit: value.unit,
            tab_order: value.tab_order,
            control: value.control,
            special: value
                .special
                .map(|s| s.into_iter().map(|v| v.into()).collect()),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputIntegerParameterDescription {
    // Bitfield: 1=Read, 2=Write, 4=Event
    pub operations: i32,
    // Bitfield: 0x01 : Visible-Flag., 0x02 : Internal-Flag, 0x04 : Transform-Flag, 0x08 : Service-Flag, 0x10 : Sticky-Flag
    pub flags: i32,
    pub default: i32,
    pub min: i32,
    pub max: i32,
    pub unit: Option<String>,
    pub tab_order: Option<i32>,
    pub control: Option<String>,
    pub special: Option<Vec<OutputSpecialI32>>,
}

impl From<IntegerParameterDescription> for OutputIntegerParameterDescription {
    fn from(value: IntegerParameterDescription) -> Self {
        Self {
            operations: value.operations,
            flags: value.flags,
            default: value.default,
            min: value.min,
            max: value.max,
            unit: value.unit,
            tab_order: value.tab_order,
            control: value.control,
            special: value
                .special
                .map(|s| s.into_iter().map(|v| v.into()).collect()),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputBoolParameterDescription {
    // Bitfield: 1=Read, 2=Write, 4=Event
    pub operations: i32,
    // Bitfield: 0x01 : Visible-Flag., 0x02 : Internal-Flag, 0x04 : Transform-Flag, 0x08 : Service-Flag, 0x10 : Sticky-Flag
    pub flags: i32,
    pub default: bool,
    pub min: bool,
    pub max: bool,
    pub unit: Option<String>,
    pub tab_order: Option<i32>,
    pub control: Option<String>,
}

impl From<BoolParameterDescription> for OutputBoolParameterDescription {
    fn from(value: BoolParameterDescription) -> Self {
        Self {
            operations: value.operations,
            flags: value.flags,
            default: value.default,
            min: value.min,
            max: value.max,
            unit: value.unit,
            tab_order: value.tab_order,
            control: value.control,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputEnumParameterDescription {
    // Bitfield: 1=Read, 2=Write, 4=Event
    pub operations: u8,
    // Bitfield: 0x01 : Visible-Flag., 0x02 : Internal-Flag, 0x04 : Transform-Flag, 0x08 : Service-Flag, 0x10 : Sticky-Flag
    pub flags: u8,
    pub default: String,
    pub min: String,
    pub max: String,
    pub unit: Option<String>,
    pub tab_order: Option<i32>,
    pub control: Option<String>,
    pub values: Vec<String>,
}

impl From<EnumParameterDescription> for OutputEnumParameterDescription {
    fn from(value: EnumParameterDescription) -> Self {
        Self {
            operations: value.operations,
            flags: value.flags,
            default: value.default,
            min: value.min,
            max: value.max,
            unit: value.unit,
            tab_order: value.tab_order,
            control: value.control,
            values: value.values,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputStringParameterDescription {
    // Bitfield: 1=Read, 2=Write, 4=Event
    pub operations: i32,
    // Bitfield: 0x01 : Visible-Flag., 0x02 : Internal-Flag, 0x04 : Transform-Flag, 0x08 : Service-Flag, 0x10 : Sticky-Flag
    pub flags: i32,
    pub default: String,
    pub max: String,
    pub min: String,
    pub unit: Option<String>,
    pub tab_order: Option<i32>,
    pub control: Option<String>,
}

impl From<StringParameterDescription> for OutputStringParameterDescription {
    fn from(value: StringParameterDescription) -> Self {
        Self {
            operations: value.operations,
            flags: value.flags,
            default: value.default,
            min: value.min,
            max: value.max,
            unit: value.unit,
            tab_order: value.tab_order,
            control: value.control,
        }
    }
}

#[derive(Subcommand)]
pub enum ParamCommand {
    /// List parameter descriptions
    List(ListParamsCommand),
    Get(GetParamCommand),
}
