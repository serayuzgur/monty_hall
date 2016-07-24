extern crate rand;

#[derive(Debug)] /// Represents the round for each selection.
pub struct Round {
    ///Array of the doors which belongs to the round.
    doors: [u8; 3],
    selection: usize,
    revealed: usize
}

impl Round {
    pub fn new() -> Round {
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
    pub fn reveal_door(&mut self) {
        for i in 0..3 {
            if i != self.selection && self.doors[i as usize] == 0 {
                self.revealed = i;
            }
        }
    }
    pub fn change_selection(&mut self) {
        for i in 0..3 {
            if i != self.selection && i != self.revealed {
                self.selection = i;
            }
        }
    }
    pub fn is_victory(&self) -> bool {
        self.doors[self.selection] == 1
    }
}
