use anyhow::*;

use crate::gslt::GsltManager;
use crate::gslt::GsltSetmemo;
use crate::gslt::encode;


impl GsltSetmemo for GsltManager {
    fn setmemo_gslt(&self, steam_id: u64, memo: impl Into<String>) -> Result<()> {
        let steam_web_api_key = &self.steam_web_api_key;
        let steam_id = steam_id.to_string();
        let memo = memo.into();
        let request_url = format!("https://api.steampowered.com/IGameServersService/SetMemo/v1/?key={}&steamid={}&memo={}", encode(&steam_web_api_key), encode(&steam_id), encode(&memo));

        let response = ureq::post(&request_url)
                            .timeout_connect(10_000)
                            .set("Content-Length", "0")
                            .call();
                            
        if response.status() != 200 { bail!("status code is not 200."); }
    
        Ok(())
    }
}
