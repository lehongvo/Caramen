# Problem 0352: Design HashMap

- **Difficulty:** Easy
- **Topics:** buckets `Vec<Vec<(i32, i32)>>`, `iter_mut().find`, `iter().find`, `retain`
- **Source:** [LeetCode 706. Design HashMap](https://leetcode.com/problems/design-hashmap/)

## Problem

Build your own map from integer keys to integer values **without** using `HashMap`, `BTreeMap`
or any other built-in map. Implement the `MyHashMap` struct:

- `MyHashMap::new() -> Self` creates an empty map;
- `put(&mut self, key: i32, value: i32)` stores `value` for `key`, replacing the old value if
  the key is already present;
- `get(&self, key: i32) -> i32` returns the value for `key`, or `-1` if the key is absent;
- `remove(&mut self, key: i32)` deletes the key and its value, if present.

As in "Design HashSet", the mutating methods take `&mut self` (LeetCode's template comment
allows replacing `&self`).

## Examples

```text
let mut map = MyHashMap::new();
map.put(1, 1);
map.put(2, 2);
map.get(1) => 1
map.get(3) => -1
map.put(2, 1);       (update)
map.get(2) => 1
map.remove(2);
map.get(2) => -1
```

## Constraints

- `0 <= key, value <= 10^6`
- At most `10^4` calls in total.

## Hints

1. Reuse the bucket idea of the set, but store `(key, value)` pairs in each bucket.
2. `bucket.iter_mut().find(|(k, _)| *k == key)` returns a mutable reference to the pair, so
   `put` can update the value in place.

## Starter code

Write your code in [`src/practice/p0352_design_hashmap.rs`](../src/practice/p0352_design_hashmap.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0352
```

```rust
/// A map from integers to integers implemented with buckets.
pub struct MyHashMap {}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MyHashMap {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn put(&mut self, key: i32, value: i32) {
        todo!("Write your solution here")
    }

    pub fn get(&self, key: i32) -> i32 {
        todo!("Write your solution here")
    }

    pub fn remove(&mut self, key: i32) {
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
    fn example_sequence() {
        let mut map = MyHashMap::new();
        map.put(1, 1);
        map.put(2, 2);
        assert_eq!(map.get(1), 1);
        assert_eq!(map.get(3), -1);
        map.put(2, 1);
        assert_eq!(map.get(2), 1);
        map.remove(2);
        assert_eq!(map.get(2), -1);
    }

    #[test]
    fn zero_key_and_zero_value() {
        let mut map = MyHashMap::new();
        assert_eq!(map.get(0), -1);
        map.put(0, 0);
        assert_eq!(map.get(0), 0);
        map.remove(0);
        map.remove(0);
        assert_eq!(map.get(0), -1);
    }

    #[test]
    fn colliding_keys() {
        let mut map = MyHashMap::new();
        for k in 0..10 {
            map.put(769 * k, k);
        }
        map.put(769 * 4, 400);
        map.remove(769 * 7);
        assert_eq!(map.get(769 * 4), 400);
        assert_eq!(map.get(769 * 7), -1);
        assert_eq!(map.get(769 * 9), 9);
    }

    #[test]
    fn many_keys() {
        let mut map = MyHashMap::new();
        for k in 0..5_000 {
            map.put(k * 200, k);
        }
        for k in 0..5_000 {
            if k % 2 == 0 {
                map.remove(k * 200);
            }
        }
        assert_eq!(map.get(999_800), 4_999);
        assert_eq!(map.get(999_600), -1);
        assert_eq!(map.get(1), -1);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** separate chaining with a fixed number of buckets (a prime, `769`). A key's bucket is
`key % 769`. `put` updates the pair if the key is in its bucket and pushes a new pair otherwise;
`get` searches the bucket; `remove` filters the pair out with `retain`.

**Complexity:** `O(n / 769)` average time per call, `O(769 + n)` space.

**Rust concepts:**
- `if let Some(pair) = bucket.iter_mut().find(...)` gives `&mut (i32, i32)`; assigning
  `pair.1 = value` changes the stored value.
- `find` hands the closure a reference to each item, so the pattern `|(k, _)|` binds `k` by
  reference (`&i32` or `&mut i32`) thanks to default binding modes; compare with `*k == key`.
- `map_or(-1, |&(_, v)| v)` converts "not found" into LeetCode's `-1`.
- The `new_without_default` Clippy lint is allowed because LeetCode's template only has `new()`.

```rust
const BUCKETS: usize = 769;

/// A map from integers to integers implemented with buckets.
pub struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}

// LeetCode's template only defines `new()`; Clippy would like a `Default` impl as well.
#[allow(clippy::new_without_default)]
impl MyHashMap {
    pub fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); BUCKETS],
        }
    }

    fn bucket(key: i32) -> usize {
        key.rem_euclid(BUCKETS as i32) as usize
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let bucket = &mut self.buckets[Self::bucket(key)];
        if let Some(pair) = bucket.iter_mut().find(|(k, _)| *k == key) {
            pair.1 = value;
        } else {
            bucket.push((key, value));
        }
    }

    pub fn get(&self, key: i32) -> i32 {
        self.buckets[Self::bucket(key)]
            .iter()
            .find(|(k, _)| *k == key)
            .map_or(-1, |&(_, v)| v)
    }

    pub fn remove(&mut self, key: i32) {
        self.buckets[Self::bucket(key)].retain(|&(k, _)| k != key);
    }
}
```

Full tested solution: [`src/solutions/p0352_design_hashmap.rs`](../src/solutions/p0352_design_hashmap.rs)

</details>
