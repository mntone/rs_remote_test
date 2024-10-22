use std::collections::HashMap;

use reqwest::{Client as ReqwestClient, Error, Response};

use super::{command::Command, res::Res, sign::Sign, token::Token};

pub struct Client {
    client: ReqwestClient,
    token: Token,
    domain: String,
}

impl Client {
    pub fn new(token: Token, domain: Option<&str>) -> Self {
        Self {
            client: ReqwestClient::new(),
            token,
            domain: match domain {
                Some(d) => d.to_string(),
                None => "api.switch-bot.com".to_string(),
            }
        }
    }

    pub async fn send(&self, device_id: &str, cmd: Command) -> Result<Response, Error> {
        let mut map = HashMap::new();
        map.insert("commandType", "command");
        map.insert("command",     &cmd.ctype);
        map.insert("parameter",   &cmd.param);
    
        let sign = Sign::new(&self.token);
        let request_url = format!("https://{}/v1.1/devices/{}/commands", self.domain, device_id);
        let res = self.client.post(request_url)
            .header(reqwest::header::AUTHORIZATION, &self.token.token)
            .header("charset", "utf8")
            .header("t", sign.timestamp.to_string())
            .header("sign", sign.sign)
            .header("nonce", sign.nonce)
            .json(&map)
            .send()
            .await;
        res
    }

    pub async fn send_command(&self, device_id: &str, cmd: Command) -> Result<Res, Error> {
        self.send(&device_id, cmd).await?.json().await
    }
}
