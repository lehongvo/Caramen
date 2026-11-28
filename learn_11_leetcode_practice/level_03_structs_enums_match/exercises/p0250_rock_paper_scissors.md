# Problem 0250: Rock, Paper, Scissors

- **Difficulty:** Easy
- **Topics:** `match` on a tuple of two enums, exhaustive tuple patterns, enum methods, tallying into a struct
- **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)

## Problem

Model the game with two fieldless enums, `Hand { Rock, Paper, Scissors }` and
`Outcome { Win, Lose, Draw }`. Rock beats scissors, scissors beat paper, paper beats rock.

Implement:

- `Hand::from_char(c: char) -> Option<Hand>`: `'R'`, `'P'`, `'S'` (either case);
- `Hand::beats(self) -> Hand`: the hand that `self` defeats;
- `play(me: Hand, them: Hand) -> Outcome`: the outcome from **my** point of view;
- `tally(rounds: &str) -> Score`: `rounds` is a space-separated list of two-letter rounds such
  as `"RS PP sr"` (my hand first, then theirs). Return a `Score { wins, losses, draws }`;
  rounds that are not exactly two valid letters are skipped.

## Examples

```text
Hand::Rock.beats()                 => Scissors
play(Rock, Scissors)               => Win
play(Rock, Paper)                  => Lose
play(Paper, Paper)                 => Draw
tally("RS PP sr RP")               => Score { wins: 1, losses: 2, draws: 1 }   ("sr" is a loss)
tally("RS XX R RPS")               => Score { wins: 1, losses: 0, draws: 0 }
```

## Constraints

- `rounds` has at most `10^5` characters.

## Hints

1. `play` can be a `match (me, them)` with three explicit winning pairs, `(a, b) if a == b`, and a
   final `_` for losses. Or: `if them == me.beats()`.
2. For `tally`, `let [a, b] = word.as_bytes() else { continue; };` checks the length.

## Starter code

Write your code in [`src/practice/p0250_rock_paper_scissors.rs`](../src/practice/p0250_rock_paper_scissors.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0250
```

```rust
/// A hand in rock, paper, scissors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

/// The result of a round from one player's point of view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

/// Totals over many rounds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Score {
    pub wins: u32,
    pub losses: u32,
    pub draws: u32,
}

impl Hand {
    /// `'R'`, `'P'` or `'S'`, in either case.
    pub fn from_char(c: char) -> Option<Hand> {
        todo!("Write your solution here")
    }

    /// The hand that `self` defeats.
    pub fn beats(self) -> Hand {
        todo!("Write your solution here")
    }
}

/// The outcome of `me` against `them`.
pub fn play(me: Hand, them: Hand) -> Outcome {
    todo!("Write your solution here")
}

/// Scores a space-separated list of rounds such as `"RS PP"`.
pub fn tally(rounds: &str) -> Score {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::Hand::*;
    use super::*;

    #[test]
    fn parsing_and_beats() {
        assert_eq!(Hand::from_char('r'), Some(Rock));
        assert_eq!(Hand::from_char('P'), Some(Paper));
        assert_eq!(Hand::from_char('s'), Some(Scissors));
        assert_eq!(Hand::from_char('x'), None);
        assert_eq!(Rock.beats(), Scissors);
        assert_eq!(Scissors.beats(), Paper);
        assert_eq!(Paper.beats(), Rock);
    }

    #[test]
    fn all_nine_pairs() {
        let hands = [Rock, Paper, Scissors];
        for &me in &hands {
            for &them in &hands {
                let expected = if me == them {
                    Outcome::Draw
                } else if me.beats() == them {
                    Outcome::Win
                } else {
                    Outcome::Lose
                };
                assert_eq!(play(me, them), expected, "{me:?} vs {them:?}");
            }
        }
        assert_eq!(play(Rock, Paper), Outcome::Lose);
    }

    #[test]
    fn tally_example() {
        assert_eq!(
            tally("RS PP sr RP"),
            Score {
                wins: 1,
                losses: 2,
                draws: 1
            }
        );
    }

    #[test]
    fn tally_skips_invalid_rounds() {
        assert_eq!(
            tally("RS XX R RPS"),
            Score {
                wins: 1,
                losses: 0,
                draws: 0
            }
        );
        assert_eq!(tally(""), Score::default());
    }

    #[test]
    fn long_tournament() {
        let rounds = "PR SP RR ".repeat(10_000);
        assert_eq!(
            tally(&rounds),
            Score {
                wins: 20_000,
                losses: 0,
                draws: 10_000
            }
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `play` matches on the pair of hands: equal hands draw, the three winning pairs win,
everything else loses. `tally` parses each word into two hands and updates a `Score`.

**Rust concepts:**
- Matching a tuple `(me, them)` of two enums lets one arm describe a whole combination, such as
  `(Hand::Rock, Hand::Scissors)`.
- `use Hand::*;` inside the function keeps the arms short.
- `let (Some(mine), Some(theirs)) = (Hand::from_char(a), Hand::from_char(b)) else { continue; };`
  parses both letters and skips invalid rounds in one statement.
- The `Score` struct derives `Default`, so `Score::default()` starts all counters at zero.

```rust
/// A hand in rock, paper, scissors.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

/// The result of a round from one player's point of view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

/// Totals over many rounds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Score {
    pub wins: u32,
    pub losses: u32,
    pub draws: u32,
}

impl Hand {
    /// `'R'`, `'P'` or `'S'`, in either case.
    pub fn from_char(c: char) -> Option<Hand> {
        match c.to_ascii_uppercase() {
            'R' => Some(Hand::Rock),
            'P' => Some(Hand::Paper),
            'S' => Some(Hand::Scissors),
            _ => None,
        }
    }

    /// The hand that `self` defeats.
    pub fn beats(self) -> Hand {
        match self {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        }
    }
}

/// The outcome of `me` against `them`.
pub fn play(me: Hand, them: Hand) -> Outcome {
    use Hand::*;
    match (me, them) {
        (a, b) if a == b => Outcome::Draw,
        (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => Outcome::Win,
        _ => Outcome::Lose,
    }
}

/// Scores a space-separated list of rounds such as `"RS PP"`.
pub fn tally(rounds: &str) -> Score {
    let mut score = Score::default();
    for word in rounds.split_whitespace() {
        let &[a, b] = word.as_bytes() else {
            continue;
        };
        let (Some(mine), Some(theirs)) = (Hand::from_char(a as char), Hand::from_char(b as char))
        else {
            continue;
        };
        match play(mine, theirs) {
            Outcome::Win => score.wins += 1,
            Outcome::Lose => score.losses += 1,
            Outcome::Draw => score.draws += 1,
        }
    }
    score
}
```

Full tested solution: [`src/solutions/p0250_rock_paper_scissors.rs`](../src/solutions/p0250_rock_paper_scissors.rs)

</details>
