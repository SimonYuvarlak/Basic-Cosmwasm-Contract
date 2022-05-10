use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint64};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Score {
    pub address: Addr,
    pub score: Uint64,
}
pub const SCORE_LIST: Map<u64, Score> = Map::new("score_list");
pub const SCORE_SEQ: Item<Uint64> = Item::new("score_seq");
pub const STATE: Item<State> = Item::new("state");
