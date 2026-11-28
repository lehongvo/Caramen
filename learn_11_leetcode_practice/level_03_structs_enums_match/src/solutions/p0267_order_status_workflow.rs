//! # Problem 0267: Order Status Workflow
//!
//! - **Difficulty:** Medium
//! - **Topics:** enum states with data, transition methods returning `Option<Self>`, `matches!` for state groups, replaying actions
//! - **Source:** Original · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) · [The Rust Book 18.3](https://doc.rust-lang.org/book/ch18-03-oo-design-patterns.html)
//!
//! ## Problem
//!
//! An online shop tracks every order with a status:
//!
//! ```text
//! enum OrderStatus {
//!     Created,
//!     Paid { amount_cents: u32 },
//!     Shipped { amount_cents: u32, tracking: String },
//!     Delivered,
//!     Cancelled { reason: String },
//!     Refunded { amount_cents: u32 },
//! }
//! ```
//!
//! Implement the transitions as methods taking `&self` and returning `Option<OrderStatus>`: the new
//! status, or `None` if the transition is not allowed from the current status.
//!
//! - `pay(&self, amount_cents: u32)`: only from `Created`, and only for a positive amount;
//! - `ship(&self, tracking: &str)`: only from `Paid`; keeps the amount;
//! - `deliver(&self)`: only from `Shipped`;
//! - `cancel(&self, reason: &str)`: from `Created` gives `Cancelled { reason }`; from `Paid` or
//!   `Shipped` gives `Refunded` with the paid amount; not allowed otherwise.
//!
//! Also implement `is_final(&self) -> bool` (`Delivered`, `Cancelled` and `Refunded` are final) and
//! the free function `process(actions: &[Action]) -> (OrderStatus, usize)`: start from `Created`,
//! apply each `Action` (`Pay(u32)`, `Ship(String)`, `Deliver`, `Cancel(String)`), skip the ones that
//! are not allowed, and return the final status and the number of **skipped** actions.
//!
//! ## Examples
//!
//! ```text
//! Created.pay(1999)                    => Some(Paid { amount_cents: 1999 })
//! Created.pay(0)                       => None
//! Paid { 1999 }.ship("TRK1")           => Some(Shipped { amount_cents: 1999, tracking: "TRK1" })
//! Created.deliver()                    => None
//! Shipped { 1999, "TRK1" }.cancel("late") => Some(Refunded { amount_cents: 1999 })
//! Created.cancel("changed my mind")    => Some(Cancelled { reason: "changed my mind" })
//! Delivered.cancel("too late")         => None
//! process(&[Ship("X"), Pay(500), Pay(600), Ship("T9"), Deliver, Cancel("no")])
//!                                      => (Delivered, 3)    (Ship, the 2nd Pay and Cancel are skipped)
//! ```
//!
//! ## Constraints
//!
//! - At most `10^4` actions.
//!
//! ## Hints
//!
//! 1. Each transition is a `match self` with one allowed arm and `_ => None`.
//! 2. In `cancel`, `OrderStatus::Paid { amount_cents } | OrderStatus::Shipped { amount_cents, .. }`
//!    binds `amount_cents` in both alternatives.
//! 3. `matches!(self, A | B { .. } | C { .. })` is a short `is_final`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every transition matches on the current status and builds the next one from the data
//! it carries (the amount) plus the method arguments. `process` keeps the current status and
//! replaces it whenever a transition returns `Some`.
//!
//! **Rust concepts:**
//! - Data that only makes sense in some states lives in those variants: there is no tracking
//!   number before shipping and no amount after cancellation of an unpaid order.
//! - Taking `&self` and returning a fresh `Option<OrderStatus>` means a rejected transition leaves
//!   the current status untouched; the amount is `Copy` and the new strings come from the arguments,
//!   so nothing is cloned.
//! - An or-pattern can bind a variable if **every** alternative binds it with the same type.
//! - `if let Some(next) = ... { status = next } else { skipped += 1 }` replays the actions.
// @solution-end

/// The lifecycle status of an order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderStatus {
    Created,
    Paid { amount_cents: u32 },
    Shipped { amount_cents: u32, tracking: String },
    Delivered,
    Cancelled { reason: String },
    Refunded { amount_cents: u32 },
}

/// A request to change an order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Pay(u32),
    Ship(String),
    Deliver,
    Cancel(String),
}

impl OrderStatus {
    /// `Created` -> `Paid`, for a positive amount.
    pub fn pay(&self, amount_cents: u32) -> Option<OrderStatus> {
        // @solution-start
        match self {
            OrderStatus::Created if amount_cents > 0 => Some(OrderStatus::Paid { amount_cents }),
            _ => None,
        }
        // @solution-end
    }

