use hmac::{Hmac, Mac};
use rocket::data::DataStream;
use sha2::Sha256;
use tokio::io::AsyncReadExt;

pub async fn check_hmac_sha256<'r>(
    secret: &str,
    mut message: DataStream<'r>,
    digest: &[u8],
) -> bool {
    let mut hmac: Hmac<Sha256> =
        Hmac::new_from_slice(secret.as_bytes()).expect("HMAC should accept a key of any size");
    let mut buffer = Vec::with_capacity(2048);
    match message.read_to_end(&mut buffer).await {
        Ok(_) => {
            hmac.update(buffer.as_slice());
            hmac.verify_slice(digest).is_ok()
        }
        Err(_) => false,
    }
}
