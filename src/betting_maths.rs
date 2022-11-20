use lazy_static::lazy_static;
extern crate lazy_static;
use std::sync::{Arc, Mutex};

//let numOfHorses = //

struct Bet {
    playerId: String,
    value: i32,
    
}

fn add_bet(playerId: String, horseId: usize, betValue: i32) -> Result<(), ()> {
    let horses_arc = HORSES.clone();
    let mut horses = match horses_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };

    let Some(horse) = horses.get_mut(horseId) else {
        return Err(());
    };

    let bet = Bet {
        playerId,
        value: betValue,
        
    };

    horse.bets.push(bet);


    //take from user balance 
    // balance - horse.bet.value ???


    Ok(())
}

/// horse struct

struct Horse {
    name: String,
    colour: String,
    totalBetsValue: i32,
    decimalOdds: f32,
    fractionalOdds: String,
    bets: Vec<Bet>,
}

lazy_static! {
    //vector of horses
    static ref HORSES: Arc<Mutex<Vec<Horse>>> = Arc::new(Mutex::new(Vec::new()));
}

fn add_horse(
    name: String,
    colour: String,
    totalBetsValue: i32,
    decimalOdds: f32,
    fractionalOdds: String,
) {
    let horses_arc = HORSES.clone();
    let mut horses = match horses_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };
    horses.push(Horse {
        name,
        colour,
        totalBetsValue,
        decimalOdds,
        fractionalOdds,
        bets: Vec::new(),
    });
}

fn clear_horses() {
    let horses_arc = HORSES.clone();
    let mut horses = match horses_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };
    horses.clear();
}

/// user struct

/*struct User {
    name: String,
    numOfBets: i64,
    // horsesBetOn : array : [string; numOfHorses],
    // valueOfEachBet : array : [int64; numOfHorses],
    balance: i64,
}

//vector of users

lazy_static! {
    //array of horses
    static ref USERS: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
}*/

/// function to take in total value of bets

fn TakeTotalValue()
{
    let horses_arc = HORSES.clone();
    let horses = match horses_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };

    let mut totalForAll = 0;

    for horse in horses.iter()
    // each horse
    {
        // add total bets on horse to new var
        totalForAll += horse.totalBetsValue;
    }
    
}

/// take 15% cut

fn TakeCut(totalForAll : i32) {
    let cut = 0.15;

    let availablePrize = (totalForAll as f32 * (1.0 - cut)).floor() as i32;
}

/// for each horse
/// take total bets on horse away from available prize
/// divide by total bets on horse
/// round down if necessary
/// convert to fractional odds for display

fn CalcOdds(availablePrize : i32) 
{
    let horses_arc = HORSES.clone();
    let mut horses = match horses_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };


    for horse in horses.iter_mut()
    {
        let mut a = availablePrize - (horse.totalBetsValue);
        let mut b = horse.totalBetsValue;
        

        let divider = rust_math::num::gcd(a,b);

        a = a/divider;
        b = b/divider;


        horse.fractionalOdds = format!("{a} to {b}");
    }
}

/// for each individual bet
/// bet * decimal odd for horse + bet
/// add to user balance

/*fn payouts() {
    for i in USERS
    //each user
    {
        for j in USERS[User.horsesBetOn]
        //each horse bet on
        {
            if USERS[User.horsesBetOn[j]] == winner {
                moneyPrize = (USERS[User.valueOfEachBet[j]] * HORSES[winner.decimalOdds])
                    + USERS[User.valueOfEachBet[j]];

                USERS[User.balance] = USERS[User.balance] + moneyPrize
            }
        }
    }
}*/

fn Payouts(winner: usize)
{
    let horses_arc = HORSES.clone();
    let horses = match horses_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };


    let Some(horse) = horses.get(winner) else {
        return
    };
    
    for bet in horse.bets.iter()
    {
        let prizeMoney = (bet.value) as f32 * (horse.decimalOdds + 1.0);

        //add to balance
        todo!()
    }
    
}