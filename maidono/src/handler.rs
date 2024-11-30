use crate::execution::run_actions;
use crate::hosts::{display_event_info, extract_signature, host_information_checks_out};
use crate::logger::Logger;
use crate::state::{ActionRef, Actions};
use maidono_core::problem;
use maidono_core::utils::Result;
use rocket::data::ToByteUnit;
use rocket::http::{Method, Status};
use rocket::outcome::Outcome;
use rocket::route::{Handler, Outcome as RouteOutcome};
use rocket::{Data, Request, Response, Route, State};
use std::path::PathBuf;

#[derive(Clone)]
pub struct WebhookHandler();

impl WebhookHandler {
    pub(crate) fn routes() -> Vec<Route> {
        vec![Route::new(Method::Post, "/<path..>", Self())]
    }

    pub(crate) async fn handle_webhook<'r>(
        &self,
        logger: Logger,
        actions: &'r Actions,
        request: &'r Request<'_>,
        data: Data<'r>,
    ) -> Result<RouteOutcome<'r>> {
        let path = request
            .segments::<PathBuf>(0..)
            .map_err(|_| problem!("Could not get path of request"))?;

        match path
            .to_str()
            .and_then(|path_str| actions.lookup_by_trigger(path_str))
        {
            None => Ok(RouteOutcome::Forward((data, Status::NotFound))),
            Some(action_ref) => {
                Self::handle_action(logger, actions, action_ref, request, data).await
            }
        }
    }

    async fn handle_action<'a>(
        logger: Logger,
        actions: &'a Actions,
        action_ref: ActionRef<'a>,
        request: &'a Request<'_>,
        data: Data<'a>,
    ) -> Result<RouteOutcome<'a>> {
        let host_ref = action_ref.action.origin();
        if !host_information_checks_out(host_ref, request) {
            logger.debug_message("Webhook trigger blocked because of invalid or missing headers");
            return Ok(RouteOutcome::Error(Status::BadRequest));
        }
        let body_size_limit = request.limits().get("bytes").unwrap_or(1.kibibytes());
        if let Some(secret) = action_ref.action.secret() {
            let signature_is_valid = if let Some(signature) = extract_signature(host_ref, request) {
                signature.matches(secret, data.open(body_size_limit)).await
            } else {
                false
            };

            if !signature_is_valid {
                logger.debug_message(
                    "Webhook trigger blocked because of invalid or missing signature",
                );
                return Ok(RouteOutcome::Error(Status::BadRequest));
            }
        }

        logger.log(format!("Action '{}' triggered by webhook", action_ref.path));
        display_event_info(&logger, host_ref, request);

        let ctx = actions.load_context_for(action_ref)?;
        tokio::spawn(run_actions(ctx, logger));
        Ok(RouteOutcome::Success(
            Response::build().status(Status::Ok).finalize(),
        ))
    }
}

#[rocket::async_trait]
impl Handler for WebhookHandler {
    async fn handle<'r>(&self, req: &'r Request<'_>, data: Data<'r>) -> RouteOutcome<'r> {
        let logger = req.guard::<Logger>().await.unwrap();
        match req.guard::<&State<Actions>>().await {
            Outcome::Success(actions) => {
                match self
                    .handle_webhook(logger, actions.inner(), req, data)
                    .await
                {
                    Ok(outcome) => outcome,
                    Err(error) => {
                        logger.error(error);
                        RouteOutcome::Error(Status::InternalServerError)
                    }
                }
            }
            Outcome::Error((status, ())) => RouteOutcome::Error(status),
            Outcome::Forward(status) => RouteOutcome::Forward((data, status)),
        }
    }
}
