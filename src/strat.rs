use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    Stand, 
    Hit,
    Double,
    Split(i8),
}

// ordre: somme de 5 -> 21
pub static STRAT_HARD: Lazy<Vec<Vec<Action>>> = Lazy::new(||vec![
    vec![Action::Hit; 10],
    vec![Action::Hit; 10],
    vec![Action::Hit; 10],
    vec![Action::Hit; 10],
    vec![Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double, Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Hit, Action::Hit,],
    vec![Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Double, Action::Hit, Action::Hit,],
    vec![Action::Hit, Action::Hit, Action::Stand, Action::Stand, Action::Stand, Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Stand, Action::Stand, Action::Stand, Action::Stand, Action::Stand, Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Stand, Action::Stand, Action::Stand, Action::Stand, Action::Stand, Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Stand, Action::Stand, Action::Stand, Action::Stand, Action::Stand, Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Stand, Action::Stand, Action::Stand, Action::Stand, Action::Stand, Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Stand; 10],
    vec![Action::Stand; 10],
    vec![Action::Stand; 10],
    vec![Action::Stand; 10],
    vec![Action::Stand; 10],
]);

// ordre: A+2 -> A+10 
pub static STRAT_SOFT: Lazy<Vec<Vec<Action>>> = Lazy::new(||vec![
    vec![Action::Hit, Action::Hit, Action::Hit, Action::Double, Action::Double, Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit, ],
    vec![Action::Hit, Action::Hit, Action::Hit, Action::Double, Action::Double,Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Hit, Action::Hit, Action::Double, Action::Double, Action::Double,Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Hit, Action::Hit, Action::Double, Action::Double, Action::Double,Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Hit, Action::Double, Action::Double, Action::Double, Action::Double,Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Stand, Action::Double, Action::Double, Action::Double, Action::Double, Action::Stand, Action::Stand, Action::Hit, Action::Hit, Action::Hit,],
    vec![Action::Stand; 10],
    vec![Action::Stand; 10],
    vec![Action::Stand; 10],
]);
// ordre: 2+2 -> 10+10    
pub static STRAT_PAIR: Lazy<Vec<Vec<Action>>> = Lazy::new(||vec![
    vec![Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Hit, Action::Hit, Action::Hit, Action::Hit],
    vec![Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Hit, Action::Hit, Action::Hit, Action::Hit],
    vec![Action::Hit, Action::Hit, Action::Hit, Action::Split(0), Action::Split(0), Action::Split(0), Action::Hit, Action::Hit, Action::Hit, Action::Hit],
    vec![Action::Stand, Action::Double, Action::Double, Action::Double, Action::Double, Action::Stand, Action::Stand, Action::Hit, Action::Hit, Action::Hit],
    vec![Action::Stand, Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Stand, Action::Stand, Action::Stand, Action::Stand],
    vec![Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Hit, Action::Hit, Action::Hit, Action::Hit, Action::Hit],
    vec![Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Hit, Action::Hit],
    vec![Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Stand, Action::Split(0), Action::Split(0), Action::Stand, Action::Stand],
    vec![Action::Stand; 10],
    vec![Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Split(0), Action::Hit],
]);