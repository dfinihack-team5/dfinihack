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
const EPSILON: f64 = 1e-6;

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

#[query(name = "buyingPower")]
fn buying_power(market: MarketName, share: Share) -> f64 {
    RUNTIME_STATE
        .with(|state| buying_power_impl(market, share, state.borrow_mut().as_mut().unwrap()))
}

fn buying_power_impl(market: MarketName, share: Share, state: &mut RuntimeState) -> f64 {
    let market = match state.data.markets.get(&market) {
        Some(market) => market,
        None => return 0.0,
    };

    let principal = state.env.caller();
    let account = match state
        .data
        .profiles
        .get(&principal)
        .map(|(_, account)| account)
    {
        Some(account) => account,
        None => return 0.0,
    };

    buying_power_(market, share, account.tokens)
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

    let remaining_tokens = match checked_sub(account.tokens, cost) {
        Ok(remaining) => remaining,
        Err(()) => return Response::Error("Not enough funds"),
    };
    account.tokens = remaining_tokens;

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

#[update]
fn sell(market: MarketName, share: Share, amount: f64) -> Response {
    RUNTIME_STATE
        .with(|state| sell_impl(market, share, amount, state.borrow_mut().as_mut().unwrap()))
}

fn sell_impl(market: MarketName, share: Share, amount: f64, state: &mut RuntimeState) -> Response {
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

    let mut position = match account.positions.entry(market.name.clone()) {
        std::collections::btree_map::Entry::Vacant(_) => {
            return Response::Error("Short selling is not supported")
        }
        std::collections::btree_map::Entry::Occupied(position) => {
            if position.get().share != share {
                return Response::Error("Account owns opposite share");
            }
            position
        }
    };

    let remaining_amount = match checked_sub(position.get().amount, amount) {
        Ok(remaining) => remaining,
        Err(()) => return Response::Error("Not enough shares"),
    };

    let tokens = -trade_cost(market, share, -amount);
    assert!(tokens > 0.0);

    account.tokens += tokens;
    if remaining_amount == 0.0 {
        position.remove();
    } else {
        position.get_mut().amount = remaining_amount;
    }

    // TODO: Return an error instead of unwrapping.
    match share {
        Share::Yes => market.yes_shares = checked_sub(market.yes_shares, amount).unwrap(),
        Share::No => market.no_shares = checked_sub(market.no_shares, amount).unwrap(),
    }

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
    assert!(yes_shares >= -EPSILON);
    assert!(no_shares >= -EPSILON);

    let yes_weight = (yes_shares / B).exp();
    let no_weight = (no_shares / B).exp();
    B * (yes_weight + no_weight).ln()
}

/// Calculates `left - right`, rounding to zero values between `[-EPSILON..EPSILON]` and
/// returning an error if the result is negative.
fn checked_sub(left: f64, right: f64) -> Result<f64, ()> {
    assert!(left > EPSILON);
    assert!(right > EPSILON);

    let res = left - right;
    if res < -EPSILON {
        Err(())
    } else if res < EPSILON {
        Ok(0.0)
    } else {
        Ok(res)
    }
}

/// Computes how many shares of the given tyoe on the given market can be bought with the
/// given amount of tokens.
fn buying_power_(market: &Market, share: Share, tokens: f64) -> f64 {
    assert!(tokens > EPSILON);

    let yes_weight = (market.yes_shares / B).exp();
    let no_weight = (market.no_shares / B).exp();
    let weight_before = yes_weight + no_weight;
    let cost_before = B * weight_before.ln();

    let cost_after = cost_before + tokens;
    let weight_after = (cost_after / B).exp();

    let weight_delta = weight_after - weight_before;

    match share {
        Share::Yes => B * (yes_weight + weight_delta).ln() - market.yes_shares,
        Share::No => B * (no_weight + weight_delta).ln() - market.no_shares,
    }
}

#[cfg(test)]
mod tests {
    use ic_cdk::export::Principal;

    use super::*;
    use crate::{
        api::profile::{get_account_impl, join_impl},
        env::TestEnvironment,
        types::Profile,
    };

    #[test]
    fn test_get_market() {
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

    #[test]
    fn test_buying_power() {
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
        assert_eq!(
            Response::Success,
            join_impl(
                Profile {
                    name: "p name".into(),
                    description: "p description".into()
                },
                &mut state
            )
        );
        assert_eq!(
            Response::Success,
            buy_impl("name".into(), Share::Yes, 10.0, &mut state)
        );
        let account = get_account_impl(&mut state).unwrap();
        let market = get_market_impl("name".into(), &mut state).unwrap().market;

        // How many `Yes` shares we can buy for all of the remaining tokens.
        let buying_power = buying_power_(&market, Share::Yes, account.tokens);
        assert_eq!(
            Response::Success,
            buy_impl("name".into(), Share::Yes, buying_power, &mut state)
        );
        let account = get_account_impl(&mut state).unwrap();

        // We should have no tokens left.
        assert_eq!(0.0, account.tokens);
    }
}
