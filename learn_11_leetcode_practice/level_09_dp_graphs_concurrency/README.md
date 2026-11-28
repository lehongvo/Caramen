# Level 9: Dynamic Programming, Graphs & Concurrency (problems 0801-0900)

> Focus: dynamic programming, graph algorithms, threads, `Arc`, `Mutex`, channels, atomics.

This level has three parts. First, 35 dynamic programming problems take you from rolling
variables (Tribonacci, House Robber) through grids, strings, knapsacks and bitmasks to interval DP
(Burst Balloons). Then 31 graph problems cover BFS/DFS, topological sort, union-find, Dijkstra,
Bellman-Ford, Floyd-Warshall, minimum spanning trees, bridges and Eulerian paths, all written with
iterative searches that cannot overflow the stack. The last 34 problems practise concurrency with
the standard library only: threads, channels, `Arc`/`Mutex`/`RwLock`, scoped threads, atomics,
`Condvar`, `Barrier`, `OnceLock`, and finally a thread pool. Every concurrency test is
deterministic: it checks results that must hold for every possible thread scheduling.

## Prerequisites

- Levels 1-8: ownership and borrowing, structs and enums, the standard collections (`Vec`,
  `HashMap`, `HashSet`, `BTreeMap`, `VecDeque`, `BinaryHeap`), closures and iterators, generics
  and trait bounds, smart pointers (`Box`, `Rc`, `RefCell`), and the algorithm techniques of
  level 8 (BFS/DFS on grids, backtracking, binary search, greedy).
- Reading signatures with closure bounds such as `F: Fn(&T) -> U + Sync`, and lifetimes on structs
  such as `SemaphorePermit<'a>`.

## What you will practice

