//! # Problem 0234: Option Chaining Through Nested Fields
//!
//! - **Difficulty:** Easy
//! - **Topics:** `Option::as_ref`, `as_deref`, `and_then`, `map`, `or_else`, the `?` operator on `Option`, nested structs
//! - **Source:** Original · [`Option`](https://doc.rust-lang.org/std/option/enum.Option.html) · [The Rust Book 9.2](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#where-the--operator-can-be-used)
//!
//! ## Problem
//!
//! A contact database stores people with optional information at several levels:
//!
//! ```text
//! struct Address { street: String, city: String, zip: Option<String> }
//! struct Company { name: String, address: Option<Address> }
//! struct Person  { name: String, employer: Option<Company>, home: Option<Address> }
//! ```
//!
//! Write these lookups. They all take `&Person` and **borrow** from it (no cloning):
//!
//! - `employer_city(p: &Person) -> Option<&str>`: the city of the employer's address;
//! - `employer_zip(p: &Person) -> Option<&str>`: the zip code of the employer's address;
//! - `mailing_city(p: &Person) -> Option<&str>`: the home city if there is a home address,
//!   otherwise the employer's city;
//! - `home_zip_region(p: &Person) -> Option<u32>`: the first **three** characters of the home zip
//!   code, parsed as a number (`"94107"` gives `941`). `None` if anything is missing, if the zip has
//!   fewer than three characters, or if those characters are not digits.
//!
//! Each answer is `None` as soon as one link of the chain is missing.
//!
//! ## Examples
//!
//! ```text
//! ana  = Person { employer: Some(Company { address: Some(Address { city: "Lyon", zip: Some("69002"), .. }), .. }),
//!                 home: None, .. }
//! employer_city(&ana)   => Some("Lyon")
//! employer_zip(&ana)    => Some("69002")
//! mailing_city(&ana)    => Some("Lyon")       (no home address, falls back to the employer)
//! home_zip_region(&ana) => None               (no home address)
//!
//! bo   = Person { employer: None, home: Some(Address { city: "Oslo", zip: Some("0150"), .. }), .. }
//! employer_city(&bo)    => None
//! mailing_city(&bo)     => Some("Oslo")
//! home_zip_region(&bo)  => Some(15)           ("015" parsed as a number)
//! ```
//!
//! ## Constraints
//!
//! - Strings are short ASCII text.
//!
//! ## Hints
//!
//! 1. `p.employer.as_ref()` turns `&Option<Company>` into `Option<&Company>` so you can chain without moving.
//! 2. Inside a function returning `Option`, `value?` returns `None` early when `value` is `None`.
//! 3. `zip.get(..3)` returns `Option<&str>` (no panic on short strings); `str::parse::<u32>().ok()`
//!    turns a `Result` into an `Option`.
// @solution-start remove
//!
//! ## Solution
//!
//! **Idea:** every lookup walks the chain of optional fields. Two styles are shown: the `?`
//! operator (`employer_city`, `home_zip_region`) and method chains with `and_then` (`employer_zip`,
//! `mailing_city`). Both stop at the first `None`.
//!
//! **Rust concepts:**
//! - `as_ref()` converts `&Option<T>` to `Option<&T>`; without it, `and_then` would try to move the
//!   field out of a borrowed struct.
//! - `as_deref()` converts `&Option<String>` (through `Option<&String>`) to `Option<&str>`.
//! - `?` on an `Option` inside a function that returns `Option` is the same as
//!   `match x { Some(v) => v, None => return None }`.
//! - `or_else(|| ...)` supplies a fallback `Option`, computed only when the first one is `None`.
//! - `"015".parse::<u32>()` gives `Ok(15)`: leading zeros are allowed, so the region of `"0150"` is `15`.
// @solution-end

/// A postal address.
#[derive(Debug, Clone, PartialEq)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub zip: Option<String>,
}

/// A company with an optional address.
#[derive(Debug, Clone, PartialEq)]
pub struct Company {
    pub name: String,
    pub address: Option<Address>,
}

