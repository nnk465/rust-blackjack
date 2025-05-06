use crate::blackjack::*;
use std::collections::HashMap;
use std::fs::File;
use serde_json;
use serde::{Serialize, Deserialize};
use rand::seq::SliceRandom;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use rayon::prelude::*;


static ROWS: Lazy<HashMap<Vec<u8>, u16>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for i in 2..11 {
        let mut counter = 0u16;
        let mut hands = Vec::new();
        gen_hands(i, &Card::all_cards(), 0, &mut Vec::new(), &mut hands);
        for hand in hands {
            let mut hand_values: Vec<u8> = hand.iter().map(|h| h.value()).collect();
            hand_values.sort_unstable();
            m.insert(hand_values, counter);
            counter += 1;
        }
    }

    m
});
static MODELS: Lazy<Mutex<Vec<Vec<Vec<ActionModel>>>>> = Lazy::new(|| Mutex::new({
    let mut models = vec![];
    for i in 2..11{
    let model = load_model(&format!("models/model{}.json", i));
    models.push(model);}
    models}));
    
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ActionModel {
    Stand(f64),
    Hit(f64),
    Double(f64),
    Split(f64),
    Undifined,
}

impl ActionModel {
    pub fn to_action(&self) -> Action {
        match self {
            ActionModel::Stand(_) => Action::Stand,
            ActionModel::Hit(_) => Action::Hit,
            ActionModel::Double(_) => Action::Double,
            ActionModel::Split(_) => Action::Split(0),
            _ => panic!("Invalid action model"),
        }
    }
}
fn gen_hands(n: usize, pool: &[Card], start: usize, cur: &mut Vec<Card>, out: &mut Vec<Vec<Card>>) {
    if cur.len() == n {
        let sum: u8 = cur.iter().map(|c| c.value()).sum();
        if sum <= 21 {
            out.push(cur.clone());
        }
        return;
    }
    for i in start..pool.len() {
        cur.push(pool[i].clone());
        gen_hands(n, pool, i, cur, out);
        cur.pop();
    }

}

pub fn get_row_model(hand: &Vec<Card>) -> usize {
    let mut hand_values = hand.iter().map(|h| h.value()).collect::<Vec<u8>>();
    hand_values.sort_unstable();
    ROWS.get(&hand_values).unwrap_or_else(|| {
        panic!("Hand {:?} not found in ROWS", hand)
    }).clone() as usize
}
    
fn draw_bar(current: usize, total: usize) {
    let width = 50; // largeur de la barre
    let filled = width * current / total;
    let empty = width - filled;

    let bar: String = format!(
        "[{}{}] {}/{}",
        "#".repeat(filled),
        " ".repeat(empty),
        current,
        total
    );
    print!("\r{}", bar); // \r pour revenir au début de la ligne
    std::io::Write::flush(&mut std::io::stdout()).unwrap(); // forcer l'affichage
}

