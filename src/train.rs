use crate::blackjack::*;
use std::collections::HashMap;
use std::fs::File;
use serde_json;
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

pub fn get_row_model2(pcard1: &Card, pcard2: &Card) -> usize {
    // formule pour associer une main a une ligne du model :
    let pp = vec![0, 10, 9, 8, 7, 6, 5, 4, 3, 2];
    (pp[0..card_index(pcard1)+1].iter().sum::<usize>() + card_index(pcard2) - card_index(pcard1)) as usize
}
pub fn train_ia2(iter_training: i32) -> Vec<Vec<Action>>{
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
                let mut ev_sum = [0.0 as f64; 4];
                let mut valid = [false; 4];
                let diff = 1.0;
                let mut max_gap = 0.1;
                let mut opp:u64 = 0;
                while diff > max_gap {
                    for _ in 0..iter_training {
                        opp+=1;
                        game.deck.shuffle(&mut rand::rng());
                        game.deal_to_dealer(1);
                        let results = try_actions(&game, 0);
                        //if results[0] == results[1]{
                        //    println!("{} : {:?}", opp, game.deck);
                        //    game.show();
                        //    println!("results: {:?}\n", results);
                        //    println!("{:?}  {}", ev_sum, max_gap);
                        //}
                        for (i, result) in results.iter().enumerate() {
                            ev_sum[i] += match *result {
                                f64::NEG_INFINITY => 0.0,
                                _ => {if !valid[i] {valid[i] = true};
                                *result }
                            }
                        }
                        game.deck.push(game.dealer.remove(1));
                    }
                    let mut v = ev_sum.clone();
                    v.sort_by(|a, b| b.partial_cmp(a).unwrap());
                    let diff = (v[0] - v[1]) / v[0];
                    max_gap *= 1.05;
                    if diff > max_gap {
                        println!("diff: {}", diff);
                        println!("ev_sum: {:?}", ev_sum);
                        println!("{:?} - {:?} | {:?} ", pcard1, pcard2, dcard);
                    }
                }
                println!("{ev_sum:?}");
                let best = ev_sum.iter().enumerate()
                .filter(|(i, _)| valid[*i])
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap()).unwrap().0;
                let best_action = match best {
                    0 => Action::Stand,
                    1 => Action::Hit,
                    2 => Action::Double,
                    3 => Action::Split(0),
                    _ => unreachable!(),
                };
                let row = get_row_model2(pcard1, pcard2);
                model[row][card_index(dcard)] = best_action;
                //println!("{:?} + {:?} => row {:?}", pcard1, pcard2, get_row_model(pcard1, pcard2));
            }
        }
    }
    model
}

