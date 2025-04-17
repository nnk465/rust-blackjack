use rand::seq::SliceRandom;
mod strat;
use strat::*;


#[derive(Clone, Debug, PartialEq)]
enum Card {
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
    fn value(&self) -> u8 {
        match self {
            Card::Ace => 11,
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

enum HandType {
    Hard(u8),
    Soft(u8),
    Pair(u8),
}

#[derive(Debug, PartialEq)]
struct Hand{
    cards: Vec<Card>,
    bet: i32,
}

#[derive(Debug)]
struct Game {
    player: Vec<Hand>,
    dealer: Vec<Card>,
    deck: Vec<Card>,
    bet: i32,
}
impl Game {
    fn new(bet: i32) -> Self {
        let mut rng = rand::rng();
        let one = vec![Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five,
        Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
        Card::Jack, Card::Queen, Card::King];
        let mut deck: Vec<_> = one.iter().cycle().take(4 * one.len()).cloned().collect();
        deck.shuffle(&mut rng);
        Game {
            player:vec!(Hand {
                cards: Vec::new(),
                bet: bet,
            }),
            dealer: Vec::new(),
            deck,
            bet: bet,
        }
    }

    fn deal_to_dealer(&mut self, n:i8) {
        for _i in 0..n{
            let card = self.draw_card();
            self.dealer.push(card);
        }
    }
    fn deal_to_player(&mut self, n:i8, pn:usize) {
        for _i in 0..n{
            let card = self.draw_card();
            self.player[pn].cards.push(card);
        }
    }

    fn draw_card(&mut self) -> Card {
        if self.deck.is_empty() {
            panic!("No cards left in the deck");
        }
        let card = self.deck[0].clone();
        self.deck.remove(0);
        card
    }


    fn show(&self) {
        println!("--== Blackjack ==--");
        println!("-Bet: {}", self.bet);
        for i in 0..self.player.len() {
            println!("-Player's hand {}: {:?}  score:{}", i+1, &self.player[i], calculate_score(&self.player[i].cards, true));
        }
        println!("-Dealer's hand: {:?}  score:{}", &self.dealer, calculate_score(&self.dealer, true));
    }

    fn split(&mut self, pn: usize) {
        if self.player[pn].cards.len() != 2 {
            panic!("Cannot split");
        }
        let card = self.player[pn].cards.remove(1);
        self.player.push(Hand { cards: vec![card], bet: self.player[pn].bet });
        self.deal_to_player(1, pn);
        self.deal_to_player(1, self.player.len() -1);
    }

    fn play(&mut self, action: &Action, show: bool, pn: usize) {
        match action {
            Action::Hit => {
                self.deal_to_player(1, pn);
                if show {
                    self.show();
                }
            }
            Action::Double => {
                self.player[pn].bet *= 2;
                self.deal_to_player(1, pn);
            }
            Action::Split(pn) => {
                self.split(*pn as usize);
            }
            Action::Stand => {}
        }
        
    }
    fn dealer_turn(&mut self){
        while calculate_score(&self.dealer, true) < 17{
            self.deal_to_dealer(1);}
    }

    fn result(&self) -> f64 {
        let pd = &self.player;
        let ds = calculate_score(&self.dealer, true);
        let dbj = {
            if self.dealer.len() == 2 && ds == 21 {true} else {false}
        };
        let mut total_score = 0.0;
        for pn in 0..(pd.len()){
            let ps = calculate_score(&pd[pn].cards, true);
            let pbj = {
                if self.player.len() == 2 && ps == 21 {true} else {false}
            };
            if pbj && !dbj{
                total_score += 1.5;
                
            } else if dbj && !pbj || ps > 21 || ds > ps && ds<=21{
                total_score -= 1.0;
                
            } else if ps > ds || ds > 21{
                total_score += 1.0;
                
            } else if pbj && dbj || ps == ds{
                
                continue;
            }else{
                panic!("bizzar, pas de result.\n game:\n {:?}\n", self);
            }
        } 
        total_score
    }   

}



fn calculate_score(hand: &[Card], count_aces: bool) -> u32 {
    let mut score = 0;
    let mut aces = 0;
    for card in hand {
        if card == &Card::Ace && count_aces {aces += 1;
            score += 11
        } else {score += card.value();}   
    }     
    while score > 21 && aces > 0 {
        score -= 10;
        aces -= 1;
    }
    score.into()
}

// actions:
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
    
    let row = (pair_value - 2) as usize;
    let col = (dealer_upcard - 2) as usize;
    strategy[row][col]
}

// general: 
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

fn strat2(game: &mut Game,) ->f64 {
    let mut pn = 0;
    loop{
        loop{
            let hand = game.player[pn].cards.clone();
            if hand.len() == 0 || calculate_score(&hand, true) > 21 {break}
            let hand_type = if game.player[pn].cards.len() == 2 && hand[0] == hand[1] {
                
                HandType::Pair(hand[0].value() as u8)
            } else if game.player[pn].cards.contains(&Card::Ace) && calculate_score(&hand, false) <= 10 {
                let score = calculate_score(&hand, false);
                HandType::Soft(score as u8)
            } else {
                HandType::Hard(calculate_score(&hand, true) as u8)
            };
            let mut act = get_action(hand_type, game.dealer[0].value(), &STRAT_HARD.as_slice(), &STRAT_SOFT.as_slice(), &STRAT_PAIR.as_slice());
            if act == Action::Stand {break};
            if act == Action::Split(0){act = Action::Split((pn as i32).try_into().unwrap());};
            game.play(&act, false, pn);
        }
        pn += 1;
        if pn >= game.player.len() {break;}
    }
    game.dealer_turn();
    game.result()

}
fn main(){
    let mut t = 0.0;
    let mut money = 1000.0;
    let bet = 10;
    for _i in 0..100{
        let mut game = Game::new(bet);
        game.deal_to_player(2, 0);
        game.deal_to_dealer(2);
        t = strat2(&mut game);
        println!("{}", t);
        money += t*(bet as f64);
    }
    println!("{}", money)
}