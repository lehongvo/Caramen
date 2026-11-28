// ==================================================================
// PRACTICE 0921: replace every `todo!(...)` with your code; keep the tests unchanged.
// Full problem statement: exercises/p0921_typestate_builder.md
// Run the tests:          cargo test practice::p0921
// ==================================================================

//! # Problem 0921: A Builder That Enforces Required Fields
//!
//! - **Difficulty:** Hard
//! - **Topics:** builder pattern, typestate with two type parameters, `PhantomData<(U, M)>`, partially generic `impl` blocks, validation in `build`
//! - **Source:** Original · [std: `PhantomData`](https://doc.rust-lang.org/std/marker/struct.PhantomData.html) · [Reference: Implementations](https://doc.rust-lang.org/reference/items/implementations.html)
//!
//! ## Problem
//!
//! An HTTP request needs a URL and a method; headers, a body and a timeout are optional. A classic
//! builder checks the required fields at run time. Make **forgetting** one a compile error instead,
//! by tracking them in two type parameters, each `Missing` or `Present`:
//!
//! ```text
//! pub struct RequestBuilder<U, M> { url, method, headers, body, timeout_ms, _marker: PhantomData<(U, M)> }
//! ```
//!
//! | impl block | method | effect |
//! | --- | --- | --- |
//! | `RequestBuilder<Missing, Missing>` | `new()` (and `Default`) | empty builder, `timeout_ms = 30_000` |
//! | `RequestBuilder<Missing, M>` | `url(self, url: &str) -> RequestBuilder<Present, M>` | sets the URL |
//! | `RequestBuilder<U, Missing>` | `method(self, m: Method) -> RequestBuilder<U, Present>` | sets the method |
//! | `RequestBuilder<U, M>` | `header(self, name, value) -> Self`, `body(self, text) -> Self`, `timeout_ms(self, ms) -> Self` | optional settings, any order |
//! | `RequestBuilder<Present, Present>` | `build(self) -> Result<Request, BuildError>` | validates |
//!
//! So `url` and `method` can be called in either order but only once each, and `build` only exists
//! once both were called. `build` still checks what types cannot, in this order:
//!
//! - the URL must start with `http://` or `https://`, else `BuildError::InvalidUrl(url)`;
//! - every header name must be non-empty, else `BuildError::EmptyHeaderName`;
//! - `Method::Get` and `Method::Delete` must not have a body, else `BuildError::BodyNotAllowed(method)`.
//!
//! `Request` has public fields `url`, `method`, `headers: Vec<(String, String)>` (in the order
//! added), `body: Option<String>` and `timeout_ms: u32`, plus `summary(&self) -> String` formatted as
//! `"<METHOD> <url> headers=<n> body=<bytes> timeout=<ms>ms"` with the method in upper case and
//! `body=0` when there is none.
//!
//! ## Examples
//!
//! ```text
//! RequestBuilder::new()
//!     .method(Method::Post)
//!     .url("https://api.example.com/items")
//!     .header("Content-Type", "application/json")
//!     .body("{\"id\":1}")
//!     .build()?
//!     .summary()
//! => "POST https://api.example.com/items headers=1 body=8 timeout=30000ms"
//!
//! RequestBuilder::new().url("ftp://x").method(Method::Get).build() => Err(InvalidUrl("ftp://x"))
//! RequestBuilder::new().url("https://x").build()   // does not compile: no method yet
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. Write a private generic helper that rebuilds the struct with new markers:
//!    `fn retype<U2, M2>(self) -> RequestBuilder<U2, M2>`.
//! 2. The marker types are empty structs; they are never instantiated.
//! 3. In `build`, the URL and method are guaranteed to be set: the typestate proves it.

use std::marker::PhantomData;

/// Marker: this required field has not been set.
#[derive(Debug)]
pub struct Missing;
/// Marker: this required field has been set.
#[derive(Debug)]
pub struct Present;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    InvalidUrl(String),
    EmptyHeaderName,
    BodyNotAllowed(Method),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub url: String,
    pub method: Method,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
    pub timeout_ms: u32,
}

impl Request {
    pub fn summary(&self) -> String {
        todo!("Write your solution here")
    }
}

#[derive(Debug)]
pub struct RequestBuilder<U, M> {
    url: Option<String>,
    method: Option<Method>,
    headers: Vec<(String, String)>,
    body: Option<String>,
    timeout_ms: u32,
    _marker: PhantomData<(U, M)>,
}

impl Default for RequestBuilder<Missing, Missing> {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestBuilder<Missing, Missing> {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }
}

impl<U, M> RequestBuilder<U, M> {
    pub fn header(mut self, name: &str, value: &str) -> Self {
        todo!("Write your solution here")
    }

    pub fn body(mut self, text: &str) -> Self {
        todo!("Write your solution here")
    }

    pub fn timeout_ms(mut self, ms: u32) -> Self {
        todo!("Write your solution here")
    }
}

impl<M> RequestBuilder<Missing, M> {
    pub fn url(mut self, url: &str) -> RequestBuilder<Present, M> {
        todo!("Write your solution here")
    }
}

impl<U> RequestBuilder<U, Missing> {
    pub fn method(mut self, method: Method) -> RequestBuilder<U, Present> {
        todo!("Write your solution here")
    }
}

impl RequestBuilder<Present, Present> {
    pub fn build(self) -> Result<Request, BuildError> {
        todo!("Write your solution here")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_with_body_and_header() {
        let request = RequestBuilder::new()
            .method(Method::Post)
            .url("https://api.example.com/items")
            .header("Content-Type", "application/json")
            .body("{\"id\":1}")
            .build()
            .unwrap();
        assert_eq!(
            request.summary(),
            "POST https://api.example.com/items headers=1 body=8 timeout=30000ms"
        );
        assert_eq!(request.body.as_deref(), Some("{\"id\":1}"));
    }

    #[test]
    fn optional_settings_in_any_order() {
        let request = RequestBuilder::default()
            .timeout_ms(500)
            .header("A", "1")
            .url("http://localhost:8080/")
            .header("B", "2")
            .method(Method::Get)
            .header("C", "3")
            .build()
            .unwrap();
        assert_eq!(request.method, Method::Get);
        assert_eq!(request.timeout_ms, 500);
        let names: Vec<&str> = request.headers.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(names, ["A", "B", "C"]);
        assert_eq!(
            request.summary(),
            "GET http://localhost:8080/ headers=3 body=0 timeout=500ms"
        );
    }

    #[test]
    fn validation_errors_in_order() {
        let bad_url = RequestBuilder::new()
            .url("ftp://x")
            .method(Method::Get)
            .build();
        assert_eq!(bad_url, Err(BuildError::InvalidUrl("ftp://x".to_string())));
        let empty_header = RequestBuilder::new()
            .url("https://x")
            .method(Method::Get)
            .header("", "v")
            .body("not allowed either")
            .build();
        assert_eq!(empty_header, Err(BuildError::EmptyHeaderName));
        let body_on_delete = RequestBuilder::new()
            .url("https://x")
            .method(Method::Delete)
            .body("x")
            .build();
        assert_eq!(
            body_on_delete,
            Err(BuildError::BodyNotAllowed(Method::Delete))
        );
    }

    #[test]
    fn put_with_empty_body() {
        let request = RequestBuilder::new()
            .url("https://x/y")
            .method(Method::Put)
            .body("")
            .build()
            .unwrap();
        assert_eq!(request.body, Some(String::new()));
        assert_eq!(
            request.summary(),
            "PUT https://x/y headers=0 body=0 timeout=30000ms"
        );
    }
}
