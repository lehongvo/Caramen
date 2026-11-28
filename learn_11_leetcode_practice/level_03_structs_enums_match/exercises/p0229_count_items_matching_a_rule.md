# Problem 0229: Count Items Matching a Rule

- **Difficulty:** Easy
- **Topics:** `enum` as a field selector, parsing `&str` into `Option<Enum>`, slice indexing, `filter` + `count`
- **Source:** [LeetCode 1773. Count Items Matching a Rule](https://leetcode.com/problems/count-items-matching-a-rule/)

## Problem

Each item in `items` is a list of three strings: `[type, color, name]`. A rule is given as a key
`rule_key` (one of `"type"`, `"color"` or `"name"`) and a value `rule_value`. An item matches
the rule when its field selected by the key equals the value exactly. Return the number of
matching items.

Implement
`Solution::count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32`.

For this level, parse `rule_key` into an `enum Field { Type, Color, Name }` with a method that
gives the field's position inside an item.

## Examples

```text
Input:  items = [["phone","blue","pixel"], ["computer","silver","lenovo"], ["phone","gold","iphone"]],
        rule_key = "color", rule_value = "silver"
Output: 1

Input:  same items, rule_key = "type", rule_value = "phone"
Output: 2
```

## Constraints

- `1 <= items.len() <= 10^4`
- Every item has exactly 3 strings of `1..=10` lowercase letters.
- `rule_key` is `"type"`, `"color"` or `"name"`.

## Hints

1. `Field::index(self) -> usize` returns `0`, `1` or `2`.
2. Compare `item[field.index()] == rule_value`; a `String` compares with a `String` directly.

## Starter code

Write your code in [`src/practice/p0229_count_items_matching_a_rule.rs`](../src/practice/p0229_count_items_matching_a_rule.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0229
```

```rust
pub struct Solution;

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
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

    fn items(list: &[[&str; 3]]) -> Vec<Vec<String>> {
        list.iter()
            .map(|item| item.iter().map(|s| s.to_string()).collect())
            .collect()
    }

    fn sample() -> Vec<Vec<String>> {
        items(&[
            ["phone", "blue", "pixel"],
            ["computer", "silver", "lenovo"],
            ["phone", "gold", "iphone"],
        ])
    }

    #[test]
    fn example_color_silver() {
        assert_eq!(
            Solution::count_matches(sample(), "color".to_string(), "silver".to_string()),
            1
        );
    }

    #[test]
    fn example_type_phone() {
        assert_eq!(
            Solution::count_matches(sample(), "type".to_string(), "phone".to_string()),
            2
        );
    }

    #[test]
    fn name_rule_and_no_match() {
        assert_eq!(
            Solution::count_matches(sample(), "name".to_string(), "iphone".to_string()),
            1
        );
        assert_eq!(
            Solution::count_matches(sample(), "name".to_string(), "phone".to_string()),
            0
        );
    }

    #[test]
    fn value_must_be_in_the_selected_field() {
        // "blue" is a color, not a type.
        assert_eq!(
            Solution::count_matches(sample(), "type".to_string(), "blue".to_string()),
            0
        );
    }

    #[test]
    fn many_items() {
        let list: Vec<[&str; 3]> = (0..10_000)
            .map(|i| {
                if i % 4 == 0 {
                    ["tv", "red", "a"]
                } else {
                    ["tv", "blue", "b"]
                }
            })
            .collect();
        assert_eq!(
            Solution::count_matches(items(&list), "color".to_string(), "red".to_string()),
            2_500
        );
        assert_eq!(
            Solution::count_matches(items(&list), "type".to_string(), "tv".to_string()),
            10_000
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** parse the key once, outside the loop, into a `Field`. Then count the items whose
string at `field.index()` equals the rule value. An unknown key matches nothing.

**Complexity:** `O(n)` time, `O(1)` extra space.

**Rust concepts:**
- An enum replaces a "stringly typed" value: after parsing, the code works with `Field::Color`
  and can no longer be confused by a typo such as `"colour"`.
- `let Some(field) = Field::parse(&rule_key) else { return 0; };` handles the impossible case
  up front without nesting.
- `item.get(i)` returns `Option<&String>`, so a short item cannot cause a panic.

```rust
pub struct Solution;

/// Which field of an item a rule looks at.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Field {
    Type,
    Color,
    Name,
}

impl Field {
    fn parse(key: &str) -> Option<Field> {
        match key {
            "type" => Some(Field::Type),
            "color" => Some(Field::Color),
            "name" => Some(Field::Name),
            _ => None,
        }
    }

    fn index(self) -> usize {
        match self {
            Field::Type => 0,
            Field::Color => 1,
            Field::Name => 2,
        }
    }
}

impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let Some(field) = Field::parse(&rule_key) else {
            return 0;
        };
        let count = items
            .iter()
            .filter(|item| item.get(field.index()) == Some(&rule_value))
            .count();
        // At most 10^4 items.
        count as i32
    }
}
```

Full tested solution: [`src/solutions/p0229_count_items_matching_a_rule.rs`](../src/solutions/p0229_count_items_matching_a_rule.rs)

</details>
