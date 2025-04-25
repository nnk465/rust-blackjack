use rand::seq::SliceRandom;
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Action {
    Stand, 
    Hit,
    Double,
    Split(i8),
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
    Jack,
    Queen,
    King,
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
            Card::Ten | Card::Jack | Card::Queen | Card::King => 10,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Hand{
    pub cards: Vec<Card>,
    pub bet: i32,
}

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
        Card::Jack, Card::Queen, Card::King];
        let mut deck: Vec<_> = one.iter().cycle().take(4 * one.len()).cloned().collect();
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

    pub fn reset(&mut self, bet: i32, deck:bool) {
        self.bet = bet;
        self.player = vec!(Hand {
            cards: Vec::new(),
            bet: bet,
        });
        self.dealer = Vec::new();
        if deck{
            self.deck.clear();
            let one = vec![Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five,
            Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
            Card::Jack, Card::Queen, Card::King];
            let mut deck: Vec<_> = one.iter().cycle().take(4 * one.len()).cloned().collect();
            deck.shuffle(&mut rand::rng());
            self.deck = deck;};
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
        let card = self.deck[0].clone();
        self.deck.remove(0);
        card
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
            let reward = if self.double_packets.contains(&i) {[-2.0, 0.0, 2.0, 3.0]} else {[-1.0, 0.0, 1.0, 1.5]};
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
        //println!("total score de {:?}: {}", pd, total_score);
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

