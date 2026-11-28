//! # Problem 0920: A Typestate Connection
//!
//! - **Difficulty:** Hard
//! - **Topics:** `PhantomData<S>`, zero-sized marker types, typestate, a sealed `State` trait, generic methods that change the type parameter
//! - **Source:** Original · [std: `PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) · [The Rustonomicon: PhantomData](https://doc.rust-lang.org/nomicon/phantom-data.html)
//!
//! ## Problem
//!
//! A network connection goes through three states: `Disconnected`, `Connected` and `Authenticated`.
//! Instead of an enum checked at run time, encode the state in the **type**:
//!
//! ```text
//! pub struct Connection<S: State> { host: String, user: Option<String>, log: Vec<String>, _state: PhantomData<S> }
//! ```
//!
//! `Disconnected`, `Connected` and `Authenticated` are empty structs. `State` is a **sealed** trait
//! (only this file can implement it) with an associated constant `NAME` (`"disconnected"`,
//! `"connected"`, `"authenticated"`).
//!
//! Methods available in **every** state (`impl<S: State> Connection<S>`): `host()`, `state()`
//! (returns `S::NAME`), `log()` (all events so far).
//!
//! State-specific methods (each transition consumes `self` and records an event in the log):
//!
//! | state | method | result | log entry |
//! | --- | --- | --- | --- |
//! | `Disconnected` | `new(host)` | `Connection<Disconnected>` | (none) |
//! | `Disconnected` | `connect(self)` | `Connection<Connected>` | `"connect <host>"` |
//! | `Connected` | `authenticate(self, user, password)` | `Result<Connection<Authenticated>, Connection<Connected>>` | `"auth <user> ok"` or `"auth <user> failed"` |
//! | `Connected` | `disconnect(self)` | `Connection<Disconnected>` | `"disconnect"` |
//! | `Authenticated` | `user(&self)` | `&str` | (none) |
//! | `Authenticated` | `send(&mut self, message)` | `usize`: bytes sent | `"send <message>"` |
//! | `Authenticated` | `logout(self)` | `Connection<Connected>` | `"logout <user>"` |
//!
//! A password is correct when it equals the user name **reversed** (`"ada"` / `"ada"`, `"bob"` /
//! `"bob"`, `"alice"` / `"ecila"`). A failed authentication gives the still-connected connection
//! back in `Err`. Logging out forgets the user.
//!
//! Calling `send` on a `Connection<Connected>` or `connect` twice does not compile.
//!
//! ## Examples
//!
//! ```text
//! let conn = Connection::new("db.local").connect();
//! conn.state()                                  => "connected"
//! let conn = conn.authenticate("alice", "nope") => Err(conn), log: [..., "auth alice failed"]
//! let mut conn = conn.authenticate("alice", "ecila")?;   // Ok
//! conn.send("hello")                            => 5
//! conn.logout().disconnect().log()
//!   => ["connect db.local", "auth alice failed", "auth alice ok", "send hello", "logout alice", "disconnect"]
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. A private generic helper `fn into_state<T: State>(self, event: Option<String>) -> Connection<T>`
//!    can move every field into the new type and push the event.
//! 2. `PhantomData` has no data: build it with `PhantomData` (the unit-like value).
//! 3. Seal the trait with a private module: `mod sealed { pub trait Sealed {} }` and
//!    `pub trait State: sealed::Sealed`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the fields are the same in every state; only the type parameter changes. `into_state`
//! rebuilds the struct with a different `S`, which costs nothing at run time (the marker is
//! zero-sized). Each state gets its own `impl` block, so a method simply does not exist in the
//! wrong state.
//!
//! **Rust concepts:**
//! - `PhantomData<S>` tells the compiler the struct is "about" an `S` without storing one; without it
//!   an unused type parameter is an error (E0392).
//! - `impl Connection<Connected> { ... }` adds methods to one instantiation only.
//! - The sealed pattern: `sealed::Sealed` is public (so it can appear in a public bound) but lives
//!   in a private module, so no other crate can name it and implement `State`.
//! - `std::mem::size_of::<Connection<Connected>>() == size_of::<Connection<Disconnected>>()`: the
//!   state really is free.
// @solution-end

use std::marker::PhantomData;

mod sealed {
    pub trait Sealed {}
}

pub trait State: sealed::Sealed {
    const NAME: &'static str;
}

#[derive(Debug)]
pub struct Disconnected;
#[derive(Debug)]
pub struct Connected;
#[derive(Debug)]
pub struct Authenticated;

impl sealed::Sealed for Disconnected {}
impl sealed::Sealed for Connected {}
impl sealed::Sealed for Authenticated {}

