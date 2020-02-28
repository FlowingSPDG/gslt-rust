use anyhow::Result;

use gslt_rust::gslt::{
    GsltManager,
    GsltCredential
};

fn main() -> Result<()> {
    let manager = GsltManager::new("STEAM_WEB_API_KEY");
    let steamid: u64 = 85568300000000000; // YOUR GSLT steamid(Not login_token,steamid!)
    let gslt = manager.create_gslt(730, "MEMO")?;
    
    println!("gslt: {:?}", gslt); // for create
    manager.delete_gslt(steamid)?; // for delete

    Ok(())
}
