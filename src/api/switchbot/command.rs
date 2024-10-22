use serde::Serialize;

#[derive(Serialize)]
pub struct Command {
    pub ctype: String,
    pub param: String,
}

impl Command {
    pub fn new(ctype: &str, param: &str) -> Self {
        Self {
            ctype: ctype.to_string(),
            param: param.to_string(),
        }
    }
}