    /// `Paid` -> `Shipped`.
    pub fn ship(&self, tracking: &str) -> Option<OrderStatus> {
        // @solution-start
        match self {
            OrderStatus::Paid { amount_cents } => Some(OrderStatus::Shipped {
                amount_cents: *amount_cents,
                tracking: tracking.to_string(),
            }),
            _ => None,
        }
        // @solution-end
    }

    /// `Shipped` -> `Delivered`.
    pub fn deliver(&self) -> Option<OrderStatus> {
        // @solution-start
        match self {
            OrderStatus::Shipped { .. } => Some(OrderStatus::Delivered),
            _ => None,
        }
        // @solution-end
    }

    /// `Created` -> `Cancelled`; `Paid`/`Shipped` -> `Refunded`.
    pub fn cancel(&self, reason: &str) -> Option<OrderStatus> {
        // @solution-start
        match self {
            OrderStatus::Created => Some(OrderStatus::Cancelled {
                reason: reason.to_string(),
            }),
            OrderStatus::Paid { amount_cents } | OrderStatus::Shipped { amount_cents, .. } => {
                Some(OrderStatus::Refunded {
                    amount_cents: *amount_cents,
                })
            }
            _ => None,
        }
        // @solution-end
    }

    /// Whether no further transition is possible.
    pub fn is_final(&self) -> bool {
        // @solution-start
        matches!(
            self,
            OrderStatus::Delivered | OrderStatus::Cancelled { .. } | OrderStatus::Refunded { .. }
        )
        // @solution-end
    }
}

/// Applies the actions from `Created`; returns the final status and how many were skipped.
pub fn process(actions: &[Action]) -> (OrderStatus, usize) {
    // @solution-start
    let mut status = OrderStatus::Created;
    let mut skipped = 0;
    for action in actions {
        let next = match action {
            Action::Pay(amount) => status.pay(*amount),
            Action::Ship(tracking) => status.ship(tracking),
            Action::Deliver => status.deliver(),
            Action::Cancel(reason) => status.cancel(reason),
        };
        if let Some(next) = next {
            status = next;
        } else {
            skipped += 1;
        }
    }
    (status, skipped)
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paid(a: u32) -> OrderStatus {
        OrderStatus::Paid { amount_cents: a }
    }

    fn shipped(a: u32, t: &str) -> OrderStatus {
        OrderStatus::Shipped {
            amount_cents: a,
            tracking: t.to_string(),
        }
    }

    #[test]
    fn happy_path() {
        let s = OrderStatus::Created.pay(1999).expect("can pay");
        assert_eq!(s, paid(1999));
        let s = s.ship("TRK1").expect("can ship");
        assert_eq!(s, shipped(1999, "TRK1"));
        let s = s.deliver().expect("can deliver");
        assert_eq!(s, OrderStatus::Delivered);
        assert!(s.is_final());
    }

    #[test]
    fn rejected_transitions() {
        assert_eq!(OrderStatus::Created.pay(0), None);
        assert_eq!(paid(5).pay(5), None);
        assert_eq!(OrderStatus::Created.ship("X"), None);
        assert_eq!(OrderStatus::Created.deliver(), None);
        assert_eq!(paid(5).deliver(), None);
        assert_eq!(OrderStatus::Delivered.cancel("too late"), None);
    }

    #[test]
    fn cancelling() {
        assert_eq!(
            OrderStatus::Created.cancel("changed my mind"),
            Some(OrderStatus::Cancelled {
                reason: "changed my mind".to_string()
            })
        );
        assert_eq!(
            paid(700).cancel("x"),
            Some(OrderStatus::Refunded { amount_cents: 700 })
        );
        assert_eq!(
            shipped(1999, "TRK1").cancel("late"),
            Some(OrderStatus::Refunded { amount_cents: 1999 })
        );
        assert_eq!(
            OrderStatus::Refunded { amount_cents: 1 }.cancel("again"),
            None
        );
    }

    #[test]
    fn final_states() {
        assert!(!OrderStatus::Created.is_final());
        assert!(!paid(1).is_final());
        assert!(!shipped(1, "t").is_final());
        assert!(OrderStatus::Cancelled { reason: "r".into() }.is_final());
        assert!(OrderStatus::Refunded { amount_cents: 1 }.is_final());
    }

    #[test]
    fn process_example() {
        let actions = [
            Action::Ship("X".into()),
            Action::Pay(500),
            Action::Pay(600),
            Action::Ship("T9".into()),
            Action::Deliver,
            Action::Cancel("no".into()),
        ];
        assert_eq!(process(&actions), (OrderStatus::Delivered, 3));
        assert_eq!(process(&[]), (OrderStatus::Created, 0));
    }

    #[test]
    fn process_many_rejected() {
        let mut actions = vec![Action::Deliver; 9_998];
        actions.push(Action::Pay(42));
        actions.push(Action::Cancel("r".into()));
        assert_eq!(
            process(&actions),
            (OrderStatus::Refunded { amount_cents: 42 }, 9_998)
        );
    }
}
