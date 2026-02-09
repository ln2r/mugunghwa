use std::ops::Add;

use crate::{
    commons::{b64, generate_snowflake, sign_hs256},
    rng::GetRandomWrapper,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use serde::{Deserialize, Serialize};
use worker::{wasm_bindgen::JsValue, Date, Response, RouteContext};

#[derive(Serialize, Deserialize, Debug)]
struct JwtBody {
    sub: String,
    iat: u64,
    exp: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    token: String,
    refresh: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Logout {
    user_id: String,
    refresh: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub created: Option<String>,
    pub updated: Option<String>,
    pub deleted: Option<String>,
}

pub async fn register(user: AuthUser, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_binding")?.to_string())?;
    let id = generate_snowflake(ctx);

    // TODO: add check user exist

    let raw_password = user.password.as_bytes();
    let mut rng = GetRandomWrapper;
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut rng);
    let hashed_password = argon2
        .hash_password(raw_password, &salt)
        .map_err(|e| worker::Error::from(e.to_string()))?
        .to_string();

    d1.prepare("INSERT INTO users(id, username, password, created, updated) VALUES (?, ?, ?, strftime('%Y-%m-%dT%H:%M:%fZ','now'), strftime('%Y-%m-%dT%H:%M:%fZ','now'))").bind(&[
        JsValue::from(&id.to_string()),
        JsValue::from(&user.username.to_string()),
        JsValue::from(&hashed_password.to_string()),
    ])?
    .run()
    .await?;

    let res = d1
        .prepare("SELECT id, username, created, updated, deleted FROM users WHERE id = ?;")
        .bind(&[JsValue::from(&id.to_string())])?
        .first::<User>(None)
        .await?;

    Response::from_json(&res)
}

pub async fn login(login: AuthUser, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_binding")?.to_string())?;
    let kv = ctx.env.kv(&ctx.env.var("kv_binding")?.to_string())?;
    let argon2 = Argon2::default();

    let db = d1
        .prepare("SELECT * FROM users WHERE username = ? AND deleted is NULL;")
        .bind(&[JsValue::from(&login.username.to_string())])?
        .first::<User>(None)
        .await?;

    let user = match db {
        Some(user) => user,
        None => {
            return Ok(Response::error("User not found", 404)?);
        }
    };

    let user_password = match PasswordHash::new(&user.password) {
        Ok(h) => h,
        Err(_) => {
            return Ok(Response::error("Invalid password hash", 500)?);
        }
    };

    // verifying
    if argon2
        .verify_password(login.password.as_bytes(), &user_password)
        .is_err()
    {
        return Ok(Response::error("Invalid credentials", 401)?);
    }

    // refresh token
    let refesh_token = generate_snowflake(ctx).to_string();

    kv.put(&refesh_token, &refesh_token)?
        .expiration_ttl(1800)
        .execute()
        .await?;

    let res = Auth {
        token: sign(user.id),
        refresh: refesh_token,
    };

    Response::from_json(&res)
}

pub async fn logout(payload: Logout, ctx: &RouteContext<()>) -> Result<Response, worker::Error> {
    let d1 = ctx.env.d1(&ctx.env.var("db_binding")?.to_string())?;
    let kv = ctx.env.kv(&ctx.env.var("kv_binding")?.to_string())?;

    let db = d1
        .prepare("SELECT * FROM users WHERE id = ? AND deleted is NULL;")
        .bind(&[JsValue::from(&payload.user_id.to_string())])?
        .first::<User>(None)
        .await?;

    let user = match db {
        Some(user) => user,
        None => {
            return Ok(Response::error("User not found", 404)?);
        }
    };

    kv.delete(&payload.refresh).await?;

    Response::from_json(&user)
}

fn sign(user_id: String) -> String {
    let now = (Date::now().as_millis() / 1000) as u64;
    let header = r#"{"alg":"HS256","typ":"JWT"}"#;
    let payload = JwtBody {
        sub: user_id.into(),
        iat: now,
        exp: now.add(900),
    };

    let header_b64 = b64(header.as_bytes());
    let payload_b64 = b64(&serde_json::to_vec(&payload).unwrap());

    let sign_payload = format!("{}.{}", header_b64, payload_b64);

    let signature = sign_hs256(b"USE_ENV_ON_LIVE_PLS", sign_payload.as_bytes());

    let jwt = format!("{}.{}.{}", header_b64, payload_b64, b64(&signature));

    jwt
}
