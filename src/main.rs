use pool_sync::{PoolSync, PoolType, Chain, PoolInfo};
use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // Configure and build the PoolSync instance
    let pool_sync = PoolSync::builder()
        .add_pool(PoolType::UniswapV2)
        .chain(Chain::Ethereum)
        .build()?;

    // Synchronize pools
    let (pools, _last_synced_block) = pool_sync.sync_pools().await?;

    // Common Info
    for pool in &pools {
        println!("Pool Address {:?}, Token 0: {:?}, Token 1: {:?}", pool.address(), pool.token0_name(), pool.token1_name());
    }

    println!("Synced {} pools!", pools.len());
    Ok(())
}