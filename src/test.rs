 use rand::seq::SliceRandom;
use rand::thread_rng;

enum Actions {
    Hit,
    Stand,
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
    other: Vec<Card>,
}
impl Game {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let one = vec![Card::Ace, Card::Two, Card::Three, Card::Four, Card::Five,
        Card::Six, Card::Seven, Card::Eight, Card::Nine, Card::Ten,
        Card::Jack, Card::Queen, Card::King];
        let mut deck: Vec<_> = one.iter().cycle().take(4 * one.len()).cloned().collect();
        deck.shuffle(&mut rng);
        Game {
            player: Vec::new(),
            dealer: Vec::new(),
            let mut deck: Vec<_> = deck;,
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

    fn draw_card(&self) -> Card {
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
}
fn main(){
    let mut game = Game::new();
    game.deal_card(true);
    game.deal_card(false);
    game.deal_card(true);
    game.deal_card(false);

    println!("Player's hand: {:?}", game.player);
    println!("Dealer's hand: {:?}", game.dealer);

    let player_score = game.calculate_score(true);
    let dealer_score = game.calculate_score(false);

    println!("Player's score: {}", player_score);
    println!("Dealer's score: {}", dealer_score);

    if player_score > 21 {
        println!("Player busts! Dealer wins.");
    } else if dealer_score > 21 {
        println!("Dealer busts! Player wins.");
    } else if player_score > dealer_score {
        println!("Player wins!");
    } else if dealer_score > player_score {
        println!("Dealer wins!");
    } else {
        println!("It's a tie!");
    }
}