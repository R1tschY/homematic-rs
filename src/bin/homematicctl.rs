use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt::Debug;

use clap::{Args, Parser, Subcommand, ValueEnum};
use comfy_table::Table;
use device::DeviceCommand;
use log::warn;
use serde::{Deserialize, Serialize};
use stderrlog::LogLevelNum;

use crate::message::MessageCommand;
use crate::param::ParamCommand;
use device::inspect::InspectDeviceCommand;
use device::list::ListDevicesCommand;
use homematic_rs::{DeviceDescription, HomeMaticClient};

mod device;
mod message;
mod param;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    namespace: Namespace,

    /// verbose level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// output format
    #[arg(short, long, default_value = "table")]
    output_format: Format,
}

#[derive(ValueEnum, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Format {
    Table,
    Json,
}

#[derive(Subcommand)]
enum Namespace {
    /// Device related commands
    Device {
        #[command(subcommand)]
        command: DeviceCommand,
    },
    Message {
        #[command(subcommand)]
        command: MessageCommand,
    },
    Param {
        #[command(subcommand)]
        command: ParamCommand,
    },
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error + 'static>> {
    let cli = Cli::parse();

    stderrlog::new()
        .show_module_names(true)
        .verbosity(LogLevelNum::Debug) //cli.verbose as usize)
        .init()?;
    println!("{}", cli.verbose);

    let xmlrpc = xrs_xmlrpc::client::reqwest::XmlRpcClientBuilder::new(
        env::var("HM_URL").expect("HM_URL env var"),
    )
    .basic_auth(
        env::var("HM_USERNAME").expect("HM_USERNAME env var"),
        Some(env::var("HM_PASSWORD").expect("HM_PASSWORD env var")),
    )
    .build()?;
    let client = HomeMaticClient::new(xmlrpc);

    return match cli.namespace {
        Namespace::Device { command } => match command {
            DeviceCommand::List(cmd) => cmd.exec(&client, cli.output_format).await,
            DeviceCommand::Inspect(cmd) => cmd.exec(&client, cli.output_format).await,
        },
        Namespace::Message { command } => match command {
            MessageCommand::List(cmd) => cmd.exec(&client, cli.output_format).await,
        },
        Namespace::Param { command } => match command {
            ParamCommand::List(cmd) => cmd.exec(&client, cli.output_format).await,
            ParamCommand::Get(cmd) => cmd.exec(&client, cli.output_format).await,
        },
    };
}
