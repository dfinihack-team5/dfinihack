use crate::types::{Market, MarketName, MarketStatus, Position, Response, Share};
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
        std::collections::btree_map::Entry::Occupied(_) => Response::Error("Market already exists"),
        std::collections::btree_map::Entry::Vacant(e) => {
            e.insert(Market::new(name, description));
            Response::Success
        }
    }
}

/// The `b` parameter, as defined at https://www.cultivatelabs.com/prediction-markets-guide/how-does-logarithmic-market-scoring-rule-lmsr-work
const B: f64 = 10.0;
const EPSILON: f64 = 1e-9;

#[query(name = "getMarket")]
fn get_market(name: MarketName) -> Option<MarketStatus> {
    RUNTIME_STATE.with(|state| get_market_impl(name, state.borrow_mut().as_mut().unwrap()))
}

fn get_market_impl(name: MarketName, state: &mut RuntimeState) -> Option<MarketStatus> {
    state.data.markets.get(&name).cloned().map(market_status)
}

#[query(name = "getMarkets")]
fn get_markets() -> Vec<MarketStatus> {
    RUNTIME_STATE.with(|state| get_markets_impl(state.borrow_mut().as_mut().unwrap()))
}

fn get_markets_impl(state: &mut RuntimeState) -> Vec<MarketStatus> {
    state
        .data
        .markets
        .values()
        .cloned()
        .map(market_status)
        .collect()
}

fn market_status(market: Market) -> MarketStatus {
    let yes_weight = (market.yes_shares / B).exp();
    let no_weight = (market.no_shares / B).exp();
    let total_weight = yes_weight + no_weight;

    MarketStatus {
        market,
        yes_price: yes_weight / total_weight,
        no_price: no_weight / total_weight,
    }
}

#[update]
fn buy(market: MarketName, share: Share, amount: f64) -> Response {
    RUNTIME_STATE
        .with(|state| buy_impl(market, share, amount, state.borrow_mut().as_mut().unwrap()))
}

fn buy_impl(market: MarketName, share: Share, amount: f64, state: &mut RuntimeState) -> Response {
    assert!(amount > 0.0);

    let market = match state.data.markets.get_mut(&market) {
        Some(market) => market,
        None => return Response::Error("Market not found"),
    };

    let principal = state.env.caller();
    let account = match state
        .data
        .profiles
        .get_mut(&principal)
        .map(|(_, account)| account)
    {
        Some(account) => account,
        None => return Response::Error("No account, must join first"),
    };

    // TODO: Sell the opposite share instead.
    if let Some(position) = account.positions.get(&market.name) {
        if position.share != share {
            return Response::Error("Account already owns opposite share");
        }
    }

    let cost = trade_cost(market, share, amount);
    if cost > account.tokens + EPSILON {
        return Response::Error("Not enough funds");
    }

    account.tokens -= cost;
    if account.tokens < EPSILON {
        account.tokens = 0.0;
    }
    match share {
        Share::Yes => market.yes_shares += amount,
        Share::No => market.no_shares += amount,
    }

    account
        .positions
        .entry(market.name.clone())
        .or_insert(Position { share, amount: 0.0 })
        .amount += amount;
    Response::Success
}

#[update(name = "resolveMarket")]
fn resolve_market(market: MarketName, outcome: Share) -> Response {
    RUNTIME_STATE
        .with(|state| resolve_market_impl(market, outcome, state.borrow_mut().as_mut().unwrap()))
}

fn resolve_market_impl(market: MarketName, outcome: Share, state: &mut RuntimeState) -> Response {
    let market = match state.data.markets.entry(market) {
        std::collections::btree_map::Entry::Vacant(_) => {
            return Response::Error("Market not found")
        }
        std::collections::btree_map::Entry::Occupied(e) => e.remove(),
    };

    for (_, (_, account)) in state.data.profiles.iter_mut() {
        if let Some(position) = account.positions.remove(&market.name) {
            if position.share == outcome {
                account.tokens += position.amount;
            }
        }
    }

    Response::Success
}

fn trade_cost(market: &Market, share: Share, amount: f64) -> f64 {
    let cost_before = cost(market.yes_shares, market.no_shares);
    let cost_after = match share {
        Share::Yes => cost(market.yes_shares + amount, market.no_shares),
        Share::No => cost(market.yes_shares, market.no_shares + amount),
    };
    cost_after - cost_before
}

/// The cost function, as defined at https://www.cultivatelabs.com/prediction-markets-guide/how-does-logarithmic-market-scoring-rule-lmsr-work
fn cost(yes_shares: f64, no_shares: f64) -> f64 {
    assert!(yes_shares >= 0.0);
    assert!(no_shares >= 0.0);

    let yes_weight = (yes_shares / B).exp();
    let no_weight = (no_shares / B).exp();
    B * (yes_weight + no_weight).ln()
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
