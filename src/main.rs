mod train;
use train::*;
mod blackjack;
use blackjack::*;

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
    let model = [[Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0)],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0)],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Hit, Action::Hit, Action::Hit, Action::Hit],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Hit],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Hit],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Hit],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Hit],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Stand, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0)],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Stand, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Hit, Action::Split(0), Action::Split(0), Action::Double, Action::Split(0), Action::Double, Action::Double, Action::Double, Action::Split(0), Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Stand, Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0)],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Stand, Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0)],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Double, Action::Split(0)],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Hit],
    [Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double],
    [Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0)],
    [Action::Double, Action::Stand, Action::Double, Action::Stand, Action::Stand, Action::Stand, Action::Stand, Action::Stand, Action::Double, Action::Double],
    [Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0)]];
    let mut total = 0.0;
    for _ in 0..10000{
        let mut game = Game::new(10);
        game.deal_to_player(2, 0);
        game.deal_to_dealer(2);
        total += test_strat(&mut game, &model);
    }
    println!("total score: {}", total/10000.0);
}