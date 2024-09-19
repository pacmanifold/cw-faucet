use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const LAST_CLAIM: Map<Addr, u64> = Map::new("last_claim");

#[cw_serde]
pub struct Config {
    pub denom: String,
    pub claim_size: u128,
    pub claim_wait_time: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
