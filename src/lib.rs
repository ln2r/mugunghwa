mod works;
mod writings;
mod commons;

use worker::*;
use crate::commons::{check_key, handle_preflight};
use crate::works::get_works;
use crate::writings::{add_writings, get_writing, get_writing_by_slug, get_writings, update_writing};

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let router = Router::new();

    router
        .options("/*any", |_req, _ctx| handle_preflight())
        .get_async("/works",| _req, _ctx | async move {
            get_works().await
        })
        // Writing Endpoints
        .get_async("/writings",| _req, ctx | async move {
            get_writings(&ctx).await
        })
        .get_async("/writing/id/:id",| _req, ctx | async move {
            let id = ctx.param("id").unwrap().to_string();

            return Ok(get_writing(id, &ctx).await.expect("Id required"));
        })
        .get_async("/writing/:slug",| _req, ctx | async move {
            let slug = ctx.param("slug").unwrap().to_string();

            return Ok(get_writing_by_slug(slug, &ctx).await.expect("Slug required"));
        })
        .post_async("/writing", |mut req, ctx | async move {
            if let Some(resp) = check_key(&req, &ctx)? {
                return Ok(resp);
            }

            let body = req.json().await?;

            return Ok(add_writings(body, &ctx).await.expect("Body required"));
        })
        .patch_async("/writing", |mut req, ctx | async move {
            if let Some(resp) = check_key(&req, &ctx)? {
                return Ok(resp);
            }

            let body = req.json().await?;

            return Ok(update_writing(body, &ctx).await.expect("Body required"));
        })
        .run(req, env)
        .await
}
