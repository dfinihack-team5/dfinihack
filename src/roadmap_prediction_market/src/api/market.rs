use crate::types::{Market, MarketName, MarketStatus, Response};
use crate::{RuntimeState, RUNTIME_STATE};

use ic_cdk_macros::*;

#[update(name = "newMarket")]
#[update]
fn new_market(name: MarketName, description: String) -> Response {
    RUNTIME_STATE
        .with(|state| new_market_impl(name, description, state.borrow_mut().as_mut().unwrap()))
}

fn new_market_impl(name: MarketName, description: String, state: &mut RuntimeState) -> Response {
    // TODO: Ensure that only some principals can create markets.
    // let principal = state.env.caller();

    match state.data.markets.entry(name.clone()) {
        std::collections::hash_map::Entry::Occupied(_) => Response::Error("Market already exists"),
        std::collections::hash_map::Entry::Vacant(e) => {
            e.insert(Market::new(name, description));
            Response::Success
        }
    }
}

/// The `b` parameter, as defined at https://www.cultivatelabs.com/prediction-markets-guide/how-does-logarithmic-market-scoring-rule-lmsr-work
const B: f64 = 10.0;

#[query(name = "getMarket")]
fn get_market(name: MarketName) -> Option<MarketStatus> {
    RUNTIME_STATE.with(|state| get_market_impl(name, state.borrow_mut().as_mut().unwrap()))
}

fn get_market_impl(name: MarketName, state: &mut RuntimeState) -> Option<MarketStatus> {
    let market = match state.data.markets.get(&name) {
        Some(market) => market,
        None => return None,
    };

    let yes_weight = (market.yes_shares / B).exp();
    let no_weight = (market.no_shares / B).exp();
    let total_weight = yes_weight + no_weight;

    Some(MarketStatus {
        market: market.clone(),
        yes_price: yes_weight / total_weight,
        no_price: no_weight / total_weight,
    })
}

#[cfg(test)]
mod tests {
    use ic_cdk::export::Principal;

    use super::*;
    use crate::env::TestEnvironment;

    #[test]
    fn foo() {
        let mut state = RuntimeState::new(
            Box::new(TestEnvironment {
                now: 0,
                caller: Principal::anonymous(),
            }),
            Default::default(),
        );

        assert_eq!(
            Response::Success,
            new_market_impl("name".into(), "description".into(), &mut state)
        );
        assert!(state.data.markets.get("name").is_some());
        assert!(get_market_impl("name".into(), &mut state).is_some());
    }
}
