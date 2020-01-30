extern crate reqwest;

use serde::{Deserialize, Serialize};
use reqwest::header::CONTENT_LENGTH;

/*
#[derive(Debug, Serialize, Deserialize)]
struct List {
    response: GSLT,
}

struct GSLT {
    steamid: String,
    appid: i32,
    login_token: String,
    memo: String,
    is_deleted: bool,
    is_expired: bool,
    rt_last_logon:i32,
}
*/


#[tokio::main]
async fn main(){
    // let _list = get_list().await;
    let _gslt = get_gslt().await;
}

async fn get_list() -> Result<(), reqwest::Error> {

    let res = reqwest::get("https://api.steampowered.com/IGameServersService/GetAccountList/v1/?key=STEAM_WEB_API_KEY").await?;
    println!("Status: {}", res.status());

    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}

async fn get_gslt() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post("https://api.steampowered.com/IGameServersService/CreateAccount/v1/?key=STEAM_WEB_API_KEY&appid=730&memo=RUST")
    .header(CONTENT_LENGTH, 0)
        .send()
        .await?;
    println!("Status: {}", res.status());
    // Content Length?? 411 ERR

    let body = res.text().await?;
    println!("Body:\n\n{}", body);

    Ok(())
}