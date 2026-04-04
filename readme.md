# mugunghwa
Experimental backend Rust-based Cloudflare worker for [ln2r.work](https://ln2r.work)

_**Notes: this project branch is under read-only state and was made as a learning purpose, feel free to fork or re-use the project.**_

# Requirements
GitHub Api Key - https://docs.github.com/en/rest

Cloudflare Worker - https://developers.cloudflare.com/workers/languages/rust/

Cloudflare D1 - https://developers.cloudflare.com/d1/

Cloudflare R2 - https://developers.cloudflare.com/r2/

Cloudflare KV - https://developers.cloudflare.com/kv/

# Deployment
For deployment please follow the [Cloudflare Worker deployment guides](https://developers.cloudflare.com/workers/get-started/guide/#2-develop-with-wrangler-cli), and fill up wrangler related env by using the `wrangler-example.toml`

# AI Assitance Disclosure
Below models/providers are used in the making of this project:

[ChatGPT](https://chatgpt.com/) - Code generation on hashing, CryptoRng and RngCore wrapper.

[Qwen](https://chat.qwen.ai/) - Api documentation.

# API Endpoints
**Base URL**: `https://<your-worker>.workers.dev`  
**CORS Headers** (included on all responses):
```
Access-Control-Allow-Origin: *
Access-Control-Allow-Methods: GET, OPTIONS, POST, PATCH
Access-Control-Allow-Headers: Content-Type, x-mugunghwa-key
```

---

## Authenticated Endpoints

### Authentication Mechanisms

| Mechanism | Header | Used For |
|-----------|--------|----------|
| **API Key** | `x-mugunghwa-key: <key>` | Write operations (create/update writings, upload files, register) |
| **JWT Bearer** | `Authorization: Bearer <token>` | User operations (logout) |

---

## Works

### `GET /works`

Fetch public GitHub repositories from `https://api.github.com/users/ln2r/repos`.

**Headers**: None required

**Query Params**: None

**Response** `200 OK`:
```json
[
  {
    "id": 123456,
    "name": "repo-name",
    "full_name": "ln2r/repo-name",
    "html_url": "https://github.com/ln2r/repo-name",
    "description": "...",
    "stargazers_count": 10,
    ...
  }
]
```

---

## Writings

### `GET /writings`

List all writings (body truncated to 150 chars, image markdown stripped).

**Headers**: None required

**Response** `200 OK`:
```json
[
  {
    "id": "snowflake-id",
    "title": "Writing Title",
    "slug": "writing-title",
    "hero": "https://...",
    "body": "Truncated body...",
    "created": "ISO timestamp",
    "updated": "ISO timestamp",
    "deleted": null
  }
]
```

---

### `GET /writing/id/:id`

Get a single writing by its ID.

**Path Params**:
| Param | Type | Required |
|-------|------|----------|
| `id` | string | Yes |

**Response** `200 OK`:
```json
{
  "id": "snowflake-id",
  "title": "Writing Title",
  "slug": "writing-title",
  "hero": "https://...",
  "body": "Full body content...",
  "created": "ISO timestamp",
  "updated": "ISO timestamp",
  "deleted": null
}
```
Returns `null` if not found.

---

### `GET /writing/:slug`

Get a single writing by its slug.

**Path Params**:
| Param | Type | Required |
|-------|------|----------|
| `slug` | string | Yes |

**Response**: Same as `GET /writing/id/:id`.

---

### `POST /writing`

Create a new writing.

**Headers**:
| Header | Value | Required |
|--------|-------|----------|
| `x-mugunghwa-key` | `<api-key>` | Yes |
| `Content-Type` | `application/json` | Yes |

**Request Body**:
```json
{
  "title": "Writing Title",
  "body": "Full markdown body...",
  "hero": "https://..." // optional
}
```

**Response** `200 OK`:
```json
{
  "id": "auto-generated-snowflake",
  "title": "Writing Title",
  "slug": "auto-generated-slug",
  "hero": "https://...",
  "body": "Full markdown body...",
  "created": "ISO timestamp",
  "updated": "ISO timestamp",
  "deleted": null
}
```

**Errors**:
- `401 Unauthorized` — Invalid or missing API key

---

### `PATCH /writing`

Update an existing writing.

**Headers**:
| Header | Value | Required |
|--------|-------|----------|
| `x-mugunghwa-key` | `<api-key>` | Yes |
| `Content-Type` | `application/json` | Yes |

**Request Body**:
```json
{
  "id": "writing-id",
  "title": "Updated Title",
  "body": "Updated body..." // optional
}
```

**Response** `200 OK`:
```json
{
  "id": "writing-id",
  "title": "Updated Title",
  "slug": "updated-slug",
  "hero": "https://...",
  "body": "Updated body...",
  "created": "ISO timestamp",
  "updated": "ISO timestamp",
  "deleted": null
}
```

**Errors**:
- `401 Unauthorized` — Invalid or missing API key

---

### `OPTIONS /writing`

CORS preflight.

**Response** `200 OK`: Empty body.

---

## Files

### `POST /utils/upload`

Upload a file to R2 storage.

**Headers**:
| Header | Value | Required |
|--------|-------|----------|
| `x-mugunghwa-key` | `<api-key>` | Yes |
| `Content-Type` | `multipart/form-data` | Yes |

**Request Body**: Multipart form with field `file`.

**Response** `200 OK`:
```json
{
  "key": "r2-object-key",
  "path": "/utils/file/r2-object-key"
}
```

**Errors**:
- `401 Unauthorized` — Invalid or missing API key

---

### `GET /utils/file/:key`

Retrieve a file by its R2 key.

**Path Params**:
| Param | Type | Required |
|-------|------|----------|
| `key` | string | Yes |

**Response** `200 OK`: Raw file binary with appropriate `Content-Type` header.

**Errors**:
- `404 Not Found` — Key does not exist

---

### `GET /utils/files`

List all files with optional search.

**Headers**: None required

**Query Params**:
| Param | Type | Required | Description |
|-------|------|----------|-------------|
| `search` | string | No | Filters by `key LIKE %search%` |

**Response** `200 OK`:
```json
[
  {
    "id": "snowflake-id",
    "key": "r2-object-key",
    "created": "ISO timestamp",
    "updated": "ISO timestamp",
    "deleted": null
  }
]
```

---

## Auth

### `POST /auth/register`

Register a new user. Password is hashed with Argon2 before storage.

**Headers**:
| Header | Value | Required |
|--------|-------|----------|
| `x-mugunghwa-key` | `<api-key>` | Yes |
| `Content-Type` | `application/json` | Yes |

**Request Body**:
```json
{
  "username": "string",
  "password": "string"
}
```

**Response** `200 OK`:
```json
{
  "id": "snowflake-id",
  "username": "string",
  "created": "ISO timestamp",
  "updated": "ISO timestamp",
  "deleted": null
}
```

**Errors**:
- `401 Unauthorized` — Invalid or missing API key

---

### `POST /auth/login`

Login — returns JWT + refresh token.

**Headers**:
| Header | Value | Required |
|--------|-------|----------|
| `Content-Type` | `application/json` | Yes |

**Request Body**:
```json
{
  "username": "string",
  "password": "string"
}
```

**Response** `200 OK`:
```json
{
  "token": "jwt-hs256-token",
  "refresh": "snowflake-refresh-id"
}
```

**Errors**:
- `404 Not Found` — User not found
- `401 Unauthorized` — Invalid credentials
- `500 Internal Server Error` — Invalid password hash in DB

---

### `POST /auth/logout`

Logout — invalidates the refresh token from KV storage.

**Headers**:
| Header | Value | Required |
|--------|-------|----------|
| `Authorization` | `Bearer <jwt-token>` | Yes |
| `Content-Type` | `application/json` | Yes |

**Request Body**:
```json
{
  "refresh": "refresh-token-string"
}
```

**Response** `200 OK`:
```json
{
  "user_id": "snowflake-id"
}
```

**Errors**:
- `401 Unauthorized` — Token missing or invalid
- `404 Not Found` — User not found

---

### `POST /auth/refresh`

Refresh an expired token pair.

**Headers**:
| Header | Value | Required |
|--------|-------|----------|
| `Content-Type` | `application/json` | Yes |

**Request Body**:
```json
{
  "token": "valid-refresh-token"
}
```

**Response** `200 OK`:
```json
{
  "token": "new-jwt-token",
  "refresh": "new-refresh-token"
}
```

**Errors**:
- `401 Unauthorized` — Refresh token expired or not found

---

## Summary Table

| # | Method | Path | Auth | Description |
|---|--------|------|------|-------------|
| 1 | `GET` | `/works` | None | Fetch public GitHub repos |
| 2 | `GET` | `/writings` | None | List all writings (body truncated) |
| 3 | `GET` | `/writing/id/:id` | None | Get writing by ID |
| 4 | `GET` | `/writing/:slug` | None | Get writing by slug |
| 5 | `POST` | `/writing` | API Key | Create a new writing |
| 6 | `PATCH` | `/writing` | API Key | Update an existing writing |
| 7 | `OPTIONS` | `/writing` | None | CORS preflight |
| 8 | `POST` | `/utils/upload` | API Key | Upload a file (multipart) |
| 9 | `GET` | `/utils/file/:key` | None | Retrieve file by R2 key |
| 10 | `GET` | `/utils/files` | None | List files (optional search) |
| 11 | `POST` | `/auth/register` | API Key | Register a new user |
| 12 | `POST` | `/auth/login` | None | Login, returns JWT + refresh token |
| 13 | `POST` | `/auth/logout` | JWT Bearer | Logout, invalidates refresh token |
| 14 | `POST` | `/auth/refresh` | None | Refresh an expired token pair |
