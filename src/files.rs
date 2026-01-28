use crate::commons::generate_snowflake;
use serde::{Deserialize, Serialize};
use serde_json::json;
use worker::wasm_bindgen::JsValue;
use worker::{Response, RouteContext};

#[derive(Serialize, Debug)]
pub struct FileUpload {
    pub filename: String,
    pub mime: String,
    pub bytes: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub id: String,
    pub key: String,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub deleted: Option<String>,
}

impl FileUpload {
    pub async fn from_file(file: worker::File) -> Result<Self, worker::Error> {
        Ok(Self {
            filename: file.name(),
            mime: file.type_(),
            bytes: file.bytes().await?,
        })
    }
}

pub async fn upload_file(
    payload: FileUpload,
    ctx: &RouteContext<()>,
) -> Result<Response, worker::Error> {
    let bucket = ctx
        .env
        .bucket(&ctx.env.var("bucket_binding")?.to_string())?;
    let d1 = ctx.env.d1(&ctx.env.var("db_binding")?.to_string())?;
    let id = generate_snowflake(ctx);

    let upload = bucket
        .put(payload.filename, payload.bytes)
        .execute()
        .await?;

    d1.prepare(
        "INSERT INTO files(id, key, created, updated) VALUES (?, ?, strftime('%Y-%m-%dT%H:%M:%fZ','now'), strftime('%Y-%m-%dT%H:%M:%fZ','now'));"
    ).bind(&[
        JsValue::from(&id.to_string()),
        JsValue::from(&upload.key().to_string()),
    ])?
    .run()
    .await?;

    // saving
    let base: &str = "/utils/file/";
    let key = upload.key();
    let res = json!({
       "key": key,
        "path": base.to_owned() + &*key,
    });

    Response::from_json(&res)
}

pub async fn get_file(key: String, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let bucket = ctx
        .env
        .bucket(&ctx.env.var("bucket_binding")?.to_string())?;

    let Some(object) = bucket.get(key).execute().await? else {
        return Response::error("Not found", 404);
    };

    let Some(body) = object.body() else {
        return Response::error("Not found", 404);
    };

    let stream = body.stream()?;
    let mut res = Response::from_stream(stream)?;
    let meta = object.http_metadata();

    if let Some(ct) = meta.content_type {
        res.headers_mut().set("Content-Type", &ct)?;
    }

    Ok(res)
}

pub async fn get_files(ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_binding")?.to_string())?;
    let res = d1
        .prepare("SELECT * FROM files WHERE deleted IS NULL;")
        .all()
        .await?;

    let files: Vec<File> = res.results()?;
    let formatted: Vec<File> = files.into_iter().map(|f| File { ..f }).collect();

    Response::from_json(&formatted)
}
