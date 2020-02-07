use anyhow::Result;

use gslt_rust::gslt::{
    GsltManager,
    GsltSetmemo
};

fn main() -> Result<()> {
    let manager = GsltManager::new("STEAM_WEB_API_KEY");
    let steamid = 85568300000000000; // YOUR GSLT steamid(Not login_token,steamid!)
    let memo = "Rust memo...";
    manager.setmemo_gslt(steamid, memo)?;
    println!("set memo steamid: {:?}, memo: {}", steamid, memo);
    Ok(())
}
