use ic_cdk::export::candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Profile {
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
}
