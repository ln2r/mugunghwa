use serde::{Deserialize, Serialize};
use worker::{console_log, Response, RouteContext};
use worker::wasm_bindgen::JsValue;

#[derive(Serialize, Deserialize, Debug)]
pub struct Writing {
    id: String,
    title: String,
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

pub async fn add_writings(body: Writing, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    console_log!("{:?}", body);
    let d1 = ctx.env.d1(&ctx.env.var("db_name")?.to_string())?;

    // FIXME: change to snowflake/uuid
    let id = '1';

    d1.prepare("INSERT INTO writings (id, title, body, created, updated) VALUES (?, ?, ?, ?, ?);")
        .bind(&[
            JsValue::from(&id.to_string()),
            JsValue::from(&body.title.to_string()),
            JsValue::from(&body.body.to_string()),
            // FIXME: actually send the time
            JsValue::from(&body.body.to_string()),
            JsValue::from(&body.body.to_string()),
        ])?
        .run()
        .await?;

    return Result::Ok(Response::ok(
        "OK"
    )?)
}