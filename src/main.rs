extern crate reqwest;

use serde::{Deserialize, Serialize};
use reqwest::header::CONTENT_LENGTH;

#[derive(Debug, Serialize, Deserialize)]
struct GsltResponse {
    response: GsltStruct
}

#[derive(Debug, Serialize, Deserialize)]
struct GsltStruct {
    steamid: String,
	login_token: String
}

#[derive(Debug, Serialize, Deserialize)]
struct GsltListResponse {
    response: GsltListResponseDetail
}

#[derive(Debug, Serialize, Deserialize)]
struct GsltListResponseDetail {
    servers: Vec<GsltListGslt>,
	is_banned:bool,
	expires:u32,
	actor:String,
	last_action_time:u32
}

#[derive(Debug, Serialize, Deserialize)]
struct GsltListGslt  {
	steamid:String,
	appid: i16,
	login_token: String,
	memo:String,
	is_deleted: bool,
	is_expired: bool,
	rt_last_logon: u32
}

#[tokio::main]
async fn main(){
    let _list = get_list().await;
    // let _gslt = get_gslt().await;
}

async fn get_list() -> Result<(), reqwest::Error> {

    let res = reqwest::get("https://api.steampowered.com/IGameServersService/GetAccountList/v1/?key=STEAM_WEB_API_KEY").await?;
    if res.status() != 200 {
        println!("Unknown ERROR : {}", res.status())
    }

    let body = res.text().await?;
    let list: GsltListResponse = match serde_json::from_str(&body){
        Ok(n) => n,
        Err(err) => panic!("ERR : {:?}\nBODY : {}",err,body),
    };
    println!("list.response : {:?}", list.response);

    Ok(())
}

async fn get_gslt() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post("https://api.steampowered.com/IGameServersService/CreateAccount/v1/?key=STEAM_WEB_API_KEY&appid=730&memo=RUST")
    .header(CONTENT_LENGTH, 0)
        .send()
        .await?;
    if res.status() != 200 {
        println!("Unknown ERROR : {}", res.status())
    }

    let body = res.text().await?;
    let gslt: GsltResponse = match serde_json::from_str(&body){
        Ok(n) => n,
        Err(err) => panic!("ERR : {:?}\nBODY : {}",err,body),
    };
    println!("login_token: {}, steamid: {}", gslt.response.login_token, gslt.response.steamid);

    Ok(())
}