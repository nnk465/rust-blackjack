use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Stand, 
    Hit,
    Double,
    Split(u8),
}

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Card {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl Card {
    pub fn value(&self) -> u8 {
        match self {
            Card::Ace => 1,
            Card::Two => 2,
            Card::Three => 3,
            Card::Four => 4,
            Card::Five => 5,
            Card::Six => 6,
            Card::Seven => 7,
            Card::Eight => 8,
            Card::Nine => 9,
            Card::Ten => 10,
        }
    }
    pub fn all_cards() -> Vec<Card> {
        vec![
            Card::Ace,
            Card::Two,
            Card::Three,
            Card::Four,
            Card::Five,
            Card::Six,
            Card::Seven,
            Card::Eight,
            Card::Nine,
            Card::Ten,
        ]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Hand{
    pub cards: Vec<Card>,
    pub bet: i32,
}


// for a 4*52 deck

#[derive(Debug, Clone)]
pub struct Game {
    pub player: Vec<Hand>,
    pub dealer: Vec<Card>,
    pub deck: Vec<Card>,
    pub bet: i32,
    double_packets: Vec<usize>,
}

impl Game {
    pub fn new(bet: i32) -> Self {
        let mut rng = rand::rng();
        let one = vec![Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five,
        Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
        Card::Ten, Card::Ten, Card::Ten];
        // paquet de 4 jeux de 52 cartes
        let mut deck: Vec<_> = one.iter().cycle().take(16 * one.len()).cloned().collect();
        deck.shuffle(&mut rng);
        Game {
            player: vec!(Hand {
                cards: Vec::new(),
                bet: bet,
            }),
            dealer: Vec::new(),
            deck,
            bet: bet,
            double_packets: Vec::new(),
        }
    }

    pub fn gen_with_true_count(player_cards: Vec<Card>, dealer_cards: Vec<Card>, bet: i32, true_count: f64) -> Self {
        let mut game = Game::new(bet);
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
    
    pub fn deal_to_dealer(&mut self, n:i8) {
        for _i in 0..n{
            let card = self.draw_card();
            self.dealer.push(card);
        }
    }
    pub fn deal_to_player(&mut self, n:i8, pn:usize) {
        for _i in 0..n{
            let card = self.draw_card();
            self.player[pn].cards.push(card);
        }
    }

    pub fn deal_choosen_card(&mut self, cards: Vec<Card>, player: bool, pn: usize) {
        for card in cards {
            if let Some(pos) = self.deck.iter().position(|x| *x == card) {
                if player{
                    self.player[pn].cards.push(card);
                } else {
                    self.dealer.push(card);
                }
                self.deck.remove(pos);
            } else {
                panic!("Card not found in deck");
            }
        }
    }

    pub fn draw_card(&mut self) -> Card {
        if self.deck.is_empty() {
            panic!("No cards left in the deck");
        }
        self.deck.remove(0)
    }


    pub fn show(&self) {
        println!("\n\n--== Blackjack ==--");
        println!("-Bet: {}", self.bet);
        for i in 0..self.player.len() {
            println!("-Player's hand {}: {:?}  score:{}", i+1, &self.player[i], calculate_score(&self.player[i].cards, true).0);
        }
        println!("-Dealer's hand: {:?}  score:{}\n\n", &self.dealer, calculate_score(&self.dealer, true).0);
    }

    pub fn split(&mut self, pn: usize){
        if self.player[pn].cards.len() != 2 {
            panic!("Cannot split");
        }
        let card = self.player[pn].cards.remove(1);
        self.player.push(Hand { cards: vec![card], bet: self.player[pn].bet });
        self.deal_to_player(1, pn);
        self.deal_to_player(1, self.player.len() -1);
    }

    pub fn double(&mut self, pn: usize) {
        if self.player[pn].cards.len() != 2 {
            //panic!("Cannot double because hand is not 2 cards");
            return
        }
        self.player[pn].bet *= 2;
        self.deal_to_player(1, pn);
        self.double_packets.push(pn);
    }

    pub fn play(&mut self, action: &Action, show: bool, pn: usize) {
        match action {
            Action::Hit => {
                self.deal_to_player(1, pn);
                if show {
                }
            }
            Action::Double => {
                self.double(pn);}
            Action::Split(pn) => {
                self.split(*pn as usize);
            }
            Action::Stand => {}
        }
        
    }
    pub fn dealer_turn(&mut self){
        while calculate_score(&self.dealer, true).0 < 17{
            self.deal_to_dealer(1);}
    }

    pub fn result(&self, pn:Option<&usize>) -> f64 {
        let (dealer_score, _) = calculate_score(&self.dealer, true);
        let dealer_blackjack = self.dealer.len() == 2 && dealer_score == 21;
    
        let mut total_score = 0.0;
        let pd = match pn {
            Some(pn) => &vec![self.player[*pn].clone()],
            None => &self.player,
        };
        
        for (i, hand) in pd.iter().enumerate() {
            let real_i = match pn {
                Some(pn) => *pn,
                None => i,
            };
            let reward = if self.double_packets.contains(&real_i) { [-2.0, 0.0, 2.0, 3.0] } else { [-1.0, 0.0, 1.0, 1.5] };            
            let (player_score, _) = calculate_score(&hand.cards, true);
            let player_blackjack = hand.cards.len() == 2 && player_score == 21;

            match (player_blackjack, dealer_blackjack) {
                (true, false) => total_score += reward[3],
                (false, true) => total_score += reward[0],
                (true, true) => continue, // égalité blackjack
                _ => {
                    if player_score > 21 || (dealer_score <= 21 && dealer_score > player_score) {
                        // joueur bust ou dealer gagne
                        total_score += reward[0];
                    } else if player_score > dealer_score || dealer_score > 21 {
                        // joueur gagne
                        total_score += reward[2];
                    } else if player_score == dealer_score {
                        // égalité
                        continue;
                    } else {
                        panic!("Cas inattendu:\n game:\n {:?}\n", self);
                    }
                }
            }
        }
        total_score
    }

    

}

pub fn calculate_score(hand: &[Card], count_aces: bool) -> (u32, u8) {
    let mut score:u32 = 0;
    let mut aces = 0;
    for card in hand {
        if card == &Card::Ace{
            score += if count_aces{aces += 1;
                11} else {0};
        } else {score += card.value() as u32;}
    }     
    while score > 21 && aces > 0 {
        score -= 10;
        aces -= 1;
    }
    (score.into(), aces.into())
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