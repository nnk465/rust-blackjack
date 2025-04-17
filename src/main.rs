use rand::seq::SliceRandom;
use std::io;
use std::io::Write;

enum Actions {
    Hit,
    Stand
}

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

    fn deal_card(&mut self, player: bool) {
        let card = self.draw_card();
        if player {
            self.player.push(card);
        } else {
            self.dealer.push(card);
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

    fn calculate_score(&self, player: bool) -> u32 {
        let hand = if player { &self.player } else { &self.dealer };
        let mut score = 0;
        let mut aces = 0;
        for card in hand {
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
        println!("Player's hand: {:?}  score:{}", &self.player, self.calculate_score(true));
        println!("Dealer's hand: {:?}  score:{}", &self.dealer, self.calculate_score(false));
    }

}
fn main(){
    let mut game = Game::new();
    game.deal_card(true);
    game.deal_card(false);
    game.deal_card(true);
    game.deal_card(false);

    game.show();
    
    loop{
        let mut input = String::new();
        println!("choose action: \n    1: hit\n    2: stand");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input = input.trim().to_string();
        let action = if input == "1"{Some(Actions::Hit)} else if input == "2"{Some(Actions::Stand)} else {None};
        if action.is_none(){
            println!("please, choose a valid action");
            continue;
        }
        let action = action.unwrap();
        match action {
            Actions::Hit => {game.deal_card(true); 
                game.show()},
            Actions::Stand => break,
        }
        if game.calculate_score(true) > 21 {
            println!("Player busts! Dealer wins.");
            return;
        }
    }

    while game.calculate_score(false) < 17 {
        game.deal_card(false);
    }

    game.show();
    if game.calculate_score(false) > 21 {
        println!("Dealer busts! Player wins.");
    } else if game.calculate_score(true) > game.calculate_score(false) {
        println!("Player wins!");
    } else if game.calculate_score(false) > game.calculate_score(true) {
        println!("Dealer wins!");
    } else {
        println!("It's a tie!");
    }
}