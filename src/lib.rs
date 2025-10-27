use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Serialize, Deserialize, Debug)]
struct Work {
    id: String,
    name: String,
    full_name: String,
    html_url: String,
    description: String,
    fork: String,
    topics: Vec<String>,
    archived: String,
    created_at: String,
    updated_at: String,
}

#[event(fetch)]
async fn fetch(
    req: Request,
    env: Env,
    _ctx: Context,
) -> Result<Response> {
    let router = Router::new();

    router.get_async("/works",| req, ctx | async move {
        let url = "https://api.github.com/users/ln2r/repos";

        // request initialization
        let mut init = RequestInit::new();
        init.with_method(Method::Get);

        // setting headers
        let mut headers = Headers::new();
        headers.set("Accept", "application/vnd.github.v3+json")?;
        headers.set("User-Agent", "mugunghwa-cfw")?;
        init.with_headers(headers);

        let mut res = Fetch::Request(Request::new_with_init(url, &init)?)
            .send()
            .await?;

        let data = res.json::<serde_json::Value>().await?;
        let empty = vec![];
        let public: Vec<_> = data
            .as_array()
            .unwrap_or(&empty)
            .iter()
            .filter(|repo| repo["visibility"] == "public")
            .collect();

        Response::from_json(&public)
    })
        .run(req, env)
        .await
}