/// A person with optional employer and home address.
#[derive(Debug, Clone, PartialEq)]
pub struct Person {
    pub name: String,
    pub employer: Option<Company>,
    pub home: Option<Address>,
}

/// The city of the employer's address.
pub fn employer_city(p: &Person) -> Option<&str> {
    // @solution-start
    let company = p.employer.as_ref()?;
    let address = company.address.as_ref()?;
    Some(address.city.as_str())
    // @solution-end
}

/// The zip code of the employer's address.
pub fn employer_zip(p: &Person) -> Option<&str> {
    // @solution-start
    p.employer
        .as_ref()
        .and_then(|c| c.address.as_ref())
        .and_then(|a| a.zip.as_deref())
    // @solution-end
}

/// The home city, or else the employer's city.
pub fn mailing_city(p: &Person) -> Option<&str> {
    // @solution-start
    p.home
        .as_ref()
        .map(|a| a.city.as_str())
        .or_else(|| employer_city(p))
    // @solution-end
}

/// The first three digits of the home zip code as a number.
pub fn home_zip_region(p: &Person) -> Option<u32> {
    // @solution-start
    let zip = p.home.as_ref()?.zip.as_deref()?;
    let prefix = zip.get(..3)?;
    if !prefix.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    prefix.parse().ok()
    // @solution-end
}

#[cfg(test)]
mod tests {
    use super::*;

    fn address(city: &str, zip: Option<&str>) -> Address {
        Address {
            street: "1 Main St".to_string(),
            city: city.to_string(),
            zip: zip.map(str::to_string),
        }
    }

    fn ana() -> Person {
        Person {
            name: "Ana".to_string(),
            employer: Some(Company {
                name: "Acme".to_string(),
                address: Some(address("Lyon", Some("69002"))),
            }),
            home: None,
        }
    }

    fn bo() -> Person {
        Person {
            name: "Bo".to_string(),
            employer: None,
            home: Some(address("Oslo", Some("0150"))),
        }
    }

    #[test]
    fn employer_lookups() {
        assert_eq!(employer_city(&ana()), Some("Lyon"));
        assert_eq!(employer_zip(&ana()), Some("69002"));
        assert_eq!(employer_city(&bo()), None);
        assert_eq!(employer_zip(&bo()), None);
    }

    #[test]
    fn employer_without_address_or_zip() {
        let mut p = ana();
        if let Some(c) = p.employer.as_mut() {
            c.address = Some(address("Rome", None));
        }
        assert_eq!(employer_city(&p), Some("Rome"));
        assert_eq!(employer_zip(&p), None);
        if let Some(c) = p.employer.as_mut() {
            c.address = None;
        }
        assert_eq!(employer_city(&p), None);
    }

    #[test]
    fn mailing_city_prefers_home() {
        assert_eq!(mailing_city(&ana()), Some("Lyon"));
        assert_eq!(mailing_city(&bo()), Some("Oslo"));
        let mut both = ana();
        both.home = Some(address("Nice", None));
        assert_eq!(mailing_city(&both), Some("Nice"));
        let nobody = Person {
            name: "X".to_string(),
            employer: None,
            home: None,
        };
        assert_eq!(mailing_city(&nobody), None);
    }

    #[test]
    fn zip_regions() {
        assert_eq!(home_zip_region(&ana()), None);
        assert_eq!(home_zip_region(&bo()), Some(15));
        let mut p = bo();
        p.home = Some(address("SF", Some("94107")));
        assert_eq!(home_zip_region(&p), Some(941));
    }

    #[test]
    fn bad_zip_codes() {
        let mut p = bo();
        p.home = Some(address("A", Some("12")));
        assert_eq!(home_zip_region(&p), None);
        p.home = Some(address("B", Some("SW1A 1AA")));
        assert_eq!(home_zip_region(&p), None);
        p.home = Some(address("C", Some("+12")));
        assert_eq!(home_zip_region(&p), None);
        p.home = Some(address("D", None));
        assert_eq!(home_zip_region(&p), None);
    }
}
