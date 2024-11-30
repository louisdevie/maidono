use crate::execution::context::ContextAction;
use crate::execution::Context;
use crate::logger::Logger;
use std::cmp::PartialEq;
use std::path::PathBuf;

#[derive(PartialEq)]
enum ActionRunStatus {
    Running,
    Failed,
}

pub async fn run_actions(context: Context, logger: Logger) {
    let working_dir = std::env::current_dir().unwrap_or(PathBuf::from("."));
    let mut status = ActionRunStatus::Running;
    for action in context.actions() {
        match status {
            ActionRunStatus::Running => {
                logger.log(format!("Running action '{}'", action.path()));
                status = run_single_action(logger, action, &working_dir).await;
                match status {
                    ActionRunStatus::Running => logger.log("  OK"),
                    ActionRunStatus::Failed => logger.error_message(format!(
                        "  Failed to run action '{}' due to the error above.",
                        action.path()
                    )),
                }
            }
            ActionRunStatus::Failed => {
                logger.log(format!("Skipping action '{}'", action.path()));
            }
        }
    }
}

async fn run_single_action(
    logger: Logger,
    action: &ContextAction,
    working_dir: &PathBuf,
) -> ActionRunStatus {
    let mut status = ActionRunStatus::Running;
    for command in action.commands() {
        logger.log(format!("  bash:{}$ {}", working_dir.display(), command));
        let command_result = tokio::process::Command::new("/bin/bash")
            .arg("-c")
            .arg(command)
            .status()
            .await;

        match command_result {
            Ok(command_status) => {
                if !command_status.success() {
                    logger.error_message(format!(
                        "    Command failed with exit status: {}",
                        command_status
                    ));
                    status = ActionRunStatus::Failed;
                }
            }
            Err(err) => {
                logger.error_message(format!("    Could not run command: {}", err));
                status = ActionRunStatus::Failed;
            }
        }

        if status == ActionRunStatus::Failed {
            break;
        }
    }
    status
}