pub fn _train_ia_generic(n_cards: usize, iter_training: usize) -> Vec<Vec<ActionModel>> {
    use std::f64;

    let all_cards = Card::all_cards();
    let mut hands = Vec::new();
    gen_hands(n_cards, &all_cards, 0, &mut Vec::new(), &mut hands);
    let rows = hands.len();
    let cols = all_cards.len();

    let mut model = vec![vec![ActionModel::Undifined; cols]; rows];
    let mut game = Game::new();
    let two = n_cards == 2;
    for (row_idx, hand) in hands.iter().enumerate() {
        game.deal_choosen_card(hand.clone(), true, 0);

        for (d_idx, upcard) in all_cards.iter().enumerate() {
            game.deal_choosen_card(vec![upcard.clone()], false, 0);

            let mut sum = [0.0; 4];
            let mut sum_sq = [0.0; 4];
            let mut count = [0; 4];
            let mut valid = [false; 4];

            loop {
                // Phase de sampling parallélisée
                let results: Vec<_> = (0..200).into_par_iter().map(|_| {
                    let mut thread_game = game.clone();
                    thread_game.deal_to_dealer(1);
                    let mut local_sum = [0.0; 4];
                    let mut local_sum_sq = [0.0; 4];
                    let mut local_count = [0; 4];
                    let mut local_valid = [false; 4];
                    for _ in 0..(iter_training/200).into() {
                        thread_game.deck.shuffle(&mut rand::rng());

                        let results = if two { try_actions2(&thread_game, 0) } else { try_actions(&thread_game, 0) };

                        // Renvoyer les stats locales pour cette itération
                        

                        for (i, &res) in results.iter().enumerate() {
                            if res != f64::NEG_INFINITY {
                                local_sum[i] += res;
                                local_sum_sq[i] += res * res;
                                local_count[i] += 1;
                                local_valid[i] = true;
                            }
                        }}

                    (local_sum, local_sum_sq, local_count, local_valid)
                }).collect();

                // Aggregation des résultats
                for (local_sum, local_sum_sq, local_count, local_valid) in results {
                    for i in 0..4 {
                        sum[i] += local_sum[i];
                        if i==0 && *hand == [Card::Ace, Card::Eight]{
                            println!("{:?}: {:?}", hand, local_sum[i]);
                        }
                        sum_sq[i] += local_sum_sq[i];
                        count[i] += local_count[i];
                        valid[i] |= local_valid[i];
                    }
                }

                // Calcul des moyennes et erreurs
                let mut mean = [f64::NEG_INFINITY; 4];
                let mut se = [f64::INFINITY; 4];

                for i in 0..4 {
                    if valid[i] && count[i] > 1 {
                        mean[i] = sum[i] / count[i] as f64;
                        let variance = (sum_sq[i] / count[i] as f64) - (mean[i] * mean[i]);
                        let std_dev = variance.max(0.0).sqrt();
                        se[i] = std_dev / (count[i] as f64).sqrt();
                    }
                }

                // Détermination des deux meilleures actions
                let mut ranked: Vec<(usize, f64)> = mean.iter().enumerate().map(|(i, &v)| (i, v)).collect();
                ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                let best = ranked[0].0;
                let second = ranked[1].0;

                // Critère d'arrêt
                let gap = mean[best] - mean[second];
                let error_margin = se[best] + se[second];

                if gap > error_margin || count[best] + count[second] > 10_000 {
                    let esp = mean[best];
                    if esp == 0.0{println!("{:?} : {:?}", hand, mean);}
                    model[row_idx][d_idx] = match best {
                        0 => ActionModel::Stand(esp),
                        1 => ActionModel::Hit(esp),
                        2 => ActionModel::Double(esp),
                        3 => ActionModel::Split(esp),
                        _ => unreachable!(),
                    };
                    break;
                }
            }

            game.deck.push(game.dealer.remove(0));
            draw_bar(row_idx * cols + d_idx + 1, rows * cols);
        }

        for _ in 0..n_cards {
            game.deck.push(game.player[0].remove(0));
        }
    }

    println!();
    let mut models = MODELS.lock().unwrap();
    models[n_cards-2] = model.clone();
    model
}

