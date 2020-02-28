use anyhow::Result;

use gslt_rust::gslt::{
    GsltManager,
    IGameServersService
};

fn main() -> Result<()> {
    let manager = GsltManager::new("STEAM_WEB_API_KEY");
    let steamid: u64 = 85568300000000000; // YOUR GSLT steamid(Not login_token,steamid!)
    let memo = "Rust memo...";

    // get account list
    let list = manager.get_list()?;
    println!("list: {:?}", list);

    // create account
    let gslt = manager.create_account(730, "MEMO")?;
    println!("gslt: {:?}", gslt);

    // delete account
    manager.delete_account(steamid)?;

    // set memo
    manager.setmemo_gslt(steamid, memo)?;
    println!("set memo steamid: {:?}, memo: {}", steamid, memo);

    Ok(())
}
