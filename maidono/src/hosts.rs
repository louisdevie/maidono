use crate::logger::Logger;
use crate::security::Signature;
use maidono_core::actions::HostRef;
use rocket::Request;

pub fn host_information_checks_out(host_ref: &HostRef, request: &Request) -> bool {
    match host_ref {
        HostRef::GitHub => {
            request
                .headers()
                .get_one("User-Agent")
                .map(|ua| ua.starts_with("GitHub-Hookshot"))
                .unwrap_or(false)
                && request.headers().contains("X-Github-Delivery")
                && request.headers().contains("X-Github-Event")
        }
        _ => true,
    }
}

pub fn display_event_info(logger: &Logger, host_ref: &HostRef, request: &Request) {
    match host_ref {
        HostRef::GitHub => {
            if let Some(delivery_id) = request.headers().get_one("X-Github-Delivery") {
                logger.log(format!("  Github delivery ID: {}", delivery_id));
            }
            if let Some(delivery_id) = request.headers().get_one("X-Github-Event") {
                logger.log(format!("  Github event type: {}", delivery_id));
            }
        }
        _ => {}
    }
}

pub fn extract_signature<'a>(host_ref: &HostRef, request: &'a Request) -> Option<Signature> {
    match host_ref {
        HostRef::GitHub => request
            .headers()
            .get_one("X-Hub-Signature-256")
            .and_then(decode_hub256_signature),
        _ => None,
    }
}

fn decode_hub256_signature(signature: &str) -> Option<Signature> {
    // 7 chars for the prefix + 64 chars for the hash
    if signature.starts_with("sha256=") && signature.len() == 71 {
        let mut signature_bytes = [0u8; 32];
        let decoded = hex::decode_to_slice(&signature[7..], &mut signature_bytes).is_ok();
        if decoded {
            Some(Signature::HS256Hex(signature_bytes))
        } else {
            None
        }
    } else {
        None
    }
}
