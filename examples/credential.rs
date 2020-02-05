use anyhow::Result;

use gslt_rust::gslt::{
    GsltManager,
    GsltCredential
};

fn main() -> Result<()> {
    let manager = GsltManager::new("STEAM_WEB_API_KEY");
    let gslt = manager.create_gslt(730, "MEMO")?;
    println!("gslt: {:?}", gslt);
    Ok(())
}
