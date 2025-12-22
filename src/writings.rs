use serde::{Deserialize, Serialize};
use worker::{Response, RouteContext};
use worker::wasm_bindgen::JsValue;
use crate::commons::{generate_slug, generate_snowflake, STRIP_IMAGE};

#[derive(Serialize, Deserialize, Debug)]
pub struct Writing {
    pub id: Option<String>,
    pub title: String,
    pub slug: Option<String>,
    pub hero: Option<String>,
    pub body: String,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub deleted: Option<String>,
}

pub async fn get_writings(ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_binding")?.to_string())?;

    let res = d1.prepare("SELECT * FROM writings WHERE deleted IS NULL;")
        .all()
        .await?;

    let writings: Vec<Writing> = res.results()?;
    let formatted: Vec<Writing> = writings.into_iter().map(|w| {
        let sanitized = STRIP_IMAGE.replace_all(&w.body, "");

        Writing {
            body: sanitized.chars().take(150).collect::<String>(),
            ..w
        }
    }).collect();

    Result::Ok(Response::from_json(&formatted)?)
}

pub async fn get_writing(id: String, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_binding")?.to_string())?;

    let res = d1.prepare("SELECT * FROM writings WHERE id = ? AND deleted is NULL;")
        .bind(&[
            JsValue::from(&id.to_string()),
        ])?
        .first::<Writing>(None)
        .await?;

    Result::Ok(Response::from_json(&res)?)
}

pub async fn get_writing_by_slug(slug: String, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_binding")?.to_string())?;

    let res = d1.prepare("SELECT * FROM writings WHERE slug = ? AND deleted is NULL;")
        .bind(&[
            JsValue::from(&slug.to_string()),
        ])?
        .first::<Writing>(None)
        .await?;

    Result::Ok(Response::from_json(&res)?)
}

pub async fn add_writings(body: Writing, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_binding")?.to_string())?;

    let id = generate_snowflake(ctx);
    let slug = generate_slug(&body.title);
    let hero = body.hero.clone();

    d1.prepare("INSERT INTO writings (id, title, slug, hero, body, created, updated) VALUES (?, ?, ?, ?, ?, strftime('%Y-%m-%dT%H:%M:%fZ','now'), strftime('%Y-%m-%dT%H:%M:%fZ','now'));")
        .bind(&[
            JsValue::from(&id.to_string()),
            JsValue::from(&body.title.to_string()),
            JsValue::from(&slug.to_string()),
            match hero {
                Some(ref s) => JsValue::from(s), // handling null
                None => JsValue::NULL,
            },
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
    let d1 = ctx.env.d1(&ctx.env.var("db_binding")?.to_string())?;
    let id = body.id.clone().expect("Id required");
    let slug = generate_slug(&body.title);

    d1.prepare("UPDATE writings SET title = ?, slug = ?, body = ?, updated = strftime('%Y-%m-%dT%H:%M:%fZ','now') WHERE id = ?;")
        .bind(&[
            JsValue::from(&body.title.to_string()),
            JsValue::from(&slug.to_string()),
            JsValue::from(&body.body.to_string()),
            JsValue::from(&id.to_string()),
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