- Dynamic programming: choosing the state and the recurrence, bottom-up tables, compressing a
  table to one or two rows, filling order for interval DP, 0/1 vs unbounded knapsack loop
  directions, bitmask states, and `Option` or `wrapping_add` to deal with unreachable states and
  overflowing intermediate counts
  ([std `u64::wrapping_add`](https://doc.rust-lang.org/std/primitive.u64.html#method.wrapping_add),
  [`slice::partition_point`](https://doc.rust-lang.org/std/primitive.slice.html#method.partition_point)).
- Graph representations (`Vec<Vec<usize>>`, adjacency matrices, `HashMap<&str, Vec<&str>>`
  borrowing names from the input), BFS with `VecDeque`, iterative DFS with an explicit stack,
  Kahn's algorithm, three-color DFS with an `enum` state, union-find as a small struct, Dijkstra
  with `BinaryHeap` and [`Reverse`](https://doc.rust-lang.org/std/cmp/struct.Reverse.html), a
  float priority via [`f64::total_cmp`](https://doc.rust-lang.org/std/primitive.f64.html#method.total_cmp),
  Bellman-Ford, Floyd-Warshall, Prim's MST, Tarjan's bridges and Hierholzer's algorithm
  ([std `BinaryHeap`](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)).
- Threads: `thread::spawn`, `JoinHandle::join`, `move` closures, panics in threads, and
  `thread::Builder` names
  ([The Rust Book 16.1](https://doc.rust-lang.org/book/ch16-01-threads.html),
  [`thread::spawn`](https://doc.rust-lang.org/std/thread/fn.spawn.html)).
- Message passing: `mpsc::channel`, cloned senders, `sync_channel` and backpressure, request-reply
  with a `Sender` inside a message, pipelines and fan-out/fan-in
  ([The Rust Book 16.2](https://doc.rust-lang.org/book/ch16-02-message-passing.html),
  [`std::sync::mpsc`](https://doc.rust-lang.org/std/sync/mpsc/index.html)).
- Shared state: `Arc<Mutex<T>>`, `Arc::into_inner`, poisoning and `PoisonError::into_inner`,
  `RwLock`, lock ordering against deadlocks, and the `Send`/`Sync` marker traits
  ([The Rust Book 16.3](https://doc.rust-lang.org/book/ch16-03-shared-state.html),
  [The Rust Book 16.4](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html),
  [`Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html),
  [`RwLock`](https://doc.rust-lang.org/std/sync/struct.RwLock.html)).
- Scoped threads that borrow local data, `chunks_mut` for disjoint mutable access, generic parallel
  map and map-reduce, parallel merge sort
  ([`thread::scope`](https://doc.rust-lang.org/std/thread/fn.scope.html)).
- Atomics: `fetch_add`, `fetch_max`, `fetch_min`, `compare_exchange_weak` loops, `AtomicBool`
  cancellation flags and memory `Ordering`
  ([`std::sync::atomic`](https://doc.rust-lang.org/std/sync/atomic/index.html)).
- Synchronization primitives: `Condvar` with `wait_while`, `Barrier`, `OnceLock`, and building a
  latch, a bounded blocking queue, a semaphore with RAII permits, and a thread pool with graceful
  shutdown
  ([`Condvar`](https://doc.rust-lang.org/std/sync/struct.Condvar.html),
  [`Barrier`](https://doc.rust-lang.org/std/sync/struct.Barrier.html),
  [`OnceLock`](https://doc.rust-lang.org/std/sync/struct.OnceLock.html),
  [The Rust Book 21.2](https://doc.rust-lang.org/book/ch21-02-multithreaded.html)).

## Roadmap

| Problems | Theme |
| -------- | ----- |
| 0801-0818 | 1-D dynamic programming: stairs, House Robber, coin change, Word Break, LIS, stock state machines |
| 0819-0823 | Grid DP: unique paths, minimum path sum, triangle, maximal square |
| 0824-0828 | String DP: LCS, edit distance, interleaving, palindromic substrings and subsequences |
| 0829-0835 | Knapsack, bitmask and interval DP, then Hard problems (distinct subsequences, burst balloons, job scheduling) |
| 0836-0845 | Graph basics: degrees, BFS/DFS, components, bipartite check, BFS over state spaces |
| 0846-0850 | Topological sort: course schedules, safe states, recipes, minimum height trees |
| 0851-0856 | Union-find and weighted graphs: redundant edges, components, equations, accounts merge, division |
| 0857-0866 | Shortest paths and beyond: Dijkstra variants, Bellman-Ford, Floyd-Warshall, MST, bridges, Eulerian path |
| 0867-0874 | Threads and channels: spawn/join, panics, `mpsc`, `sync_channel`, request-reply, pipelines, fan-out/fan-in |
| 0875-0882 | Shared state and scoped threads: `Arc<Mutex>`, `Send`/`Sync`, poisoning, parallel map, map-reduce, merge sort |
| 0883-0891 | Atomics, `OnceLock`, `RwLock`, lock ordering, `Barrier`, `Condvar` latch |
| 0892-0900 | Coordination puzzles adapted from LeetCode (print in order, FooBar, zero-even-odd, FizzBuzz, H2O, dining philosophers), a blocking queue, a semaphore and a thread pool |

## How to work on a problem

```bash
cd learn_11_leetcode_practice/level_09_dp_graphs_concurrency

cargo test practice::p0801      # run the tests for the problem you are solving
cargo test solutions::p0801     # run the tests against the reference solution
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
| 0801 | [N-th Tribonacci Number](exercises/p0801_n_th_tribonacci_number.md) | Easy | dynamic programming intro, rolling state, destructuring assignment `(a, b, c) = (...)` | [LC 1137](https://leetcode.com/problems/n-th-tribonacci-number/) |
| 0802 | [Min Cost Climbing Stairs](exercises/p0802_min_cost_climbing_stairs.md) | Easy | 1-D dynamic programming, `Iterator::fold` with a tuple state | [LC 746](https://leetcode.com/problems/min-cost-climbing-stairs/) |
| 0803 | [House Robber](exercises/p0803_house_robber.md) | Medium | 1-D dynamic programming, take/skip recurrence, `fold` | [LC 198](https://leetcode.com/problems/house-robber/) |
| 0804 | [House Robber II](exercises/p0804_house_robber_ii.md) | Medium | 1-D dynamic programming, sub-slices `&nums[1..]`, private helper on `&[i32]` | [LC 213](https://leetcode.com/problems/house-robber-ii/) |
| 0805 | [Delete and Earn](exercises/p0805_delete_and_earn.md) | Medium | dynamic programming, reduction to House Robber, value buckets in a `Vec` | [LC 740](https://leetcode.com/problems/delete-and-earn/) |
| 0806 | [Solving Questions With Brainpower](exercises/p0806_solving_questions_with_brainpower.md) | Medium | backward (suffix) dynamic programming, `i64` results, `Vec::get` for out-of-range | [LC 2140](https://leetcode.com/problems/solving-questions-with-brainpower/) |
| 0807 | [Perfect Squares](exercises/p0807_perfect_squares.md) | Medium | unbounded "minimum count" DP, `take_while` on an infinite range, `map`/`min` | [LC 279](https://leetcode.com/problems/perfect-squares/) |
| 0808 | [Decode Ways](exercises/p0808_decode_ways.md) | Medium | DP over bytes, rolling variables, `as_bytes`, `windows`-style lookback | [LC 91](https://leetcode.com/problems/decode-ways/) |
| 0809 | [Coin Change](exercises/p0809_coin_change.md) | Medium | unbounded knapsack (minimum), `Option<u32>` as "unreachable", `filter_map` | [LC 322](https://leetcode.com/problems/coin-change/) |
| 0810 | [Coin Change II](exercises/p0810_coin_change_ii.md) | Medium | counting combinations (unbounded knapsack), loop order, `u64::wrapping_add` | [LC 518](https://leetcode.com/problems/coin-change-ii/) |
| 0811 | [Combination Sum IV](exercises/p0811_combination_sum_iv.md) | Medium | counting ordered sequences with DP, loop order vs problem 0810, `wrapping_add` | [LC 377](https://leetcode.com/problems/combination-sum-iv/) |
| 0812 | [Word Break](exercises/p0812_word_break.md) | Medium | prefix DP over a string, `HashSet<&str>` borrowed from a `Vec<String>`, `any` | [LC 139](https://leetcode.com/problems/word-break/) |
| 0813 | [Maximum Product Subarray](exercises/p0813_maximum_product_subarray.md) | Medium | DP that tracks both a maximum and a minimum, `std::mem::swap` | [LC 152](https://leetcode.com/problems/maximum-product-subarray/) |
| 0814 | [Longest Increasing Subsequence](exercises/p0814_longest_increasing_subsequence.md) | Medium | DP + binary search ("patience sorting"), `slice::partition_point` | [LC 300](https://leetcode.com/problems/longest-increasing-subsequence/) |
| 0815 | [Number of Longest Increasing Subsequence](exercises/p0815_number_of_longest_increasing_subsequence.md) | Medium | `O(n^2)` DP with two arrays (length and count), `std::cmp::Ordering` | [LC 673](https://leetcode.com/problems/number-of-longest-increasing-subsequence/) |
| 0816 | [Longest String Chain](exercises/p0816_longest_string_chain.md) | Medium | DP over words sorted by length, `HashMap<&str, i32>`, `sort_unstable_by_key` | [LC 1048](https://leetcode.com/problems/longest-string-chain/) |
| 0817 | [Best Time to Buy and Sell Stock with Cooldown](exercises/p0817_best_time_to_buy_and_sell_stock_with_cooldown.md) | Medium | state machine DP (holding / just sold / resting), rolling variables | [LC 309](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/) |
| 0818 | [Best Time to Buy and Sell Stock with Transaction Fee](exercises/p0818_best_time_to_buy_and_sell_stock_with_transaction_fee.md) | Medium | two-state DP (cash / holding), `fold` with a tuple state | [LC 714](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/) |
| 0819 | [Unique Paths](exercises/p0819_unique_paths.md) | Medium | 2-D grid DP compressed to one row, `vec![1; n]`, in-place updates | [LC 62](https://leetcode.com/problems/unique-paths/) |
| 0820 | [Unique Paths II](exercises/p0820_unique_paths_ii.md) | Medium | grid DP with obstacles, one-row DP, `iter().zip()`, `wrapping_add` | [LC 63](https://leetcode.com/problems/unique-paths-ii/) |
| 0821 | [Minimum Path Sum](exercises/p0821_minimum_path_sum.md) | Medium | grid DP done in place, taking ownership of the input with `let mut grid = grid` | [LC 64](https://leetcode.com/problems/minimum-path-sum/) |
| 0822 | [Triangle](exercises/p0822_triangle.md) | Medium | bottom-up DP over rows, `iter().rev()`, `split_last`, `windows`-free updates | [LC 120](https://leetcode.com/problems/triangle/) |
| 0823 | [Maximal Square](exercises/p0823_maximal_square.md) | Medium | grid DP, `Vec<Vec<char>>`, minimum of three neighbours, one-row DP | [LC 221](https://leetcode.com/problems/maximal-square/) |
| 0824 | [Longest Common Subsequence](exercises/p0824_longest_common_subsequence.md) | Medium | 2-D DP over two strings, `as_bytes`, two rolling rows with `std::mem::swap` | [LC 1143](https://leetcode.com/problems/longest-common-subsequence/) |
| 0825 | [Edit Distance](exercises/p0825_edit_distance.md) | Medium | Levenshtein distance, 2-D DP with rolling rows, `(0..=n).collect()` | [LC 72](https://leetcode.com/problems/edit-distance/) |
| 0826 | [Interleaving String](exercises/p0826_interleaving_string.md) | Medium | 2-D boolean DP compressed to one row, byte slices, early length check | [LC 97](https://leetcode.com/problems/interleaving-string/) |
| 0827 | [Palindromic Substrings](exercises/p0827_palindromic_substrings.md) | Medium | expand around centers (a space-free alternative to the `O(n^2)` DP table), `sum` | [LC 647](https://leetcode.com/problems/palindromic-substrings/) |
| 0828 | [Longest Palindromic Subsequence](exercises/p0828_longest_palindromic_subsequence.md) | Medium | interval DP (`dp[i][j]` over substrings), filling order, one-row compression | [LC 516](https://leetcode.com/problems/longest-palindromic-subsequence/) |
| 0829 | [Partition Equal Subset Sum](exercises/p0829_partition_equal_subset_sum.md) | Medium | 0/1 knapsack (subset sum), iterating capacities backwards, `Vec<bool>` | [LC 416](https://leetcode.com/problems/partition-equal-subset-sum/) |
| 0830 | [Target Sum](exercises/p0830_target_sum.md) | Medium | reducing a +/- assignment to subset counting, 0/1 knapsack (counting), `checked` | [LC 494](https://leetcode.com/problems/target-sum/) |
| 0831 | [Ones and Zeroes](exercises/p0831_ones_and_zeroes.md) | Medium | 0/1 knapsack with two capacities, 2-D DP table updated backwards, `bytes().filter` | [LC 474](https://leetcode.com/problems/ones-and-zeroes/) |
| 0832 | [Partition to K Equal Sum Subsets](exercises/p0832_partition_to_k_equal_sum_subsets.md) | Medium | bitmask DP over subsets, `1 << n`, `Option<i32>` table, bit tests with `&` | [LC 698](https://leetcode.com/problems/partition-to-k-equal-sum-subsets/) |
| 0833 | [Distinct Subsequences](exercises/p0833_distinct_subsequences.md) | Hard | counting DP over two strings, one row updated right to left, `wrapping_add` | [LC 115](https://leetcode.com/problems/distinct-subsequences/) |
| 0834 | [Burst Balloons](exercises/p0834_burst_balloons.md) | Hard | interval DP ("which one goes last?"), padding with sentinels, filling by length | [LC 312](https://leetcode.com/problems/burst-balloons/) |
| 0835 | [Maximum Profit in Job Scheduling](exercises/p0835_maximum_profit_in_job_scheduling.md) | Hard | weighted interval scheduling, sort + binary search (`partition_point`) + DP, | [LC 1235](https://leetcode.com/problems/maximum-profit-in-job-scheduling/) |
| 0836 | [Find the Town Judge](exercises/p0836_find_the_town_judge.md) | Easy | directed graphs from an edge list, in-degree and out-degree, `position` | [LC 997](https://leetcode.com/problems/find-the-town-judge/) |
| 0837 | [Find if Path Exists in Graph](exercises/p0837_find_if_path_exists_in_graph.md) | Easy | adjacency list `Vec<Vec<usize>>`, breadth-first search with `VecDeque`, visited | [LC 1971](https://leetcode.com/problems/find-if-path-exists-in-graph/) |
| 0838 | [All Paths From Source to Target](exercises/p0838_all_paths_from_source_to_target.md) | Medium | depth-first search with backtracking on a DAG, recursive helper with `&mut Vec` | [LC 797](https://leetcode.com/problems/all-paths-from-source-to-target/) |
| 0839 | [Number of Provinces](exercises/p0839_number_of_provinces.md) | Medium | connected components, adjacency **matrix**, iterative DFS with a `Vec` stack | [LC 547](https://leetcode.com/problems/number-of-provinces/) |
| 0840 | [Minimum Number of Vertices to Reach All Nodes](exercises/p0840_minimum_number_of_vertices_to_reach_all_nodes.md) | Medium | in-degree in a DAG, a one-line graph insight, `filter` + `collect` | [LC 1557](https://leetcode.com/problems/minimum-number-of-vertices-to-reach-all-nodes/) |
| 0841 | [Reorder Routes to Make All Paths Lead to the City Zero](exercises/p0841_reorder_routes_to_make_all_paths_lead_to_the_city_zero.md) | Medium | trees as graphs, adjacency list with edge flags `Vec<Vec<(usize, bool)>>`, | [LC 1466](https://leetcode.com/problems/reorder-routes-to-make-all-paths-lead-to-the-city-zero/) |
| 0842 | [Is Graph Bipartite?](exercises/p0842_is_graph_bipartite.md) | Medium | BFS 2-coloring, `Vec<Option<bool>>` as "uncolored / color", several components | [LC 785](https://leetcode.com/problems/is-graph-bipartite/) |
| 0843 | [Shortest Path with Alternating Colors](exercises/p0843_shortest_path_with_alternating_colors.md) | Medium | BFS over an expanded state space `(node, last color)`, `[Option<i32>; 2]` per node | [LC 1129](https://leetcode.com/problems/shortest-path-with-alternating-colors/) |
| 0844 | [Open the Lock](exercises/p0844_open_the_lock.md) | Medium | BFS on an implicit graph of states, encoding a state as a number, `[bool; 10_000]` | [LC 752](https://leetcode.com/problems/open-the-lock/) |
| 0845 | [Word Ladder](exercises/p0845_word_ladder.md) | Hard | BFS where neighbours are generated on the fly, `HashSet<Vec<u8>>`, removing visited | [LC 127](https://leetcode.com/problems/word-ladder/) |
| 0846 | [Course Schedule](exercises/p0846_course_schedule.md) | Medium | cycle detection in a directed graph, Kahn's algorithm (topological sort by | [LC 207](https://leetcode.com/problems/course-schedule/) |
| 0847 | [Course Schedule II](exercises/p0847_course_schedule_ii.md) | Medium | topological order with Kahn's algorithm, returning the order itself, testing an | [LC 210](https://leetcode.com/problems/course-schedule-ii/) |
| 0848 | [Find Eventual Safe States](exercises/p0848_find_eventual_safe_states.md) | Medium | three-color DFS with a private `enum`, iterative DFS with an explicit | [LC 802](https://leetcode.com/problems/find-eventual-safe-states/) |
| 0849 | [Find All Possible Recipes from Given Supplies](exercises/p0849_find_all_possible_recipes_from_given_supplies.md) | Medium | topological sort over `String` names, `HashMap<&str, Vec<usize>>`, borrowing keys | [LC 2115](https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies/) |
| 0850 | [Minimum Height Trees](exercises/p0850_minimum_height_trees.md) | Medium | trees as undirected graphs, peeling leaves layer by layer (topological trimming), | [LC 310](https://leetcode.com/problems/minimum-height-trees/) |
| 0851 | [Redundant Connection](exercises/p0851_redundant_connection.md) | Medium | union-find (disjoint set union) as a small struct with methods, path halving, | [LC 684](https://leetcode.com/problems/redundant-connection/) |
| 0852 | [Number of Operations to Make Network Connected](exercises/p0852_number_of_operations_to_make_network_connected.md) | Medium | union-find with a component counter, counting spare edges | [LC 1319](https://leetcode.com/problems/number-of-operations-to-make-network-connected/) |
| 0853 | [Satisfiability of Equality Equations](exercises/p0853_satisfiability_of_equality_equations.md) | Medium | union-find over a fixed alphabet (`[usize; 26]`), two passes, byte patterns | [LC 990](https://leetcode.com/problems/satisfiability-of-equality-equations/) |
| 0854 | [Smallest String With Swaps](exercises/p0854_smallest_string_with_swaps.md) | Medium | union-find over indices, grouping with `HashMap<usize, Vec<usize>>`, sorting each | [LC 1202](https://leetcode.com/problems/smallest-string-with-swaps/) |
| 0855 | [Accounts Merge](exercises/p0855_accounts_merge.md) | Medium | union-find keyed by strings via `HashMap<&str, usize>`, grouping into | [LC 721](https://leetcode.com/problems/accounts-merge/) |
| 0856 | [Evaluate Division](exercises/p0856_evaluate_division.md) | Medium | weighted directed graph with `HashMap<&str, Vec<(&str, f64)>>`, DFS that multiplies | [LC 399](https://leetcode.com/problems/evaluate-division/) |
| 0857 | [Network Delay Time](exercises/p0857_network_delay_time.md) | Medium | Dijkstra's algorithm, `BinaryHeap` as a min-heap with `std::cmp::Reverse`, | [LC 743](https://leetcode.com/problems/network-delay-time/) |
| 0858 | [Path with Maximum Probability](exercises/p0858_path_with_maximum_probability.md) | Medium | Dijkstra on products (maximize), `f64` in a `BinaryHeap` via a newtype that | [LC 1514](https://leetcode.com/problems/path-with-maximum-probability/) |
| 0859 | [Path With Minimum Effort](exercises/p0859_path_with_minimum_effort.md) | Medium | "minimax" Dijkstra on a grid (path cost = largest step), `Reverse`, direction | [LC 1631](https://leetcode.com/problems/path-with-minimum-effort/) |
| 0860 | [Number of Ways to Arrive at Destination](exercises/p0860_number_of_ways_to_arrive_at_destination.md) | Medium | Dijkstra that also counts shortest paths, `i64` distances, counts modulo | [LC 1976](https://leetcode.com/problems/number-of-ways-to-arrive-at-destination/) |
| 0861 | [Cheapest Flights Within K Stops](exercises/p0861_cheapest_flights_within_k_stops.md) | Medium | Bellman-Ford limited to `k + 1` rounds, copying the previous round with `clone`, | [LC 787](https://leetcode.com/problems/cheapest-flights-within-k-stops/) |
| 0862 | [Find the City With the Smallest Number of Neighbors at a Threshold Distance](exercises/p0862_find_the_city_with_the_smallest_number_of_neighbors_at_a_threshold_distance.md) | Medium | all-pairs shortest paths with Floyd-Warshall, `Vec<Vec<u32>>` matrix, tie-breaking | [LC 1334](https://leetcode.com/problems/find-the-city-with-the-smallest-number-of-neighbors-at-a-threshold-distance/) |
| 0863 | [Min Cost to Connect All Points](exercises/p0863_min_cost_to_connect_all_points.md) | Medium | minimum spanning tree with Prim's algorithm on a dense graph (`O(n^2)`, no heap), | [LC 1584](https://leetcode.com/problems/min-cost-to-connect-all-points/) |
| 0864 | [Swim in Rising Water](exercises/p0864_swim_in_rising_water.md) | Hard | minimax path with Dijkstra on node weights, `BinaryHeap<Reverse<_>>`, visited grid | [LC 778](https://leetcode.com/problems/swim-in-rising-water/) |
| 0865 | [Critical Connections in a Network](exercises/p0865_critical_connections_in_a_network.md) | Hard | bridges with Tarjan's low-link values, iterative DFS with a `(node, parent edge, | [LC 1192](https://leetcode.com/problems/critical-connections-in-a-network/) |
| 0866 | [Reconstruct Itinerary](exercises/p0866_reconstruct_itinerary.md) | Hard | Eulerian path with Hierholzer's algorithm, `HashMap<&str, Vec<&str>>` sorted in | [LC 332](https://leetcode.com/problems/reconstruct-itinerary/) |
| 0867 | [Spawn and Join](exercises/p0867_spawn_and_join.md) | Easy | `std::thread::spawn`, `JoinHandle<T>::join`, `move` closures, collecting handles | Original |
| 0868 | [Parallel Chunk Sum](exercises/p0868_parallel_chunk_sum.md) | Easy | splitting owned data into balanced chunks, moving each chunk into a thread, | Original |
| 0869 | [Catch Worker Panics](exercises/p0869_catch_worker_panics.md) | Medium | `JoinHandle::join` returning `Err` when a thread panics, panic payloads | Original |
| 0870 | [Gather from a Channel](exercises/p0870_gather_from_a_channel.md) | Easy | `std::sync::mpsc::channel`, cloning a `Sender` for several producers, ending the | Original |
| 0871 | [Bounded Channel Backpressure](exercises/p0871_bounded_channel_backpressure.md) | Medium | `mpsc::sync_channel` with a fixed capacity, blocking `send` versus `try_send`, | Original |
| 0872 | [Request-Reply Server](exercises/p0872_request_reply_server.md) | Medium | a server thread that owns a `Receiver`, `enum` messages that carry their own reply | Original |
| 0873 | [Multi-Stage Pipeline](exercises/p0873_multi_stage_pipeline.md) | Medium | producer-consumer pipelines, a generic stage function with `Send + 'static` | Original |
| 0874 | [Fan-Out, Fan-In](exercises/p0874_fan_out_fan_in.md) | Medium | distributing jobs over one channel per worker (fan-out), merging results into a | Original |
| 0875 | [Shared Counter with Arc and Mutex](exercises/p0875_shared_counter_with_arc_and_mutex.md) | Easy | shared mutable state with `Arc<Mutex<T>>`, `lock()` and `MutexGuard`, | Original |
| 0876 | [Make It Send and Sync](exercises/p0876_make_it_send_and_sync.md) | Medium | the `Send` and `Sync` marker traits, why `Rc<RefCell<T>>` cannot cross threads, | Original |
| 0877 | [Recover a Poisoned Mutex](exercises/p0877_recover_a_poisoned_mutex.md) | Medium | mutex poisoning, `LockResult`, `PoisonError::into_inner`, `Mutex::is_poisoned`, | Original |
| 0878 | [Borrow Data with Scoped Threads](exercises/p0878_scoped_threads_borrow.md) | Easy | `std::thread::scope`, threads that borrow local data (no `Arc`, no cloning), | Original |
| 0879 | [Parallel In-Place Update](exercises/p0879_parallel_in_place_update.md) | Medium | `chunks_mut` to hand out disjoint `&mut` sub-slices, scoped threads, closure bounds | Original |
| 0880 | [Generic Parallel Map](exercises/p0880_generic_parallel_map.md) | Medium | generic functions over threads, choosing the right bounds (`T: Sync`, `U: Send`, | Original |
| 0881 | [Parallel Word Count](exercises/p0881_parallel_word_count.md) | Medium | map-reduce with scoped threads, a private `HashMap` per worker merged at the end, | Original |
| 0882 | [Parallel Merge Sort](exercises/p0882_parallel_merge_sort.md) | Medium | divide and conquer across threads, `split_at_mut`, recursive `thread::scope`, | Original |
| 0883 | [Atomic Statistics](exercises/p0883_atomic_statistics.md) | Easy | `AtomicU64` with `fetch_add`, `fetch_max` and `load`, `Ordering::Relaxed`, shared | Original |
| 0884 | [Atomic Work Distribution](exercises/p0884_atomic_work_distribution.md) | Medium | dynamic load balancing with a shared `AtomicU64` cursor, claiming blocks of work | Original |
| 0885 | [Stop Early with an Atomic Flag](exercises/p0885_stop_early_with_an_atomic_flag.md) | Medium | cooperative cancellation with `AtomicBool`, `AtomicUsize::fetch_min` to keep the | Original |
| 0886 | [Lock-Free Maximum](exercises/p0886_lock_free_maximum.md) | Medium | compare-and-swap loops with `compare_exchange_weak`, retrying with the value | Original |
| 0887 | [Compute Once with OnceLock](exercises/p0887_compute_once_with_oncelock.md) | Medium | lazy one-time initialization with `std::sync::OnceLock`, `get_or_init`, a lazily | Original |
| 0888 | [Read-Mostly Config Store](exercises/p0888_read_mostly_config_store.md) | Medium | `std::sync::RwLock` (many readers or one writer), `read()` / `write()` guards, | Original |
| 0889 | [Deadlock-Free Bank Transfers](exercises/p0889_deadlock_free_bank_transfers.md) | Medium | one `Mutex` per account (`Vec<Mutex<i64>>`), deadlock avoidance by a global lock | Original |
| 0890 | [Phased Work with a Barrier](exercises/p0890_phased_work_with_a_barrier.md) | Medium | `std::sync::Barrier` to separate computation rounds, reusing a barrier, double | Original |
| 0891 | [Countdown Latch](exercises/p0891_countdown_latch.md) | Medium | `std::sync::Condvar` paired with a `Mutex`, `Condvar::wait_while`, `notify_all`, | Original |
| 0892 | [Print in Order](exercises/p0892_print_in_order.md) | Easy | ordering work across threads with a `Mutex<u8>` "turn" and a `Condvar`, callbacks | [Adapted from LC 1114](https://leetcode.com/problems/print-in-order/) |
| 0893 | [Bounded Blocking Queue](exercises/p0893_bounded_blocking_queue.md) | Medium | a generic producer-consumer queue from `Mutex<VecDeque<T>>` and **two** | Original |
| 0894 | [Counting Semaphore](exercises/p0894_counting_semaphore.md) | Medium | a semaphore from `Mutex<usize>` + `Condvar`, RAII permits released in `Drop`, | Original |
| 0895 | [Print FooBar Alternately](exercises/p0895_print_foobar_alternately.md) | Medium | strict alternation between two threads with a shared boolean turn and a `Condvar`, | [Adapted from LC 1115](https://leetcode.com/problems/print-foobar-alternately/) |
| 0896 | [Print Zero Even Odd](exercises/p0896_print_zero_even_odd.md) | Medium | coordinating three threads with a small state struct behind one `Mutex`, a | [Adapted from LC 1116](https://leetcode.com/problems/print-zero-even-odd/) |
| 0897 | [Fizz Buzz Multithreaded](exercises/p0897_fizz_buzz_multithreaded.md) | Medium | four threads sharing one counter, each waiting for "its" kind of number, clean | [Adapted from LC 1195](https://leetcode.com/problems/fizz-buzz-multithreaded/) |
| 0898 | [Building H2O](exercises/p0898_building_h2o.md) | Medium | grouping threads into fixed-size teams (2 H + 1 O), slots in a shared state struct, | [Adapted from LC 1117](https://leetcode.com/problems/building-h2o/) |
| 0899 | [The Dining Philosophers](exercises/p0899_the_dining_philosophers.md) | Medium | deadlock avoidance by resource ordering, an array of `Mutex<()>` used as plain | [Adapted from LC 1226](https://leetcode.com/problems/the-dining-philosophers/) |
| 0900 | [Thread Pool](exercises/p0900_thread_pool.md) | Hard | a fixed set of worker threads sharing one job queue (`Arc<Mutex<Receiver<Job>>>`), | Original |

<!-- PROBLEM_TABLE:END -->
