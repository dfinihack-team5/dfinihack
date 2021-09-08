use crate::types::{Account, Profile, Response};
use crate::{RuntimeState, RUNTIME_STATE};

use ic_cdk_macros::*;

#[update]
fn join(profile: Profile) -> Response {
    RUNTIME_STATE.with(|state| join_impl(profile, state.borrow_mut().as_mut().unwrap()))
}

fn join_impl(profile: Profile, state: &mut RuntimeState) -> Response {
    let principal = state.env.caller();

    if state.data.profiles.contains_key(&principal) {
        return Response::Error("Principal already joined");
    }
    if state.data.profile_index.contains_key(&profile.name) {
        return Response::Error("Name already in use");
    }

    state
        .data
        .profile_index
        .insert(profile.name.clone(), principal.clone());
    state
        .data
        .profiles
        .insert(principal, (profile, Account::default()));

    Response::Success
}

#[update(name = "updateProfile")]
fn update_profile(profile: Profile) -> Response {
    RUNTIME_STATE.with(|state| update_profile_impl(profile, state.borrow_mut().as_mut().unwrap()))
}

fn update_profile_impl(profile: Profile, state: &mut RuntimeState) -> Response {
    let principal = state.env.caller();

    match state.data.profiles.entry(principal.clone()) {
        std::collections::hash_map::Entry::Occupied(mut e) => {
            let v = e.get_mut();
            if v.0.name != profile.name {
                if state.data.profile_index.contains_key(&profile.name) {
                    return Response::Error("Name already in use");
                }
                state.data.profile_index.remove(&v.0.name);
                state
                    .data
                    .profile_index
                    .insert(profile.name.clone(), principal.clone());
            }
            v.0 = profile;
            Response::Success
        }

        std::collections::hash_map::Entry::Vacant(_) => {
            return Response::Error("No profile for principal, must join first")
        }
    }
}

#[query(name = "getSelf")]
fn get_self() -> Option<(Profile, Account)> {
    RUNTIME_STATE.with(|state| get_self_impl(state.borrow_mut().as_mut().unwrap()))
}

fn get_self_impl(state: &mut RuntimeState) -> Option<(Profile, Account)> {
    let principal = state.env.caller();

    state.data.profiles.get(&principal).cloned()
}

#[query(name = "getProfile")]
fn get_profile(name: String) -> Option<Profile> {
    RUNTIME_STATE.with(|state| get_profile_impl(name, state.borrow_mut().as_mut().unwrap()))
}

fn get_profile_impl(name: String, state: &mut RuntimeState) -> Option<Profile> {
    state.data.profile_index.get(&name).and_then(|principal| {
        state
            .data
            .profiles
            .get(principal)
            .map(|(profile, _)| profile)
            .cloned()
    })
}

#[query(name = "getAccount")]
fn get_account() -> Option<Account> {
    RUNTIME_STATE.with(|state| get_account_impl(state.borrow_mut().as_mut().unwrap()))
}

fn get_account_impl(state: &mut RuntimeState) -> Option<Account> {
    let principal = state.env.caller();

    state
        .data
        .profiles
        .get(&principal)
        .map(|(_, account)| account)
        .cloned()
}

#[query(name = "searchProfile")]
fn search_profile(text: String) -> Option<Profile> {
    RUNTIME_STATE.with(|state| search_profile_impl(text, state.borrow_mut().as_mut().unwrap()))
}

fn search_profile_impl(text: String, state: &mut RuntimeState) -> Option<Profile> {
    let text = text.to_lowercase();

    for (p, _) in state.data.profiles.values() {
        if p.name.to_lowercase().contains(&text) || p.description.to_lowercase().contains(&text) {
            return Some(p.clone());
        }
    }

    None
}
