use anyhow::Result;

use gslt_rust::gslt::{
    GsltManager,
    GsltDelete
};

fn main() -> Result<()> {
    let manager = GsltManager::new("STEAM_WEB_API_KEY");
    let steamid = 85568300000000000; // YOUR GSLT steamid(Not login_token,steamid!)
    manager.delete_gslt(steamid)?;
    println!("deleted steamid: {:?}", steamid);
    Ok(())
}
