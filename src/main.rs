mod train;
use train::*;
mod blackjack;
use blackjack::*;
fn main(){
    let mut game = Game::new(10);
    
    game.deal_to_dealer(2);
//    game.deal_to_player(2, 0);
    game.deal_choosen_card(vec![Card::Jack, Card::Jack], 0);
    let bm = best_mooves(&game, 0);
    println!("game: {:?}", bm);
    game.play(&bm[0], false, 0);
    game.dealer_turn();
    game.show();
    println!("result: {}", game.result());
}