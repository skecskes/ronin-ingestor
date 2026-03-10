use alloy::primitives::{address, Address, B256, U256};
use alloy::providers::{Provider, RootProvider};
use alloy::rpc::types::{Filter, Log};
use sea_orm::ActiveValue::Set;
use sea_orm::{DatabaseConnection, EntityTrait};
use crate::entity::prelude::Transfers;
use crate::entity::transfers::ActiveModel as Transfer;

const LAND: Address = address!("8c811e3c958e190f5ec15fb376533a3398620500"); // 721 NFT
const LAND_ITEM: Address = address!("a96660f0e4a3e9bc7388925d245a6d4d79e21259"); // 721 NFT
const AXIE: Address = address!("32950db2a7164ae833121501c797d79e7b79d74c"); // 721 NFT
const SMOOTH_LOVE_POTION: Address = address!("a8754b9fa15fc18bb59458815510e40a12cd2014"); // ERC20
const AXIE_INFINITY_SHARD: Address = address!("97a9107c1793bc407d6f527b77e7fff4d812bece"); // ERC20
const TRANSFER_EVENTS: &str = "Transfer(address,address,uint256)";

pub struct Web3 {
    pub client: RootProvider<alloy::network::Ethereum>,
    pub db_conn: DatabaseConnection
}

impl Web3 {

    pub async fn get_last_block(&self) -> u64 {
        self.client.get_block_number().await.expect("We should be able to get the current block without issues")
    }

    fn nft_addresses(&self) -> Vec<Address> {
        vec![LAND, LAND_ITEM, AXIE, SMOOTH_LOVE_POTION, AXIE_INFINITY_SHARD]
    }

    pub fn is_erc20(&self, address: &Address) -> bool {
        *address == SMOOTH_LOVE_POTION || *address == AXIE_INFINITY_SHARD
    }

    pub async fn get_logs(&self, from: u64, to: u64) -> Result<Vec<Log>, alloy::transports::RpcError<alloy::transports::TransportErrorKind>> {
        self.client.get_logs(
            &Filter::new()
                .address(self.nft_addresses())
                .event(TRANSFER_EVENTS)
                .from_block(from)
                .to_block(to)
        ).await
    }

    pub async fn handle_erc721_transfer(&self, log: &Log) {
        let contract: Address = log.address();
        let block_number: u64 = log.block_number.unwrap();
        let from: Address = Address::from_word(log.topics()[1]);
        let to: Address = Address::from_word(log.topics()[2]);
        let token_id: U256 = U256::from_be_bytes(log.topics()[3].0);
        let transaction_hash: B256 = log.transaction_hash.unwrap();
        let transaction_index: u64 = log.transaction_index.unwrap_or(0);
        let log_index = log.log_index.unwrap_or_default().to_string();
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
        let contract: Address = log.address();
        let block_number: u64 = log.block_number.unwrap();
        let from: Address = Address::from_word(log.topics()[1]);
        let to: Address = Address::from_word(log.topics()[2]);
        let value: U256 = U256::from_be_slice(&log.data().data);
        let transaction_hash: B256 = log.transaction_hash.unwrap();
        let transaction_index: u64 = log.transaction_index.unwrap_or(0);
        let log_index = log.log_index.unwrap_or_default().to_string();
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