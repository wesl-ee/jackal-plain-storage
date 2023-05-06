use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Config {}

pub mod msg {
    use super::*;
    use schemars::JsonSchema;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub struct InstantiateMsg {
        pub config: Config,
        pub bytes: String,
        pub duration: String,
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum ExecuteMsg {
        Store { cid_map: Vec<(u32, String)> },
        RenewCollection {},
    }

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub enum QueryMsg {
        Item { token_id: Vec<u32> },
        Config {},
    }

    #[derive(Serialize, Deserialize, Clone, Debug, JsonSchema)]
    pub struct MigrateMsg {}
}

pub mod response {
    use super::*;

    pub type ConfigResponse = Config;

    #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
    #[serde(rename_all = "snake_case")]
    pub struct QueryItem {
        pub token_id: u32,
        pub cid: String,
        // TODO The burden should not be put on the client to do this. Queries
        // should be handled by a custom chain token_ider (that is not in-scope of
        // this contract, however)
        pub provider: Vec<String>,
    }

    pub type ItemResponse = Vec<QueryItem>;
}
