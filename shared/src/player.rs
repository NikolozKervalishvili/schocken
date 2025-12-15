use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

#[derive(Default, Debug)]
pub struct Player {
    // ++Ordering++
    combination: Option<Combination>,
    throw: Option<Throw>, // combination player threw
    num_throws: u8,
    id: AtomicU32, // must be unique
    // -- Ordering --
    name: String, // non unique
}

// NOTE: SORTING OF PLAYERS
// WARNING: sort backwards, e.g. [`sort_by(|a,b| b.cmp(a))`] so highest score first
impl Ord for Player {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // if guard, because only on shockout later player wins (in same amount of throws)
        if self.combination == other.combination && self.combination == Some(Combination::Schockout)
        {
            return other
                .num_throws
                .cmp(&self.num_throws)
                .then_with(|| self.id.load(Relaxed).cmp(&self.id.load(Relaxed)));
        }

        self.combination
            .cmp(&other.combination)
            .then_with(|| self.throw.cmp(&other.throw))
            .then_with(|| other.num_throws.cmp(&self.num_throws))
            .then_with(|| other.id.load(Relaxed).cmp(&self.id.load(Relaxed)))
    }
}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == std::cmp::Ordering::Equal
    }
}

impl Eq for Player {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Combination {
    Vegetable,
    Straight,
    General,
    Schock,
    Jule,
    Schockout,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Throw(u8, u8, u8);
