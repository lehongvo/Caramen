// ==================================================================
// PRACTICE 0280: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0280_rpg_battle_simulation.md
// Run the tests:          cargo test practice::p0280
// ==================================================================

//! # Problem 0280: RPG Battle Simulation
//!
//! - **Difficulty:** Medium
//! - **Topics:** structs and enums working together, `match` on `(class, action)`, borrowing two elements of a `Vec` one after the other, outcome enum
//! - **Source:** Original · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) · [The Rust Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)
//!
//! ## Problem
//!
//! A turn-based battle has a list of fighters, each with a `Class`:
//!
//! | Class | Starting hp (= max hp) | Starting mana | Attack damage |
//! | --- | --- | --- | --- |
//! | `Warrior` | 100 | 0 | 12 |
//! | `Mage` | 60 | 30 | 4 |
//! | `Healer` | 70 | 25 | 3 |
//!
//! A `Fighter` has public fields `name`, `class`, `hp`, `max_hp`, `mana` and `defending: bool`;
//! `Fighter::new(name, class)` uses the table above (not defending). One turn is
//! `(actor_index, Action)` with
//!
//! ```text
//! enum Action { Attack { target: usize }, Fireball { target: usize }, Heal { target: usize }, Defend }
//! ```
//!
//! and produces an `Outcome`: `Damage { target, amount }`, `Defeated { target }`, `Healed { target, amount }`,
//! `Defending`, or `Ignored`. Rules, for `Battle::act(&mut self, actor: usize, action: Action) -> Outcome`:
//!
//! - an actor that does not exist or is defeated (`hp == 0`) does nothing: `Ignored`;
//! - `Attack`: the actor's class attack damage. `Fireball`: only a `Mage` with at least 10 mana;
//!   costs 10 mana and deals 20 damage. For both, the target must exist and not be defeated,
//!   otherwise `Ignored` (and no mana is spent);
//! - damage against a **defending** target is halved (rounded down) and the target stops
//!   defending. `hp` never goes below `0`. If the target's `hp` reaches `0` the outcome is
//!   `Defeated { target }`, otherwise `Damage { target, amount }` with the damage dealt;
//! - `Heal`: only a `Healer` with at least 5 mana, on an existing target that is not defeated (the
//!   healer may heal itself). Costs 5 mana and restores up to 15 hp without exceeding `max_hp`.
//!   The outcome reports the hp actually restored (possibly `0`);
//! - `Defend`: the actor starts defending. Outcome `Defending`;
//! - any other combination (for example a `Warrior` casting `Fireball`) is `Ignored`.
//!
//! Also implement `Battle::new(fighters)`, `fighter(&self, i) -> Option<&Fighter>`, and
//! `survivors(&self) -> Vec<&str>` (names of fighters with `hp > 0`, in order).
//!
//! ## Examples
//!
//! ```text
//! fighters: 0 = Conan (Warrior), 1 = Merlin (Mage), 2 = Mira (Healer)
//! act(0, Attack { target: 1 })    => Damage { target: 1, amount: 12 }    (Merlin 48 hp)
//! act(1, Defend)                  => Defending
//! act(0, Attack { target: 1 })    => Damage { target: 1, amount: 6 }     (halved; Merlin 42 hp)
//! act(2, Heal { target: 1 })      => Healed { target: 1, amount: 15 }    (Merlin 57 hp)
//! act(1, Fireball { target: 0 })  => Damage { target: 0, amount: 20 }    (Conan 80 hp, Merlin 20 mana)
//! act(0, Fireball { target: 1 })  => Ignored                             (warriors cannot cast)
//! ```
//!
//! ## Constraints
//!
//! - At most `100` fighters and `10^4` turns.
//!
//! ## Hints
//!
//! 1. Decide what happens with a `match (actor.class, action)` that copies out the values you
//!    need (damage, mana cost); only then borrow the target mutably.
//! 2. You cannot hold `&mut fighters[a]` and `&mut fighters[t]` at the same time; do the target's
//!    update first, then come back to the actor to spend mana.

/// A fighter's class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    Warrior,
    Mage,
    Healer,
}

/// One combatant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fighter {
    pub name: String,
    pub class: Class,
    pub hp: u32,
    pub max_hp: u32,
    pub mana: u32,
    pub defending: bool,
}

/// What an actor does on its turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Attack { target: usize },
    Fireball { target: usize },
    Heal { target: usize },
    Defend,
}

/// What happened on a turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Damage { target: usize, amount: u32 },
    Defeated { target: usize },
    Healed { target: usize, amount: u32 },
    Defending,
    Ignored,
}

impl Fighter {
    /// A fresh fighter with the class's starting stats.
    pub fn new(name: &str, class: Class) -> Fighter {
        todo!("Write your solution here")
    }
}

/// A battle between several fighters.
#[derive(Debug, Clone)]
pub struct Battle {}

