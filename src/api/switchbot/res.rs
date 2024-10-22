use serde::Deserialize;

#[derive(Deserialize)]
pub struct EmptyRes {
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Res<T = EmptyRes> {
    pub status_code: Option<u16>,
    pub body: Option<T>,
    pub message: String,
}
