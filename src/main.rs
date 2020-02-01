use anyhow::*;
use serde::{Deserialize, Serialize};
use url::form_urlencoded;

#[derive(Debug, Serialize, Deserialize)]
struct GsltCredentialResponse {
    response: GsltCredentialData
}

#[derive(Debug, Serialize, Deserialize)]
struct GsltCredentialData {
    steamid: String,
    login_token: String
}

#[derive(Debug, Serialize, Deserialize)]
struct GsltListResponse {
    response: GsltListResponseData
}

#[derive(Debug, Serialize, Deserialize)]
struct GsltListResponseData {
    servers: Vec<GsltData>,
	is_banned:bool,
	expires:u32,
	actor:String,
	last_action_time:u32
}

#[derive(Debug, Serialize, Deserialize)]
struct GsltData  {
	steamid:String,
	appid: i16,
	login_token: String,
	memo:String,
	is_deleted: bool,
	is_expired: bool,
	rt_last_logon: u32
}

fn main() -> Result<()> {
    let list = get_list("STEAM_WEB_API_KEY")?;
    println!("list: {:?}", list);

    let gslt = create_gslt("STEAM_WEB_API_KEY", 730, "MEMO")?;
    println!("gslt: {:?}", gslt);

    Ok(())
}

fn get_list(steam_web_api_key: impl Into<String>) -> Result<GsltListResponse> {
    let steam_web_api_key = steam_web_api_key.into();
    let request_url = format!("https://api.steampowered.com/IGameServersService/GetAccountList/v1/?key={}", encode(&steam_web_api_key));

    let response = ureq::get(&request_url)
                        .timeout_connect(10_000)
                        .call();

    if response.status() != 200 { anyhow!("status code is not 200."); }

    let body = response.into_string()?;

    let list: GsltListResponse = serde_json::from_str(&body).with_context(|| format!("Failed parse json."))?;

    Ok(list)
}

fn create_gslt(steam_web_api_key: impl Into<String>, app_id: u32, memo: impl Into<String>) -> Result<GsltCredentialResponse> {
    let steam_web_api_key = steam_web_api_key.into();
    let app_id = app_id.to_string();
    let memo = memo.into();
    let request_url = format!("https://api.steampowered.com/IGameServersService/CreateAccount/v1/?key={}&appid={}&memo={}", encode(&steam_web_api_key), encode(&app_id), encode(&memo));

    let response = ureq::post(&request_url)
                        .timeout_connect(10_000)
                        .set("Content-Length", "0")
                        .call();

    if response.status() != 200 { anyhow!("status code is not 200."); }

    let body = response.into_string()?;

    let gslt: GsltCredentialResponse = serde_json::from_str(&body).with_context(|| format!("Failed parse json."))?;

    Ok(gslt)
}

fn encode(s: &str) -> String {
    form_urlencoded::byte_serialize(s.as_bytes()).collect::<String>()
}
