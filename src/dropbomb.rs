// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

use core::panic;
use std::thread::panicking;

struct DropBomb {
    defused: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        DropBomb { defused: false }
    }

    pub fn defuse(&mut self) {
        self.defused = true;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!("Bomb dropped!");
        }
    }
}

fn main() {
    // Example usage
    let mut bomb = DropBomb::new();
    // Uncommenting the next line will panic when the bomb is dropped
    // drop(bomb);
    
    // Defusing the bomb
    // bomb.defuse();
    // Now dropping the bomb will not panic
    drop(bomb);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
