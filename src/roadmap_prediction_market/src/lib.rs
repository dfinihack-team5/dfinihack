mod api;
mod env;
mod types;

use env::{CanisterEnvironment, Environment};
use types::{Account, Market, Profile};

use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
};

thread_local! {
    pub static RUNTIME_STATE: RefCell<Option<RuntimeState>> = RefCell::default();
}

#[init]
#[post_upgrade]
fn init() {
    ic_cdk::print("init() called");
    let env = CanisterEnvironment {};
    let data = Data::default();
    let runtime_state = RuntimeState::new(Box::new(env), data);

    RUNTIME_STATE.with(|state| *state.borrow_mut() = Some(runtime_state));
}

pub struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data) -> RuntimeState {
        RuntimeState { env, data }
    }
}

#[derive(Default)]
pub struct Data {
    profiles: HashMap<Principal, (Profile, Account)>,
    profile_index: HashMap<String, Principal>,
    markets: BTreeMap<String, Market>,
}
