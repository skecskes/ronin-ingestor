use std::borrow::Borrow;
use std::ops::Add;
use std::sync::Arc;
use ethers::abi::AbiDecode;
use ethers::providers::{Http, Middleware, Provider, ProviderError};
use ethers::types::{Address, Filter, H160, H256, Log, U256, U64};
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::entity::prelude::Transfers;
use crate::entity::transfers::ActiveModel as Transfer;

const LAND: &str = "0x8c811e3c958e190f5ec15fb376533a3398620500"; // 721 NFT
const LAND_ITEM: &str = "0xa96660f0e4a3e9bc7388925d245a6d4d79e21259"; //721 NFT
const AXIE: &str = "0x32950db2a7164ae833121501c797d79e7b79d74c"; //721 NFT
const SMOOTH_LOVE_POTION: &str = "a8754b9fa15fc18bb59458815510e40a12cd2014"; //20
const AXIE_INFINITY_SHARD: &str = "0x97a9107c1793bc407d6f527b77e7fff4d812bece"; //20
const TRANSFER_EVENTS: &str = "Transfer(address,address,uint256)";

pub struct Web3 {
    pub client: Arc<Provider<Http>>,
    pub db_conn: DatabaseConnection
}

impl Web3 {

    pub async fn get_last_block(&self) -> U64 {
        let max_block = self.client.get_block_number().await.expect("We should be able to get the current block without issues");
        max_block
    }

    fn nft_addresses(&self) -> Vec<Address> {
        vec![
            LAND.parse::<Address>().unwrap(),
            LAND_ITEM.parse::<Address>().unwrap(),
            AXIE.parse::<Address>().unwrap(),
            SMOOTH_LOVE_POTION.parse::<Address>().unwrap(),
            AXIE_INFINITY_SHARD.parse::<Address>().unwrap(),
        ]
    }

    pub fn is_erc20(&self, address: &Address) -> bool {
        let slp = SMOOTH_LOVE_POTION.parse::<Address>().unwrap();
        let axs = AXIE_INFINITY_SHARD.parse::<Address>().unwrap();
        address.eq(&slp) || address.eq(&axs)
    }

    pub async fn get_logs(&self, from: u64, to: u64) -> Result<Vec<Log>, ProviderError> {
        self.client.get_logs(
            Filter::new()
                .address(self.nft_addresses())
                .event(TRANSFER_EVENTS)
                .from_block(from)
                .to_block(to)
                .borrow()
        ).await
    }

    pub async fn handle_erc721_transfer(&self, log: &Log) {
        let contract = Address::from(log.address);
        let block_number = log.block_number.unwrap();
        let from: H160 = Address::from(log.topics[1]);
        let to: H160 = Address::from(log.topics[2]);
        let token_id: U256 = U256::decode(log.topics[3]).unwrap();
        let transaction_hash: H256 = log.transaction_hash.unwrap();
        let transaction_index: u64 = u64::try_from(log.transaction_index.unwrap_or(U64::zero())).unwrap();
        let log_index = log.log_index.unwrap_or_default().0.iter().fold(String::new(), |acc, elem| {
            if acc.is_empty() {
                acc.add(&*elem.to_string())
            } else {
                acc.add("_").add(&*elem.to_string())
            }
        });
        println!(
            "erc721: block_number = {block_number}, contract = {contract:#032x}, from = {from:#032x}, to = {to:#032x}, tokenId = {token_id}, transactionHash = {transaction_hash:#032x}"
        );

        // unique composite primary key from transaction_hash and log_index
        let id = format!("{transaction_hash:#032x}:{log_index}");
        let transfer = Transfer {
            id: Set(id),
            token_type: Set(String::from("ERC721")),
            block_number: Set(i32::try_from(block_number).unwrap()),
            contract: Set(format!("{contract:#032x}")),
            from: Set(format!("{from:#032x}")),
            to: Set(format!("{to:#032x}")),
            token_id: Set(Some(token_id.to_string())),
            transaction_hash: Set(format!("{transaction_hash:#032x}")),
            transaction_index: Set(transaction_index.to_string()),
            log_index: Set(log_index),
            ..Default::default()
        };
        let _insert_res = Transfers::insert(transfer).exec(&self.db_conn).await;
    }

    pub async fn handle_erc20_transfer(&self, log: &Log) {
        let contract = Address::from(log.address);
        let block_number = log.block_number.unwrap();
        let from: H160 = Address::from(log.topics[1]);
        let to: H160 = Address::from(log.topics[2]);
        let value: U256 = U256::decode(&log.data.0).unwrap_or(U256::zero());
        let transaction_hash: H256 = log.transaction_hash.unwrap();
        let transaction_index: u64 = u64::try_from(log.transaction_index.unwrap_or(U64::zero())).unwrap();
        let log_index = log.log_index.unwrap_or_default().0.iter().fold(String::new(), |acc, elem| {
            if acc.is_empty() {
                acc.add(&*elem.to_string())
            } else {
                acc.add("_").add(&*elem.to_string())
            }
        });
        println!(
            "erc20: block_number = {block_number}, contract = {contract:#032x},from = {from:#032x}, to = {to:#032x}, value = {value}, transactionHash = {transaction_hash:#032x}"
        );

        // unique composite primary key from transaction_hash and log_index
        let id = format!("{transaction_hash:#032x}:{log_index}");
        let transfer = Transfer {
            id: Set(id),
            token_type: Set(String::from("ERC20")),
            block_number: Set(i32::try_from(block_number).unwrap()),
            contract: Set(format!("{contract:#032x}")),
            from: Set(format!("{from:#032x}")),
            to: Set(format!("{to:#032x}")),
            value: Set(Some(value.to_string())),
            transaction_hash: Set(format!("{transaction_hash:#032x}")),
            transaction_index: Set(transaction_index.to_string()),
            log_index: Set(log_index),
            ..Default::default()
        };
        let _insert_res = Transfers::insert(transfer).exec(&self.db_conn).await;
    }
}