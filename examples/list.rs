use anyhow::Result;

use gslt_rust::gslt::{
    GsltManager,
    GsltList
};

fn main() -> Result<()> {
    let manager = GsltManager::new("STEAM_WEB_API_KEY");
    let list = manager.get_list()?;
    println!("list: {:?}", list);
    Ok(())
}
