mod pools;
use crate::pools::load_all_pools;
use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();
    let wss_url = env::var("WSS_RPC_URL")?;

    let (pools, _) = load_all_pools(wss_url.clone(), 10000000, 50000)
        .await
        .unwrap();

    for pool in pools.iter() {
        pool.pretty_print();
    }

    Ok(())
}
