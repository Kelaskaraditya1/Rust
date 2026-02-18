/*

JWT Authentication:

🔐 JWT (JSON Web Token) – Quick Revision Notes

1️⃣ Structure
JWT has 3 parts separated by dots:
header.payload.signature

2️⃣ Header
Example:
{
  "alg": "HS256",
  "typ": "JWT"
}
• Contains algorithm info
• Base64URL encoded

3️⃣ Payload (Claims)
Example:
{
  "sub": "user_id",
  "exp": 1712345678
}
• Contains data (claims)
• Base64URL encoded
• NOT encrypted (anyone can decode it)
• Do NOT store sensitive data like passwords

Common claims:
• sub → subject (user id)
• exp → expiration time
• iat → issued at
• iss → issuer
• aud → audience

4️⃣ Signature (Core Security Part)

For HS256:

signature = Base64Url(
                HMAC_SHA256(
                    Base64Url(header) + "." + Base64Url(payload),
                    secret_key
                )
            )

• Uses secret key
• Ensures integrity
• Detects tampering
• It is signing, NOT encryption

5️⃣ How Verification Works (HS256)

When server receives token:
1. Split into header, payload, signature
2. Recompute signature using SAME secret key
3. Compare signatures
4. If match → token valid
5. Then check claims like exp

If signature does not match → token invalid.



*/

use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use crate::users::keys;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // subject (user email)
    pub iat: usize,  // issued at
    pub exp: usize,  // expiration time
}

pub fn encode_jwt(email: String) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expiration = Duration::hours(7 * 24);

    let secret_key = (*keys::keys::JWT_SECRET).clone();

    let claims = Claims {
        sub: email,
        iat: now.timestamp() as usize,
        exp: (now + expiration).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
    let secret_key = (*keys::keys::JWT_SECRET).clone();

    decode(
        &jwt_token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)
}
