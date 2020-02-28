use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

pub mod i_game_servers_service;

pub struct GsltManager {
    pub steam_web_api_key: String,
    pub servers: Vec<GsltData>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GsltData  {
    steamid: String,
    appid: i16,
    login_token: String,
    memo: String,
    is_deleted: bool,
    is_expired: bool,
    rt_last_logon: u32
}

impl GsltManager {
    pub fn new(steam_web_api_key: impl Into<String>) -> GsltManager {
        GsltManager {
            steam_web_api_key: steam_web_api_key.into(),
            servers: Vec::new()
        }
    }
}

use i_game_servers_service::GsltListResponse;
use i_game_servers_service::GsltCredentialResponse;

pub trait IGameServersService {
    fn get_list(&self) -> Result<GsltListResponse>;
    fn create_account(&self, app_id: u32, memo: impl Into<String>) -> Result<GsltCredentialResponse>;
    fn setmemo_gslt(&self, steam_id: u64, memo: impl Into<String>) -> Result<()>;
    fn delete_account(&self, steam_id: u64) -> Result<()>;
}

pub(crate) fn encode(s: &str) -> String {
    form_urlencoded::byte_serialize(s.as_bytes()).collect::<String>()
}
