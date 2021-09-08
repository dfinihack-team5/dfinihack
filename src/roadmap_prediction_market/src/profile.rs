use crate::types::Profile;
use crate::{RuntimeState, RUNTIME_STATE};
use ic_cdk_macros::*;

#[query(name = "getSelf")]
fn get_self() -> Option<Profile> {
    RUNTIME_STATE.with(|state| get_self_impl(state.borrow_mut().as_mut().unwrap()))
}

fn get_self_impl(state: &mut RuntimeState) -> Option<Profile> {
    let principal = state.env.caller();

    state.data.profiles.get(&principal).cloned()
}

#[update(name = "updateSelf")]
fn update_self(profile: Profile) {
    RUNTIME_STATE.with(|state| update_self_impl(profile, state.borrow_mut().as_mut().unwrap()))
}

fn update_self_impl(profile: Profile, state: &mut RuntimeState) {
    let principal = state.env.caller();

    state
        .data
        .profile_index
        .insert(profile.name.clone(), principal.clone());
    state.data.profiles.insert(principal, profile);
}

#[query(name = "getProfile")]
fn get_profile(name: String) -> Option<Profile> {
    RUNTIME_STATE.with(|state| get_profile_impl(name, state.borrow_mut().as_mut().unwrap()))
}

fn get_profile_impl(name: String, state: &mut RuntimeState) -> Option<Profile> {
    state
        .data
        .profile_index
        .get(&name)
        .and_then(|principal| state.data.profiles.get(principal).cloned())
}

#[query(name = "searchProfile")]
fn search_profile(text: String) -> Option<Profile> {
    RUNTIME_STATE.with(|state| search_profile_impl(text, state.borrow_mut().as_mut().unwrap()))
}

fn search_profile_impl(text: String, state: &mut RuntimeState) -> Option<Profile> {
    let text = text.to_lowercase();

    for p in state.data.profiles.values() {
        if p.name.to_lowercase().contains(&text) || p.description.to_lowercase().contains(&text) {
            return Some(p.clone());
        }

        for x in p.keywords.iter() {
            if x.to_lowercase() == text {
                return Some(p.clone());
            }
        }
    }

    None
}