impl Battle {
    /// A battle with the given fighters.
    pub fn new(fighters: Vec<Fighter>) -> Battle {
        todo!("Write your solution here")
    }

    /// The fighter at index `i`.
    pub fn fighter(&self, i: usize) -> Option<&Fighter> {
        todo!("Write your solution here")
    }

    /// Names of the fighters still standing.
    pub fn survivors(&self) -> Vec<&str> {
        todo!("Write your solution here")
    }

    /// Plays one turn.
    pub fn act(&mut self, actor: usize, action: Action) -> Outcome {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::Action::*;
    use super::*;

    fn trio() -> Battle {
        Battle::new(vec![
            Fighter::new("Conan", Class::Warrior),
            Fighter::new("Merlin", Class::Mage),
            Fighter::new("Mira", Class::Healer),
        ])
    }

    #[test]
    fn starting_stats() {
        let w = Fighter::new("W", Class::Warrior);
        assert_eq!((w.hp, w.max_hp, w.mana, w.defending), (100, 100, 0, false));
        let m = Fighter::new("M", Class::Mage);
        assert_eq!((m.hp, m.mana), (60, 30));
        let h = Fighter::new("H", Class::Healer);
        assert_eq!((h.hp, h.mana), (70, 25));
    }

    #[test]
    fn example_turns() {
        let mut b = trio();
        assert_eq!(
            b.act(0, Attack { target: 1 }),
            Outcome::Damage {
                target: 1,
                amount: 12
            }
        );
        assert_eq!(b.act(1, Defend), Outcome::Defending);
        assert_eq!(
            b.act(0, Attack { target: 1 }),
            Outcome::Damage {
                target: 1,
                amount: 6
            }
        );
        assert_eq!(
            b.act(2, Heal { target: 1 }),
            Outcome::Healed {
                target: 1,
                amount: 15
            }
        );
        assert_eq!(
            b.act(1, Fireball { target: 0 }),
            Outcome::Damage {
                target: 0,
                amount: 20
            }
        );
        assert_eq!(b.act(0, Fireball { target: 1 }), Outcome::Ignored);
        assert_eq!(b.fighter(1).map(|f| (f.hp, f.mana)), Some((57, 20)));
        assert_eq!(b.fighter(0).map(|f| f.hp), Some(80));
        assert_eq!(b.fighter(2).map(|f| f.mana), Some(20));
    }

    #[test]
    fn mana_runs_out() {
        let mut b = trio();
        for _ in 0..3 {
            assert!(matches!(
                b.act(1, Fireball { target: 0 }),
                Outcome::Damage { amount: 20, .. }
            ));
        }
        assert_eq!(b.act(1, Fireball { target: 0 }), Outcome::Ignored);
        assert_eq!(b.fighter(1).map(|f| f.mana), Some(0));
        assert_eq!(b.fighter(0).map(|f| f.hp), Some(40));
    }

    #[test]
    fn healing_is_capped_and_self_heal_works() {
        let mut b = trio();
        assert_eq!(
            b.act(2, Heal { target: 2 }),
            Outcome::Healed {
                target: 2,
                amount: 0
            }
        );
        b.act(0, Attack { target: 2 });
        assert_eq!(
            b.act(2, Heal { target: 2 }),
            Outcome::Healed {
                target: 2,
                amount: 12
            }
        );
        assert_eq!(b.fighter(2).map(|f| (f.hp, f.mana)), Some((70, 15)));
    }

    #[test]
    fn defeat_and_ignored_turns() {
        let mut b = trio();
        let mut last = Outcome::Ignored;
        for _ in 0..5 {
            last = b.act(0, Attack { target: 1 });
        }
        assert_eq!(last, Outcome::Defeated { target: 1 });
        assert_eq!(b.fighter(1).map(|f| f.hp), Some(0));
        assert_eq!(b.act(1, Defend), Outcome::Ignored);
        assert_eq!(b.act(0, Attack { target: 1 }), Outcome::Ignored);
        assert_eq!(b.act(2, Heal { target: 1 }), Outcome::Ignored);
        assert_eq!(b.act(9, Defend), Outcome::Ignored);
        assert_eq!(b.act(0, Attack { target: 9 }), Outcome::Ignored);
        assert_eq!(b.survivors(), vec!["Conan", "Mira"]);
    }

    #[test]
    fn long_battle() {
        let mut b = Battle::new(vec![
            Fighter::new("A", Class::Warrior),
            Fighter::new("B", Class::Warrior),
        ]);
        let mut turns = 0;
        while b.survivors().len() == 2 {
            b.act(
                turns % 2,
                Attack {
                    target: 1 - turns % 2,
                },
            );
            turns += 1;
        }
        // 100 hp / 12 damage = 9 hits; A hits first, so A lands the 9th hit on turn 17.
        assert_eq!(turns, 17);
        assert_eq!(b.survivors(), vec!["A"]);
    }
}
