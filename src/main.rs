mod train;
use train::*;
mod blackjack;
use blackjack::*;
use std::process::Command;

fn _test_hand(){
    let mut game = Game::new(10);
    game.deal_choosen_card(vec![Card::Three, Card::Four], true, 0);
    game.deal_choosen_card(vec![Card::Eight], false, 0);
    game.deal_to_dealer(1);
    println!("{:?}", best_mooves(&game, 0));
    println!("{:?}", game.deck);
    game.show();
}

fn main(){
    let model2 = train_ia2(1000);
    save_model(&model2, "models/model2.json");
    Command::new("python3").arg("formatter.py").output().expect("Failed to execute script");
    //println!("{model2:?}");

    //let model3 = train_ia3(1000);
    //save_model(&model3, "models/model3.json");


    //let mut total = 0.0;
    //for _ in 0..10000{
    //    let mut game = Game::new(10);
    //    game.deal_to_player(2, 0);
    //    game.deal_to_dealer(2);
    //    total += test_strat(&mut game, &model);
    //}
    //println!("total score: {}", total/10000.0);

}