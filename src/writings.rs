use serde::{Deserialize, Serialize};
use worker::{console_log, Response};

#[derive(Serialize, Deserialize, Debug)]
pub struct Writing {
    id: String,
    body: String,
    created: String,
    updated: String,
    deleted: String,
}

pub async fn get_writings() -> Result<Response, worker::Error> {
    Result::Ok(Response::ok(
        "OK"
    )?)
}

pub async fn get_writing(id: String) -> Result<Response, worker::Error> {
    console_log!("{:?}", id);

    Result::Ok(Response::ok(
        "OK"
    )?)
}

pub async fn add_writings(body: Writing) -> Result<Response, worker::Error> {
    console_log!("{:?}", body);

    return Result::Ok(Response::ok(
        "OK"
    )?)
}