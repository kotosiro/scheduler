use anyhow::Context;
use anyhow::Result;
use jsonwebtoken::Algorithm;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use jsonwebtoken::Header;
use jsonwebtoken::TokenData;
use jsonwebtoken::Validation;
use std::fs;
use std::time::Duration;
use std::time::SystemTime;
use tracing::debug;
use tracing::trace;

const KOTOSIRO_ISSUER: &str = "kotosiro";

const KOTOSIRO_STASH_AUDIENCE: &str = "kotosiro.stash";

const KOTOSIRO_CONFIG_AUDIENCE: &str = "kotosiro.config";

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    iss: String,
    sub: String,
    aud: String,
    exp: u64,
}

pub struct Keys {
    algorithm: Algorithm,
    decoding: DecodingKey,
    encoding: EncodingKey,
}

fn generate(keys: &Keys, aud: String, sub: String) -> Result<String> {
    trace!(r#"generating jwt for aud="{}" sub="{}""#, aud, sub);
    let header = Header::new(keys.algorithm);
    let claims = Claims {
        iss: KOTOSIRO_ISSUER.to_owned(),
        sub,
        aud,
        exp: (SystemTime::now() + Duration::from_secs(5 * 60))
            .duration_since(SystemTime::UNIX_EPOCH)
            .context("failed to create expiration time")?
            .as_secs(),
    };
    let token = jsonwebtoken::encode(&header, &claims, &keys.encoding)
        .context("failed to encode jwt token")?;
    Ok(token)
}

pub fn stash(keys: &Keys, id: &str) -> Result<String> {
    generate(keys, KOTOSIRO_STASH_AUDIENCE.to_owned(), id.to_owned())
}

pub fn config(keys: &Keys, id: &str) -> Result<String> {
    generate(keys, KOTOSIRO_CONFIG_AUDIENCE.to_owned(), id.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_generate() {}
}
