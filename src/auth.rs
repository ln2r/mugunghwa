use std::ops::Add;

use serde::{Deserialize, Serialize};
use worker::Date;

use crate::commons::{b64, sign_hs256};

#[derive(Serialize, Deserialize, Debug)]
struct JwtBody {
    sub: String,
    iat: u64,
    exp: u64,
}

fn sign() -> String {
    let now = (Date::now().as_millis() / 1000) as u64;
    let header = r#"{"alg":"HS256","typ":"JWT"}"#;
    let payload = JwtBody {
        sub: "ae9b3582-0169-4241-99cf-c635fcabda2e".into(),
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
