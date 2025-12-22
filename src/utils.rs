use serde::{Serialize};
use serde_json::json;
use worker::{Response, RouteContext};

#[derive(Serialize, Debug)]
pub struct FileUpload {
    pub filename: String,
    pub mime: String,
    pub bytes: Vec<u8>,
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

pub async fn upload_file(payload: FileUpload, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let bucket = ctx.env.bucket(&ctx.env.var("bucket_binding")?.to_string())?;

    let upload = bucket.put(payload.filename, payload.bytes).execute().await?;

    // saving
    let base: &str = "/utils/file/";
    let key = upload.key();
    let res = json!({
       "key": key,
        "path": base.to_owned() + &*key,
    });

    Result::Ok(Response::from_json(&res)?)
}

pub async fn get_file(key: String, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let bucket = ctx.env.bucket(&ctx.env.var("bucket_binding")?.to_string())?;

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