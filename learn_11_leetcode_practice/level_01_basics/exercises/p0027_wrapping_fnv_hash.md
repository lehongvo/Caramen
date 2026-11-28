# Problem 0027: Wrapping FNV-1a Hash

- **Difficulty:** Easy
- **Topics:** `wrapping_mul`, `u32`, XOR `^`, `u8` to `u32` conversion, byte string literals
- **Source:** Original · [`u32::wrapping_mul`](https://doc.rust-lang.org/std/primitive.u32.html#method.wrapping_mul)

## Problem

Hash functions **want** arithmetic that wraps around: they mix bits, and the overflowing high
bits are thrown away on purpose. Implement the 32-bit FNV-1a hash of a byte slice:

```text
hash = 2166136261                     (the "offset basis", 0x811C9DC5)
for each byte b in the input:
    hash = hash XOR b
    hash = hash * 16777619 (mod 2^32) (the "FNV prime", 0x01000193)
return hash
```

Write `fnv1a_32(bytes: &[u8]) -> u32`.

## Examples

```text
fnv1a_32(b"")       -> 0x811C9DC5
fnv1a_32(b"a")      -> 0xE40C292C
fnv1a_32(b"foobar") -> 0xBF9CF968
```

`b"foobar"` is a **byte string literal**: a `&[u8; 6]` holding the ASCII codes of the letters.

## Constraints

- `0 <= bytes.len() <= 10^6`

## Hints

1. A plain `hash * 16777619` panics with "attempt to multiply with overflow" in tests.
2. `u32::wrapping_mul` computes the product modulo `2^32`.
3. XOR needs both sides to have the same type: convert the byte with `u32::from(b)`.

## Starter code

Write your code in [`src/practice/p0027_wrapping_fnv_hash.rs`](../src/practice/p0027_wrapping_fnv_hash.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0027
```

```rust
pub fn fnv1a_32(bytes: &[u8]) -> u32 {
    todo!("Write your solution here")
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
    fn empty_input_is_the_offset_basis() {
        assert_eq!(fnv1a_32(b""), 0x811C_9DC5);
    }

    #[test]
    fn single_letter() {
        assert_eq!(fnv1a_32(b"a"), 0xE40C_292C);
    }

    #[test]
    fn known_vectors() {
        assert_eq!(fnv1a_32(b"foobar"), 0xBF9C_F968);
        assert_eq!(fnv1a_32(b"hello"), 0x4F9F_2CAB);
    }

    #[test]
    fn order_matters() {
        assert_ne!(fnv1a_32(b"ab"), fnv1a_32(b"ba"));
    }

    #[test]
    fn long_input_does_not_panic() {
        let data = vec![0xFFu8; 1_000_000];
        let h = fnv1a_32(&data);
        assert_eq!(h, fnv1a_32(&data));
        assert_ne!(h, fnv1a_32(&data[..999_999]));
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** translate the pseudocode directly, using `^=` for XOR and `wrapping_mul` for the
multiplication modulo `2^32`. Name the two magic numbers with `const`.

**Rust concepts:**
- `wrapping_*` methods (`wrapping_add`, `wrapping_mul`, ...) give modular arithmetic in every
  build mode. Use them when wrap-around is intended; use `checked_*` when overflow is a bug.
  `std::num::Wrapping<u32>` is a wrapper type that makes all operators wrap.
- `u32::from(b)` widens a `u8` losslessly (`b as u32` is equivalent here).
- Hexadecimal literals and `_` separators (`0x0100_0193`) keep constants readable.

```rust
const FNV_OFFSET_BASIS: u32 = 0x811C_9DC5;
const FNV_PRIME: u32 = 0x0100_0193;

pub fn fnv1a_32(bytes: &[u8]) -> u32 {
    let mut hash = FNV_OFFSET_BASIS;
    for &b in bytes {
        hash ^= u32::from(b);
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}
```

Full tested solution: [`src/solutions/p0027_wrapping_fnv_hash.rs`](../src/solutions/p0027_wrapping_fnv_hash.rs)

</details>
