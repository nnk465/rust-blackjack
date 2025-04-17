mod strat;
use strat::*;


enum HandType {
    Hard(u8), // total (par exemple 5 à 21)
    Soft(u8), // total soft, de 13 (A+2) à 20 (A+9)
    Pair(u8), // la valeur de la carte en double (11 pour As, puis 10, 9, …, 2)
}

/// Pour les mains "hard" de total 5 à 21.
/// Les lignes (index 0 correspond à total=5, index 16 à total=21).
/// Colonnes : dealer 2 à 10 (indices 0..8) et 11 pour l’As (index 9)
fn get_hard_action(player_total: u8, dealer_upcard: u8, strategy: &[Vec<Action>]) -> Action {
    let row = (player_total - 5) as usize;
    let col = (dealer_upcard -2) as usize;
    strategy[row][col]
}

fn get_soft_action(soft_total: u8, dealer_upcard: u8, strategy: &[Vec<Action>]) -> Action {
    let row = (soft_total - 13) as usize;
    let col = (dealer_upcard - 2) as usize;
    strategy[row][col]
}

fn get_pair_action(pair_value: u8, dealer_upcard: u8, strategy: &[Vec<Action>]) -> Action {
    let row = (pair_value/2 - 2) as usize;
    let col = (dealer_upcard - 2) as usize;
    strategy[row][col]
}

fn get_action(hand: HandType, dealer_upcard: u8, 
              hard_strategy: &[Vec<Action>],
              soft_strategy: &[Vec<Action>],
              pair_strategy: &[Vec<Action>]) -> Action {
    match hand {
        HandType::Hard(total) => get_hard_action(total, dealer_upcard, hard_strategy),
        HandType::Soft(total) => get_soft_action(total, dealer_upcard, soft_strategy),
        HandType::Pair(value) => get_pair_action(value, dealer_upcard, pair_strategy),
    }
}
fn main() {
    let action1 = get_action(HandType::Hard(10), 9, &STRAT_HARD.as_slice(), &STRAT_SOFT.as_slice(), &STRAT_PAIR.as_slice());
    println!("Hard(10) vs Dealer 9 => Action: {:?}", action1);

    let action2 = get_action(HandType::Soft(18), 6, &STRAT_HARD.as_slice(), &STRAT_SOFT.as_slice(), &STRAT_PAIR.as_slice());
    println!("Soft(18) vs Dealer 6 => Action: {:?}", action2);

    let action3 = get_action(HandType::Pair(16), 10, &STRAT_HARD.as_slice(), &STRAT_SOFT.as_slice(), &STRAT_PAIR.as_slice());
    println!("Pair(8,8) vs Dealer 10 => Action: {:?}", action3);
}