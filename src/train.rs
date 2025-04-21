use std::collections::HashMap;
use crate::blackjack::*;
use rand::seq::SliceRandom;

enum Strat{
    Hit(u8),
    Double,
    Split(Vec<Strat>, Vec<Strat>),
}


fn most_common<T: Eq + std::hash::Hash + Clone>(vec: &[T]) -> Option<T> {
    let mut freq = HashMap::new();

    for item in vec {
        *freq.entry(item.clone()).or_insert(0) += 1;
    }

    freq.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
}

pub fn train_ia() -> Vec<Vec<Action>>{
    let mut model: Vec<Vec<Action>> = vec![vec![Action::Stand; 10]; 100];
    let all_cards = vec![
        Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five,
        Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
    ];
    let all_actions = vec![
        Action::Hit, Action::Stand, Action::Double, Action::Split(0),
    ];
    let values = HashMap::from([
        (Card::Ace, 0 as usize),
        (Card::Two, 1 as usize),
        (Card::Three, 2 as usize),
        (Card::Four, 3 as usize),
        (Card::Five, 4 as usize),
        (Card::Six, 5 as usize),
        (Card::Seven, 6 as usize),
        (Card::Eight, 7 as usize),
        (Card::Nine, 8 as usize),
        (Card::Ten, 9 as usize),
    ]);
    for pcard1 in &all_cards{
        for pcard2 in &all_cards{
            for dcard in &all_cards{
                let mut game = Game::new(10);
                game.deal_choosen_card(vec![pcard1.clone(), pcard2.clone()], true, 0);
                game.deal_choosen_card(vec![dcard.clone()], false, 0);
                game.deal_to_dealer(1);
                let mut acts = vec![];
                for _ in 0..20{
                    acts.push(best_mooves(&game, 0));
                    game.deck.shuffle(&mut rand::rng());
                };
                let best_moove = most_common(&acts);
                model[10*values[pcard1]+values[pcard2]][values[dcard]] = best_moove.unwrap()[0];
            }
        }
    }
    model
}

pub fn test_model(model: &Vec<Vec<Action>>, game: &Game) -> Vec<Action> {
    let mut actions = vec![];
    let hand = &game.player[0].cards;
    let dealer_card = game.dealer[0];
    let hand_value = calculate_score(hand, true).0;
    let dealer_value = calculate_score(&vec![dealer_card], true).0;
    if hand.len() == 2 && hand[0] == hand[1] {
        actions.push(model[10*hand_value as usize + dealer_value as usize][0].clone());
    } else {
        actions.push(model[10*hand_value as usize + dealer_value as usize][0].clone());
    }
    actions
}


fn number_hit(game: Game, pn: usize) -> usize {
    let mut hand = game.player[pn].cards.clone();
    let mut scores:Vec<u8> = vec![0; game.deck.len()];
    for i in 0..game.deck.len() {
        let (score, aces) = calculate_score(&hand, true);
        scores[i] = score as u8;        
        if score > 21 && aces == 0 {
            let (i, _) = scores.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
            return i + 1;
        }
        hand.push(game.deck[i].clone()); 
    }
    panic!("No more cards in the deck to hit");
}

fn try_action(mut game: Game, actions: Vec<Action>, pn: usize) -> f64 {
    for act in actions{
        game.play(&act, false, pn);
    }
    game.dealer_turn();
    game.result()
}

fn try_split(mut game: Game, pn: usize) -> f64 {
    game.split(pn);
    let pn2 = game.player.len() - 1;
    let bm1 = best_mooves(&game, pn);
    let bm2 = best_mooves(&game, pn2);
    let result1 = try_action(game.clone(), bm1, pn);
    let result2 = try_action(game.clone(), bm2, pn2);
    result1 + result2 
}

//differents strats:
// Double
// Hit (max to find) => Stand
// Split

pub fn best_mooves(game: &Game, pn: usize) -> Vec<Action> {
    println!("traitement du paquet {pn}");
    let mut actions = vec![];

    let double_result = try_action(game.clone(), vec![Action::Double], pn);

    let nhit = number_hit(game.clone(), pn);
    let hit_result = try_action(game.clone(), vec![Action::Hit; nhit], pn);
    let hand = &game.player[pn].cards;
    let splitable = hand.len() == 2  && hand[0] == hand[1];
    let split_result = if splitable {
        try_split(game.clone(), pn)
        
    } else {
        0.0
    };
    if splitable && split_result > hit_result && split_result > double_result {
        actions.push(Action::Split(pn.try_into().unwrap()));
    } else if double_result > hit_result {
        actions.push(Action::Double);
    } else if hit_result > double_result {
        actions.push(Action::Hit);
    } else {
        println!("égal");
        actions.push(Action::Stand);
    }
    println!("Hit: {}, Double: {}, Split: {}", hit_result, double_result, split_result);
    actions
}

fn main(){
    
}


// fn test_strat(game: &mut Game,) ->f64 {
//     let mut pn = 0;
//     loop{
//         loop{
//             let hand = game.player[pn].cards.clone();
//             if hand.len() == 0 || calculate_score(&hand, true).0 > 21 {break}
//             let mut act = strat_optimale(&game, &hand);
//             if act == Action::Stand {break};
//             if act == Action::Split(0){act = Action::Split((pn as i32).try_into().unwrap());};
//             game.play(&act, false, pn);
//         }
//         pn += 1;
//         if pn >= game.player.len() {break;}
//     }
//     game.dealer_turn();
//     game.result()
// 
// }