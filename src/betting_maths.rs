use lazy_static::lazy_static;
use probability_to_friendly_string::FriendlyProbability;
extern crate lazy_static;
use std::sync::{Arc, Mutex};
//let numOfHorses = //

/// horse struct

struct Horse
{
    name : String,
    colour : String,
    totalBetsValue : i64,
    decimalOdds: f64,
    fractionalOdds: String,
}

lazy_static!{
    //vector of horses
    static ref HORSES: Arc<Mutex<Vec<Horse>>> = Arc::new(Mutex::new(Vec::new()));
}

fn add_horse(name: String, colour: String, totalBetsValue: i64, decimalOdds: f64, fractionalOdds: String) {
    let horses_arc = HORSES.clone();
    let mut horses = match horses_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };
    *horses.push(horse { name, colour, totalBetsValue, decimalOdds, fractionalOdds });
}

fn clear_horses() {
    let horses_arc = HORSES.clone();
    let mut horses = match horses_arc.lock() {
        Ok(content) => content,
        Err(content) => content.into_inner(),
    };
    *horses.clear();
}

/// user struct

struct User
{
    name : String,
    numOfBets : i64,
    horsesBetOn : array : [string; numOfHorses],
    valueOfEachBet : array : [int64; numOfHorses],
    balance : int64,
}

//vector of users

lazy_static!{
    //array of horses
    static ref USERS: Arc<Mutex<Vec<User>>> = Arc::new(Mutex::new(Vec::new()));
}


/// function to take in total value of bets

fn TakeTotalValue()
{

    for i in HORSES  // each horse
    {
        // add total bets on horse to new var
        let totalForAll = totalForAll + HORSES[Horse.totalBetsValue];
        
    }
    


}



/// take 15% cut

fn TakeCut()
{

    let cut = 0.15;

    let availablePrize = totalForAll - (totalForAll * cut);

}


/// for each horse
/// take total bets on horse away from available prize
/// divide by total bets on horse
/// round down if necessary
/// convert to fractional odds for display

fn CalcOdds()
{

    for i in HORSES//each horse
    {
        HORSES[Horse.decimalOdds[i]] = (availablePrize - HORSES[Horse.totalBetsValue]) / HORSES[Horse.totalBetsValue];
        HORSES[Horse.decimalOdds[i]] = math::round::floor(HORSES[Horse.decimalOdds[i]],2);

        HORSES[Horse.fractionalOdds[i]] = HORSES[Horse.decimalOdds[i].friendly_string()]
    }

        

}



/// for each individual bet
/// bet * decimal odd for horse + bet
/// add to user balance 

fn payouts()
{

    for i in USERS//each user
    {
        for j in USERS[User.horsesBetOn] //each horse bet on 
        {
            if USERS[User.horsesBetOn[j]] == winner
            {
                moneyPrize = (USERS[User.valueOfEachBet[j]] * HORSES[winner.decimalOdds]) + USERS[User.valueOfEachBet[j]];

                USERS[User.balance] = USERS[User.balance] + moneyPrize
            }

            
        }

            

    }
        
}