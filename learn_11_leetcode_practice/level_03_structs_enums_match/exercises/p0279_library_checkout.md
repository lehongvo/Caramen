# Problem 0279: Library Checkout

- **Difficulty:** Medium
- **Topics:** enum status with data inside a struct, `&mut` access to a variant's fields, `match` on `&mut self.status`, late fees
- **Source:** Original · [The Rust Book 6.1](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html) · [The Rust Book 5.3](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)

## Problem

A small library tracks each copy of a book with a `Book { id, title, status }`, where

```text
enum Status {
    Available,
    OnLoan { member: u32, due_day: u32, renewals: u8 },
    OnHold { member: u32 },          // reserved for a member, waiting to be picked up
}
```

Days are plain numbers (`day 0`, `day 1`, ...). A loan lasts **14 days**. Implement `Library`:

- `new(books: Vec<(u32, &str)>) -> Library`: all books available;
- `status(&self, id: u32) -> Option<&Status>`;
- `checkout(&mut self, id: u32, member: u32, today: u32) -> bool`: allowed if the book is
  `Available`, or `OnHold` **for this member**. The book becomes `OnLoan` with
  `due_day = today + 14` and `renewals = 0`;
- `renew(&mut self, id: u32, member: u32) -> bool`: allowed only for the member who has the book
  on loan, at most **2** times; each renewal adds 14 days to `due_day`;
- `return_book(&mut self, id: u32, today: u32) -> Option<u32>`: for a book on loan, make it
  available and return the late fee: `25` cents per day after `due_day` (`0` if on time);
  `None` if the book is not on loan (or does not exist);
- `place_hold(&mut self, id: u32, member: u32) -> bool`: only an `Available` book can be put on hold.

## Examples

```text
let mut lib = Library::new(vec![(1, "Dune"), (2, "Emma")]);
lib.checkout(1, 7, 0)    => true     (OnLoan { member: 7, due_day: 14, renewals: 0 })
lib.checkout(1, 8, 1)    => false    (already on loan)
lib.renew(1, 8)          => false    (not the borrower)
lib.renew(1, 7)          => true     (due_day 28)
lib.renew(1, 7)          => true     (due_day 42)
lib.renew(1, 7)          => false    (at most 2 renewals)
lib.return_book(1, 45)   => Some(75) (3 days late)
lib.place_hold(2, 9)     => true
lib.checkout(2, 7, 50)   => false    (on hold for member 9)
lib.checkout(2, 9, 50)   => true
```

## Constraints

- At most `10^4` books; ids are unique; days fit in a `u32`.

## Hints

1. A private `book_mut(&mut self, id) -> Option<&mut Book>` helper finds a book.
2. `match &mut book.status { Status::OnLoan { member: m, due_day, renewals } if *m == member && *renewals < 2 => { ... } }`
   lets you modify the fields of the variant in place.
3. Replacing the whole status (`book.status = Status::Available`) is often simpler than editing it.

## Starter code

Write your code in [`src/practice/p0279_library_checkout.rs`](../src/practice/p0279_library_checkout.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0279
```

```rust
/// Loan length in days.
pub const LOAN_DAYS: u32 = 14;
/// Late fee per day, in cents.
pub const FEE_PER_DAY: u32 = 25;

/// Where a copy of a book is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Available,
    OnLoan {
        member: u32,
        due_day: u32,
        renewals: u8,
    },
    OnHold {
        member: u32,
    },
}

/// One copy of a book.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub status: Status,
}

/// A collection of books.
#[derive(Debug, Clone)]
pub struct Library {}

impl Library {
    /// A library where every book is available.
    pub fn new(books: Vec<(u32, &str)>) -> Library {
        todo!("Write your solution here")
    }

    /// The status of a book.
    pub fn status(&self, id: u32) -> Option<&Status> {
        todo!("Write your solution here")
    }

    /// Lends a book that is available or on hold for `member`.
    pub fn checkout(&mut self, id: u32, member: u32, today: u32) -> bool {
        todo!("Write your solution here")
    }

    /// Extends a loan by 14 days, at most twice, for the borrower only.
    pub fn renew(&mut self, id: u32, member: u32) -> bool {
        todo!("Write your solution here")
    }

    /// Ends a loan and returns the late fee in cents.
    pub fn return_book(&mut self, id: u32, today: u32) -> Option<u32> {
        todo!("Write your solution here")
    }

    /// Reserves an available book for `member`.
    pub fn place_hold(&mut self, id: u32, member: u32) -> bool {
        todo!("Write your solution here")
    }
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_walkthrough() {
        let mut lib = Library::new(vec![(1, "Dune"), (2, "Emma")]);
        assert!(lib.checkout(1, 7, 0));
        assert_eq!(
            lib.status(1),
            Some(&Status::OnLoan {
                member: 7,
                due_day: 14,
                renewals: 0
            })
        );
        assert!(!lib.checkout(1, 8, 1));
        assert!(!lib.renew(1, 8));
        assert!(lib.renew(1, 7));
        assert!(lib.renew(1, 7));
        assert!(!lib.renew(1, 7));
        assert_eq!(lib.return_book(1, 45), Some(75));
        assert_eq!(lib.status(1), Some(&Status::Available));
        assert!(lib.place_hold(2, 9));
        assert!(!lib.checkout(2, 7, 50));
        assert!(lib.checkout(2, 9, 50));
    }

