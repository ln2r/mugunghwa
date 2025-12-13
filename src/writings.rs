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

pub async fn get_writings(ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_name")?.to_string())?;

    let res = d1.prepare("SELECT * FROM writings WHERE deleted IS NULL;")
        .all()
        .await?;

    let writings: Vec<Writing> = res.results()?;

    Result::Ok(Response::from_json(&writings)?)
}

pub async fn get_writing(id: String, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_name")?.to_string())?;

    let res = d1.prepare("SELECT * FROM writings WHERE id = ? AND deleted IS NULL;")
        .bind(&[
            JsValue::from(&id.to_string()),
        ])?
        .first::<Writing>(None)
        .await?;

    Result::Ok(Response::from_json(&res)?)
}

pub async fn add_writings(body: Writing, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_name")?.to_string())?;

    let id = generate_snowflake(ctx);

    d1.prepare("INSERT INTO writings (id, title, body, created, updated) VALUES (?, ?, ?, strftime('%Y-%m-%dT%H:%M:%fZ','now'), strftime('%Y-%m-%dT%H:%M:%fZ','now'));")
        .bind(&[
            JsValue::from(&id.to_string()),
            JsValue::from(&body.title.to_string()),
            JsValue::from(&body.body.to_string()),
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

    d1.prepare("UPDATE writings SET title = ?, body = ?, updated = strftime('%Y-%m-%dT%H:%M:%fZ','now') WHERE id = ?;")
        .bind(&[
            JsValue::from(&body.title.to_string()),
            JsValue::from(&body.body.to_string()),
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