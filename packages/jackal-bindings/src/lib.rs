use cosmwasm_schema::cw_serde;
use cosmwasm_std::{CosmosMsg, CustomMsg};

#[cw_serde]
pub enum JackalMsg {
    BuyStorage {
        creator: String,
        for_address: String,
        duration: String,
        bytes: String, 
        payment_denom: String,
    },
    SignContract {
        creator: String,
        cid: String,
        pay_once: bool,
    }
}

impl From<JackalMsg> for CosmosMsg<JackalMsg> {
    fn from(msg: JackalMsg) -> CosmosMsg<JackalMsg> {
        CosmosMsg::Custom(msg)
    }
}

impl CustomMsg for JackalMsg {}
