use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::gslt::{
    encode,
    GsltData,
    GsltManager,
    IGameServersService
};

#[derive(Debug, Serialize, Deserialize)]
pub struct GsltCredentialResponse {
    response: GsltCredentialData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GsltCredentialData {
    steamid: String,
    login_token: String
}

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

impl IGameServersService for GsltManager {
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

    fn create_account(&self, app_id: u32, memo: impl Into<String>) -> Result<GsltCredentialResponse> {
        let steam_web_api_key = &self.steam_web_api_key;
        let app_id = app_id.to_string();
        let memo = memo.into();
        let request_url = format!("https://api.steampowered.com/IGameServersService/CreateAccount/v1/?key={}&appid={}&memo={}", encode(&steam_web_api_key), encode(&app_id), encode(&memo));

        let response = ureq::post(&request_url)
                            .timeout_connect(10_000)
                            .set("Content-Length", "0")
                            .call();

        if response.status() != 200 { bail!("status code is not 200."); }
    
        let body = response.into_string()?;
    
        let gslt: GsltCredentialResponse = serde_json::from_str(&body).with_context(|| format!("Failed parse json."))?;

        Ok(gslt)
    }

    fn delete_account(&self, steam_id: u64) -> Result<()> {
        let steam_web_api_key = &self.steam_web_api_key;
        let steam_id = steam_id.to_string();
        let request_url = format!("https://api.steampowered.com/IGameServersService/DeleteAccount/v1/?key={}&steamid={}", encode(&steam_web_api_key), encode(&steam_id));

        let response = ureq::post(&request_url)
                            .timeout_connect(10_000)
                            .set("Content-Length", "0")
                            .call();
                            
        if response.status() != 200 { bail!("status code is not 200."); }
    
        Ok(())
    }

    fn setmemo_gslt(&self, steam_id: u64, memo: impl Into<String>) -> Result<()> {
        let steam_web_api_key = &self.steam_web_api_key;
        let steam_id = steam_id.to_string();
        let memo = memo.into();
        let request_url = format!("https://api.steampowered.com/IGameServersService/SetMemo/v1/?key={}&steamid={}&memo={}", encode(&steam_web_api_key), encode(&steam_id), encode(&memo));

        let response = ureq::post(&request_url)
                            .timeout_connect(10_000)
                            .set("Content-Length", "0")
                            .call();
                            
        if response.status() != 200 { bail!("status code is not 200."); }
    
        Ok(())
    }
}
