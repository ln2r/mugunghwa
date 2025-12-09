use worker::{Request, Response, RouteContext, Result, console_log};

pub fn check_key(req: &Request, ctx: &RouteContext<()>) -> Result<Option<Response>> {
    let request_key = req.headers().get("x-mugunghwa-key")?.unwrap_or_default();
    let key = ctx.env.var("api_key")?.to_string();

    if request_key != key {
        return Ok(Some(Response::error("Unauthorized", 401)?));
    }

    Ok(None)
}