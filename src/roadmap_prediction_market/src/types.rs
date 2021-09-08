use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Account {
    // Token balance.
    pub tokens: f64,
}

impl Default for Account {
    fn default() -> Self {
        Self { tokens: 100.0 }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum Response {
    Success,
    Error(&'static str),
}
