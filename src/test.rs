fn number_hit(game: Game, pn: usize) -> usize {
    let mut hand = game.player[pn].cards.clone();
    if hand.len() == 0 {game.show(); panic!("No cards in hand")};
    let mut scores:Vec<u8> = vec![];
    for i in 0..game.deck.len() {
        let (score, aces) = calculate_score(&hand, true);
        scores.push(score as u8);
        if i > 10{
            println!("{:?}: {:?}", scores, hand);
            panic!("")
        } 
        if score > 21 && aces == 0 {
            let (mut max, _) = scores.iter().enumerate().max_by_key(|&(_, v)| v).unwrap();
            max -= 1;
            return max;
        }
        hand.push(game.deck[i].clone()); 
    }
    panic!("No more cards in the deck to hit");
}


fn fast_try_split(mut game: Game, pn: usize) -> f64 {
    
    game.split(pn);
    let pn2 = game.player.len() - 1;
    
    best_mooves(&game, pn).1 + best_mooves(&game, pn2).1
    }


fn best_mooves(game: &Game, pn: usize) -> (Vec<Action>, f64 ){
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

pub fn card_to_count(card:&Card) ->i8{
    match card {
        Card::Two | Card::Three | Card::Four | Card::Five | Card::Six => 1,
        Card::Seven | Card::Eight | Card::Nine => 0,
        Card::Ten| Card::Ace=> -1,
    }
}

fn cards_to_count(cards:&Vec<Card>) -> f64{
    cards.iter().map(|card| card_to_count(&card)).sum::<i8>() as f64 / 4.0
}


pub fn gen_with_true_count(player_cards: Vec<Card>, dealer_cards: Vec<Card>, true_count: f64) -> Self {
    let mut game = Game::new();
    game.player[0].cards = player_cards.clone();
    game.dealer = dealer_cards.clone();

    let draws = rand::rng().random_range(30..150);
    let mut passed_cards = Vec::with_capacity(draws + player_cards.len() + dealer_cards.len());
    let mut discards = Vec::with_capacity(draws);

    for _ in 0..draws {
        let c = game.deck.remove(0);
        passed_cards.push(c.clone());
        discards.push(c);
    }

    passed_cards.extend(player_cards.iter().cloned());
    passed_cards.extend(dealer_cards.iter().cloned());

    let running_count: i32 = passed_cards
        .iter()
        .map(|c| card_to_count(c) as i32)
        .sum();

    let decks_remaining = game.deck.len() as f64 / 52.0;
    let act_true_count = running_count as f64 / decks_remaining;
    let diff_tc = (true_count - act_true_count).round();    
    let cards_to_swap = (diff_tc * decks_remaining).abs().round() as usize;

    if diff_tc > 0.0 {
        let mut negs = discards
            .into_iter()
            .filter(|c| card_to_count(c) == -1)
            .collect::<Vec<_>>();
        for _ in 0..cards_to_swap {
            if let Some(c) = negs.pop() {
                game.deck.push(c);
            }
        }
    } else if diff_tc < 0.0 {
        let mut poss = discards
            .into_iter()
            .filter(|c| card_to_count(c) == 1)
            .collect::<Vec<_>>();
        for _ in 0..cards_to_swap {
            if let Some(c) = poss.pop() {
                game.deck.push(c);
            }
        }
    }
    
    game.double_packets = Vec::new();
    game
}

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