# Problem 0385: Display Table of Food Orders in a Restaurant

- **Difficulty:** Medium
- **Topics:** `BTreeSet<&str>` for sorted column names, `BTreeMap<u32, HashMap<&str, u32>>` for sorted rows, building a `Vec<Vec<String>>`
- **Source:** [LeetCode 1418. Display Table of Food Orders in a Restaurant](https://leetcode.com/problems/display-table-of-food-orders-in-a-restaurant/)

## Problem

Each order is a list `[customer_name, table_number, food_item]` (all strings). Build the
restaurant's **display table**:

- the first row is the header: `"Table"` followed by every distinct food item, sorted
  alphabetically (by byte value, so uppercase letters come before lowercase ones);
- then one row per table that ordered something, sorted by table number **numerically**; each
  row starts with the table number and continues with, for each food column, how many times
  that table ordered it (`"0"` if never).

The customer names are not part of the output.

Implement `Solution::display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>>`.

## Examples

```text
Input:  [["David", "3", "Ceviche"], ["Corina", "10", "Beef Burrito"], ["David", "3", "Fried Chicken"],
         ["Carla", "5", "Water"], ["Carla", "5", "Ceviche"], ["Rous", "3", "Ceviche"]]
Output: [["Table", "Beef Burrito", "Ceviche", "Fried Chicken", "Water"],
         ["3",     "0",            "2",       "1",             "0"],
         ["5",     "0",            "1",       "0",             "1"],
         ["10",    "1",            "0",       "0",             "0"]]
```

Note that table `"10"` comes after `"5"`: tables are sorted as numbers, not as strings.

## Constraints

- `1 <= orders.len() <= 5 * 10^4`, each order has exactly 3 strings
- Table numbers are integers between `1` and `500`.
- Food items have at most 20 characters (letters and spaces).

## Hints

1. A `BTreeSet<&str>` collects the food names in sorted order, borrowing them from `orders`.
2. A `BTreeMap<u32, ...>` keyed by the **parsed** table number sorts the rows numerically.
3. For each table, store a `HashMap<&str, u32>` of counts; missing foods read as `0`.

## Starter code

Write your code in [`src/practice/p0385_display_table_of_food_orders_in_a_restaurant.rs`](../src/practice/p0385_display_table_of_food_orders_in_a_restaurant.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0385
```

```rust
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub struct Solution;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
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

    fn table(rows: &[&[&str]]) -> Vec<Vec<String>> {
        rows.iter()
            .map(|r| r.iter().map(|s| s.to_string()).collect())
            .collect()
    }

    #[test]
    fn example_one() {
        let orders = table(&[
            &["David", "3", "Ceviche"],
            &["Corina", "10", "Beef Burrito"],
            &["David", "3", "Fried Chicken"],
            &["Carla", "5", "Water"],
            &["Carla", "5", "Ceviche"],
            &["Rous", "3", "Ceviche"],
        ]);
        assert_eq!(
            Solution::display_table(orders),
            table(&[
                &["Table", "Beef Burrito", "Ceviche", "Fried Chicken", "Water"],
                &["3", "0", "2", "1", "0"],
                &["5", "0", "1", "0", "1"],
                &["10", "1", "0", "0", "0"],
            ])
        );
    }

    #[test]
    fn one_table_many_orders() {
        let orders = table(&[
            &["James", "12", "Fried Chicken"],
            &["Ratesh", "12", "Fried Chicken"],
            &["Amadeus", "12", "Fried Chicken"],
            &["Adam", "1", "Canadian Waffles"],
            &["Brianna", "1", "Canadian Waffles"],
        ]);
        assert_eq!(
            Solution::display_table(orders),
            table(&[
                &["Table", "Canadian Waffles", "Fried Chicken"],
                &["1", "2", "0"],
                &["12", "0", "3"],
            ])
        );
    }

    #[test]
    fn uppercase_sorts_first() {
        let orders = table(&[&["L", "2", "apple"], &["M", "2", "Zucchini"]]);
        assert_eq!(
            Solution::display_table(orders),
            table(&[&["Table", "Zucchini", "apple"], &["2", "1", "1"]])
        );
    }

    #[test]
    fn many_orders() {
        let foods = ["Tea", "Cake", "Soup"];
        let orders: Vec<Vec<String>> = (0..50_000)
            .map(|i| {
                vec![
                    format!("c{i}"),
                    (1 + i % 500).to_string(),
                    foods[i % 3].to_string(),
                ]
            })
            .collect();
        let result = Solution::display_table(orders);
        assert_eq!(result.len(), 501);
        assert_eq!(result[0], vec!["Table", "Cake", "Soup", "Tea"]);
        assert_eq!(result[1][0], "1");
        assert_eq!(result[500][0], "500");
        let total: u32 = result[1..]
            .iter()
            .flat_map(|row| row[1..].iter().map(|c| c.parse::<u32>().unwrap()))
            .sum();
        assert_eq!(total, 50_000);
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** one pass over the orders fills two ordered structures: the set of foods (the columns)
and, per table number, a count per food (the rows). Ordered collections make both sorts
automatic; the output is then assembled row by row.

**Complexity:** `O(n log n)` time for `n` orders (with `t` tables and `f` foods, the output costs
`O(t * f)`), `O(n)` space.

**Rust concepts:**
- Borrowing `&str` keys from `orders` avoids cloning every food name; the names are converted to
  `String` only once, for the output.
- `entry(table).or_default()` creates the inner `HashMap` the first time a table appears.
- `table.parse::<u32>().unwrap_or(0)` parses the table number; the constraints guarantee a valid
  number, so the fallback is never used.
- Slice patterns: `if let [_, table, food] = order.as_slice()` checks the shape and binds the
  two fields in one step.

```rust
use std::collections::{BTreeMap, BTreeSet, HashMap};

pub struct Solution;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut foods: BTreeSet<&str> = BTreeSet::new();
        let mut tables: BTreeMap<u32, HashMap<&str, u32>> = BTreeMap::new();
        for order in &orders {
            if let [_, table, food] = order.as_slice() {
                let table: u32 = table.parse().unwrap_or(0);
                foods.insert(food.as_str());
                *tables
                    .entry(table)
                    .or_default()
                    .entry(food.as_str())
                    .or_insert(0) += 1;
            }
        }
        let mut result = Vec::with_capacity(tables.len() + 1);
        let mut header = vec!["Table".to_string()];
        header.extend(foods.iter().map(|f| f.to_string()));
        result.push(header);
        for (table, counts) in &tables {
            let mut row = vec![table.to_string()];
            row.extend(
                foods
                    .iter()
                    .map(|f| counts.get(f).copied().unwrap_or(0).to_string()),
            );
            result.push(row);
        }
        result
    }
}
```

Full tested solution: [`src/solutions/p0385_display_table_of_food_orders_in_a_restaurant.rs`](../src/solutions/p0385_display_table_of_food_orders_in_a_restaurant.rs)

</details>
