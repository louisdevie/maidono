use crate::printer::Printer;
use maidono_core::utils::ErrorPrinter;
use std::process::Command;

pub fn systemctl(verb: &'static str) {
    let mut printer = Printer::new();

    let result = Command::new("systemctl").args([verb, "maidono"]).status();

    match result {
        Ok(status) => match (status.success(), status.code()) {
            (false, Some(code)) => {
                printer.print_error(format!("systemctl exited with status code {}", code));
            }
            (false, None) => {
                printer.print_error("systemctl was terminated by a signal");
            }
            (true, _) => {}
        },
        Err(err) => {
            printer.print_error(format!("Unable to use systemctl: {}", err));
        }
    }
}
