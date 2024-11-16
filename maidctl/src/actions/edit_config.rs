use crate::Printer;
use maidono_core::actions::{try_read_all_groups, Group};
use maidono_core::enabled_actions::{dump_enabled_actions, load_enabled_actions, EnabledList};
use maidono_core::utils::{ActionPath, ActionPathPattern, ErrorPrinter, Result};
use owo_colors::OwoColorize;

pub fn enable(actions: Vec<String>) {
    let mut printer = Printer::new();

    match for_each_pattern(actions, &mut printer, enable_pattern) {
        Ok(list) => match dump_enabled_actions(&list) {
            Ok(()) => {}
            Err(err) => {
                printer.print_detailed_error(err);
            }
        },
        Err(err) => {
            printer.print_detailed_error(err);
        }
    };
}

fn enable_pattern(
    printer: &mut Printer,
    available: &Vec<ActionPath>,
    enabled: &mut EnabledList,
    pattern: ActionPathPattern,
) {
    let mut matched_any = false;
    for path in available {
        if path.matches(&pattern) {
            matched_any = true;
            if enabled.enable_path(path.clone()) {
                printer
                    .print_text("●".green())
                    .print_em_text(path)
                    .println_text("is now enabled");
            } else {
                printer
                    .print_text("~".green())
                    .print_em_text(path)
                    .println_text("is already enabled");
            }
        }
    }
    if !matched_any {
        printer
            .print_text("pattern")
            .print_em_text(pattern)
            .println_text("did not match any actions");
    }
}

pub fn disable(actions: Vec<String>) {
    let mut printer = Printer::new();

    match for_each_pattern(actions, &mut printer, disable_pattern) {
        Ok(list) => match dump_enabled_actions(&list) {
            Ok(()) => {}
            Err(err) => {
                printer.print_detailed_error(err);
            }
        },
        Err(err) => {
            printer.print_detailed_error(err);
        }
    };
}

fn disable_pattern(
    printer: &mut Printer,
    available: &Vec<ActionPath>,
    enabled: &mut EnabledList,
    pattern: ActionPathPattern,
) {
    let mut matched_any = false;
    for path in available {
        if path.matches(&pattern) {
            matched_any = true;
            if enabled.disable_path(path.clone()) {
                printer
                    .print_text("○")
                    .print_em_text(path)
                    .println_text("is now disabled");
            } else {
                printer
                    .print_text("~")
                    .print_em_text(path)
                    .println_text("is already disabled");
            }
        }
    }
    if !matched_any {
        printer
            .print_text("pattern")
            .print_em_text(pattern)
            .println_text("did not match any actions");
    }
}

fn for_each_pattern<
    H: FnMut(&mut Printer, &Vec<ActionPath>, &mut EnabledList, ActionPathPattern),
>(
    actions: Vec<String>,
    printer: &mut Printer,
    mut handler: H,
) -> Result<EnabledList> {
    let available = try_read_all_groups()?
        .into_iter()
        .flat_map(group_to_action_path_list)
        .collect();
    let mut enabled = load_enabled_actions()?;

    for action in actions {
        match ActionPathPattern::parse(&action) {
            Ok(pattern) => handler(printer, &available, &mut enabled, pattern),
            Err(_) => {
                printer.print_error(format!("'{}' is not a valid action path pattern", action));
            }
        }
    }

    Ok(enabled)
}

fn group_to_action_path_list((group_name, result): (String, Result<Group>)) -> Vec<ActionPath> {
    match result {
        Ok(group) => group
            .enumerate_action_names()
            .map(|action_name| ActionPath::from_parts(&group_name, action_name))
            .collect(),
        Err(_) => vec![],
    }
}
