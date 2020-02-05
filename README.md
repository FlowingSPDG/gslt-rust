# gslt-rust

## What's this?

A tool to manage **Game Server Login Token**(GSLT) written in Rust.

## install
`WIP`

## example
see `examples` directory.

basically, Create `GsltManager` with your own `STEAM_WEB_API_KEY`,

and `GsltList`, `GsltCredential`, etc...(still developing) according to the API you want to use.

```rs
// example::create gslt token
// u can use standard result.
// i personally use `anyhow`, so it is used here as well.

use anyhow::Result;
use gslt_rust::gslt::{
    GsltManager,
    GsltCredential
};

fn main() -> Result<()> {
    let manager = GsltManager::new("STEAM_WEB_API_KEY");
    let gslt = manager.create_gslt(730, "MEMO")?;
    println!("gslt: {:?}", gslt);
}
```