pub fn train_ia_generic(n_cards: usize, iter_training: usize) -> Vec<Vec<ActionModel>> {
    use std::f64;

    let all_cards = Card::all_cards();
    let mut hands = Vec::new();
    gen_hands(n_cards, &all_cards, 0, &mut Vec::new(), &mut hands);
    let rows = hands.len();
    let cols = all_cards.len();

    let mut model = vec![vec![ActionModel::Undifined; cols]; rows];
    let mut game = Game::new();
    let two = n_cards == 2;
    for (row_idx, hand) in hands.iter().enumerate() {
        game.deal_choosen_card(hand.clone(), true, 0);

        for (d_idx, upcard) in all_cards.iter().enumerate() {
            game.deal_choosen_card(vec![upcard.clone()], false, 0);

            let mut sum = [0.0; 4];
            let mut sum_sq = [0.0; 4];
            let mut count = [0; 4];
            let mut valid = [false; 4];

            loop {
                for _ in 0..iter_training {
                    game.deck.shuffle(&mut rand::rng());
                    game.deal_to_dealer(1);

                    let results = if two{try_actions2(&game, 0) }else{try_actions(&game, 0)};

                    for (i, &res) in results.iter().enumerate() {
                        if res != f64::NEG_INFINITY {
                            sum[i] += res;
                            sum_sq[i] += res * res;
                            count[i] += 1;
                            valid[i] = true;
                        }
                    }
                    game.deck.push(game.dealer.remove(1));
                }
                // Calcul de la moyenne et de l'écart-type pour chaque action
                let mut mean = [f64::NEG_INFINITY; 4];
                let mut se = [f64::INFINITY; 4];

                for i in 0..4 {
                    if valid[i] && count[i] > 1 {
                        mean[i] = sum[i] / count[i] as f64;
                        let variance = (sum_sq[i] / count[i] as f64) - (mean[i] * mean[i]);
                        let std_dev = variance.max(0.0).sqrt();
                        se[i] = std_dev / (count[i] as f64).sqrt();
                    }
                    if i==0 && *hand == [Card::Ace, Card::Eight]{
                        println!("{:?}: {:?}", hand, sum[i]);
                    }
                }

                // Détermination des deux meilleures actions
                let mut ranked: Vec<(usize, f64)> = mean.iter().enumerate().map(|(i, &v)| (i, v)).collect();

                ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
                let best = ranked[0].0;
                let second = ranked[1].0;

                // Critère d'arrêt : la meilleure action doit être significativement meilleure
                let gap = mean[best] - mean[second];
                let error_margin = se[best] + se[second];
                if gap > error_margin || count[best] + count[second] > 10_000 {
                    let esp = mean[best];
                    // Si la meilleure action est clairement meilleure, ou si on a fait beaucoup de samples
                    model[row_idx][d_idx] = match best {
                        0 => ActionModel::Stand(esp),
                        1 => ActionModel::Hit(esp),
                        2 => ActionModel::Double(esp),
                        3 => ActionModel::Split(esp),
                        _ => unreachable!(),
                    };
                    break;
                }
            }
            game.deck.push(game.dealer.remove(0));
            draw_bar(row_idx * cols + d_idx + 1, rows * cols);
        }

        for _ in 0..n_cards {
            game.deck.push(game.player[0].remove(0));
        }
    }

    println!();
    let mut models = MODELS.lock().unwrap();
    models[n_cards-2] = model.clone();
    model
}

pub fn save_model<T: serde::Serialize>(model: &Vec<T>, filename: &str) {
    let f = File::create(filename).expect("Unable to create file");
    serde_json::to_writer(f, &model).expect("Unable to write data");
}

