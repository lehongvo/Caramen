//! # Problem 0872: Request-Reply Server
//!
//! - **Difficulty:** Medium
//! - **Topics:** a server thread that owns a `Receiver`, `enum` messages that carry their own reply
//!   `Sender`, clean shutdown when every client hangs up
//! - **Source:** Original · [The Rust Book 16.2](https://doc.rust-lang.org/book/ch16-02-message-passing.html) ·
//!   [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html)
//!
//! ## Problem
//!
//! Channels are one-way. To get an answer back, a client can put a fresh `Sender` **inside** its
//! request; the server answers on that sender. Build such a server.
//!
//! The request type is given:
//!
//! ```text
//! pub enum Request {
//!     Square { value: u64, reply: Sender<u64> },     // answer value * value
//!     Sum { values: Vec<u64>, reply: Sender<u64> },  // answer the sum of values
//! }
//! ```
//!
//! Implement:
//!
//! 1. `spawn_server() -> (Sender<Request>, JoinHandle<usize>)`: start a server thread and return a
//!    sender for requests plus the server's handle. The server answers requests one by one, in
//!    arrival order, until **every** request sender has been dropped; then it stops and its thread
//!    returns the number of requests it handled. If a client stopped listening before its reply
//!    was sent, the server ignores that and keeps running.
//! 2. `ask_square(server: &Sender<Request>, value: u64) -> Option<u64>` and
//!    `ask_sum(server: &Sender<Request>, values: Vec<u64>) -> Option<u64>`: client helpers that
//!    create a reply channel, send the request and wait for the answer. They return `None` if the
//!    server is gone (the request or the reply cannot be delivered).
//!
//! ## Examples
//!
//! ```text
//! let (server, handle) = spawn_server();
//! ask_square(&server, 12)        -> Some(144)
//! ask_sum(&server, vec![1, 2, 3]) -> Some(6)
//! drop(server);
//! handle.join()                  -> Ok(2)
//! ```
//!
//! ## Constraints
//!
//! - Values are small enough that squares and sums fit in a `u64`.
//! - Up to 16 client threads, each with its own clone of the request sender.
//!
//! ## Hints
//!
//! 1. The server loop is just `for request in rx { match request { ... } }`.
//! 2. Ignore the result of `reply.send(answer)`: an error only means that this client went away.
//! 3. A client helper: `let (tx, rx) = mpsc::channel(); server.send(Request::Square { value,
//!    reply: tx }).ok()?; rx.recv().ok()`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** the server thread owns the request `Receiver` and loops over it, matching on the
//! request kind and answering through the embedded reply sender. The loop ends when all request
//! senders are dropped, and the thread returns its counter. Each client call creates a one-shot
//! reply channel, sends the request, and blocks on `recv` for the answer.
//!
//! **Rust concepts:**
//! - `Sender<T>` is `Send`, so it can travel inside a message to another thread: channels of
//!   channels are a classic way to build request/response on top of one-way messaging.
//! - `?` works on `Option` too: `.ok()?` turns a send error into an early `None`.
//! - The server's `JoinHandle<usize>` doubles as a shutdown report.
// @solution-end

use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};

/// A request for the server; every request carries the channel for its answer.
pub enum Request {
    Square {
        value: u64,
        reply: Sender<u64>,
    },
    Sum {
        values: Vec<u64>,
        reply: Sender<u64>,
    },
}

pub fn spawn_server() -> (Sender<Request>, JoinHandle<usize>) {
    // @solution-start
    let (tx, rx) = mpsc::channel::<Request>();
    let handle = thread::spawn(move || {
        let mut handled = 0;
        for request in rx {
            handled += 1;
            // A failed reply only means that this client stopped listening.
            let _ = match request {
                Request::Square { value, reply } => reply.send(value * value),
                Request::Sum { values, reply } => reply.send(values.iter().sum()),
            };
        }
        handled
    });
    (tx, handle)
    // @solution-end
}

pub fn ask_square(server: &Sender<Request>, value: u64) -> Option<u64> {
    // @solution-start
    let (reply, answer) = mpsc::channel();
    server.send(Request::Square { value, reply }).ok()?;
    answer.recv().ok()
    // @solution-end
}

pub fn ask_sum(server: &Sender<Request>, values: Vec<u64>) -> Option<u64> {
    // @solution-start
    let (reply, answer) = mpsc::channel();
    server.send(Request::Sum { values, reply }).ok()?;
    answer.recv().ok()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_two_requests() {
        let (server, handle) = spawn_server();
        assert_eq!(ask_square(&server, 12), Some(144));
        assert_eq!(ask_sum(&server, vec![1, 2, 3]), Some(6));
        drop(server);
        assert_eq!(handle.join().unwrap(), 2);
    }

    #[test]
    fn server_with_no_requests() {
        let (server, handle) = spawn_server();
        drop(server);
        assert_eq!(handle.join().unwrap(), 0);
    }

    #[test]
    fn dead_server_gives_none() {
        let (server, rx) = mpsc::channel::<Request>();
        drop(rx);
        assert_eq!(ask_square(&server, 3), None);
        assert_eq!(ask_sum(&server, vec![1]), None);
    }

    #[test]
    fn client_that_hangs_up_does_not_stop_the_server() {
        let (server, handle) = spawn_server();
        let (reply, answer) = mpsc::channel();
        drop(answer);
        server.send(Request::Square { value: 2, reply }).unwrap();
        assert_eq!(ask_sum(&server, vec![]), Some(0));
        drop(server);
        assert_eq!(handle.join().unwrap(), 2);
    }

    #[test]
    fn many_clients_share_one_server() {
        let (server, handle) = spawn_server();
        let clients: Vec<_> = (0..16_u64)
            .map(|c| {
                let server = server.clone();
                thread::spawn(move || {
                    for v in 0..100 {
                        let value = c * 1000 + v;
                        assert_eq!(ask_square(&server, value), Some(value * value));
                    }
                    assert_eq!(ask_sum(&server, vec![c; 10]), Some(10 * c));
                })
            })
            .collect();
        for client in clients {
            client.join().unwrap();
        }
        drop(server);
        assert_eq!(handle.join().unwrap(), 16 * 101);
    }
}