    #[test]
    fn returns_on_time_and_twice() {
        let mut lib = Library::new(vec![(5, "Ulysses")]);
        assert!(lib.checkout(5, 1, 10));
        assert_eq!(lib.return_book(5, 24), Some(0));
        assert_eq!(lib.return_book(5, 25), None);
        assert_eq!(lib.return_book(99, 25), None);
    }

    #[test]
    fn holds() {
        let mut lib = Library::new(vec![(1, "A")]);
        assert!(lib.place_hold(1, 3));
        assert!(!lib.place_hold(1, 4));
        assert!(!lib.renew(1, 3));
        assert!(lib.checkout(1, 3, 0));
        assert!(!lib.place_hold(1, 4));
        assert!(!lib.place_hold(2, 4));
    }

    #[test]
    fn unknown_books() {
        let mut lib = Library::new(vec![]);
        assert_eq!(lib.status(1), None);
        assert!(!lib.checkout(1, 1, 0));
        assert!(!lib.renew(1, 1));
    }

    #[test]
    fn many_books() {
        let titles: Vec<String> = (0..10_000).map(|i| format!("Book {i}")).collect();
        let mut lib = Library::new(
            titles
                .iter()
                .enumerate()
                .map(|(i, t)| (i as u32, t.as_str()))
                .collect(),
        );
        for id in 0..10_000 {
            assert!(lib.checkout(id, id, 0));
        }
        let fees: u32 = (0..10_000).filter_map(|id| lib.return_book(id, 20)).sum();
        assert_eq!(fees, 10_000 * 6 * FEE_PER_DAY);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each operation finds the book, then matches on its status. Allowed transitions either
replace the status with a new variant or edit the fields of the current one.

**Rust concepts:**
- Matching on `&mut book.status` binds the variant's fields as `&mut` references, so
  `*due_day += 14` updates the loan in place.
- Guards check conditions that involve both the pattern's bindings and outer variables
  (`*m == member`).
- In `return_book`, `if let Status::OnLoan { due_day, .. } = book.status` copies `due_day` out
  (`u32` is `Copy`) before the status is overwritten.
- `today.saturating_sub(due_day)` is the number of late days, `0` when returned early.

```rust
/// Loan length in days.
pub const LOAN_DAYS: u32 = 14;
/// Late fee per day, in cents.
pub const FEE_PER_DAY: u32 = 25;

/// Where a copy of a book is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Available,
    OnLoan {
        member: u32,
        due_day: u32,
        renewals: u8,
    },
    OnHold {
        member: u32,
    },
}

/// One copy of a book.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub status: Status,
}

/// A collection of books.
#[derive(Debug, Clone)]
pub struct Library {
    books: Vec<Book>,
}

impl Library {
    /// A library where every book is available.
    pub fn new(books: Vec<(u32, &str)>) -> Library {
        Library {
            books: books
                .into_iter()
                .map(|(id, title)| Book {
                    id,
                    title: title.to_string(),
                    status: Status::Available,
                })
                .collect(),
        }
    }

    /// The status of a book.
    pub fn status(&self, id: u32) -> Option<&Status> {
        self.books.iter().find(|b| b.id == id).map(|b| &b.status)
    }

    /// Lends a book that is available or on hold for `member`.
    pub fn checkout(&mut self, id: u32, member: u32, today: u32) -> bool {
        let Some(book) = self.book_mut(id) else {
            return false;
        };
        let allowed = match book.status {
            Status::Available => true,
            Status::OnHold { member: m } => m == member,
            Status::OnLoan { .. } => false,
        };
        if allowed {
            book.status = Status::OnLoan {
                member,
                due_day: today + LOAN_DAYS,
                renewals: 0,
            };
        }
        allowed
    }

    /// Extends a loan by 14 days, at most twice, for the borrower only.
    pub fn renew(&mut self, id: u32, member: u32) -> bool {
        let Some(book) = self.book_mut(id) else {
            return false;
        };
        match &mut book.status {
            Status::OnLoan {
                member: m,
                due_day,
                renewals,
            } if *m == member && *renewals < 2 => {
                *due_day += LOAN_DAYS;
                *renewals += 1;
                true
            }
            _ => false,
        }
    }

    /// Ends a loan and returns the late fee in cents.
    pub fn return_book(&mut self, id: u32, today: u32) -> Option<u32> {
        let book = self.book_mut(id)?;
        let Status::OnLoan { due_day, .. } = book.status else {
            return None;
        };
        book.status = Status::Available;
        Some(today.saturating_sub(due_day) * FEE_PER_DAY)
    }

    /// Reserves an available book for `member`.
    pub fn place_hold(&mut self, id: u32, member: u32) -> bool {
        match self.book_mut(id) {
            Some(book) if book.status == Status::Available => {
                book.status = Status::OnHold { member };
                true
            }
            _ => false,
        }
    }

    fn book_mut(&mut self, id: u32) -> Option<&mut Book> {
        self.books.iter_mut().find(|b| b.id == id)
    }
}
```

Full tested solution: [`src/solutions/p0279_library_checkout.rs`](../src/solutions/p0279_library_checkout.rs)

</details>
