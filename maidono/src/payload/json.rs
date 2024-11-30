// use crate::payload::Payload;
use rocket::data::{FromData, Outcome};
use rocket::serde::json::{Error, Json, Value};
use rocket::{async_trait, Data, Request};

pub struct JsonPayload {
    _value: Value,
}

#[async_trait]
impl<'r> FromData<'r> for JsonPayload {
    type Error = Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let json: Outcome<Json<Value>> = Json::from_data(req, data).await;
        json.map(|value| JsonPayload { _value: value.0 })
    }
}

// impl Payload<'_> for JsonPayload {}
