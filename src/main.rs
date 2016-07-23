extern crate rand;

use std::io;
use rand::Rng;

#[derive(Debug)] /// Represents the round for each selection.
struct Round {
    ///Array of the doors which belongs to the round.
    doors: [u8; 3],
    selection: usize,
    revealed: usize
}

impl Round {
    fn new() -> Round {
        let car = rand::random::<usize>() % 3;
        let mut doors = [0, 0, 0];
        doors[car] = 1;

        Round {
            doors: doors,
            selection: rand::random::<usize>() % 3,
            revealed: 0
        }
    }
    /// Select a random door from doors.
    fn reveal_door(&mut self) {
        for i in 0..3 {
            if i != self.selection && self.doors[i as usize] == 0 {
                self.revealed = i;
            }
        }
    }
    fn change_selection(&mut self) {
        for i in 0..3 {
            if i != self.selection && i != self.revealed {
                self.selection = i;
            }
        }
    }
    fn is_victory(&self) -> bool {
        self.doors[self.selection] == 1
    }
}


/// https://en.wikipedia.org/wiki/Monty_Hall_problem
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
