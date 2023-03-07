mod web3;
mod abi;
mod entity;

use std::env;
use std::sync::Arc;
use ethers::providers::{Http, Provider};
use log::{error, LevelFilter};
use simple_logger::SimpleLogger;
use migration::{Migrator, MigratorTrait, prepare_connection};
use crate::web3::Web3;

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let db_conn = prepare_connection().await;
    let _migrated = Migrator::up(&db_conn, None).await;

    let rpc_url = &env::var("RPC_URL").expect("Invalid RPC URL in config");
    let provider = Provider::<Http>::try_from(rpc_url).unwrap();
    let client = Arc::new(provider);
    let web3 = Web3{
        client,
        db_conn: entity::lib::db_conn().await.unwrap()
    };

    let max_block = u64::try_from(web3.get_last_block().await).unwrap();
    let blocks_to_ingest = env::var("BLOCKS_TO_INGEST").expect("User should define how many blocks to ingest").parse::<u64>().unwrap();
    let start_block = max_block - blocks_to_ingest;
    let blocks_chunk_size = env::var("BLOCKS_CHUNK_SIZE").expect("User should define batch size of one get_log call").parse::<u64>().unwrap();

    let mut current_block = start_block;
    while current_block < max_block {
        let from = current_block;
        let to = current_block + blocks_chunk_size -1;
        match web3.get_logs(from, to).await {
            Ok(logs) => {
                println!("{} events found in blocks [{} - {}]", logs.iter().len(), from, to);
                for log in logs.iter() {
                    let contract = log.address;
                    if web3.is_erc20(&contract) {
                        web3.handle_erc20_transfer(&log).await;
                    } else {
                        web3.handle_erc721_transfer(&log).await;
                    }

                }
                current_block += blocks_chunk_size;
            },
            Err(e) => {
                error!("HttpProvider failed with: {e}");
            }
        }
    }
}