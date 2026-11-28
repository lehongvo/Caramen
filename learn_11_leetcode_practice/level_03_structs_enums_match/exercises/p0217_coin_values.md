# Problem 0217: Coins and State Quarters

- **Difficulty:** Easy
- **Topics:** enum variant holding another enum, binding a value inside a pattern, `{:?}` on an enum
- **Source:** Original · [The Rust Book 6.2](https://doc.rust-lang.org/book/ch06-02-match.html)

## Problem

A coin jar holds US coins. Quarters from the "50 States" series also record which state is on
the back. The types are:

```text
enum UsState { Alabama, Alaska, Delaware, Hawaii, Texas }
enum Coin    { Penny, Nickel, Dime, Quarter(UsState) }
```

Implement:

- `UsState::admitted(self) -> u16`: the year the state joined the union (Alabama 1819, Alaska
  1959, Delaware 1787, Hawaii 1959, Texas 1845);
- `value_in_cents(coin: &Coin) -> u32`: 1, 5, 10 or 25;
- `describe(coin: &Coin) -> String`: `"penny"`, `"nickel"`, `"dime"`, or for a quarter
  `"quarter from <State>"`, where `<State>` is the variant name as printed by `{:?}`;
- `jar_total(coins: &[Coin]) -> u32`: the total value in cents;
- `quarters_admitted_before(coins: &[Coin], year: u16) -> usize`: how many quarters show a state
  admitted **strictly before** `year`.

## Examples

```text
value_in_cents(&Coin::Dime)                        => 10
value_in_cents(&Coin::Quarter(UsState::Alaska))    => 25
describe(&Coin::Quarter(UsState::Texas))           => "quarter from Texas"
describe(&Coin::Penny)                             => "penny"
jar_total(&[Penny, Quarter(Hawaii), Nickel, Quarter(Delaware)])         => 56
quarters_admitted_before(&[Quarter(Alabama), Dime, Quarter(Alaska)], 1900) => 1
```

## Constraints

- At most `10^5` coins.

## Hints

1. The pattern `Coin::Quarter(state)` both checks the variant and binds the inner value to `state`.
2. `format!("quarter from {state:?}")` prints the variant name thanks to `#[derive(Debug)]`.
3. In `quarters_admitted_before`, a `match` that returns `bool` (or `filter`) keeps the count short.

## Starter code

Write your code in [`src/practice/p0217_coin_values.rs`](../src/practice/p0217_coin_values.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0217
```

```rust
/// The state shown on a quarter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsState {
    Alabama,
    Alaska,
    Delaware,
    Hawaii,
    Texas,
}

impl UsState {
    /// The year the state was admitted to the union.
    pub fn admitted(self) -> u16 {
        todo!("Write your solution here")
    }
}

/// A US coin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/// The value of one coin in cents.
pub fn value_in_cents(coin: &Coin) -> u32 {
    todo!("Write your solution here")
}

/// A short description, e.g. `"quarter from Texas"`.
pub fn describe(coin: &Coin) -> String {
    todo!("Write your solution here")
}

/// The total value of the jar in cents.
pub fn jar_total(coins: &[Coin]) -> u32 {
    todo!("Write your solution here")
}

/// The number of quarters whose state was admitted strictly before `year`.
pub fn quarters_admitted_before(coins: &[Coin], year: u16) -> usize {
    todo!("Write your solution here")
}
```

## Tests

<details>
<summary>Show the tests used to check your solution</summary>

```rust
#[cfg(test)]
mod tests {
    use super::Coin::*;
    use super::UsState::*;
    use super::*;

    #[test]
    fn values() {
        assert_eq!(value_in_cents(&Penny), 1);
        assert_eq!(value_in_cents(&Nickel), 5);
        assert_eq!(value_in_cents(&Dime), 10);
        assert_eq!(value_in_cents(&Quarter(Alaska)), 25);
    }

    #[test]
    fn descriptions() {
        assert_eq!(describe(&Quarter(Texas)), "quarter from Texas");
        assert_eq!(describe(&Quarter(Delaware)), "quarter from Delaware");
        assert_eq!(describe(&Penny), "penny");
        assert_eq!(describe(&Nickel), "nickel");
        assert_eq!(describe(&Dime), "dime");
    }

    #[test]
    fn admission_years() {
        assert_eq!(Alabama.admitted(), 1819);
        assert_eq!(Hawaii.admitted(), 1959);
        assert_eq!(Texas.admitted(), 1845);
    }

    #[test]
    fn jar_totals() {
        assert_eq!(
            jar_total(&[Penny, Quarter(Hawaii), Nickel, Quarter(Delaware)]),
            56
        );
        assert_eq!(jar_total(&[]), 0);
        let big = vec![Dime; 100_000];
        assert_eq!(jar_total(&big), 1_000_000);
    }

    #[test]
    fn quarters_before_a_year() {
        let jar = [Quarter(Alabama), Dime, Quarter(Alaska)];
        assert_eq!(quarters_admitted_before(&jar, 1900), 1);
        assert_eq!(quarters_admitted_before(&jar, 1819), 0);
        assert_eq!(quarters_admitted_before(&jar, 1960), 2);
        assert_eq!(quarters_admitted_before(&[Penny, Dime], 3000), 0);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** each function is one `match` over `Coin`. Only the `Quarter` arm binds the state.
Totals and counts are iterator chains over the slice.

**Rust concepts:**
- An enum variant can carry data of another enum type. Matching `Coin::Quarter(state)` pulls that
  data out; when matching on `&Coin`, `state` is a `&UsState` (default binding modes), and
  because `UsState: Copy` you can call `state.admitted()` directly.
- `#[derive(Debug)]` on an enum prints the variant name, which is exactly what `describe` needs.
- `iter().map(value_in_cents).sum()` passes a function name where a closure is expected.

```rust
/// The state shown on a quarter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsState {
    Alabama,
    Alaska,
    Delaware,
    Hawaii,
    Texas,
}

impl UsState {
    /// The year the state was admitted to the union.
    pub fn admitted(self) -> u16 {
        match self {
            UsState::Alabama => 1819,
            UsState::Alaska => 1959,
            UsState::Delaware => 1787,
            UsState::Hawaii => 1959,
            UsState::Texas => 1845,
        }
    }
}

/// A US coin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

/// The value of one coin in cents.
pub fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(_) => 25,
    }
}

/// A short description, e.g. `"quarter from Texas"`.
pub fn describe(coin: &Coin) -> String {
    match coin {
        Coin::Penny => String::from("penny"),
        Coin::Nickel => String::from("nickel"),
        Coin::Dime => String::from("dime"),
        Coin::Quarter(state) => format!("quarter from {state:?}"),
    }
}

/// The total value of the jar in cents.
pub fn jar_total(coins: &[Coin]) -> u32 {
    coins.iter().map(value_in_cents).sum()
}

/// The number of quarters whose state was admitted strictly before `year`.
pub fn quarters_admitted_before(coins: &[Coin], year: u16) -> usize {
    coins
        .iter()
        .filter(|coin| match coin {
            Coin::Quarter(state) => state.admitted() < year,
            _ => false,
        })
        .count()
}
```

Full tested solution: [`src/solutions/p0217_coin_values.rs`](../src/solutions/p0217_coin_values.rs)

</details>
