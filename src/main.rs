use rand::prelude::SliceRandom;
use rand;
use std::io;
use std::io::Write;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::sync::MutexGuard;


#[derive(Clone, Debug)]
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
#[derive(Debug)]
struct Game {
    player: Vec<Card>,
    dealer: Vec<Card>,
    deck: Vec<Card>,
}
impl Game {
    fn new() -> Self {
        let mut rng = rand::rng();
        let one = vec![Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five,
        Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
        Card::Jack, Card::Queen, Card::King];
        let mut deck: Vec<_> = one.iter().cycle().take(4 * one.len()).cloned().collect();
        deck.shuffle(&mut rng);
        Game {
            player: Vec::new(),
            dealer: Vec::new(),
            deck,
        }
    }

    fn deal_card(&mut self, player: bool, n:i32) {
        for _i in 0..n {
            let card = self.draw_card();
            if player {
                self.player.push(card);
            } else {
                self.dealer.push(card);
            }
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

    fn calculate_score(&self, player: bool, mut n:i8) -> u32 {
        let hand = if player { &self.player } else { &self.dealer };
        let mut score = 0;
        let mut aces = 0;
        n = if n > hand.len() as i8 || n==0{ hand.len() as i8 } else { n };
        for i in 0..n {
            let card = &hand[i as usize];
            match card {
                Card::Ace => {
                    score += 11;
                    aces += 1;
                }
                Card::Two => score += 2,
                Card::Three => score += 3,
                Card::Four => score += 4,
                Card::Five => score += 5,
                Card::Six => score += 6,
                Card::Seven => score += 7,
                Card::Eight => score += 8,
                Card::Nine => score += 9,
                Card::Ten | Card::Jack | Card::Queen | Card::King => score += 10,
            }
        }
        while score > 21 && aces > 0 {
            score -= 10;
            aces -= 1;
        }
        score
    }

    fn show(&self) {
        println!("Player's hand: {:?}  score:{}", &self.player, self.calculate_score(true, 0));
        println!("Dealer's hand: {:?}  score:{}", &self.dealer, self.calculate_score(false, 0));
    }

    fn dealer_turn(&mut self) {
        while self.calculate_score(false, 0) < 17 {
            self.deal_card(false, 1);
        }
    }

    fn result(&self) -> f64 {
        let ds = self.calculate_score(false, 0);
        let ps = self.calculate_score(true, 0);

        let pbj = {
            if self.player.len() == 2 && ps == 21 {true} else {false}
        };
        let dbj = {
            if self.dealer.len() == 2 && ds == 21 {true} else {false}
        };
        if pbj && !dbj{
            1.5
        } else if dbj && !pbj || ps > 21 || ds > ps && ds<=21{
            -1.0
        } else if ps > ds || ds > 21{
            1.0
        } else if pbj && dbj || ps == ds{            
            0.0
        }else{
            panic!("bizzar, pas de result.\n game:\n {:?}\n", self);
        }
        

    }   

}


static GLOBAL_GAME: Lazy<Mutex<Game>> = Lazy::new(|| Mutex::new(Game::new()));
static MONEY: Lazy<Mutex<f64>> = Lazy::new(|| Mutex::new(1000.0));

fn new_game(show:bool) -> MutexGuard<'static, Game> {
    let mut game = GLOBAL_GAME.lock().unwrap(); 
    *game = Game::new();                        
    game.deal_card(true, 2);
    game.deal_card(false, 2);
    if show{game.show()}
    game
}

fn strat1() {
    let mut game = new_game(false);
    let mut money = MONEY.lock().unwrap();
    let bet = 10.0;
    loop{  
        let player_score = game.calculate_score(true, 0);
        let dealer_score = game.calculate_score(false, 1);

        if player_score > 16 || player_score >= 12 && dealer_score < 7 || player_score > 21{
            break;
        } else if player_score < 17 && dealer_score > 7 {
            game.deal_card(true, 1);
        }else{break}
    }
        
    if game.calculate_score(true, 0) > 21 {
        *money -= bet;
        return;
    }
 
    game.dealer_turn();
    let r = game.result();
    *money+=r*bet;


}
fn main() {
    for i in 0..100000 {
        strat1();
        println!("{}: money: {}", i, *MONEY.lock().unwrap());
        io::stdout().flush().unwrap();
        

    }
}
