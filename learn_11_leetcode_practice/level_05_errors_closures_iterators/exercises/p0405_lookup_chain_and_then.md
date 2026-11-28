# Problem 0405: Follow a Lookup Chain with and_then

- **Difficulty:** Easy
- **Topics:** `Option::and_then`, `Option::map`, `Iterator::find`, `Option<&str>`
- **Source:** Original · [Rust By Example: Combinators `and_then`](https://doc.rust-lang.org/rust-by-example/error/option_unwrap/and_then.html)

## Problem

A small `Directory` stores two tables:

- `users`: pairs `(id, username)`;
- `emails`: pairs `(username, email address)`.

Finding the email of a user id needs **two** lookups, and each may fail. `map` is not enough to
chain them: the closure for the second lookup itself returns an `Option`, so `map` would give
you an `Option<Option<&str>>`. `and_then` (also called *flat map*) is made for this case.

Implement these methods on `Directory`:

- `username(&self, id: u32) -> Option<&str>`: the username of the first pair with that id.
- `email_of(&self, username: &str) -> Option<&str>`: the email of the first pair with that
  username.
- `email_for_id(&self, id: u32) -> Option<&str>`: id -> username -> email.
- `domain_for_id(&self, id: u32) -> Option<&str>`: the part after `'@'` in that email, or
  `None` if any step fails or the email has no `'@'`.

## Examples

```text
users  = [(1, "ann"), (2, "bob"), (3, "cy")]
emails = [("ann", "ann@example.com"), ("bob", "bob-at-home")]

username(2)         -> Some("bob")
username(9)         -> None
email_for_id(1)     -> Some("ann@example.com")
email_for_id(3)     -> None          ("cy" has no email)
domain_for_id(1)    -> Some("example.com")
domain_for_id(2)    -> None          ("bob-at-home" has no '@')
```

## Constraints

- Each table has at most `10^3` entries. Lookups may scan the vectors linearly.

## Hints

1. `self.users.iter().find(|(uid, _)| *uid == id)` returns `Option<&(u32, String)>`; follow it
   with `.map(|(_, name)| name.as_str())`.
2. `a.and_then(|x| f(x))` calls `f` only for `Some(x)`, and `f` itself returns an `Option`.
3. `str::split_once('@')` returns `Option<(&str, &str)>`, so it chains with `and_then` too.

## Starter code

Write your code in [`src/practice/p0405_lookup_chain_and_then.rs`](../src/practice/p0405_lookup_chain_and_then.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0405
```

```rust
pub struct Directory {
    pub users: Vec<(u32, String)>,
    pub emails: Vec<(String, String)>,
}

impl Directory {
    pub fn username(&self, id: u32) -> Option<&str> {
        todo!("Write your solution here")
    }

    pub fn email_of(&self, username: &str) -> Option<&str> {
        todo!("Write your solution here")
    }

    pub fn email_for_id(&self, id: u32) -> Option<&str> {
        todo!("Write your solution here")
    }

    pub fn domain_for_id(&self, id: u32) -> Option<&str> {
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

    fn sample() -> Directory {
        Directory {
            users: vec![
                (1, "ann".to_string()),
                (2, "bob".to_string()),
                (3, "cy".to_string()),
            ],
            emails: vec![
                ("ann".to_string(), "ann@example.com".to_string()),
                ("bob".to_string(), "bob-at-home".to_string()),
            ],
        }
    }

    #[test]
    fn username_lookup() {
        let d = sample();
        assert_eq!(d.username(2), Some("bob"));
        assert_eq!(d.username(9), None);
    }

    #[test]
    fn email_lookup() {
        let d = sample();
        assert_eq!(d.email_of("ann"), Some("ann@example.com"));
        assert_eq!(d.email_of("zed"), None);
    }

    #[test]
    fn email_for_id_chains_two_lookups() {
        let d = sample();
        assert_eq!(d.email_for_id(1), Some("ann@example.com"));
        assert_eq!(d.email_for_id(3), None);
        assert_eq!(d.email_for_id(42), None);
    }

    #[test]
    fn domain_for_id_chains_three_steps() {
        let d = sample();
        assert_eq!(d.domain_for_id(1), Some("example.com"));
        assert_eq!(d.domain_for_id(2), None);
        assert_eq!(d.domain_for_id(3), None);
    }

    #[test]
    fn first_matching_pair_wins() {
        let d = Directory {
            users: vec![(7, "first".to_string()), (7, "second".to_string())],
            emails: vec![
                ("first".to_string(), "a@x.org".to_string()),
                ("first".to_string(), "b@y.org".to_string()),
            ],
        };
        assert_eq!(d.username(7), Some("first"));
        assert_eq!(d.domain_for_id(7), Some("x.org"));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** build the two basic lookups with `find` + `map`, then chain them with `and_then`.
Each step either produces the next value or short-circuits the whole chain to `None`.

**Rust concepts:**
- `Option::map` is for a closure that always succeeds; `Option::and_then` is for a closure that
  can fail (returns an `Option`), and it flattens the result.
- The returned `&str` borrows from the `Directory` (lifetime elision ties the output to
  `&self`), so no `String` is cloned.
- The closure pattern `|(uid, _)|` destructures a `&(u32, String)`; thanks to match
  ergonomics `uid` is a `&u32`, hence `*uid == id`.

```rust
pub struct Directory {
    pub users: Vec<(u32, String)>,
    pub emails: Vec<(String, String)>,
}

impl Directory {
    pub fn username(&self, id: u32) -> Option<&str> {
        self.users
            .iter()
            .find(|(uid, _)| *uid == id)
            .map(|(_, name)| name.as_str())
    }

    pub fn email_of(&self, username: &str) -> Option<&str> {
        self.emails
            .iter()
            .find(|(name, _)| name == username)
            .map(|(_, email)| email.as_str())
    }

    pub fn email_for_id(&self, id: u32) -> Option<&str> {
        self.username(id).and_then(|name| self.email_of(name))
    }

    pub fn domain_for_id(&self, id: u32) -> Option<&str> {
        self.email_for_id(id)
            .and_then(|email| email.split_once('@'))
            .map(|(_, domain)| domain)
    }
}
```

Full tested solution: [`src/solutions/p0405_lookup_chain_and_then.rs`](../src/solutions/p0405_lookup_chain_and_then.rs)

</details>
