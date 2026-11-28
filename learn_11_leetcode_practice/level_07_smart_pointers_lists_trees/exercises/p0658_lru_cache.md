# Problem 0658: LRU Cache

- **Difficulty:** Medium
- **Topics:** `HashMap<i32, Rc<RefCell<Node>>>`, doubly linked list with `Weak` back links, unlinking and re-linking shared nodes, iterative `Drop`
- **Source:** [LeetCode 146. LRU Cache](https://leetcode.com/problems/lru-cache/)

## Problem

Design a key-value cache with a fixed `capacity` that evicts the **least recently used** entry
when it is full:

- `LRUCache::new(capacity: i32) -> Self`;
- `get(&mut self, key: i32) -> i32`: the value for `key`, or `-1` if it is absent. A successful
  `get` marks the key as the most recently used;
- `put(&mut self, key: i32, value: i32)`: insert or update the key and mark it as the most
  recently used. If inserting a **new** key makes the cache exceed `capacity`, first remove the
  least recently used key.

Both operations must run in `O(1)` average time. (LeetCode's template shows `&self`; its
comment says to switch to `&mut self` when needed, as done here.)

## Examples

```text
let mut cache = LRUCache::new(2);
cache.put(1, 1);            // {1=1}
cache.put(2, 2);            // {1=1, 2=2}
cache.get(1)    -> 1        // 1 is now the most recent
cache.put(3, 3);            // evicts key 2
cache.get(2)    -> -1
cache.put(4, 4);            // evicts key 1
cache.get(1)    -> -1
cache.get(3)    -> 3
cache.get(4)    -> 4
```

## Constraints

- `1 <= capacity <= 3000`, `0 <= key <= 10^4`, `0 <= value <= 10^5`
- At most `2 * 10^5` calls in total.

## Hints

1. A `HashMap` finds a key in `O(1)`; a doubly linked list ordered by recency moves an entry to
   the front and removes the back in `O(1)`. Use both, pointing at the same nodes.
2. Nodes are shared by the map and the list, so they are `Rc<RefCell<Node>>`. Forward links are
   `Rc`; backward links are `Weak` to avoid reference cycles (problem 0626).
3. Write two helpers: `detach(node)` (unlink it from wherever it is) and `push_front(node)`.
   `get` and `put` are then a few lines each.

## Starter code

Write your code in [`src/practice/p0658_lru_cache.rs`](../src/practice/p0658_lru_cache.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0658
```

```rust
pub struct LRUCache {}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        todo!("Write your solution here")
    }

    pub fn get(&mut self, key: i32) -> i32 {
        todo!("Write your solution here")
    }

    pub fn put(&mut self, key: i32, value: i32) {
        todo!("Write your solution here")
    }
}

