use rsrl::{
    control::td::QLearning,
    domains::Domain,
    fa::linear::{
        basis::{Bias, Combinators, Fourier, Stack},
        optim::SGD,
        LFA,
    },
    make_shared,
    policies::Greedy,
    spaces::Space,
    Shared,
};
use ndarray::{ArrayBase, Dim, OwnedRepr};
extern crate lazy_static;
use crate::database::{balance_add, balance_sub};
use std::{
    cell::RefCell,
    sync::{Arc, Mutex},
};

struct Bet {
    playerId: String,
    value: i32,
}


fn add_bet(playerId: String, horseId: usize, betValue: i32) -> Result<(), ()> {
    let mut r = Ok(());
    HORSES.with(|horses_mutex| {
        let mut horses = match horses_mutex.lock() {
            Ok(content) => content,
            Err(content) => content.into_inner(),
        };

        let Some(horse) = horses.get_mut(horseId) else {
        return Err(());
    };

        balance_sub(&playerId, betValue);

        let bet = Bet {
            playerId,
            value: betValue,
        };

        horse.bets.push(bet);

        //take from user balance
        // balance - horse.bet.value ???
        
        Ok(())
    });

    r
}

/// horse struct
type AIT = Shared<LFA<Stack<Fourier, Bias>, ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>>>;

struct Horse {
    name: String,
    colour: String,
    totalBetsValue: i32,
    decimalOdds: f32,
    fractionalOdds: String,
    bets: Vec<Bet>,
    form: Vec<u8>,
    alpha: f32,
    gamma: f32,
    omega: f32,
    ai: (QLearning<AIT>, Greedy<AIT>),
    env: crate::simulation::Simulation,
}

impl Horse {
    fn new(name: String, colour: String, alpha: f32, gamma: f32, omega: f32) -> Self {
        let totalBetsValue = 0;
        let decimalOdds = 0.;
        let fractionalOdds = String::new();
        let bets = Vec::new();
        let form = Vec::new();

        let env = crate::simulation::Simulation::default();
        let n_actions = env.action_space().card().into();

        let ai = {
            let basis = Fourier::from_space(5, env.state_space()).with_bias();
            let q_func = make_shared(LFA::vector(basis, SGD(0.001), n_actions));
            let policy = Greedy::new(q_func.clone());

            (QLearning { q_func, gamma: 0.9 }, policy)
        };

        Self {
            name,
            colour,
            totalBetsValue,
            decimalOdds,
            fractionalOdds,
            bets,
            form,
            alpha,
            gamma,
            omega,
            ai,
            env,
        }
    }
}

thread_local! {
    //vector of horses
    static HORSES: Mutex<Vec<Horse>> = Mutex::new(Vec::new());
    // static HORSES: Mutex<Vec<Horse>> = Mutex::new(Vec::new());
}

pub fn name_horses() -> Vec<String> {
    HORSES.with(|horses_mutex| {
        let horses = match horses_mutex.lock() {
            Ok(content) => content,
            Err(content) => content.into_inner(),
        };
        
        horses.iter().map(|horse| horse.name).collect::<Vec<String>>()
        }
    )
}

pub fn add_horse(name: String, colour: String, alpha: f32, gamma: f32, omega: f32) {
    HORSES.with(|horses_mutex| {
        let mut horses = match horses_mutex.lock() {
            Ok(content) => content,
            Err(content) => content.into_inner(),
        };
        let mut horses = match horses_mutex.lock() {
            Ok(content) => content,
            Err(content) => content.into_inner(),
        };
        horses.push(Horse::new(name, colour, alpha, gamma, omega));
    });
}

fn clear_horses() {
    HORSES.with(|horses_mutex| {
        let mut horses = match horses_mutex.lock() {
            Ok(content) => content,
            Err(content) => content.into_inner(),
        };
        *horses = Vec::new();
    });
}

/// function to take in total value of bets

fn TakeTotalValue() {
    HORSES.with(|horses_mutex| {
        let mut horses = match horses_mutex.lock() {
            Ok(content) => content,
            Err(content) => content.into_inner(),
        };
        let mut totalForAll = 0;

        for horse in horses.iter() {
            // each horse
            // add total bets on horse to new var
            totalForAll += horse.totalBetsValue;
        }
    });
}

/// take 15% cut

fn TakeCut(totalForAll: i32) {
    let cut = 0.15;

    let availablePrize = (totalForAll as f32 * (1.0 - cut)).floor() as i32;
}

/// for each horse
/// take total bets on horse away from available prize
/// divide by total bets on horse
/// round down if necessary
/// convert to fractional odds for display

fn CalcOdds(availablePrize: i32) {
    HORSES.with(|horses_mutex| {
        let mut horses = match horses_mutex.lock() {
            Ok(content) => content,
            Err(content) => content.into_inner(),
        };

        for horse in horses.iter_mut() {
            let mut a = availablePrize - horse.totalBetsValue;
            let mut b = horse.totalBetsValue;

            let divider = rust_math::num::gcd(a, b);

            a = a / divider;
            b = b / divider;

            horse.fractionalOdds = format!("{a} to {b}");
        }
    });
}

/// for each individual bet
/// bet * decimal odd for horse + bet
/// add to user balance

fn Payouts(winner: usize, playerId: String) {
    HORSES.with(|horses_mutex| {
        let mut horses = match horses_mutex.lock() {
            Ok(content) => content,
            Err(content) => content.into_inner(),
        };

        let Some(horse) = horses.get(winner) else {
            return;
        };

        for bet in horse.bets.iter() {
            let prizeMoney = ((bet.value) as f32 * (horse.decimalOdds + 1.0)) as i32;

            //add to balance
            balance_add(&playerId, prizeMoney);
        }
    });
}
