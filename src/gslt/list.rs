use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::gslt::GsltManager;
use crate::gslt::GsltData;
use crate::gslt::GsltList;
use crate::gslt::encode;

#[derive(Debug, Serialize, Deserialize)]
pub struct GsltListResponse {
    response: GsltListResponseData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GsltListResponseData {
    servers: Vec<GsltData>,
    is_banned: bool,
    expires: u32,
    actor: String,
    last_action_time: u32
}

impl GsltList for GsltManager {
    fn get_list(&self) -> Result<GsltListResponse> {
        let steam_web_api_key = &self.steam_web_api_key;
        let request_url = format!("https://api.steampowered.com/IGameServersService/GetAccountList/v1/?key={}", encode(&steam_web_api_key));

        let response = ureq::get(&request_url)
                            .timeout_connect(10_000)
                            .call();

        if response.status() != 200 { bail!("status code is not 200."); }

        let body = response.into_string()?;
        let list: GsltListResponse = serde_json::from_str(&body).with_context(|| format!("Failed parse json."))?;

        Ok(list)
    }
}