impl State for Disconnected {
    const NAME: &'static str = "disconnected";
}
impl State for Connected {
    const NAME: &'static str = "connected";
}
impl State for Authenticated {
    const NAME: &'static str = "authenticated";
}

#[derive(Debug)]
pub struct Connection<S: State> {
    host: String,
    user: Option<String>,
    log: Vec<String>,
    _state: PhantomData<S>,
}

impl<S: State> Connection<S> {
    pub fn host(&self) -> &str {
        // @solution-start
        &self.host
        // @solution-end
    }

    pub fn state(&self) -> &'static str {
        // @solution-start
        S::NAME
        // @solution-end
    }

    pub fn log(&self) -> &[String] {
        // @solution-start
        &self.log
        // @solution-end
    }

    // @solution-start remove
    fn into_state<T: State>(mut self, event: Option<String>) -> Connection<T> {
        self.log.extend(event);
        Connection {
            host: self.host,
            user: self.user,
            log: self.log,
            _state: PhantomData,
        }
    }
    // @solution-end
}

impl Connection<Disconnected> {
    pub fn new(host: &str) -> Self {
        // @solution-start
        Connection {
            host: host.to_string(),
            user: None,
            log: Vec::new(),
            _state: PhantomData,
        }
        // @solution-end
    }

    pub fn connect(self) -> Connection<Connected> {
        // @solution-start
        let event = format!("connect {}", self.host);
        self.into_state(Some(event))
        // @solution-end
    }
}

impl Connection<Connected> {
    pub fn authenticate(
        mut self,
        user: &str,
        password: &str,
    ) -> Result<Connection<Authenticated>, Connection<Connected>> {
        // @solution-start
        let expected: String = user.chars().rev().collect();
        if password == expected {
            self.user = Some(user.to_string());
            Ok(self.into_state(Some(format!("auth {user} ok"))))
        } else {
            self.log.push(format!("auth {user} failed"));
            Err(self)
        }
        // @solution-end
    }

    pub fn disconnect(self) -> Connection<Disconnected> {
        // @solution-start
        self.into_state(Some("disconnect".to_string()))
        // @solution-end
    }
}

impl Connection<Authenticated> {
    pub fn user(&self) -> &str {
        // @solution-start
        // Every `Connection<Authenticated>` is built by `authenticate`, which sets the user.
        self.user.as_deref().unwrap_or_default()
        // @solution-end
    }

    pub fn send(&mut self, message: &str) -> usize {
        // @solution-start
        self.log.push(format!("send {message}"));
        message.len()
        // @solution-end
    }

    pub fn logout(mut self) -> Connection<Connected> {
        // @solution-start
        let user = self.user.take().unwrap_or_default();
        self.into_state(Some(format!("logout {user}")))
        // @solution-end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_session() {
        let conn = Connection::new("db.local").connect();
        assert_eq!(conn.state(), "connected");
        let conn = conn.authenticate("alice", "nope").unwrap_err();
        let mut conn = conn.authenticate("alice", "ecila").unwrap();
        assert_eq!((conn.state(), conn.user()), ("authenticated", "alice"));
        assert_eq!(conn.send("hello"), 5);
        let conn = conn.logout().disconnect();
        assert_eq!(conn.state(), "disconnected");
        assert_eq!(
            conn.log(),
            [
                "connect db.local",
                "auth alice failed",
                "auth alice ok",
                "send hello",
                "logout alice",
                "disconnect"
            ]
        );
    }

    #[test]
    fn new_connection_has_no_events() {
        let conn = Connection::new("example.org");
        assert_eq!((conn.host(), conn.state()), ("example.org", "disconnected"));
        assert!(conn.log().is_empty());
    }

    #[test]
    fn palindromic_names_use_themselves_as_password() {
        let conn = Connection::new("h").connect();
        let conn = conn.authenticate("bob", "bob").unwrap();
        assert_eq!(conn.user(), "bob");
        // Logging out and in again as somebody else works.
        let conn = conn.logout().authenticate("ada", "ada").unwrap();
        assert_eq!(conn.user(), "ada");
        assert_eq!(conn.log().last().map(String::as_str), Some("auth ada ok"));
    }

    #[test]
    fn the_state_marker_is_free() {
        use std::mem::size_of;
        let conn = Connection::new("x");
        assert_eq!(conn.state(), Disconnected::NAME);
        assert_eq!(size_of::<Disconnected>(), 0);
        assert_eq!(
            size_of::<Connection<Connected>>(),
            size_of::<Connection<Authenticated>>()
        );
    }

    #[test]
    fn reconnect_many_times() {
        let mut conn = Connection::new("loop");
        for _ in 0..100 {
            conn = conn.connect().disconnect();
        }
        assert_eq!(conn.log().len(), 200);
        assert_eq!(conn.log()[198], "connect loop");
    }
}
