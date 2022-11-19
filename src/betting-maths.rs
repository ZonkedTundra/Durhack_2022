use probability_to_friendly_string::FriendlyProbability;
extern crate lazy_static;


/// function to take in total value of bets

fn TakeTotalValue()
{

    for// each horse
    /// add total bets on horse to new var
        let totalForAll = ///


}



/// take 15% cut

fn TakeCut()
{

    let cut = 0.15

    let availablePrize = totalForAll - (totalForAll * cut)

}


/// for each horse
/// take total bets on horse away from available prize
/// divide by total bets on horse
/// round down if necessary
/// convert to fractional odds for display

fn CalcOdds()
{

    for //each horse

        let decimalOdds[] = (availablePrize - totalBets[]) / totalBets[]
        decimalOdds[] = math::round::floor(decimalOdds,2)

}



/// for each individual bet
/// bet * decimal odd for horse + bet
/// add to user balance 

fn payouts()
{

    for //each user
        for //each bet
            moneyPrize = (amountBet * decimalOdds[]) + amountBet

}