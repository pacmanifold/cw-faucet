use cosmwasm_schema::{cw_serde, QueryResponses};
use cw_ownable::{cw_ownable_execute, cw_ownable_query};

#[cw_serde]
pub struct InstantiateMsg {
    pub claim_size: u128,
    pub claim_wait_time: u64,
}

#[cw_ownable_execute]
#[cw_serde]
pub enum ExecuteMsg {
    Claim {},
    UpdateConfig {
        claim_size: Option<u128>,
        claim_wait_time: Option<u64>,
    },
}

#[cw_ownable_query]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
