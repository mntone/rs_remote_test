pub mod api;

use std::env;

use api::switchbot::{client::Client, command::Command, token::Token};
use dotenvy::dotenv;
use windows::{
    core::{w, PCWSTR},
    Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_OK},
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let token = match (env::var("TOKEN"), env::var("SECRET")) {
        (Ok(token), Ok(secret)) => Token::new(&token, &secret),
        _ => panic!("Can't get token and secret")
    };
    let client = Client::new(token, None);
    let Ok(device_id) = env::var("DEVICE_ID") else { return };
    let command = Command::new("brightnessUp", "default");
    let Ok(res) = client.send_command(&device_id, command).await else { return };
    
    let mut win32text: Vec<u16> = res.message.encode_utf16().collect();
    win32text.push(0);
    unsafe {
        let message = PCWSTR::from_raw(win32text.as_ptr());
        MessageBoxW(None, message, w!("Caption"), MB_OK);
    }
}
