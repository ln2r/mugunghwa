use worker::{Fetch, Headers, Method, Request, RequestInit, Response};

pub async fn get_works() -> Result<Response, worker::Error> {
    let url = "https://api.github.com/users/ln2r/repos";

    // request initialization
    let mut init = RequestInit::new();
    init.with_method(Method::Get);

    let headers = Headers::new();
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
        .filter(|repo| repo["visibility"] == "public" && repo["full_name"] != "ln2r/ln2r")
        .collect();

    Result::Ok(Response::from_json(&public)?)
}
