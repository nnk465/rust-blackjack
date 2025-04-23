mod train;
use train::*;
mod blackjack;
use blackjack::*;
fn main(){
    let mut game = Game::new(10);
    
    game.deal_to_dealer(2);
//    game.deal_to_player(2, 0);
    game.deal_choosen_card(vec![Card::Six; 2], true,  0);
    println!("{:?}",&game.deck);
    println!("best moov: {:?}", best_mooves(&game, 0));
    //let bm = best_mooves(&game, 0);
    //game.play(&bm[0], true, 0);
    //if matches!(bm[0], Action::Split(_)) {
    //    let bm2 = best_mooves(&game, 0);
    //    game.play(&bm2[0], true, 1);
    //    let bm3 = best_mooves(&game, 0);
    //    game.play(&bm3[0], true, 1);
    //}
    //game.dealer_turn();
    //println!("game: {:?}\nresult: {}", bm, game.result());
    game.show();
    
}