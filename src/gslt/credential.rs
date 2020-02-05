use anyhow::*;
use serde::{Deserialize, Serialize};

use crate::gslt::GsltManager;
use crate::gslt::GsltCredential;
use crate::gslt::encode;

#[derive(Debug, Serialize, Deserialize)]
pub struct GsltCredentialResponse {
    response: GsltCredentialData
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GsltCredentialData {
    steamid: String,
    login_token: String
}

impl GsltCredential for GsltManager {
    fn create_gslt(&self, app_id: u32, memo: impl Into<String>) -> Result<GsltCredentialResponse> {
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
}