pub fn load_model(filename: &str) -> Vec<Vec<ActionModel>> {
    let f = File::open(filename).expect("Unable to open file");
    let loaded: Vec<Vec<ActionModel>> = serde_json::from_reader(f).expect("Unable to read data");
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



//differents strats:
// Double
// Hit (max to find) => Stand
// Split

fn eval_hit(game:&mut Game, pn:usize) -> f64 {
    game.deal_to_player(1, pn);
    let upcard = game.dealer[0].clone();
    let hand = game.player[pn].clone();
    let score = calculate_score(&hand, true).0;
    if score > 21 {
        return -1.0;
    }
    if hand.len() == 11{
        game.dealer_turn();
        return game.result(None);        
        }
    let row = get_row_model(&hand);
    let col = card_index(&upcard);
    let models = MODELS.lock().unwrap();
    let model = &models[hand.len()-2];
    let action = &model[row][col];
    let e = match action {
        ActionModel::Hit(esp) => esp,
        ActionModel::Stand(esp) => esp,
        ActionModel::Double(esp) => esp,
        ActionModel::Split(esp) => esp,
        _ => panic!("Invalid action model"),
    };    
    *e
}


fn eval_split(game:&mut Game, pn:usize) -> f64 {
    game.split(pn);
    let pn2 = game.player.len() - 1;
    let hand1 = game.player[pn].clone();
    let hand2 = game.player[pn2].clone();

    let row1 = get_row_model(&hand1);
    let row2 = get_row_model(&hand2);
    let col = card_index(&game.dealer[0]);
    let models = MODELS.lock().unwrap();
    let model = &models[hand1.len()];
    let action1 = &model[row1][col];
    let action2 = &model[row2][col];
    
    let e1 = match action1 {
        ActionModel::Hit(esp) => esp,
        ActionModel::Stand(esp) => esp,
        ActionModel::Double(esp) => esp,
        ActionModel::Split(esp) => esp,
        _ => panic!("Invalid action model"),
    };
    
    let e2 = match action2 {
        ActionModel::Hit(esp) => esp,
        ActionModel::Stand(esp) => esp,
        ActionModel::Double(esp) => esp,
        ActionModel::Split(esp) => esp,
        _ => panic!("Invalid action model"),
    };
    
    *e1 + *e2
}

pub fn try_actions(game: &Game, pn: usize) -> Vec<f64>{

    let stand_esp = try_action(&mut game.clone(), vec![Action::Stand], pn);    

    let hit_esp = eval_hit(&mut game.clone(), pn);    
   
    vec![stand_esp, hit_esp]

}

pub fn try_actions2(game: &Game, pn: usize) -> Vec<f64>{
    let hand = &game.player[pn];

    let mut esps = try_actions(game, pn);    

    let double_result = if hand.len() == 2{try_action(&mut game.clone(), vec![Action::Double], pn)} else {f64::NEG_INFINITY};

    let split_result = if is_splitable(hand) {eval_split(&mut game.clone(), pn)} else {f64::NEG_INFINITY};
    
    esps.extend(vec![double_result, split_result]);
    esps

}
 
pub fn use_model(model: &Vec<Vec<ActionModel>>, hand: &Vec<Card>, dcard: &Card) -> ActionModel {
    let row = get_row_model(&hand);
    let col = card_index(dcard);
    model[row][col].clone()
}

pub fn test_strat(game: &mut Game) ->f64 {
    let mut pn = 0;
    let models = MODELS.lock().unwrap();
    loop{
        let hand = game.player[pn].clone();
        let mut act = use_model(&models[hand.len()], &game.player[pn], &game.dealer[0]);
        while !matches!(act, ActionModel::Stand(_)) {
            if !calculate_score(&hand, true).0 > 21 {
                break;
            }
            game.play(&act.to_action(), false, pn);
            act = use_model(&models[hand.len()], &game.player[pn], &game.dealer[0]);
        }

        pn += 1;
        if pn >= game.player.len() {break;}
    }
    game.dealer_turn();
    game.result(None)

}

//fn test(){
//    let model = load_model("models/model2.json");
//    let mut total = 0.0;
//    for _ in 0..10000{
//       let hand = vec![Card::all_cards.choose(&mut rand::thread_rng()).unwrap().clone(), Card::all_cards.choose(&mut rand::thread_rng()).unwrap().clone()]; 
//       let col = rand::thread_rng().gen_range(0..10);
//       row = get_row_model(&hand);
//       total += model[row][col].1;
//    }
//}


fn _real_try_split(mut game: Game, pn: usize) -> f64 {
    
    game.split(pn);
    let pn2 = game.player.len() - 1;
    
    let mut possible_mooves1 = vec![vec![Action::Hit], vec![Action::Hit; 2], vec![Action::Hit; 3], vec![Action::Hit; 4], vec![Action::Hit; 5], vec![Action::Hit; 6], vec![Action::Hit; 7], vec![Action::Hit; 8],
    vec![Action::Stand]];
    if is_splitable(&game.player[pn]){possible_mooves1.push(vec![Action::Split(pn as u8)]);};
    if game.player[pn].len() == 2{possible_mooves1.push(vec![Action::Double])}

    let mut possible_mooves2 = vec![vec![Action::Hit], vec![Action::Hit; 2], vec![Action::Hit; 3], vec![Action::Hit; 4], vec![Action::Hit; 5], vec![Action::Hit; 6], vec![Action::Hit; 7], vec![Action::Hit; 8], 
    vec![Action::Stand]];
    if is_splitable(&game.player[pn2]){possible_mooves2.push(vec![Action::Split(pn2 as u8)]);};
    if game.player[pn2].len() == 2{possible_mooves2.push(vec![Action::Double])}
    
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