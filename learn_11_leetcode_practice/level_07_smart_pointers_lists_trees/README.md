# Level 7: Smart Pointers, Linked Lists & Trees (problems 0601-0700)

> Focus: `Box`, `Rc`, `RefCell`, `Weak`, `Drop`, linked lists, binary trees.

This level is about values that live behind pointers: heap allocations with `Box`, shared
ownership with `Rc`, mutation through shared references with `Cell` and `RefCell`, and non-owning
links with `Weak`. The first 26 problems practise each smart pointer on its own (The Rust Book,
chapter 15); the rest apply them to the two classic pointer-based structures, linked lists and
binary trees, mostly as LeetCode problems with LeetCode's exact Rust types.

## Prerequisites

- Levels 1-6, in particular ownership and borrowing (level 2), `Option` and `match` (level 3),
  `HashMap`/`VecDeque`/`BinaryHeap` (level 4), closures and iterators (level 5), and traits and
  lifetimes (level 6).
- `src/common.rs` holds LeetCode's `ListNode` and `TreeNode` and the test helpers `list`,
  `list_to_vec`, `tree`, `parse_tree`, `tree_to_vec`, `tree_to_string` and `find_node`. Read it
  once before problem 0627.

## What you will practise

- `Box<T>` for recursive types such as cons lists and expression trees
  ([The Rust Book 15.1](https://doc.rust-lang.org/book/ch15-01-box.html),
  [`Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html)).
- `Deref`/`DerefMut` and deref coercion
  ([The Rust Book 15.2](https://doc.rust-lang.org/book/ch15-02-deref.html),
  [`Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)).
- `Drop`, drop order and `std::mem::drop`, including iterative destructors for long lists
  ([The Rust Book 15.3](https://doc.rust-lang.org/book/ch15-03-drop.html),
  [destructors](https://doc.rust-lang.org/reference/destructors.html)).
- `Rc<T>`, `Rc::clone`, `strong_count`, `ptr_eq`, `make_mut` and structural sharing
  ([The Rust Book 15.4](https://doc.rust-lang.org/book/ch15-04-rc.html),
  [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html)).
- Interior mutability with `Cell<T>` and `RefCell<T>`, runtime borrow checks and their panics,
  and the `Rc<RefCell<T>>` pattern
  ([The Rust Book 15.5](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html),
  [`std::cell`](https://doc.rust-lang.org/std/cell/index.html)).
- `Weak<T>` for parent pointers, caches and doubly linked lists, and how reference cycles leak
  ([The Rust Book 15.6](https://doc.rust-lang.org/book/ch15-06-reference-cycles.html),
  [`Weak`](https://doc.rust-lang.org/std/rc/struct.Weak.html)).
- Linked lists with `Option<Box<ListNode>>`: `take`, `as_deref`, tail cursors, dummy heads,
  splitting, reversing and merging
  ([`Option`](https://doc.rust-lang.org/std/option/enum.Option.html)).
- Binary trees with `Option<Rc<RefCell<TreeNode>>>`: recursive and iterative DFS, BFS by levels,
  binary search trees, construction and serialization.

## Roadmap

| Problems | Topic |
| -------- | ----- |
| 0601-0604 | `Box<T>`: cons list, expression trees, a `Box`-based BST |
| 0605-0609 | `Deref`, `DerefMut`, `Drop`, scope guards, iterative drop |
| 0610-0613 | `Rc<T>`: shared tails, interning, persistent stack, copy-on-write |
| 0614-0618 | `Cell<T>` and `RefCell<T>`: counters, caches, mocks, runtime borrow rules |
| 0619-0622 | `Rc<RefCell<T>>`: shared accounts, event bus, graphs, a circular list |
| 0623-0626 | `Weak<T>`: caches, parent pointers, breaking cycles, doubly linked list |
| 0627-0635 | Linked list basics: building, walking, reversing, merging, iterators |
| 0636-0656 | Linked list techniques: arithmetic, splitting, partitioning, sorting |
| 0657-0660 | Linked list design and hard problems: `MyLinkedList`, LRU cache, k-way merge, k-group reversal |
| 0661-0677 | Binary tree basics: building, depth, traversals, symmetry, paths, sharing subtrees |
| 0678-0681 | Binary search tree basics: search, range sum, building from a sorted array |
| 0682-0687 | Breadth-first levels and depth-first paths |
| 0688-0694 | Binary search tree operations: validate, insert, delete, k-th smallest, LCA, iterator |
| 0695-0700 | Construction, flattening, list/tree conversions, maximum path sum, serialization |

## How to work on a problem

```bash
cd learn_11_leetcode_practice/level_07_smart_pointers_lists_trees

cargo test practice::p0601      # run the tests for the problem you are solving
cargo test solutions::p0601     # run the tests against the reference solution
cargo test solutions                # run every reference solution in this level
```

1. Read the statement in `exercises/` (click a problem in the table below).
2. Write your code in the file with the same name in `src/practice/`: replace each `todo!(...)`. **Do not edit the tests.**
3. Run `cargo test practice::pNNNN` until every test passes.
4. Open the **Answer** section at the bottom of the Markdown file to compare (the tested reference lives in `src/solutions/`).

Problems whose source is **LC** use LeetCode's exact Rust signature, so you can paste the `impl Solution { ... }` block into LeetCode and submit it.

## Problems

<!-- PROBLEM_TABLE:START -->

| # | Problem | Difficulty | Topics | Source |
| --- | ------- | ---------- | ------ | ------ |
| 0601 | [Cons List with Box](exercises/p0601_cons_list_with_box.md) | Easy | `Box<T>`, recursive `enum`, `while let`, `Iterator::rev` + `fold` | Original |
| 0602 | [Evaluate an Expression Tree](exercises/p0602_evaluate_expression_tree.md) | Easy | `Box<T>` in enum variants, recursion, `match` through `&Box<T>`, `checked_*` arithmetic, `?` on `Option` | Original |
| 0603 | [Expression Tree to String](exercises/p0603_expression_tree_to_string.md) | Easy | `Box<T>` recursive enum, `impl fmt::Display`, `write!`, operator precedence | Original |
| 0604 | [Binary Search Tree with Box](exercises/p0604_box_binary_search_tree.md) | Medium | `Option<Box<Node>>`, `&mut Option<Box<T>>` cursor, `Ordering`, recursion vs loops | Original |
| 0605 | [MyBox with Deref](exercises/p0605_my_box_with_deref.md) | Easy | `Deref` trait, `*` operator on custom types, deref coercion, generic tuple structs | Original |
| 0606 | [A Stack that Derefs to a Slice](exercises/p0606_deref_to_slice_wrapper.md) | Easy | `Deref<Target = [T]>`, `DerefMut`, slice methods through a wrapper, generic structs | Original |
| 0607 | [Drop Order Logger](exercises/p0607_drop_order_logger.md) | Easy | `Drop` trait, drop order of locals, fields and `Vec` items, `std::mem::drop`, `Option::take` | Original |
| 0608 | [Scope Guard](exercises/p0608_scope_guard.md) | Easy | `Drop` running cleanup code, `FnOnce` stored in `Option`, `Option::take`, panics and unwinding | Original |
| 0609 | [Iterative Drop for a Long List](exercises/p0609_iterative_drop_for_long_list.md) | Medium | custom `Drop`, `Option<Box<Node<T>>>`, `Option::take`, `Option::map`, stack overflow in recursive destructors | Original |
| 0610 | [Shared Tail Lists with Rc](exercises/p0610_shared_tail_lists_with_rc.md) | Easy | `Rc<T>`, `Rc::clone`, `Rc::strong_count`, `Rc::ptr_eq`, sharing instead of copying | Original |
| 0611 | [String Interner with Rc](exercises/p0611_string_interner_with_rc.md) | Medium | `Rc<str>`, `Rc::ptr_eq`, `Rc::as_ptr`, `HashSet<Rc<str>>` looked up by `&str` (`Borrow`) | Original |
| 0612 | [Persistent Stack](exercises/p0612_persistent_stack.md) | Medium | structural sharing with `Rc`, immutable data structures, `Rc::try_unwrap`, iterative `Drop` | Original |
| 0613 | [Copy-on-Write Document](exercises/p0613_copy_on_write_with_make_mut.md) | Medium | `Rc::make_mut`, clone-on-write, `Rc::ptr_eq`, cheap snapshots | Original |
| 0614 | [Call Counter with Cell](exercises/p0614_cell_call_counter.md) | Easy | `Cell<T>`, interior mutability through `&self`, `Cell::get`/`set`/`replace`, `Cell` in a shared slice | Original |
| 0615 | [Lazy Cached Value with Cell](exercises/p0615_lazy_value_with_cell.md) | Easy | `Cell<Option<T>>`, memoization behind `&self`, `Cell::take`, `Cell::replace` | Original |
| 0616 | [Mock Messenger with RefCell](exercises/p0616_mock_messenger_with_refcell.md) | Easy | `RefCell<Vec<String>>`, `borrow_mut` behind `&self`, test doubles, traits with `&self` methods | Original |
| 0617 | [Runtime Borrow Checks](exercises/p0617_runtime_borrow_checks.md) | Medium | `RefCell::try_borrow`, `try_borrow_mut`, `BorrowError`/`BorrowMutError`, borrow panics, guard lifetimes | Original |
| 0618 | [Transfer Between RefCells](exercises/p0618_transfer_between_cells.md) | Medium | aliasing `&RefCell<T>` arguments, `std::ptr::eq`, `RefCell::swap`, `Vec::drain`, short-lived guards | Original |
| 0619 | [Shared Bank Account](exercises/p0619_shared_bank_account.md) | Easy | `Rc<RefCell<T>>`, several owners mutating one value, `Rc::strong_count` | Original |
| 0620 | [Event Bus with Shared Inboxes](exercises/p0620_event_bus_subscribers.md) | Medium | `Rc<RefCell<Vec<String>>>`, `HashMap<String, Vec<Rc<..>>>`, `Rc::ptr_eq`, `Vec::retain` | Original |
| 0621 | [Graph Reachability with Rc and RefCell](exercises/p0621_graph_reachability_with_rc_refcell.md) | Medium | `Rc<RefCell<GraphNode>>` graphs, `Rc::as_ptr` as an identity key, BFS with `VecDeque`, breaking cycles in `Drop` | Original |
| 0622 | [Josephus Circle with Rc](exercises/p0622_josephus_circle_with_rc.md) | Medium | circular `Rc<RefCell<..>>` list, unlinking nodes with `Option::take`, `std::iter::from_fn`, breaking a cycle in `Drop` | Original |
| 0623 | [Weak Cache](exercises/p0623_weak_cache.md) | Medium | `Weak<T>`, `Rc::downgrade`, `Weak::upgrade`, `Rc::weak_count`, `HashMap::retain` | Original |
| 0624 | [Tree with Parent Pointers](exercises/p0624_tree_with_parent_pointers.md) | Medium | `Weak<T>` parent links, `Rc<T>` child links, `RefCell<Weak<T>>`, `strong_count`/`weak_count` | Original |
| 0625 | [Breaking Reference Cycles](exercises/p0625_breaking_reference_cycles.md) | Medium | reference cycles and leaks, `Weak<T>` back links, `RefCell<Option<Rc<T>>>`, `Drop` as a leak detector | Original |
| 0626 | [Doubly Linked List with Rc and Weak](exercises/p0626_doubly_linked_list_with_weak.md) | Medium | `Rc<RefCell<Node>>` forward links, `Weak` backward links, `Rc::try_unwrap`, `RefCell::into_inner`, iterative `Drop` | Original |
| 0627 | [Build a List with a Tail Cursor](exercises/p0627_list_builder_with_tail_cursor.md) | Easy | LeetCode's `ListNode`, `&mut Option<Box<ListNode>>` cursors, `Option::insert`, `Option::as_deref` | Original |
| 0628 | [Convert Binary Number in a Linked List to Integer](exercises/p0628_convert_binary_number_in_a_linked_list_to_integer.md) | Easy | walking `Option<Box<ListNode>>` with `as_deref`, bit shifting, `std::iter::successors` | [LC 1290](https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/) |
| 0629 | [Middle of the Linked List](exercises/p0629_middle_of_the_linked_list.md) | Easy | counting pass with `std::iter::successors`, owning cursor, moving out of `Box` fields, why slow/fast pointers do not fit owned lists | [LC 876](https://leetcode.com/problems/middle-of-the-linked-list/) |
| 0630 | [Reverse Linked List](exercises/p0630_reverse_linked_list.md) | Easy | `Option::take`, moving boxes between two lists, `while let`, recursion vs iteration | [LC 206](https://leetcode.com/problems/reverse-linked-list/) |
| 0631 | [Merge Two Sorted Lists](exercises/p0631_merge_two_sorted_lists.md) | Easy | `match` on a pair of `Option<Box<ListNode>>`, tail cursor with `Option::insert`, moving nodes | [LC 21](https://leetcode.com/problems/merge-two-sorted-lists/) |
| 0632 | [Remove Duplicates from Sorted List](exercises/p0632_remove_duplicates_from_sorted_list.md) | Easy | `Option::as_mut` cursor, unlinking with `take`, `Option::is_some_and` | [LC 83](https://leetcode.com/problems/remove-duplicates-from-sorted-list/) |
| 0633 | [Remove Linked List Elements](exercises/p0633_remove_linked_list_elements.md) | Easy | rebuilding a list with a tail cursor, `Option::take`, why the "cursor on the current slot" loop fights the borrow checker | [LC 203](https://leetcode.com/problems/remove-linked-list-elements/) |
| 0634 | [Palindrome Linked List](exercises/p0634_palindrome_linked_list.md) | Easy | collecting a list into a `Vec`, `std::iter::successors`, comparing with a reversed iterator | [LC 234](https://leetcode.com/problems/palindrome-linked-list/) |
| 0635 | [Iterators for a Box List](exercises/p0635_linked_list_iterators.md) | Medium | `Iterator` for a linked list, `IntoIter`/`Iter<'a>`/`IterMut<'a>`, `Option::as_deref_mut`, `IntoIterator` for `&List` and `&mut List` | Original |
| 0636 | [Merge Nodes in Between Zeros](exercises/p0636_merge_nodes_in_between_zeros.md) | Medium | reading with `as_deref`, building with a tail cursor, running sums | [LC 2181](https://leetcode.com/problems/merge-nodes-in-between-zeros/) |
| 0637 | [Insert Greatest Common Divisors in Linked List](exercises/p0637_insert_greatest_common_divisors_in_linked_list.md) | Medium | inserting nodes through `&mut Box<ListNode>`, `Option::take`, Euclid's algorithm | [LC 2807](https://leetcode.com/problems/insert-greatest-common-divisors-in-linked-list/) |
| 0638 | [Delete the Middle Node of a Linked List](exercises/p0638_delete_the_middle_node_of_a_linked_list.md) | Medium | counting pass, stopping a `&mut` cursor before the target, unlinking with `take` | [LC 2095](https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/) |
| 0639 | [Maximum Twin Sum of a Linked List](exercises/p0639_maximum_twin_sum_of_a_linked_list.md) | Medium | list to `Vec`, pairing the front and back halves with `zip` and `rev`, `Iterator::max` | [LC 2130](https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/) |
| 0640 | [Remove Nth Node From End of List](exercises/p0640_remove_nth_node_from_end_of_list.md) | Medium | dummy head node, length pass, `&mut Box<ListNode>` cursor, unlinking | [LC 19](https://leetcode.com/problems/remove-nth-node-from-end-of-list/) |
| 0641 | [Add Two Numbers](exercises/p0641_add_two_numbers.md) | Medium | consuming two lists with `Option::take`, carry arithmetic, tail cursor | [LC 2](https://leetcode.com/problems/add-two-numbers/) |
| 0642 | [Add Two Numbers II](exercises/p0642_add_two_numbers_ii.md) | Medium | `Vec` as a stack of digits, prepending nodes (`next: result`), carry arithmetic | [LC 445](https://leetcode.com/problems/add-two-numbers-ii/) |
| 0643 | [Swap Nodes in Pairs](exercises/p0643_swap_nodes_in_pairs.md) | Medium | detaching two nodes at a time, re-linking owned boxes, tail cursor | [LC 24](https://leetcode.com/problems/swap-nodes-in-pairs/) |
| 0644 | [Rotate List](exercises/p0644_rotate_list.md) | Medium | splitting a list with `take`, reattaching at the end, `k % len` | [LC 61](https://leetcode.com/problems/rotate-list/) |
| 0645 | [Partition List](exercises/p0645_partition_list.md) | Medium | two output lists with two tail cursors, stable partition, joining lists | [LC 86](https://leetcode.com/problems/partition-list/) |
| 0646 | [Odd Even Linked List](exercises/p0646_odd_even_linked_list.md) | Medium | splitting by position with two tail cursors, `Option::insert`, a flipping `bool`, joining lists | [LC 328](https://leetcode.com/problems/odd-even-linked-list/) |
| 0647 | [Remove Duplicates from Sorted List II](exercises/p0647_remove_duplicates_from_sorted_list_ii.md) | Medium | skipping runs of nodes, `Option::is_some_and`, rebuilding with a tail cursor | [LC 82](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/) |
| 0648 | [Delete Nodes From Linked List Present in Array](exercises/p0648_delete_nodes_from_linked_list_present_in_array.md) | Medium | `HashSet` lookups, filtering a list while moving its nodes, tail cursor | [LC 3217](https://leetcode.com/problems/delete-nodes-from-linked-list-present-in-array/) |
| 0649 | [Remove Nodes From Linked List](exercises/p0649_remove_nodes_from_linked_list.md) | Medium | reversing to look "to the right", running maximum, prepending restores the order | [LC 2487](https://leetcode.com/problems/remove-nodes-from-linked-list/) |
| 0650 | [Next Greater Node In Linked List](exercises/p0650_next_greater_node_in_linked_list.md) | Medium | list to `Vec`, monotonic stack of indices, `while let Some(&top) = waiting.last()` | [LC 1019](https://leetcode.com/problems/next-greater-node-in-linked-list/) |
| 0651 | [Reverse Linked List II](exercises/p0651_reverse_linked_list_ii.md) | Medium | dummy head, detaching a segment, reversing by prepending, re-attaching both ends | [LC 92](https://leetcode.com/problems/reverse-linked-list-ii/) |
| 0652 | [Split Linked List in Parts](exercises/p0652_split_linked_list_in_parts.md) | Medium | cutting a list into owned pieces, `Vec<Option<Box<ListNode>>>`, size arithmetic | [LC 725](https://leetcode.com/problems/split-linked-list-in-parts/) |
| 0653 | [Reorder List](exercises/p0653_reorder_list.md) | Medium | `&mut Option<Box<ListNode>>` parameters, splitting, reversing and weaving owned lists | [LC 143](https://leetcode.com/problems/reorder-list/) |
| 0654 | [Insertion Sort List](exercises/p0654_insertion_sort_list.md) | Medium | dummy head of a sorted list, searching with a `&mut Box<ListNode>` cursor, splicing a node in | [LC 147](https://leetcode.com/problems/insertion-sort-list/) |
| 0655 | [Sort List](exercises/p0655_sort_list.md) | Medium | merge sort on owned lists, splitting by length, recursive helpers inside `impl Solution` | [LC 148](https://leetcode.com/problems/sort-list/) |
| 0656 | [Remove Zero Sum Consecutive Nodes from Linked List](exercises/p0656_remove_zero_sum_consecutive_nodes_from_linked_list.md) | Medium | prefix sums, `HashMap` of last positions, rebuilding a list from kept values | [LC 1171](https://leetcode.com/problems/remove-zero-sum-consecutive-nodes-from-linked-list/) |
| 0657 | [Design Linked List](exercises/p0657_design_linked_list.md) | Medium | design struct over `Option<Box<Node>>`, a helper returning `&mut Option<Box<Node>>`, `usize::try_from` | [LC 707](https://leetcode.com/problems/design-linked-list/) |
| 0658 | [LRU Cache](exercises/p0658_lru_cache.md) | Medium | `HashMap<i32, Rc<RefCell<Node>>>`, doubly linked list with `Weak` back links, unlinking and re-linking shared nodes, iterative `Drop` | [LC 146](https://leetcode.com/problems/lru-cache/) |
| 0659 | [Merge k Sorted Lists](exercises/p0659_merge_k_sorted_lists.md) | Hard | `BinaryHeap` with `std::cmp::Reverse` as a min-heap, moving nodes out of a `Vec<Option<Box<ListNode>>>`, tail cursor | [LC 23](https://leetcode.com/problems/merge-k-sorted-lists/) |
| 0660 | [Reverse Nodes in k-Group](exercises/p0660_reverse_nodes_in_k_group.md) | Hard | detaching groups of nodes, reversing by prepending, splicing groups with a tail cursor | [LC 25](https://leetcode.com/problems/reverse-nodes-in-k-group/) |
| 0661 | [Build and Mutate a Tree by Hand](exercises/p0661_build_and_mutate_a_tree_by_hand.md) | Easy | LeetCode's `TreeNode`, `Rc<RefCell<TreeNode>>`, `borrow`/`borrow_mut`, shared handles into a tree | Original |
| 0662 | [Root Equals Sum of Children](exercises/p0662_root_equals_sum_of_children.md) | Easy | `let ... else` on `Option<Rc<RefCell<TreeNode>>>`, `borrow()`, matching a pair of children | [LC 2236](https://leetcode.com/problems/root-equals-sum-of-children/) |
| 0663 | [Maximum Depth of Binary Tree](exercises/p0663_maximum_depth_of_binary_tree.md) | Easy | recursion over `&Option<Rc<RefCell<TreeNode>>>`, `borrow()`, `i32::max` | [LC 104](https://leetcode.com/problems/maximum-depth-of-binary-tree/) |
| 0664 | [Binary Tree Preorder Traversal](exercises/p0664_binary_tree_preorder_traversal.md) | Easy | explicit stack of `Rc<RefCell<TreeNode>>`, `Rc::clone`, iterative traversal | [LC 144](https://leetcode.com/problems/binary-tree-preorder-traversal/) |
| 0665 | [Binary Tree Inorder Traversal](exercises/p0665_binary_tree_inorder_traversal.md) | Easy | recursive helper with `&Option<Rc<RefCell<TreeNode>>>` and `&mut Vec<i32>` | [LC 94](https://leetcode.com/problems/binary-tree-inorder-traversal/) |
| 0666 | [Same Tree](exercises/p0666_same_tree.md) | Easy | `match` on a pair of optional nodes, recursion with borrowed trees, derived `PartialEq` | [LC 100](https://leetcode.com/problems/same-tree/) |
| 0667 | [Invert Binary Tree](exercises/p0667_invert_binary_tree.md) | Easy | mutating through `borrow_mut()`, `std::mem::swap` on two fields, returning the same `Rc` | [LC 226](https://leetcode.com/problems/invert-binary-tree/) |
| 0668 | [Symmetric Tree](exercises/p0668_symmetric_tree.md) | Easy | mirror recursion over two subtrees, borrowing two nodes at once | [LC 101](https://leetcode.com/problems/symmetric-tree/) |
| 0669 | [Minimum Depth of Binary Tree](exercises/p0669_minimum_depth_of_binary_tree.md) | Easy | breadth-first search with `VecDeque<(Rc<RefCell<TreeNode>>, i32)>`, leaf detection | [LC 111](https://leetcode.com/problems/minimum-depth-of-binary-tree/) |
| 0670 | [Path Sum](exercises/p0670_path_sum.md) | Easy | root-to-leaf recursion, passing the remaining target down, `let ... else`, short-circuiting `\|\|` | [LC 112](https://leetcode.com/problems/path-sum/) |
| 0671 | [Sum of Left Leaves](exercises/p0671_sum_of_left_leaves.md) | Easy | passing context (`is_left`) down a recursion, leaf tests on borrowed nodes | [LC 404](https://leetcode.com/problems/sum-of-left-leaves/) |
| 0672 | [Binary Tree Paths](exercises/p0672_binary_tree_paths.md) | Easy | backtracking with a shared path `Vec`, `join`, collecting `String`s | [LC 257](https://leetcode.com/problems/binary-tree-paths/) |
| 0673 | [Balanced Binary Tree](exercises/p0673_balanced_binary_tree.md) | Easy | post-order recursion returning `Option<i32>`, early exit with `?` | [LC 110](https://leetcode.com/problems/balanced-binary-tree/) |
| 0674 | [Diameter of Binary Tree](exercises/p0674_diameter_of_binary_tree.md) | Easy | post-order recursion that returns one value and updates another through `&mut` | [LC 543](https://leetcode.com/problems/diameter-of-binary-tree/) |
| 0675 | [Merge Two Binary Trees](exercises/p0675_merge_two_binary_trees.md) | Easy | or-patterns with bindings (`(None, tree) \| (tree, None)`), `borrow_mut` on two nodes, `take()` of children | [LC 617](https://leetcode.com/problems/merge-two-binary-trees/) |
| 0676 | [Subtree of Another Tree](exercises/p0676_subtree_of_another_tree.md) | Easy | tree equality through derived `PartialEq` on `Rc<RefCell<TreeNode>>`, DFS with a stack | [LC 572](https://leetcode.com/problems/subtree-of-another-tree/) |
| 0677 | [Deep Clone a Shared Tree](exercises/p0677_deep_clone_a_shared_tree.md) | Medium | `Rc::clone` (shallow) vs deep copy, `Rc::ptr_eq`/`Rc::as_ptr`, structural sharing between trees | Original |
| 0678 | [Search in a Binary Search Tree](exercises/p0678_search_in_a_binary_search_tree.md) | Easy | BST walk with an owned `Option<Rc<..>>` cursor, `Ordering`, returning a shared subtree handle | [LC 700](https://leetcode.com/problems/search-in-a-binary-search-tree/) |
| 0679 | [Range Sum of BST](exercises/p0679_range_sum_of_bst.md) | Easy | pruned DFS with an explicit stack, using the BST order to skip subtrees | [LC 938](https://leetcode.com/problems/range-sum-of-bst/) |
| 0680 | [Convert Sorted Array to Binary Search Tree](exercises/p0680_convert_sorted_array_to_binary_search_tree.md) | Easy | divide and conquer on slices, `split_at`, building `Rc<RefCell<TreeNode>>` bottom-up | [LC 108](https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/) |
| 0681 | [Minimum Absolute Difference in BST](exercises/p0681_minimum_absolute_difference_in_bst.md) | Easy | iterative in-order traversal with a stack, carrying the previous value in an `Option` | [LC 530](https://leetcode.com/problems/minimum-absolute-difference-in-bst/) |
| 0682 | [Binary Tree Level Order Traversal](exercises/p0682_binary_tree_level_order_traversal.md) | Medium | breadth-first search level by level, building the next level as a `Vec`, `flat_map` | [LC 102](https://leetcode.com/problems/binary-tree-level-order-traversal/) |
| 0683 | [Binary Tree Right Side View](exercises/p0683_binary_tree_right_side_view.md) | Medium | level-by-level BFS, `VecDeque` with a per-level count, `let ... else`, `Extend` from `Option` | [LC 199](https://leetcode.com/problems/binary-tree-right-side-view/) |
| 0684 | [Binary Tree Zigzag Level Order Traversal](exercises/p0684_binary_tree_zigzag_level_order_traversal.md) | Medium | level-by-level BFS, reversing every other row, a flipping `bool` | [LC 103](https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/) |
| 0685 | [Find Bottom Left Tree Value](exercises/p0685_find_bottom_left_tree_value.md) | Medium | BFS that visits right children before left ones, "last dequeued" as the answer | [LC 513](https://leetcode.com/problems/find-bottom-left-tree-value/) |
| 0686 | [Count Good Nodes in Binary Tree](exercises/p0686_count_good_nodes_in_binary_tree.md) | Medium | DFS carrying the path maximum, explicit stack of `(node, max_so_far)` pairs | [LC 1448](https://leetcode.com/problems/count-good-nodes-in-binary-tree/) |
| 0687 | [Path Sum II](exercises/p0687_path_sum_ii.md) | Medium | backtracking with one path `Vec`, cloning the path at a leaf, `Vec<Vec<i32>>` results | [LC 113](https://leetcode.com/problems/path-sum-ii/) |
| 0688 | [Validate Binary Search Tree](exercises/p0688_validate_binary_search_tree.md) | Medium | recursion with `Option<i32>` bounds, avoiding sentinel values, `is_none_or`-style checks | [LC 98](https://leetcode.com/problems/validate-binary-search-tree/) |
| 0689 | [Insert into a Binary Search Tree](exercises/p0689_insert_into_a_binary_search_tree.md) | Medium | walking a BST with `Rc` handles, writing into an empty child slot through `borrow_mut` | [LC 701](https://leetcode.com/problems/insert-into-a-binary-search-tree/) |
| 0690 | [Delete Node in a BST](exercises/p0690_delete_node_in_a_bst.md) | Medium | recursive rebuild with `take()`, matching on a pair of children, in-order successor | [LC 450](https://leetcode.com/problems/delete-node-in-a-bst/) |
| 0691 | [Kth Smallest Element in a BST](exercises/p0691_kth_smallest_element_in_a_bst.md) | Medium | iterative in-order traversal with an explicit stack of `Rc`s, stopping early | [LC 230](https://leetcode.com/problems/kth-smallest-element-in-a-bst/) |
| 0692 | [Lowest Common Ancestor of a Binary Search Tree](exercises/p0692_lowest_common_ancestor_of_a_binary_search_tree.md) | Medium | BST walk with an owned `Rc` cursor, returning a node handle, reading values from `Option<Rc<..>>` arguments | [LC 235](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/) |
| 0693 | [Lowest Common Ancestor of a Binary Tree](exercises/p0693_lowest_common_ancestor_of_a_binary_tree.md) | Medium | post-order search returning `Option<Rc<..>>`, combining two results with `match`, `Option::or` | [LC 236](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/) |
| 0694 | [Binary Search Tree Iterator](exercises/p0694_binary_search_tree_iterator.md) | Medium | design struct holding a `Vec<Rc<RefCell<TreeNode>>>`, lazy in-order traversal, `#[allow(clippy::should_implement_trait)]` | [LC 173](https://leetcode.com/problems/binary-search-tree-iterator/) |
| 0695 | [Construct Binary Tree from Preorder and Inorder Traversal](exercises/p0695_construct_binary_tree_from_preorder_and_inorder_traversal.md) | Medium | `HashMap` of positions, recursion over index ranges, a shared `&mut usize` cursor | [LC 105](https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/) |
| 0696 | [Flatten Binary Tree to Linked List](exercises/p0696_flatten_binary_tree_to_linked_list.md) | Medium | `&mut Option<Rc<RefCell<TreeNode>>>` parameter, collecting `Rc` handles in pre-order, re-linking with `borrow_mut`, `windows(2)` | [LC 114](https://leetcode.com/problems/flatten-binary-tree-to-linked-list/) |
| 0697 | [Convert Sorted List to Binary Search Tree](exercises/p0697_convert_sorted_list_to_binary_search_tree.md) | Medium | from `Option<Box<ListNode>>` to `Option<Rc<RefCell<TreeNode>>>`, slices, divide and conquer | [LC 109](https://leetcode.com/problems/convert-sorted-list-to-binary-search-tree/) |
| 0698 | [Linked List in Binary Tree](exercises/p0698_linked_list_in_binary_tree.md) | Medium | matching a borrowed `Box` list (`Option<&ListNode>`) against an `Rc<RefCell<TreeNode>>` tree, two nested recursions | [LC 1367](https://leetcode.com/problems/linked-list-in-binary-tree/) |
| 0699 | [Binary Tree Maximum Path Sum](exercises/p0699_binary_tree_maximum_path_sum.md) | Hard | post-order "gain" recursion, clamping negative branches with `max(0)`, `&mut i32` accumulator | [LC 124](https://leetcode.com/problems/binary-tree-maximum-path-sum/) |
| 0700 | [Serialize and Deserialize Binary Tree](exercises/p0700_serialize_and_deserialize_binary_tree.md) | Hard | design struct, level-order encoding with `VecDeque`, parsing with `split` and `str::parse`, rebuilding `Rc<RefCell<TreeNode>>` links | [LC 297](https://leetcode.com/problems/serialize-and-deserialize-binary-tree/) |

<!-- PROBLEM_TABLE:END -->
