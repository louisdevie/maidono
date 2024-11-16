use clap::{Parser, Subcommand};
use clio::Input;

#[derive(Parser, Debug)]
#[command(name = "maidctl")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List the available actions
    List {
        #[arg(long)]
        enabled: bool,
        #[arg(long)]
        disabled: bool,
        #[arg(long)]
        invalid: bool,
    },

    /// Show the configuration of a group
    Show { group: String },

    /// Enable an action or a group
    Enable { actions: Vec<String> },

    /// Disable an action or a group
    Disable { actions: Vec<String> },

    /// Trigger an action for testing
    Test {
        name_or_url: String,
        #[arg(short = 'P', long)]
        payload: Option<String>,
        #[arg(short = 'F', long)]
        payload_file: Option<Input>,
    },

    /// Show the status of the service
    Status,

    /// Reload the service
    Reload,

    /// Start the service
    Start,

    /// Stop the service
    Stop,

    /// Restart the service
    Restart,
}
