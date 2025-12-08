mod works;

use worker::*;
use crate::works::get_works;

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
        .get_async("/writings", |_req, _ctx | async move {

            Response::ok("ok")
        })
        .run(req, env)
        .await
}