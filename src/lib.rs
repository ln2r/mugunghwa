mod auth;
mod commons;
mod files;
mod rng;
mod works;
mod writings;

use std::collections::HashMap;

use crate::auth::{check_api_key, login, logout, register};
use crate::commons::return_response;
use crate::files::{get_file, get_files, upload_file, FileUpload};
use crate::works::get_works;
use crate::writings::{
    add_writings, get_writing, get_writing_by_slug, get_writings, update_writing,
};
use worker::*;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get_async("/works", |_req, _ctx| async move {
            return_response(get_works().await)
        })
        // Writing Endpoints
        .get_async("/writings", |_req, ctx| async move {
            return_response(get_writings(&ctx).await)
        })
        .get_async("/writing/id/:id", |_req, ctx| async move {
            let id = ctx.param("id").unwrap().to_string();

            return_response(Ok(get_writing(id, &ctx).await.expect("Id required")))
        })
        .get_async("/writing/:slug", |_req, ctx| async move {
            let slug = ctx.param("slug").unwrap().to_string();

            return_response(Ok(get_writing_by_slug(slug, &ctx)
                .await
                .expect("Slug required")))
        })
        .post_async("/writing", |mut req, ctx| async move {
            if let Some(resp) = check_api_key(&req, &ctx)? {
                return Ok(resp);
            }

            let body = req.json().await?;

            return_response(Ok(add_writings(body, &ctx).await.expect("Body required")))
        })
        .patch_async("/writing", |mut req, ctx| async move {
            if let Some(resp) = check_api_key(&req, &ctx)? {
                return Ok(resp);
            }

            let body = req.json().await?;

            return_response(Ok(update_writing(body, &ctx).await.expect("Body required")))
        })
        .options_async("/writing", |_req, _ctx| async move {
            return_response(Ok(Response::empty()?))
        })
        .post_async("/utils/upload", |mut req, ctx| async move {
            if let Some(resp) = check_api_key(&req, &ctx)? {
                return Ok(resp);
            }

            let form = req.form_data().await?;
            let file = match form.get("file") {
                Some(FormEntry::File(file)) => file,
                _ => return Response::error(" file required", 400),
            };

            let payload = FileUpload::from_file(file).await?;

            return_response(Ok(upload_file(payload, &ctx).await.expect("REASON")))
        })
        .get_async("/utils/file/:key", |_req, ctx| async move {
            let key = ctx.param("key").unwrap().to_string();

            return_response(Ok(get_file(key, &ctx).await.expect("key required")))
        })
        .get_async("/utils/files", |req, ctx| async move {
            let params: HashMap<_, _> = req.url()?.query_pairs().into_owned().collect();
            let search = params.get("search").cloned();

            return_response(get_files(&ctx, search).await)
        })
        .post_async("/auth/register", |mut req, ctx| async move {
            if let Some(resp) = check_api_key(&req, &ctx)? {
                return Ok(resp);
            }

            let body = req.json().await?;

            return_response(Ok(register(body, &ctx).await.expect("Body required")))
        })
        .post_async("/auth/login", |mut req, ctx| async move {
            let body = req.json().await?;

            return_response(Ok(login(body, &ctx).await.expect("Body required")))
        })
        .post_async("/auth/logout", |mut req, ctx| async move {
            if let Some(resp) = check_api_key(&req, &ctx)? {
                return Ok(resp);
            }

            let body = req.json().await?;

            return_response(Ok(logout(body, &ctx).await.expect("Body required")))
        })
        .run(req, env)
        .await
}
