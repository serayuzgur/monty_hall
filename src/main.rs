// This is like import at java. We declare the package that we want to use.
// We could also write mod round::Round to use struct as Round::new();
mod round;

/// https://en.wikipedia.org/wiki/Monty_Hall_problem
fn main() {
    let mut changed_wins = 0;
    let mut not_changed_wins = 0;
    let limit = 1000000;

    println!("Creating {} Rounds...", limit);
    for _ in 0..limit {
        let mut round = round::Round::new();
        round.reveal_door();
        if round.is_victory() {
            not_changed_wins += 1;
        }
    }

    println!("Creating {} Rounds...", limit);
    for _ in 0..limit {
        let mut round = round::Round::new();
        round.reveal_door();
        round.change_selection();
        if round.is_victory() {
            changed_wins += 1;
        }
    }

    println!(" Changed:{} Not Changed:{} Rate:{}",
             changed_wins,
             not_changed_wins,
             changed_wins as f32 / not_changed_wins as f32);
}
