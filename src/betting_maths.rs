use lazy_static::lazy_static;
use probability_to_friendly_string::FriendlyProbability;
extern crate lazy_static;

//let numOfHorses = //

/// horse struct

struct horse
{
    name : String,
    colour : String,
    totalBetsValue : int64,
    decimalOdds: f64,
    fractionalOdds: String,
}

lazy_static!{
    //array of horses
    static ref HORSES: Arc<Mutex<Vec<horse>>> = Arc::new(Mutex::new(Vec::new()))
}

fn add_horse(name: String, colour: String, totalBetsValue: int64, decimalOdds: f64, fractionalOdds: String) {
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

struct user
{
    name : string,
    numOfBets : int64,
    horsesBetOn : array : [string; numOfHorses],
    valueOfEachBet : array : [int64; numOfHorses],
    balance : int64,
}

///array of users
let users array: [struct; numOfHorses] = [user{}, numOfHorses]


/// function to take in total value of bets

fn TakeTotalValue()
{

    for i in horses  // each horse
    {
        /// add total bets on horse to new var
        let totalForAll = totalForAll + horses[horse.totalBetsValue];
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

    for i in horses//each horse
    {
        horses[horse.decimalOdds[i]] = (availablePrize - horses[horse.totalBetsValue]) / horses[horse.totalBetsValue.];
        horses[horse.decimalOdds[i]] = math::round::floor(horses[horse.decimalOdds[i]],2);

        horses[horse.fractionalOdds[i]] = horses[horse.decimalOdds[i].friendly_string()]
    }

        

}



/// for each individual bet
/// bet * decimal odd for horse + bet
/// add to user balance 

fn payouts()
{

    for i in users//each user
    {
        for j in user[user.horsesBetOn] //each horse bet on 
        {
            if users[user.horsesBetOn[j]] == winner
            {
                moneyPrize = (users[user.valueOfEachBet[j]] * horses[winner.decimalOdds]) + users[user.valueOfEachBet[j]];

                users[user.balance] = users[user.balance] + moneyPrize
            }

            
        }

            

    }
        
}