impl Drop for LRUCache {
    fn drop(&mut self) {
        // TODO (optional): unlink the nodes with a loop so a long list is not dropped recursively.
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
    use std::collections::VecDeque;

    #[test]
    fn example_sequence() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), 1);
        cache.put(3, 3);
        assert_eq!(cache.get(2), -1);
        cache.put(4, 4);
        assert_eq!(cache.get(1), -1);
        assert_eq!(cache.get(3), 3);
        assert_eq!(cache.get(4), 4);
    }

    #[test]
    fn updating_a_key_refreshes_it() {
        let mut cache = LRUCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        cache.put(1, 10);
        cache.put(3, 3);
        assert_eq!(cache.get(1), 10);
        assert_eq!(cache.get(2), -1);
        assert_eq!(cache.get(3), 3);
    }

    #[test]
    fn capacity_one() {
        let mut cache = LRUCache::new(1);
        assert_eq!(cache.get(5), -1);
        cache.put(5, 50);
        assert_eq!(cache.get(5), 50);
        cache.put(6, 60);
        assert_eq!(cache.get(5), -1);
        assert_eq!(cache.get(6), 60);
        cache.put(6, 61);
        assert_eq!(cache.get(6), 61);
    }

    #[test]
    fn matches_a_simple_model() {
        // Model: a VecDeque of (key, value), front = most recent. O(n) per call, but obviously right.
        let capacity = 50;
        let mut cache = LRUCache::new(capacity as i32);
        let mut model: VecDeque<(i32, i32)> = VecDeque::new();
        for step in 0..20_000 {
            let key = (step * 7_919) % 120;
            if step % 3 == 0 {
                let expected = match model.iter().position(|&(k, _)| k == key) {
                    Some(i) => {
                        let entry = model.remove(i).unwrap();
                        model.push_front(entry);
                        entry.1
                    }
                    None => -1,
                };
                assert_eq!(cache.get(key), expected, "step {step}");
            } else {
                if let Some(i) = model.iter().position(|&(k, _)| k == key) {
                    model.remove(i);
                } else if model.len() == capacity {
                    model.pop_back();
                }
                model.push_front((key, step));
                cache.put(key, step);
            }
        }
    }

    #[test]
    fn large_capacity_drops_cleanly() {
        let mut cache = LRUCache::new(3_000);
        for key in 0..10_000 {
            cache.put(key, key * 2);
        }
        assert_eq!(cache.get(6_999), -1);
        assert_eq!(cache.get(7_000), 14_000);
        drop(cache);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** the list runs from the most recently used entry (`head`) to the least recently used
(`tail`). Every access detaches the node and pushes it to the front. Eviction detaches the tail
and removes its key from the map. Nodes store their key so the evicted entry can be removed from
the map.

**Complexity:** `get` and `put` are `O(1)` on average; space `O(capacity)`.

**Rust concepts:**
- The map and the neighbouring nodes each hold an `Rc` to a node; `RefCell` lets any holder
  rewrite its links.
- `Weak` back links mean the only cycles would come from `next` links, and there are none, so
  dropping the cache frees everything; the custom `Drop` just avoids a deep recursive drop of
  the `next` chain.
- `node.borrow_mut()` guards are kept inside small blocks so no two guards on one node overlap.

```rust
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

type NodeRef = Rc<RefCell<Node>>;

struct Node {
    key: i32,
    value: i32,
    prev: Option<Weak<RefCell<Node>>>,
    next: Option<NodeRef>,
}

pub struct LRUCache {
    capacity: usize,
    map: HashMap<i32, NodeRef>,
    head: Option<NodeRef>,
    tail: Option<NodeRef>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            capacity: capacity as usize,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    fn detach(&mut self, node: &NodeRef) {
        let (prev, next) = {
            let mut node = node.borrow_mut();
            (
                node.prev.take().and_then(|weak| weak.upgrade()),
                node.next.take(),
            )
        };
        match &prev {
            Some(prev) => prev.borrow_mut().next = next.clone(),
            None => self.head = next.clone(),
        }
        match &next {
            Some(next) => next.borrow_mut().prev = prev.as_ref().map(Rc::downgrade),
            None => self.tail = prev,
        }
    }

    fn push_front(&mut self, node: NodeRef) {
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(Rc::downgrade(&node));
                node.borrow_mut().next = Some(old_head);
            }
            None => self.tail = Some(Rc::clone(&node)),
        }
        self.head = Some(node);
    }

    pub fn get(&mut self, key: i32) -> i32 {
        let Some(node) = self.map.get(&key).cloned() else {
            return -1;
        };
        self.detach(&node);
        let value = node.borrow().value;
        self.push_front(node);
        value
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.map.get(&key).cloned() {
            node.borrow_mut().value = value;
            self.detach(&node);
            self.push_front(node);
            return;
        }
        if self.map.len() == self.capacity {
            if let Some(lru) = self.tail.clone() {
                self.detach(&lru);
                self.map.remove(&lru.borrow().key);
            }
        }
        let node = Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }));
        self.map.insert(key, Rc::clone(&node));
        self.push_front(node);
    }
}

impl Drop for LRUCache {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(node) = link {
            link = node.borrow_mut().next.take();
        }
    }
}
```

Full tested solution: [`src/solutions/p0658_lru_cache.rs`](../src/solutions/p0658_lru_cache.rs)

</details>
