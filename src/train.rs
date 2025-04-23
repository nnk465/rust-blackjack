use std::collections::HashMap;
use crate::blackjack::*;
use rand::seq::SliceRandom;


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
    let _all_actions = vec![
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

//pub fn test_model(model: &Vec<Vec<Action>>, game: &Game) {}


fn number_hit(game: Game, pn: usize) -> usize {
    let mut hand = game.player[pn].cards.clone();
    let mut scores:Vec<u8> = vec![0; 10];
    for i in 0..game.deck.len() {
        let (score, aces) = calculate_score(&hand, true);
        scores[i] = score as u8;        
        if score > 21 && aces == 0 {
            let (mut max, _) = scores.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
            max -= 1;
            //println!("{scores:?}");
            //println!("i: {max}");
            return max;
        }
        hand.push(game.deck[i].clone()); 
    }
    panic!("No more cards in the deck to hit");
}

fn try_action(game: &mut Game, actions: Vec<Action>, pn: usize) -> f64 {
    for act in actions{
        game.play(&act, false, pn);
    }
    game.dealer_turn();
    game.result(Some(&pn))
}

//fn try_split(mut game: Game, pn: usize) -> f64 {
//    println!("try split");
//    game.split(pn);
//    game.show();
//    let pn2 = game.player.len() - 1;
//    let bm1 = best_mooves(&game, pn);
//
//    println!("paquet {}: {:?}", pn, bm1);
//    for act in bm1.clone(){
//        game.play(&act, false, pn);
//    }
//    game.show();
//    let bm2 = best_mooves(&game, pn2);
//    println!("paquet {}: {:?}", pn2, bm2);
//    let result = try_action(&mut game, bm2, pn2);
//    //println!("paquet {}: {:?}, paquet {}: {:?}", pn, bm1, pn2, pn2); 
//    
//    //game.show();
//    //println!("result: {}", result);
//    result
//
//}

fn is_splitable(hand:&Vec<Card>) -> bool{
    hand.len() == 2 && hand[0] == hand[1]
}
fn try_split(mut game: Game, pn: usize) -> f64 {
    game.split(pn);
    let pn2 = game.player.len() - 1;
    let mut possible_mooves1 = vec![vec![Action::Hit], vec![Action::Hit; 2], vec![Action::Hit; 3], vec![Action::Hit; 4], vec![Action::Hit; 5], vec![Action::Hit; 6], vec![Action::Hit; 7], vec![Action::Hit; 8],
    vec![Action::Stand]];
    if is_splitable(&game.player[pn].cards){possible_mooves1.push(vec![Action::Split(pn as i8)]);};
    if game.player[pn].cards.len() == 2{possible_mooves1.push(vec![Action::Double])}

    let mut possible_mooves2 = vec![vec![Action::Hit], vec![Action::Hit; 2], vec![Action::Hit; 3], vec![Action::Hit; 4], vec![Action::Hit; 5], vec![Action::Hit; 6], vec![Action::Hit; 7], vec![Action::Hit; 8], 
    vec![Action::Stand]];
    if is_splitable(&game.player[pn2].cards){possible_mooves2.push(vec![Action::Split(pn2 as i8)]);};
    if game.player[pn2].cards.len() == 2{possible_mooves2.push(vec![Action::Double])}
    
    let mut best_score = f64::NEG_INFINITY;
    let mut bmm = [&possible_mooves1[0], &possible_mooves2[0]];
    for act1 in possible_mooves1.clone() {
        for act2 in possible_mooves2.clone() {
            let mut cloned_game = game.clone();
            for moove in &act1{
                cloned_game.play(moove, false, pn);    
            }
            for moove in &act2{
                cloned_game.play(moove, false, pn2);    
            }
            // simule jusqu'à la fin
            cloned_game.dealer_turn(); // ou try_action ou autre
            let score = cloned_game.result(None);

            if score > best_score {
                best_score = score;
                let bmm = [&act1, &act2];
                cloned_game.show();
                println!("score: {score}");
                println!("best mooves after split: \n  {bmm:?}");
            }
        }
    }
//    println!("best mooves after split: \n  {bmm:?}");
    best_score
}

//differents strats:
// Double
// Hit (max to find) => Stand
// Split

pub fn best_mooves(game: &Game, pn: usize) -> Vec<Action> {
    println!("traitement du paquet {:?}", game.player[pn].cards);
    let mut actions = vec![];

    let double_result = try_action(&mut game.clone(), vec![Action::Double], pn);

    let nhit = number_hit(game.clone(), pn);
    let hit_result = try_action(&mut game.clone(), vec![Action::Hit; nhit], pn);
    let hand = &game.player[pn].cards;
    let splitable = is_splitable(hand);
    let split_result = if splitable {
        try_split(game.clone(), pn)        
    } else {
        -500.0
    };
    let stand_result = try_action(&mut game.clone(), vec![], pn);
    if splitable && split_result > hit_result && split_result > double_result && split_result > stand_result {
        actions.push(Action::Split(pn.try_into().unwrap()));
    } else if double_result > hit_result && double_result > stand_result {
        actions.push(Action::Double);
    } else if hit_result > stand_result && nhit > 0 {
        actions.push(Action::Hit);
    } else if stand_result > double_result && stand_result > hit_result || nhit == 0 {
        actions.push(Action::Stand);
    } else{
        //println!("égal");
        actions.push(Action::Hit);
    }
    println!("Paquet {}: Hit x{}: {}, Double: {}, Split: {}, Stand {}",pn, nhit,  hit_result, double_result, if splitable {split_result} else {999.999}, stand_result);
    actions
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