use serde::{Deserialize, Serialize};
use worker::{console_log, Date, Response, RouteContext};
use worker::wasm_bindgen::JsValue;
use crate::commons::generate_snowflake;

#[derive(Serialize, Deserialize, Debug)]
pub struct Writing {
    pub id: String,
    pub title: String,
    pub body: String,
    pub created: String,
    pub updated: String,
    pub deleted: Option<String>,
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
    let d1 = ctx.env.d1(&ctx.env.var("db_name")?.to_string())?;

    let id = generate_snowflake(ctx);
    let now = Date::now();

    d1.prepare("INSERT INTO writings (id, title, body, created, updated) VALUES (?, ?, ?, ?, ?);")
        .bind(&[
            JsValue::from(&id.to_string()),
            JsValue::from(&body.title.to_string()),
            JsValue::from(&body.body.to_string()),
            JsValue::from(now.to_string()),
            JsValue::from(now.to_string()),
        ])?
        .run()
        .await?;

    let res = d1.prepare("SELECT * FROM writings WHERE id = ?;")
        .bind(&[
            JsValue::from(&id.to_string()),
        ])?
        .first::<Writing>(None)
        .await?;

    Result::Ok(Response::from_json(&res)?)
}

pub async fn update_writing(body: Writing, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    console_log!("{:?}", body);
    let d1 = ctx.env.d1(&ctx.env.var("db_name")?.to_string())?;

    let now = Date::now();

    d1.prepare("UPDATE writings SET title = ?, body = ?, updated = ? WHERE id = ?;")
        .bind(&[
            JsValue::from(&body.title.to_string()),
            JsValue::from(&body.body.to_string()),
            JsValue::from(now.to_string()),
            JsValue::from(&body.id.to_string()),
        ])?
        .run()
        .await?;

    let res = d1.prepare("SELECT * FROM writings WHERE id = ?;")
        .bind(&[
            JsValue::from(&body.id.to_string()),
        ])?
        .first::<Writing>(None)
        .await?;

    Result::Ok(Response::from_json(&res)?)
}