# Problem 0874: Fan-Out, Fan-In

- **Difficulty:** Medium
- **Topics:** distributing jobs over one channel per worker (fan-out), merging results into a
  single channel (fan-in), restoring the input order by index, `fn` pointers as `Send` values
- **Source:** Original · [The Rust Book 16.2](https://doc.rust-lang.org/book/ch16-02-message-passing.html)

## Problem

Implement `fan_out_fan_in(inputs: Vec<u64>, workers: usize, work: fn(u64) -> u64) -> Vec<u64>`
that computes `work(x)` for every input on a pool of `workers` threads:

- **fan-out:** create one job channel per worker and hand out the jobs round-robin: job `i`
  goes to worker `i % workers`. A job is the pair `(i, inputs[i])`;
- **fan-in:** every worker sends `(i, work(x))` into one shared result channel;
- the calling thread receives all results and returns them **in input order**
  (`output[i] == work(inputs[i])`), whatever order they arrive in.

The function must not hang: workers stop when their job channel is closed.

## Examples

```text
fan_out_fan_in(vec![1, 2, 3, 4], 2, |x| x * x) -> [1, 4, 9, 16]
fan_out_fan_in(vec![], 3, |x| x)              -> []
```

## Constraints

- `1 <= workers <= 32`; `0 <= inputs.len() <= 10^5`

## Hints

1. Build a `Vec<Sender<(usize, u64)>>` of job senders, spawning one worker per receiver. Each
   worker also gets a clone of the result sender.
2. After sending every job, drop the job senders (so workers finish) **and** your own result
   sender (so the result loop finishes).
3. Preallocate `vec![0; n]` and write each result at its index.

## Starter code

Write your code in [`src/practice/p0874_fan_out_fan_in.rs`](../src/practice/p0874_fan_out_fan_in.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0874
```

```rust
use std::sync::mpsc;
use std::thread;

pub fn fan_out_fan_in(inputs: Vec<u64>, workers: usize, work: fn(u64) -> u64) -> Vec<u64> {
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

    fn collatz_steps(mut x: u64) -> u64 {
        let mut steps = 0;
        while x > 1 {
            x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
            steps += 1;
        }
        steps
    }

    #[test]
    fn example_squares() {
        assert_eq!(
            fan_out_fan_in(vec![1, 2, 3, 4], 2, |x| x * x),
            vec![1, 4, 9, 16]
        );
    }

    #[test]
    fn example_empty_input() {
        assert!(fan_out_fan_in(vec![], 3, |x| x).is_empty());
    }

    #[test]
    fn more_workers_than_jobs() {
        assert_eq!(fan_out_fan_in(vec![10, 20], 32, |x| x + 1), vec![11, 21]);
    }

    #[test]
    fn single_worker() {
        assert_eq!(
            fan_out_fan_in(vec![5, 4, 3], 1, |x| x * 10),
            vec![50, 40, 30]
        );
    }

    #[test]
    fn uneven_work_still_in_order() {
        let inputs: Vec<u64> = (1..=100_000).collect();
        let want: Vec<u64> = inputs.iter().map(|&x| collatz_steps(x)).collect();
        assert_eq!(fan_out_fan_in(inputs, 8, collatz_steps), want);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** spawn `workers` threads, each owning a job `Receiver` and a result `Sender`. Send job
`i` to `job_senders[i % workers]`, then drop all senders held by the calling thread. Receive
from the result channel until it closes, storing each value at its index.

**Rust concepts:**
- A plain function pointer `fn(u64) -> u64` is `Copy + Send + 'static`, so every worker can get
  its own copy without `Arc`; non-capturing closures like `|x| x * x` coerce to it.
- Indices travel with the data so the order can be restored at the end: concurrency gives no
  ordering guarantee between different workers.
- Dropping a `Vec<Sender<_>>` drops every sender in it.

```rust
use std::sync::mpsc;
use std::thread;

pub fn fan_out_fan_in(inputs: Vec<u64>, workers: usize, work: fn(u64) -> u64) -> Vec<u64> {
    let (result_tx, result_rx) = mpsc::channel::<(usize, u64)>();
    let job_senders: Vec<mpsc::Sender<(usize, u64)>> = (0..workers)
        .map(|_| {
            let (job_tx, job_rx) = mpsc::channel::<(usize, u64)>();
            let result_tx = result_tx.clone();
            thread::spawn(move || {
                for (i, x) in job_rx {
                    if result_tx.send((i, work(x))).is_err() {
                        break;
                    }
                }
            });
            job_tx
        })
        .collect();
    let n = inputs.len();
    for (i, x) in inputs.into_iter().enumerate() {
        // A worker only stops once its job sender is dropped, so this send cannot fail.
        let _ = job_senders[i % workers].send((i, x));
    }
    drop(job_senders);
    drop(result_tx);
    let mut out = vec![0; n];
    for (i, y) in result_rx {
        out[i] = y;
    }
    out
}
```

Full tested solution: [`src/solutions/p0874_fan_out_fan_in.rs`](../src/solutions/p0874_fan_out_fan_in.rs)

</details>
