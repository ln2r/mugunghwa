use base64::prelude::BASE64_URL_SAFE_NO_PAD;
use base64::Engine;
use hmac::{Hmac, Mac};
use once_cell::sync::Lazy;
use regex::Regex;
use sha2::Sha256;
use worker::js_sys::Date;
use worker::{Request, Response, Result, RouteContext};

static NON_WORD: Lazy<Regex> = Lazy::new(|| Regex::new(r"\W").unwrap());
static CLEANUP: Lazy<Regex> = Lazy::new(|| Regex::new(r"-+").unwrap());

type HmacSha256 = Hmac<Sha256>;

pub static STRIP_IMAGE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"!\[\w+\s\w+].+(\.png|jpeg|jpg|gif|webp)\)\s").unwrap());

pub fn check_key(req: &Request, ctx: &RouteContext<()>) -> Result<Option<Response>> {
    let request_key = req.headers().get("x-mugunghwa-key")?.unwrap_or_default();
    let key = ctx.env.var("api_key")?.to_string();

    if request_key != key {
        return Ok(Some(Response::error("Unauthorized", 401)?));
    }

    Ok(None)
}

pub fn generate_slug(title: &str) -> String {
    let initial = NON_WORD.replace_all(title, "-");

    CLEANUP.replace_all(&initial, "-").into_owned()
}

pub fn return_response(res: Result<Response>) -> Result<Response> {
    let mut res = res?;
    let headers = res.headers_mut();

    headers.set("Access-Control-Allow-Origin", "*")?;
    headers.set("Access-Control-Allow-Methods", "GET, OPTIONS, POST, PATCH")?;
    headers.set(
        "Access-Control-Allow-Headers",
        "Content-Type, x-mugunghwa-key",
    )?;

    Ok(res)
}

pub fn b64(data: &[u8]) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(data)
}

pub fn sign_hs256(secret: &[u8], message: &[u8]) -> Vec<u8> {
    let mut mac = <HmacSha256 as Mac>::new_from_slice(secret).expect("Key");

    mac.update(message);

    mac.finalize().into_bytes().to_vec()
}

pub fn generate_snowflake(ctx: &RouteContext<()>) -> u64 {
    let epoch = 1735689600000u64;
    let now = Date::now() as u64;
    let machine_name = match ctx.env.var("name") {
        Ok(secret) => secret.to_string(),
        Err(_) => "default".to_string(),
    };
    let machine_id = fnv1a_hash64(&machine_name) & 0x3FF;
    // hardcoded to decrease the complexity
    let sequence = 0;

    // this is actually returning value and treated as so when there's no semicolon
    ((now - epoch) << 22) | (machine_id << 12) | sequence
}

fn fnv1a_hash64(input: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis (64-bit)
    let prime: u64 = 0x100000001b3; // FNV prime (64-bit)

    for byte in input.as_bytes() {
        hash ^= *byte as u64; // XOR with the byte
        hash = hash.wrapping_mul(prime); // multiply with overflow allowed
    }

    hash
}
