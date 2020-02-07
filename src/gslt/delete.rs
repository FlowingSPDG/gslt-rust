use anyhow::*;

use crate::gslt::GsltManager;
use crate::gslt::GsltDelete;
use crate::gslt::encode;


impl GsltDelete for GsltManager {
    fn delete_gslt(&self, steam_id: u64) -> Result<()> {
        let steam_web_api_key = &self.steam_web_api_key;
        let steam_id = steam_id.to_string();
        let request_url = format!("https://api.steampowered.com/IGameServersService/DeleteAccount/v1/?key={}&steamid={}", encode(&steam_web_api_key), encode(&steam_id));

        let response = ureq::post(&request_url)
                            .timeout_connect(10_000)
                            .set("Content-Length", "0")
                            .call();
                            
        if response.status() != 200 { bail!("status code is not 200."); }
    
        Ok(())
    }
}
