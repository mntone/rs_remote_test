pub struct Token {
    pub token: String,
    pub secret: String,
}

impl Token {
    pub fn new(token: &str, secret: &str) -> Self {
        Self {
            token: token.to_string(),
            secret: secret.to_string(),
        }
    }
}
