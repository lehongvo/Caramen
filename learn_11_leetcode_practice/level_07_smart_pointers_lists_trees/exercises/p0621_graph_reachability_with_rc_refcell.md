# Problem 0621: Graph Reachability with Rc and RefCell

- **Difficulty:** Medium
- **Topics:** `Rc<RefCell<GraphNode>>` graphs, `Rc::as_ptr` as an identity key, BFS with `VecDeque`, breaking cycles in `Drop`
- **Source:** Original · [The Rust Book 15.6](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html) · [`Rc::as_ptr`](https://doc.rust-lang.org/std/rc/struct.Rc.html#method.as_ptr)

## Problem

A directed graph can be stored as nodes that point at their neighbors with `Rc`. Edges can be
added after the nodes exist, so each node sits in a `RefCell`:

```text
pub type NodeRef = Rc<RefCell<GraphNode>>;

pub struct GraphNode {
    pub label: String,
    pub neighbors: Vec<NodeRef>,
}
```

Implement `Graph`, which owns every node it creates:

- `Graph::new() -> Graph` and `add_node(&mut self, label: &str) -> NodeRef`;
- `add_edge(&self, from: &NodeRef, to: &NodeRef)`: append `to` to `from`'s neighbors;
- `reachable(&self, start: &NodeRef) -> Vec<String>`: the labels of all nodes reachable from
  `start` (including `start`), in **breadth-first** order, neighbors visited in the order their
  edges were added, each node once (labels may repeat, so track nodes by identity);
- `impl Drop for Graph`: graphs may contain cycles (`a -> b -> a`), and a cycle of `Rc`s is
  never freed. When the graph is dropped, clear every node's neighbor list so all nodes can be
  freed once the caller's handles are gone.

## Examples

```text
a -> b, a -> c, b -> d, c -> d, d -> a
reachable(a) -> ["a", "b", "c", "d"]
reachable(d) -> ["d", "a", "b", "c"]

After drop(graph), a handle the caller kept to `a` has strong_count == 1:
the edge d -> a no longer keeps `a` alive.
```

## Constraints

- At most `10^4` nodes and `10^5` edges.

## Hints

1. A `HashSet<*const RefCell<GraphNode>>` filled with `Rc::as_ptr(&node)` records visited nodes
   by identity.
2. Pop a node from a `VecDeque`, then clone its neighbors' `Rc`s **while the `borrow()` guard is
   alive**, and push the unvisited ones.
3. In `drop`, `node.borrow_mut().neighbors.clear()` for every node the graph created.

## Starter code

Write your code in [`src/practice/p0621_graph_reachability_with_rc_refcell.rs`](../src/practice/p0621_graph_reachability_with_rc_refcell.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0621
```

```rust
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<GraphNode>>;

#[derive(Debug)]
pub struct GraphNode {
    pub label: String,
    pub neighbors: Vec<NodeRef>,
}

#[derive(Default)]
pub struct Graph {}

impl Graph {
    pub fn new() -> Self {
        todo!("Write your solution here")
    }

    pub fn add_node(&mut self, label: &str) -> NodeRef {
        todo!("Write your solution here")
    }

    pub fn add_edge(&self, from: &NodeRef, to: &NodeRef) {
        todo!("Write your solution here")
    }

    pub fn reachable(&self, start: &NodeRef) -> Vec<String> {
        todo!("Write your solution here")
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        // TODO: clear every node's neighbors so cycles are broken.
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

    fn diamond_with_back_edge() -> (Graph, Vec<NodeRef>) {
        let mut graph = Graph::new();
        let nodes: Vec<NodeRef> = ["a", "b", "c", "d"]
            .iter()
            .map(|l| graph.add_node(l))
            .collect();
        for (from, to) in [(0, 1), (0, 2), (1, 3), (2, 3), (3, 0)] {
            graph.add_edge(&nodes[from], &nodes[to]);
        }
        (graph, nodes)
    }

    #[test]
    fn breadth_first_order() {
        let (graph, nodes) = diamond_with_back_edge();
        assert_eq!(graph.reachable(&nodes[0]), vec!["a", "b", "c", "d"]);
        assert_eq!(graph.reachable(&nodes[3]), vec!["d", "a", "b", "c"]);
    }

    #[test]
    fn unreachable_nodes_are_skipped() {
        let mut graph = Graph::new();
        let a = graph.add_node("a");
        let b = graph.add_node("b");
        let lonely = graph.add_node("lonely");
        graph.add_edge(&a, &b);
        assert_eq!(graph.reachable(&b), vec!["b"]);
        assert_eq!(graph.reachable(&lonely), vec!["lonely"]);
        assert_eq!(graph.reachable(&a), vec!["a", "b"]);
    }

    #[test]
    fn duplicate_labels_and_self_loops() {
        let mut graph = Graph::new();
        let x1 = graph.add_node("x");
        let x2 = graph.add_node("x");
        graph.add_edge(&x1, &x1);
        graph.add_edge(&x1, &x2);
        graph.add_edge(&x2, &x1);
        assert_eq!(graph.reachable(&x1), vec!["x", "x"]);
    }

    #[test]
    fn dropping_the_graph_breaks_cycles() {
        let (graph, nodes) = diamond_with_back_edge();
        // The graph's own handle + ours + the edge from d.
        assert_eq!(Rc::strong_count(&nodes[0]), 3);
        drop(graph);
        for node in &nodes {
            assert_eq!(Rc::strong_count(node), 1);
            assert!(node.borrow().neighbors.is_empty());
        }
    }

    #[test]
    fn long_chain() {
        let mut graph = Graph::new();
        let nodes: Vec<NodeRef> = (0..5_000).map(|i| graph.add_node(&i.to_string())).collect();
        for pair in nodes.windows(2) {
            graph.add_edge(&pair[0], &pair[1]);
        }
        let labels = graph.reachable(&nodes[0]);
        assert_eq!(labels.len(), 5_000);
        assert_eq!(labels[4_999], "4999");
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** standard BFS. The queue holds `NodeRef` clones (cheap counter bumps); identity is the
address of the shared allocation. The destructor breaks every cycle by emptying all adjacency
lists, so the only strong handles left are the caller's own.

**Complexity:** `reachable` is `O(V + E)` time and `O(V)` space; `drop` is `O(V + E)`.

**Rust concepts:**
- `Rc` reference counting cannot free cycles; the Book calls this a memory leak that is still
  memory-safe. Clearing edges (or using `Weak`, next problems) fixes it.
- `Rc::as_ptr` gives a stable address for the lifetime of the allocation, ideal as a hash key.
- Short `borrow()` scopes keep `RefCell` happy even when a node points to itself.

```rust
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

pub type NodeRef = Rc<RefCell<GraphNode>>;

#[derive(Debug)]
pub struct GraphNode {
    pub label: String,
    pub neighbors: Vec<NodeRef>,
}

#[derive(Default)]
pub struct Graph {
    nodes: Vec<NodeRef>,
}

impl Graph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, label: &str) -> NodeRef {
        let node = Rc::new(RefCell::new(GraphNode {
            label: label.to_string(),
            neighbors: Vec::new(),
        }));
        self.nodes.push(Rc::clone(&node));
        node
    }

    pub fn add_edge(&self, from: &NodeRef, to: &NodeRef) {
        from.borrow_mut().neighbors.push(Rc::clone(to));
    }

    pub fn reachable(&self, start: &NodeRef) -> Vec<String> {
        let mut visited: HashSet<*const RefCell<GraphNode>> = HashSet::from([Rc::as_ptr(start)]);
        let mut queue = VecDeque::from([Rc::clone(start)]);
        let mut labels = Vec::new();
        while let Some(node) = queue.pop_front() {
            let node = node.borrow();
            labels.push(node.label.clone());
            for next in &node.neighbors {
                if visited.insert(Rc::as_ptr(next)) {
                    queue.push_back(Rc::clone(next));
                }
            }
        }
        labels
    }
}

impl Drop for Graph {
    fn drop(&mut self) {
        for node in &self.nodes {
            node.borrow_mut().neighbors.clear();
        }
    }
}
```

Full tested solution: [`src/solutions/p0621_graph_reachability_with_rc_refcell.rs`](../src/solutions/p0621_graph_reachability_with_rc_refcell.rs)

</details>
