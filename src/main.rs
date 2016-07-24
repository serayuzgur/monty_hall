use round::Round;

mod round;

/// https://en.wikipedia.org/wiki/Monty_Hall_problem
#[allow(unused_variables)]
fn main() {
    let limit = 1000000;
    let mut changed_wins = 0;
    let mut not_changed_wins = 0;
    println!("Creating {} Rounds...", limit);
    for i in 0..1000000 {
        let mut round = Round::new();
        round.reveal_door();
        if round.is_victory() {
            not_changed_wins += 1;
        }
    }
    println!("Creating {} Rounds...", limit);
    for i in 0..1000000 {
        let mut round = Round::new();
        round.reveal_door();
        round.change_selection();
        if round.is_victory() {
            changed_wins += 1;
        }
    }

    println!("Not Changed:{} Changed:{} Rate:{}", not_changed_wins, changed_wins, changed_wins as f32 / not_changed_wins as f32);
}
