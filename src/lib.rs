mod works;
mod writings;
mod commons;

use worker::*;
use crate::commons::check_key;
use crate::works::get_works;
use crate::writings::{add_writings, get_writing, get_writings};

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let router = Router::new();

    router.get_async("/works",| _req, _ctx | async move {
        get_works().await
    })
        // Writing Endpoints
        .get_async("/writings",| _req, _ctx | async move {
            get_writings().await
        })
        .get_async("/writing/:id",| req, ctx | async move {
            if let Some(resp) = check_key(&req, &ctx)? {
                return Ok(resp);
            }

            let id = ctx.param("id").unwrap().to_string();

            return Ok(get_writing(id).await.expect("Id required"));
        })
        .post_async("/writing", |mut req, ctx | async move {
            if let Some(resp) = check_key(&req, &ctx)? {
                return Ok(resp);
            }

            let body = req.json().await?;

            return Ok(add_writings(body).await.expect("Body required"));
        })
        .run(req, env)
        .await
}