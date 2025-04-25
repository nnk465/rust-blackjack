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

pub fn get_row_model(pcard1: &Card, pcard2: &Card) -> usize {
    // formule pour associer une main a une ligne du model :
    let pp = vec![0, 10, 9, 8, 7, 6, 5, 4, 3, 2];
    (pp[0..card_index(pcard1)+1].iter().sum::<usize>() + card_index(pcard2) - card_index(pcard1)) as usize
}
pub fn train_ia() -> Vec<Vec<Action>>{
    let mut model: Vec<Vec<Action>> = vec![vec![Action::Stand; 10];   55];
    let all_cards = vec![
        Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five,
        Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
    ];
    let _all_actions = vec![
        Action::Hit, Action::Stand, Action::Double, Action::Split(0),
    ];
    let mut game = Game::new(10);
    for pcard1 in &all_cards{
        for pcard2 in &all_cards{
            if card_index(pcard2)<card_index(pcard1){continue;}
            for dcard in &all_cards{
                game.reset(10, true);
                game.deal_choosen_card(vec![pcard1.clone(), pcard2.clone()], true, 0);
                game.deal_choosen_card(vec![dcard.clone()], false, 0);
                game.deal_to_dealer(1);
                let mut acts = vec![f64::NEG_INFINITY; 4];
                for _ in 0..10000{
                    let (act, result) = best_mooves(&game, 0);
                    game.deck.shuffle(&mut rand::rng());
                    match act[0] {
                        Action::Stand => {if acts[0] == f64::NEG_INFINITY{acts[0] = 0.0};acts[0] += result},
                        Action::Hit => {if acts[1] == f64::NEG_INFINITY{acts[1] = 0.0};acts[1] += result},
                        Action::Double => {if acts[2] == f64::NEG_INFINITY{acts[2] = 0.0};acts[2] += result},
                        Action::Split(_) => {if acts[3] == f64::NEG_INFINITY{acts[3] = 0.0};acts[3] += result},
                    }
                };
                let row = get_row_model(pcard1, pcard2);
                let (mut max, _) = acts.iter().enumerate().max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap();
                let best_moove = match max {
                    0 => Action::Stand,
                    1 => Action::Hit,
                    2 => {//println!("Double: ");
                    //println!("{:?}", game.deck);
                    //game.show();
                    Action::Double}
                    3 => Action::Split(0), 
                    _ => panic!("Invalid action"),
                };
                let n = acts.len().min(10);
                //we put back the cards in the deck                    
                //game.deck.push(dcard.clone());
                //game.deck.push(pcard1.clone());
                //game.deck.push(pcard2.clone());
                model[row][card_index(dcard)] = best_moove;
                //println!("{:?} + {:?} => row {:?}", pcard1, pcard2, get_row_model(pcard1, pcard2));
            }
        }
    }
    model
}

fn card_index(c: &Card) -> usize {
    match c {
      Card::Ace   => 0,
      Card::Two   => 1,
      Card::Three => 2,
      Card::Four  => 3,
      Card::Five  => 4,
      Card::Six   => 5,
      Card::Seven => 6,
      Card::Eight => 7,
      Card::Nine  => 8,
      _   => 9,
    }
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


fn is_splitable(hand:&Vec<Card>) -> bool{
    hand.len() == 2 && hand[0] == hand[1]
}
fn _real_try_split(mut game: Game, pn: usize) -> f64 {
    
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
    for act1 in possible_mooves1.clone() {
        for act2 in possible_mooves2.clone() {
            let mut cloned_game = game.clone();
            for moove in &act1{
                cloned_game.play(moove, false, pn);    
            }
            for moove in &act2{
                cloned_game.play(moove, false, pn2);    
            }
            cloned_game.dealer_turn();
            let score = cloned_game.result(None);

            if score > best_score {
                best_score = score;
            }
        }
    }
    best_score
}

fn fast_try_split(mut game: Game, pn: usize) -> f64 {
    
    game.split(pn);
    let pn2 = game.player.len() - 1;
    
    best_mooves(&game, pn).1 + best_mooves(&game, pn2).1
    }

//differents strats:
// Double
// Hit (max to find) => Stand
// Split

pub fn best_mooves(game: &Game, pn: usize) -> (Vec<Action>, f64 ){
    //println!("traitement du paquet {:?}", game.player[pn].cards);
    let double_result = try_action(&mut game.clone(), vec![Action::Double], pn);

    let nhit = number_hit(game.clone(), pn);
    let hit_result = try_action(&mut game.clone(), vec![Action::Hit; nhit], pn);
    
    let hand = &game.player[pn].cards;
    let splitable = is_splitable(hand);
    let split_result = if splitable {
        fast_try_split(game.clone(), pn)        
    } else {
        -500.0
    };
    let stand_result = try_action(&mut game.clone(), vec![], pn);
    if splitable && split_result > hit_result && split_result > double_result && split_result > stand_result {
        return (vec![Action::Split(pn.try_into().unwrap())], split_result);
    } else if double_result > hit_result && double_result > stand_result {
        return (vec![Action::Double], double_result);
    } else if hit_result > stand_result && nhit > 0 {
        return (vec![Action::Hit; nhit], hit_result);
    } else if stand_result > double_result && stand_result > hit_result || nhit == 0 {
        return (vec![Action::Stand], stand_result);
    } else{
        return (vec![Action::Hit; nhit], hit_result);
    }
    //println!("Paquet {}: Hit x{}: {}, Double: {}, Split: {}, Stand {}",pn, nhit,  hit_result, double_result, if splitable {split_result} else {999.999}, stand_result);
}
 
pub fn use_model(model: &[[Action; 10]; 55], game: &Game, pn: usize) -> Action {
    let hand = &game.player[pn].cards;
    let dealer_card = &game.dealer[0];
    let row = get_row_model(&hand[0], &hand[1]);
    let col = card_index(dealer_card);
    model[row][col].clone()
}

pub fn test_strat(game: &mut Game, model: &[[Action;10 ]; 55]) ->f64 {
    let mut pn = 0;
    loop{
        let hand = game.player[pn].cards.clone();
        let mut act = use_model(model, &game, pn);
        game.play(&act, false, pn);
        if act == Action::Hit{game.play(&act, false, pn);};

        pn += 1;
        if pn >= game.player.len() {break;}
    }
    game.dealer_turn();
    game.result(None)

}