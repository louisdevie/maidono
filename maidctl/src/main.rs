use crate::printer::Printer;
use clap::Parser;
use maidono_core::enabled_actions::ActionStatus;
use maidono_core::utils::ErrorPrinter;

use crate::cli::{Cli, Commands};
use crate::commands::{FileTestPayload, NoTestPayload, StringTestPayload};

mod cli;
mod commands;
mod printer;

fn main() {
    let args = Cli::parse();
    let mut printer = Printer::new();

    match args.command {
        Commands::List {
            enabled,
            disabled,
            invalid,
        } => commands::list(ActionStatus::from_flags(enabled, disabled), invalid),

        Commands::Show { group } => commands::show(group),
        Commands::Enable { actions } => commands::enable(actions),
        Commands::Disable { actions } => commands::disable(actions),

        Commands::Test {
            name_or_url,
            payload: None,
            payload_file: None,
        } => commands::test(name_or_url, NoTestPayload()),

        Commands::Test {
            name_or_url,
            payload: Some(string_payload),
            payload_file: None,
        } => commands::test(name_or_url, StringTestPayload(string_payload)),

        Commands::Test {
            name_or_url,
            payload: None,
            payload_file: Some(file_payload),
        } => {
            commands::test(name_or_url, FileTestPayload(file_payload));
        }

        Commands::Test {
            name_or_url: _,
            payload: Some(_),
            payload_file: Some(_),
        } => {
            printer.print_error("both literal and file payload given");
        }

        Commands::Status => commands::systemctl("status"),
        Commands::Reload => commands::reload(),
        Commands::Start => commands::systemctl("start"),
        Commands::Stop => commands::systemctl("stop"),
        Commands::Restart => commands::systemctl("restart"),
    }
}
