use revm_primitives::{Address, B256};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterateConfig {
    pub url: String,
    pub start: Option<u64>,
    pub end: Option<u64>,
    pub interval: Option<u64>,
    pub target: Option<Address>,
    pub topics: Option<Vec<B256>>,
}
