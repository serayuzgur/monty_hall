extern crate rand;

#[derive(Debug)]
/// Represents the round for each selection.
pub struct Round {
    /// Array of the doors which belongs to the round.
    doors: [u8; 3],
    selection: usize,
    revealed: usize,
}

impl Round {
    pub fn new() -> Round {
        let car = rand::random::<usize>() % 3;
        let mut doors = [0, 0, 0];
        doors[car] = 1;

        Round {
            doors: doors,
            selection: rand::random::<usize>() % 3,
            revealed: 3, // invalid number out of index
        }
    }
    /// Select a random door from doors.
    pub fn reveal_door(&mut self) {
        for i in 0..3 {
            if i != self.selection && self.doors[i as usize] == 0 {
                self.revealed = i;
                break;
            }
        }
    }
    pub fn change_selection(&mut self) {
        for i in 0..3 {
            if i != self.selection && i != self.revealed {
                self.selection = i;
                break;
            }
        }
    }
    pub fn is_victory(&self) -> bool {
        self.doors[self.selection] == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reveal_door() {
        let mut round = Round::new();
        round.reveal_door();

        assert!(round.revealed >= 0 as usize && round.revealed < 3 as usize,
                "reveal_door must assign a value to round.revealed");
    }

    #[test]
    fn change_selection() {
        let mut round = Round::new();
        let first_selection = round.selection;
        round.reveal_door();
        round.change_selection();
        let is_changed = round.selection != first_selection && round.selection != round.revealed;
        assert!(is_changed,
                "change_selection must change round.selection before:{} after:{}",
                first_selection,
                round.selection);
    }
}