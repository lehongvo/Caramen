//! # Problem 0248: HTTP Status Classes with @ Bindings
//!
//! - **Difficulty:** Easy
//! - **Topics:** `name @ pattern` bindings, `@` with ranges and with `|` sub-patterns, binding inside struct patterns
//! - **Source:** Original · [The Rust Book 19.3](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html#-bindings) · [The Rust Reference: Identifier patterns](https://doc.rust-lang.org/reference/patterns.html#identifier-patterns)
//!
//! ## Problem
//!
//! HTTP status codes are grouped by their first digit. A pattern like `100..=199` tests a range,
//! but then the matched value is not named; `code @ 100..=199` tests the range **and** binds the
//! value to `code`.
//!
//! Implement:
//!
//! - `classify(status: u16) -> Option<StatusClass>`: `100..=199` gives `Informational(status)`,
//!   `200..=299` `Success(status)`, `300..=399` `Redirect(status)`, `400..=499`
//!   `ClientError(status)`, `500..=599` `ServerError(status)`, anything else `None`;
//! - `describe(status: u16) -> String`: `"200 OK"` for 200, `"404 Not Found"` for 404,
//!   `"<code> retry later"` for 429 and 503, otherwise `"<code> client error"` for `400..=499`,
//!   `"<code> server error"` for `500..=599`, and `"<code> other"` for any other code;
//! - `retry_delay(resp: &Response) -> Option<u32>`: a `Response` has `status: u16` and
//!   `retry_after: Option<u32>` (seconds). Return how long to wait before retrying:
//!   - status `429` or `503` **with** a `retry_after` value: that value;
//!   - status `429` or `503` without one: `60`;
//!   - any other `5xx` status: `5`;
//!   - anything else: `None` (do not retry).
//!
//! ## Examples
//!
//! ```text
//! classify(204)  => Some(Success(204))
//! classify(302)  => Some(Redirect(302))
//! classify(99)   => None
//! describe(404)  => "404 Not Found"
//! describe(418)  => "418 client error"
//! describe(503)  => "503 retry later"
//! describe(502)  => "502 server error"
//! retry_delay(&Response { status: 429, retry_after: Some(30) })  => Some(30)
//! retry_delay(&Response { status: 503, retry_after: None })      => Some(60)
//! retry_delay(&Response { status: 500, retry_after: Some(99) })  => Some(5)
//! retry_delay(&Response { status: 404, retry_after: Some(1) })   => None
//! ```
//!
//! ## Constraints
//!
//! - No special constraints.
//!
//! ## Hints
//!
//! 1. `c @ 200..=299 => Some(StatusClass::Success(c))`.
//! 2. A binding can wrap alternatives: `s @ (429 | 503)`. The parentheses are required.
//! 3. Struct patterns nest: `Response { status: 429 | 503, retry_after: Some(secs) }`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every function is one `match` whose arms test a range or a set of values and bind
//! the matched number with `@` when it is needed in the result.
//!
//! **Rust concepts:**
//! - `name @ PATTERN` binds the whole value that matched `PATTERN`; it is the only way to both
//!   test a range and keep the value without repeating it in a guard.
//! - With alternatives, the parentheses matter: `s @ (429 | 503)` binds in both cases, whereas
//!   `s @ 429 | 503` would be parsed as `(s @ 429) | 503` and is rejected because `s` is not bound in
//!   every alternative.
//! - Inside a struct pattern, a field can hold any pattern: a literal set (`status: 429 | 503`),
//!   a nested enum pattern (`retry_after: Some(secs)`), or a range with a binding
//!   (`status: s @ 500..=599`).
//! - Specific arms (`200`, `404`, `429 | 503`) come before the range arms that also contain them.
// @solution-end

/// The class of an HTTP status code, keeping the code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusClass {
    Informational(u16),
    Success(u16),
    Redirect(u16),
    ClientError(u16),
    ServerError(u16),
}

/// A minimal HTTP response.
#[derive(Debug, Clone, PartialEq)]
pub struct Response {
    pub status: u16,
    pub retry_after: Option<u32>,
}

/// The class of `status`, or `None` outside 100..=599.
pub fn classify(status: u16) -> Option<StatusClass> {
    // @solution-start
    match status {
        c @ 100..=199 => Some(StatusClass::Informational(c)),
        c @ 200..=299 => Some(StatusClass::Success(c)),
        c @ 300..=399 => Some(StatusClass::Redirect(c)),
        c @ 400..=499 => Some(StatusClass::ClientError(c)),
        c @ 500..=599 => Some(StatusClass::ServerError(c)),
        _ => None,
    }
    // @solution-end
}

/// A short human-readable description.
pub fn describe(status: u16) -> String {
    // @solution-start
    match status {
        200 => String::from("200 OK"),
        404 => String::from("404 Not Found"),
        c @ (429 | 503) => format!("{c} retry later"),
        c @ 400..=499 => format!("{c} client error"),
        c @ 500..=599 => format!("{c} server error"),
        c => format!("{c} other"),
    }
    // @solution-end
}

/// Seconds to wait before retrying, or `None` for "do not retry".
pub fn retry_delay(resp: &Response) -> Option<u32> {
    // @solution-start
    match resp {
        Response {
            status: 429 | 503,
            retry_after: Some(secs),
        } => Some(*secs),
        Response {
            status: 429 | 503,
            retry_after: None,
        } => Some(60),
        Response {
            status: 500..=599, ..
        } => Some(5),
        _ => None,
    }
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::StatusClass::*;
    use super::*;

    fn resp(status: u16, retry_after: Option<u32>) -> Response {
        Response {
            status,
            retry_after,
        }
    }

    #[test]
    fn classes() {
        assert_eq!(classify(100), Some(Informational(100)));
        assert_eq!(classify(204), Some(Success(204)));
        assert_eq!(classify(302), Some(Redirect(302)));
        assert_eq!(classify(451), Some(ClientError(451)));
        assert_eq!(classify(599), Some(ServerError(599)));
    }

    #[test]
    fn out_of_range_codes() {
        assert_eq!(classify(99), None);
        assert_eq!(classify(600), None);
        assert_eq!(classify(0), None);
    }

    #[test]
    fn descriptions() {
        assert_eq!(describe(200), "200 OK");
        assert_eq!(describe(404), "404 Not Found");
        assert_eq!(describe(418), "418 client error");
        assert_eq!(describe(503), "503 retry later");
        assert_eq!(describe(429), "429 retry later");
        assert_eq!(describe(502), "502 server error");
        assert_eq!(describe(301), "301 other");
        assert_eq!(describe(999), "999 other");
    }

    #[test]
    fn retry_rules() {
        assert_eq!(retry_delay(&resp(429, Some(30))), Some(30));
        assert_eq!(retry_delay(&resp(503, None)), Some(60));
        assert_eq!(retry_delay(&resp(429, None)), Some(60));
        assert_eq!(retry_delay(&resp(500, Some(99))), Some(5));
        assert_eq!(retry_delay(&resp(599, None)), Some(5));
        assert_eq!(retry_delay(&resp(404, Some(1))), None);
        assert_eq!(retry_delay(&resp(200, None)), None);
    }
}
