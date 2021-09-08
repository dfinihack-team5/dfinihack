use std::collections::BTreeMap;

use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
}

pub type MarketName = String;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Market {
    pub name: MarketName,
    pub description: String,
    pub yes_shares: f64,
    pub no_shares: f64,
}

impl Market {
    pub fn new(name: MarketName, description: String) -> Market {
        Market {
            name,
            description,
            yes_shares: 0.0,
            no_shares: 0.0,
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct MarketStatus {
    pub market: Market,
    pub yes_price: f64,
    pub no_price: f64,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, Copy)]
pub enum Share {
    Yes,
    No,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Position {
    pub share: Share,
    pub amount: f64,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    // Token balance.
    pub tokens: f64,
    pub positions: BTreeMap<MarketName, Position>,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            tokens: 100.0,
            positions: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq)]
pub enum Response {
    Success,
    Error(&'static str),
}
