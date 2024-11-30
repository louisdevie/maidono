use maidono_core::actions::{read_group_by_name, try_read_all_groups, Action, Group};
use maidono_core::enabled_actions::{load_enabled_actions, ActionStatus, EnabledList};
use maidono_core::utils::ErrorPrinter;
use owo_colors::OwoColorize;

use crate::printer::{Printer, SensitiveStr};

pub fn list(_with_status: Option<ActionStatus>, invalid_only: bool) {
    let mut printer = Printer::new();

    let enabled = match load_enabled_actions() {
        Ok(el) => Some(el),
        Err(err) => {
            printer.print_detailed_error(err);
            None
        }
    };

    match try_read_all_groups() {
        Ok(groups) => {
            for (name, result) in groups {
                match result {
                    Ok(group) => {
                        if !invalid_only {
                            list_group(&mut printer, enabled.as_ref(), &name, group);
                        }
                    }
                    Err(error) => {
                        printer
                            .print_text(name)
                            .print_very_compact_error(error)
                            .println();
                    }
                }
            }
        }
        Err(err) => {
            printer.print_detailed_error(err);
        }
    }
}

fn list_group(printer: &mut Printer, enabled: Option<&EnabledList>, name: &str, group: Group) {
    printer.print_text(name);
    if group.action_count() == 0 {
        printer.print_text("(empty)");
    }
    printer.println().indent();

    for action_name in group.enumerate_action_names() {
        list_action(printer, enabled, name, action_name);
    }
}

pub fn list_action(printer: &mut Printer, enabled: Option<&EnabledList>, group: &str, name: &str) {
    let (is_enabled, is_disabled) = if let Some(enabled_list) = enabled {
        if enabled_list.is_action_enabled(group, name) {
            (true, false)
        } else {
            (false, true)
        }
    } else {
        (false, false)
    };

    if is_enabled {
        printer.print_text("●".green());
    }
    if is_disabled {
        printer.print_text("○");
    }
    printer.print_text(name).println();
}

pub fn show(name: String) {
    let mut printer = Printer::new();

    let enabled = match load_enabled_actions() {
        Ok(el) => Some(el),
        Err(err) => {
            printer.print_detailed_error(err);
            None
        }
    };

    match read_group_by_name(&name) {
        Ok(group) => {
            printer.print_em_text(&name);
            if group.action_count() == 0 {
                printer.print_text("(empty)");
            }
            printer.println().indent();

            for (action_name, action) in group.enumerate_actions() {
                show_action(&mut printer, enabled.as_ref(), &name, action_name, action);
            }
        }
        Err(err) => {
            printer.print_detailed_error(err);
        }
    }
}

pub fn show_action(
    printer: &mut Printer,
    enabled: Option<&EnabledList>,
    group: &str,
    name: &str,
    action: &Action,
) {
    let (is_enabled, is_disabled) = if let Some(enabled_list) = enabled {
        if enabled_list.is_action_enabled(group, name) {
            (true, false)
        } else {
            (false, true)
        }
    } else {
        (false, false)
    };

    if is_enabled {
        printer.print_text("●".green());
    }
    if is_disabled {
        printer.print_text("○");
    }
    printer.print_em_text(name);

    if is_enabled {
        printer.print_text("(enabled)");
    }
    if is_disabled {
        printer.print_text("(disabled)");
    }
    printer.println().indent();

    printer
        .print_text("trigger:")
        .println_text(action.trigger())
        .print_text("origin:")
        .println_text(action.origin());
    if let Some(secret) = action.secret() {
        printer
            .print_text("secret:")
            .println_text(SensitiveStr::from(secret));
    }
    if !action.before().is_empty() {
        printer.print_text("before:").println_list(action.before());
    }
    if !action.after().is_empty() {
        printer.print_text("after:").println_list(action.after());
    }
    if action.action().has_multiple_commands() {
        printer
            .println_text("command:")
            .indent()
            .print_multiline(action.action())
            .unindent();
    } else {
        printer
            .print_text("command:")
            .print_multiline(action.action());
    }

    printer.unindent().println();
}
