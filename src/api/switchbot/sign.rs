use std::time::SystemTime;

use base64::{engine::general_purpose::STANDARD, Engine};
use random_string::{charsets::ALPHANUMERIC, generate};
use rs_hmac::Hmac;
use rs_sha256::Sha256State;

use super::token::Token;

pub struct Sign {
    pub nonce: String,
    pub sign: String,
    pub timestamp: u128,
}

type HmacSha256 = Hmac::<Sha256State, 32>;

impl Sign {
    pub fn new(token: &Token) -> Self {
        let timestamp = if let Ok(n) = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            n.as_millis()
        } else {
            panic!("Can't get unixtime")
        };

        let nonce: String = generate(32, ALPHANUMERIC);
        let message = format!("{}{}{}", token.token, timestamp.to_string(), nonce);
        let hmac = HmacSha256::digest(token.secret.as_bytes(), message.as_bytes());
        let sign = STANDARD.encode(hmac);
        Self {
            nonce,
            sign,
            timestamp,
        }
    }
}
