// ==================================================================
// PRACTICE 0228: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0228_playing_card_ordering.md
// Run the tests:          cargo test practice::p0228
// ==================================================================

//! # Problem 0228: Ordering Playing Cards
//!
//! - **Difficulty:** Easy
//! - **Topics:** `#[derive(PartialOrd, Ord)]` on enums and structs, field order matters, `max`, `sort`, `match` on a tuple of `bool`s
//! - **Source:** Original · [The Rust Book, Appendix C](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html) · [`Ord`](https://doc.rust-lang.org/std/cmp/trait.Ord.html)
//!
//! ## Problem
//!
//! A card has a `rank` (`Two` .. `Ten`, `Jack`, `Queen`, `King`, `Ace`, from lowest to highest) and a
//! `suit` (`Clubs`, `Diamonds`, `Hearts`, `Spades`, from lowest to highest). Cards are ordered first
//! by rank and, for equal ranks, by suit. The given types derive `PartialOrd` and `Ord`, so this
//! order comes from the **declaration order** of the variants and of the struct fields: `<`, `>`,
//! `max` and `sort` already work on cards. Your job is to use that order.
//!
//! Implement:
//!
//! - `highest(cards: &[Card]) -> Option<Card>`: the greatest card, `None` for an empty hand;
//! - `sort_hand(cards: &mut [Card])`: sorts the hand from lowest to highest;
//! - `is_flush(cards: &[Card]) -> bool`: `true` if the hand is non-empty and all cards share one suit;
//! - `beats(challenger: Card, leader: Card, trump: Suit) -> bool`: in a trick-taking game, `leader`
//!   was played first. The challenger wins if it is a trump and the leader is not, or if both have
//!   the same suit and the challenger's rank is higher. In every other case the leader wins.
//!
//! ## Examples
//!
//! ```text
//! (Card { rank: Ace, suit: Clubs } > Card { rank: King, suit: Spades })      => true   (rank first)
//! (Card { rank: Ten, suit: Hearts } > Card { rank: Ten, suit: Diamonds })    => true   (then suit)
//! highest(&[Two of Spades, Queen of Clubs, Queen of Hearts])                 => Some(Queen of Hearts)
//! is_flush(&[Two of Hearts, King of Hearts])                                 => true
//! beats(Two of Spades, Ace of Hearts, trump = Spades)                        => true   (trump wins)
//! beats(King of Clubs, Ace of Hearts, trump = Spades)                        => false  (did not follow suit)
//! ```
//!
//! ## Constraints
//!
//! - Hands have at most `52` cards.
//!
//! ## Hints
//!
//! 1. `rank` is declared **before** `suit` in `Card`, and derived `Ord` compares fields from top to bottom.
//! 2. `iter().max().copied()` and `slice::sort` use the derived order directly.
//! 3. For `beats`, match on `(challenger.suit == trump, leader.suit == trump)`.

/// Card ranks, lowest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

/// Card suits, lowest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// A playing card, ordered by rank, then by suit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

/// The greatest card in the hand.
pub fn highest(cards: &[Card]) -> Option<Card> {
    todo!("Write your solution here")
}

/// Sorts the hand from lowest to highest.
pub fn sort_hand(cards: &mut [Card]) {
    todo!("Write your solution here")
}

/// Whether the hand is non-empty and has a single suit.
pub fn is_flush(cards: &[Card]) -> bool {
    todo!("Write your solution here")
}

/// Whether `challenger` wins the trick that `leader` started.
pub fn beats(challenger: Card, leader: Card, trump: Suit) -> bool {
    todo!("Write your solution here")
}

#[cfg(test)]
mod tests {
    use super::Rank::*;
    use super::Suit::*;
    use super::*;

    fn card(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    #[test]
    fn derived_order_rank_then_suit() {
        assert!(card(Ace, Clubs) > card(King, Spades));
        assert!(card(Ten, Hearts) > card(Ten, Diamonds));
        assert!(card(Two, Spades) < card(Three, Clubs));
        assert_eq!(highest(&[card(Two, Clubs)]), Some(card(Two, Clubs)));
    }

    #[test]
    fn highest_card() {
        let hand = [card(Two, Spades), card(Queen, Clubs), card(Queen, Hearts)];
        assert_eq!(highest(&hand), Some(card(Queen, Hearts)));
        assert_eq!(highest(&[]), None);
    }

    #[test]
    fn sorting_a_hand() {
        let mut hand = vec![
            card(King, Hearts),
            card(Two, Spades),
            card(King, Clubs),
            card(Ace, Diamonds),
        ];
        sort_hand(&mut hand);
        assert_eq!(
            hand,
            vec![
                card(Two, Spades),
                card(King, Clubs),
                card(King, Hearts),
                card(Ace, Diamonds),
            ]
        );
    }

    #[test]
    fn flushes() {
        assert!(is_flush(&[card(Two, Hearts), card(King, Hearts)]));
        assert!(!is_flush(&[card(Two, Hearts), card(King, Spades)]));
        assert!(is_flush(&[card(Seven, Clubs)]));
        assert!(!is_flush(&[]));
    }

    #[test]
    fn trick_rules() {
        assert!(beats(card(Two, Spades), card(Ace, Hearts), Spades));
        assert!(!beats(card(King, Clubs), card(Ace, Hearts), Spades));
        assert!(!beats(card(Ace, Hearts), card(Two, Spades), Spades));
        assert!(beats(card(Ace, Hearts), card(Two, Hearts), Spades));
        assert!(beats(card(Four, Spades), card(Three, Spades), Spades));
        assert!(!beats(card(Ace, Clubs), card(Two, Diamonds), Hearts));
    }

    #[test]
    fn full_deck_sorts_with_ace_of_spades_last() {
        let ranks = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ];
        let mut deck: Vec<Card> = [Spades, Hearts, Diamonds, Clubs]
            .iter()
            .flat_map(|&s| ranks.iter().map(move |&r| card(r, s)))
            .collect();
        sort_hand(&mut deck);
        assert_eq!(deck.len(), 52);
        assert_eq!(deck[0], card(Two, Clubs));
        assert_eq!(deck[51], card(Ace, Spades));
        assert_eq!(highest(&deck), Some(card(Ace, Spades)));
    }
}
