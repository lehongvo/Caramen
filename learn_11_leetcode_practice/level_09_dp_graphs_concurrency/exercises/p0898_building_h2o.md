# Problem 0898: Building H2O

- **Difficulty:** Medium
- **Topics:** grouping threads into fixed-size teams (2 H + 1 O), slots in a shared state struct,
  resetting the group with `notify_all`
- **Source:** Adapted from [LeetCode 1117. Building H2O](https://leetcode.com/problems/building-h2o/)

## Problem

LeetCode has no Rust template for this problem, so the API is defined here.

Many threads share one `H2O` value. Each **hydrogen** thread calls `hydrogen` once and each
**oxygen** thread calls `oxygen` once; the callbacks `release_hydrogen` / `release_oxygen`
append `'H'` / `'O'` to the output. The threads must be grouped into water molecules:

- every group consists of exactly two hydrogen and one oxygen threads;
- all three releases of a group happen before any release of the next group, so every
  consecutive block of 3 output characters contains two `'H'` and one `'O'` (in any order).

A thread that arrives when its kind is already complete in the current group must wait for the
next group. The input always has exactly twice as many hydrogen as oxygen threads.

```text
pub struct H2O { /* your fields */ }
impl H2O {
    pub fn new() -> Self;
    pub fn hydrogen(&self, release_hydrogen: impl FnOnce());
    pub fn oxygen(&self, release_oxygen: impl FnOnce());
}
```

## Examples

```text
threads started as "HOH"      -> output such as "HHO", "HOH" or "OHH"
threads started as "OOHHHH"   -> output such as "HHOHHO" or "HOHOHH" (each block of 3 is valid)
```

## Constraints

- `1 <= molecules <= 100`; threads start in any order.

## Hints

1. State of the current group: how many hydrogen and oxygen slots are taken, and how many members
   have finished releasing.
2. A hydrogen thread waits while 2 hydrogen slots are taken; an oxygen thread while 1 oxygen slot
   is taken. After taking a slot, release (the callback), then count yourself as finished.
3. The third member to finish resets the group and wakes everybody. New members cannot join
   before that, because all slots stay taken until the reset.

## Starter code

Write your code in [`src/practice/p0898_building_h2o.rs`](../src/practice/p0898_building_h2o.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0898
```

```rust
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

pub struct H2O {}

impl H2O {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn hydrogen(&self, release_hydrogen: impl FnOnce()) {
        todo!("Write your solution here")
    }

    pub fn oxygen(&self, release_oxygen: impl FnOnce()) {
        todo!("Write your solution here")
    }
}

impl Default for H2O {
    fn default() -> Self {
        Self::new()
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
    use std::thread;

    /// Starts one thread per character of `atoms`, in order, and returns the output.
    fn run(atoms: &str) -> String {
        let h2o = H2O::new();
        let out = Mutex::new(String::new());
        thread::scope(|s| {
            for atom in atoms.chars() {
                let (h2o, out) = (&h2o, &out);
                s.spawn(move || {
                    if atom == 'H' {
                        h2o.hydrogen(|| out.lock().unwrap().push('H'));
                    } else {
                        h2o.oxygen(|| out.lock().unwrap().push('O'));
                    }
                });
            }
        });
        out.into_inner().unwrap()
    }

    fn assert_molecules(output: &str, molecules: usize) {
        assert_eq!(output.len(), 3 * molecules, "{output}");
        for block in output.as_bytes().chunks(3) {
            let h = block.iter().filter(|&&b| b == b'H').count();
            assert_eq!(
                h,
                2,
                "invalid molecule {:?} in {output}",
                std::str::from_utf8(block)
            );
        }
    }

    #[test]
    fn example_one_molecule() {
        assert_molecules(&run("HOH"), 1);
    }

    #[test]
    fn example_two_molecules() {
        assert_molecules(&run("OOHHHH"), 2);
    }

    #[test]
    fn hydrogens_first() {
        assert_molecules(&run("HHHHHHOOO"), 3);
    }

    #[test]
    fn many_molecules_mixed_order() {
        let atoms: String = (0..100)
            .map(|i| if i % 2 == 0 { "HOH" } else { "OHH" })
            .collect();
        assert_molecules(&run(&atoms), 100);
        for _ in 0..20 {
            assert_molecules(&run("OHHHOHHHO"), 3);
        }
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** a `Group { hydrogen, oxygen, finished }` in a mutex. `join(is_hydrogen, release)`:
wait until a slot of the right kind is free, take it, unlock, call `release`, lock again, count
the member as finished; the third one resets the group and calls `notify_all`. Since the slots of
the current group are full until the reset, members of the next group can only take slots after
all three releases of this group have returned.

**Complexity:** `O(1)` work per thread besides waiting; with `m` molecules, a waiting thread can be woken
up to `O(m)` times (once per completed molecule). `O(1)` extra space.

**Rust concepts:**
- The callback runs **without** the lock held, so the three members of a group can release in
  parallel; the slot counting alone guarantees the grouping.
- `drop(guard)` ends a lock early inside a function, before re-locking later.

```rust
use std::sync::{Condvar, Mutex, MutexGuard, PoisonError};

pub struct H2O {
    group: Mutex<Group>,
    changed: Condvar,
}

#[derive(Default)]
struct Group {
    hydrogen: usize,
    oxygen: usize,
    finished: usize,
}

impl H2O {
    pub fn new() -> Self {
        Self {
            group: Mutex::new(Group::default()),
            changed: Condvar::new(),
        }
    }

    fn lock(&self) -> MutexGuard<'_, Group> {
        self.group.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn join(&self, is_hydrogen: bool, release: impl FnOnce()) {
        let guard = self.lock();
        let mut group = self
            .changed
            .wait_while(guard, |g| {
                if is_hydrogen {
                    g.hydrogen == 2
                } else {
                    g.oxygen == 1
                }
            })
            .unwrap_or_else(PoisonError::into_inner);
        if is_hydrogen {
            group.hydrogen += 1;
        } else {
            group.oxygen += 1;
        }
        drop(group);
        release();
        let mut group = self.lock();
        group.finished += 1;
        if group.finished == 3 {
            *group = Group::default();
            self.changed.notify_all();
        }
    }

    pub fn hydrogen(&self, release_hydrogen: impl FnOnce()) {
        self.join(true, release_hydrogen);
    }

    pub fn oxygen(&self, release_oxygen: impl FnOnce()) {
        self.join(false, release_oxygen);
    }
}

impl Default for H2O {
    fn default() -> Self {
        Self::new()
    }
}
```

Full tested solution: [`src/solutions/p0898_building_h2o.rs`](../src/solutions/p0898_building_h2o.rs)

</details>
