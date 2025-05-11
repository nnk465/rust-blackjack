 mod train;
use train::*;
mod blackjack;
use blackjack::*;
use std::process::Command;
use rayon::prelude::*;

fn training(){
    //println!("Training model 10...");
    //let model = train_ia_generic(10, 2000);
    //save_model(&model, &format!("models/model10.json"));
    for i in 0..9{
        println!("Training model {}...", 10-i);
        let model = train_ia_generic(10-i, 4000*(i+1));
        save_model(&model, &format!("models/model{}.json", 10-i));
    }
    Command::new("python3").arg("formatter.py").output().expect("Failed to execute script");
}


fn test_ia(i:usize){
    let total: f64 = (0..i).into_par_iter()
    .map(|_| {
        let mut game = Game::new();
        game.deal_to_dealer(2);
        game.deal_to_player(2, 0);
        test_strat(&mut game)
    })
    .sum();
    println!("mean {}%", total*100.0/i as f64);

}

fn pass_warnings(){
    if false{
        training();
        train_ia_generic(0, 0);
        train_ia_generic2(0, 0);
        test_ia(0);
    }
}
fn main(){
    pass_warnings();
    //training();
    test_ia(1_000_000);
    //let model = train_ia_generic(2, 1000);
    //save_model(&model, "models/model2.json");



    //println!("{model2:?}");

    //let model3 = train_ia3(1000);
    //save_model(&model3, "models/model3.json");

    //let model = load_model("models/model3.json");
    //let mut total = 0.0;
    //let mut game = Game::new(10);
    //    game.deal_to_player(3, 0);
    //for _ in 0..10000{
    //    while game.player[0].cards.sum() > 21 {
    //        let mut game = Game::new(10);
    //        game.deal_to_player(3, 0);
    //        game.deal_to_dealer(2);
    //        continue;
    //    }
    //    total += test_strat(&mut game, &model);
    //}
    //println!("total score: {}", total/10000.0);

}