pub fn get_row_model3(pcard1: &Card, pcard2: &Card, pcard3: &Card) -> usize {
    let mut v = [pcard1.value() -1, pcard2.value()-1, pcard3.value()-1];
    v.sort_unstable();
    let (x, y, z) = (v[0], v[1], v[2]);
    let mut idx = 0;

    // 1) pour tout i0 < x
    for i0 in 0..x {
        for j0 in i0..10 {
            for k0 in j0..10 {
                if i0 + j0 + k0 <= 21 {
                    idx += 1;
                }
            }
        }
    }

    // 2) pour i0 == x, j0 < y
    for j0 in x..y {
        for k0 in j0..10 {
            if x + j0 + k0 <= 21 {
                idx += 1;
            }
        }
    }

    // 3) pour i0 == x, j0 == y, k0 < z
    for k0 in y..z {
        if x + y + k0 <= 21 {
            idx += 1;
        }
    }

    idx
}

    
pub fn train_ia3(i: i32) -> Vec<Vec<Action>>{
    let mut model: Vec<Vec<Action>> = vec![vec![Action::Stand; 10]; 200];
    let all_cards = vec![
        Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five,
        Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
    ];
    let mut game = Game::new(10);
    for pcard1 in &all_cards{
        for pcard2 in &all_cards{
            for pcard3 in &all_cards{
                if pcard1.value() + pcard2.value() + pcard3.value() > 21 {continue;}
                if card_index(pcard2)<card_index(pcard1){continue;}
                for dcard in &all_cards{
                    game.reset(10, true);
                    game.deal_choosen_card(vec![pcard1.clone(), pcard2.clone(), pcard3.clone()], true, 0);
                    game.deal_choosen_card(vec![dcard.clone()], false, 0);

                    game.deal_to_dealer(1);
                    let mut ev_sum = [0.0; 2];
                    let mut count  = [0;   2];

                    for _ in 0..i {
                        let (act, ev) = best_mooves(&game, 0);
                        let idx = match act[0] {
                            Action::Stand   => 0,
                            Action::Hit     => 1,
                            _ => panic!("unexpected action with 3 cards"),
                        };
                        ev_sum[idx] += ev;
                        count[idx]  += 1;
                    }

                    // calcule l'EV moyen
                    let ev_mean: Vec<f64> = ev_sum.iter()
                        .zip(count.iter())
                        .map(|(&sum, &c)| if c>0 { sum / (c as f64) } else { f64::NEG_INFINITY })
                        .collect();

                    // choisis l'action au plus grand EV moyen
                    let best = ev_mean.iter()
                        .enumerate()
                        .max_by(|a,b| a.1.partial_cmp(b.1).unwrap())
                        .unwrap().0;
                    let best_action = match best {
                        0 => Action::Stand,
                        1 => Action::Hit,
                        _ => unreachable!(),
                    };
                    let row = get_row_model3(pcard1, pcard2, pcard3);
                    model[row][card_index(dcard)] = best_action;
                    //println!("{:?} + {:?} => row {:?}", pcard1, pcard2, get_row_model(pcard1, pcard2));
                }
            }
        }
    }
    model
}

pub fn save_model(model: &Vec<Vec<Action>>, filename: &str) {
    let f = File::create(filename).expect("Unable to create file");
    serde_json::to_writer(f, &model).expect("Unable to write data");
}

pub fn load_model(filename: &str) -> Vec<Vec<Action>> {
    let f = File::open(filename).expect("Unable to open file");
    let loaded: Vec<Vec<Action>> = serde_json::from_reader(f).expect("Unable to read data");
    loaded
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

pub fn try_actions(game: &Game, pn: usize) -> Vec<f64>{
    let hand = &game.player[pn].cards;

    let stand_result = try_action(&mut game.clone(), vec![Action::Stand], pn);    

    let nhit = number_hit(game.clone(), pn).max(1);
    let hit_result = try_action(&mut game.clone(), vec![Action::Hit; nhit], pn);    

    let is_doublable = hand.len() == 2;
    let double_result = if is_doublable{try_action(&mut game.clone(), vec![Action::Double], pn)} else {f64::NEG_INFINITY};

    let split_result = if is_splitable(hand) {fast_try_split(game.clone(), pn)} else {f64::NEG_INFINITY};
    
    vec![stand_result, hit_result, double_result, split_result]

}
pub fn best_mooves(game: &Game, pn: usize) -> (Vec<Action>, f64 ){
    //println!("traitement du paquet {:?}", game.player[pn].cards);

    let results = try_actions(&game, pn);
    let nhit = number_hit(game.clone(), pn);

    let (stand_result, hit_result, double_result, split_result) = (results[0], results[1], results[2], results[3]);
    if is_splitable(&game.player[pn].cards) && split_result > hit_result && split_result > double_result && split_result > stand_result {
        return (vec![Action::Split(pn.try_into().unwrap())], split_result);
    } else if double_result > hit_result && double_result > stand_result {
        return (vec![Action::Double], double_result);
    } else if hit_result > stand_result && nhit> 0 {
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
    let row = get_row_model2(&hand[0], &hand[1]);
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