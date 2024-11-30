use rocket::data::DataStream;

mod hmac;

pub enum Signature {
    HS256Hex([u8; 32]),
}

impl Signature {
    pub async fn matches<'r>(&self, secret: &str, message: DataStream<'r>) -> bool {
        match self {
            Signature::HS256Hex(digest) => hmac::check_hmac_sha256(secret, message, digest).await,
        }
    }
}
