use crate::printer::Printer;
use clap::Parser;
use maidono_core::enabled_actions::ActionStatus;
use maidono_core::utils::ErrorPrinter;

use crate::actions::{FileTestPayload, NoTestPayload, StringTestPayload};
use crate::cli::{Cli, Commands};

mod actions;
mod cli;
mod printer;

fn main() {
    let args = Cli::parse();
    let mut printer = Printer::new();

    match args.command {
        Commands::List {
            enabled,
            disabled,
            invalid,
        } => actions::list(ActionStatus::from_flags(enabled, disabled), invalid),

        Commands::Show { group } => actions::show(group),
        Commands::Enable { actions } => actions::enable(actions),
        Commands::Disable { actions } => actions::disable(actions),

        Commands::Test {
            name_or_url,
            payload: None,
            payload_file: None,
        } => actions::test(name_or_url, NoTestPayload()),

        Commands::Test {
            name_or_url,
            payload: Some(string_payload),
            payload_file: None,
        } => actions::test(name_or_url, StringTestPayload(string_payload)),

        Commands::Test {
            name_or_url,
            payload: None,
            payload_file: Some(file_payload),
        } => {
            actions::test(name_or_url, FileTestPayload(file_payload));
        }

        Commands::Test {
            name_or_url: _,
            payload: Some(_),
            payload_file: Some(_),
        } => {
            printer.print_error("both literal and file payload given");
        }

        Commands::Status => actions::systemctl("status"),
        Commands::Reload => actions::reload(),
        Commands::Start => actions::systemctl("start"),
        Commands::Stop => actions::systemctl("stop"),
        Commands::Restart => actions::systemctl("restart"),
    }
}
