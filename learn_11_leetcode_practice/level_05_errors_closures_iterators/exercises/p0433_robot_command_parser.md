# Problem 0433: Parse Robot Commands

- **Difficulty:** Medium
- **Topics:** `FromStr` for an enum, `split_whitespace` + `next`, `enumerate` + `map_err` with line numbers, collecting `Result`s, `fold` as an interpreter
- **Source:** Original · [std `FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) · [std `Iterator::fold`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.fold)

## Problem

A robot is programmed with a small text language, one command per line:

```text
# square dance
FORWARD 3
TURN LEFT
wait
forward 2
```

Keywords are case-insensitive (`FORWARD`, `forward`, `Forward`...). The commands are
`FORWARD <n>` (`n` a `u32`), `TURN LEFT`, `TURN RIGHT` (direction also case-insensitive) and
`WAIT`. Tokens are separated by whitespace.

1. Implement `FromStr for Command` with `type Err = CommandError`:
   - no tokens -> `Empty`;
   - an unknown first token -> `UnknownCommand(token as written)`;
   - `FORWARD` / `TURN` without an argument -> `MissingArgument("FORWARD")` / `MissingArgument("TURN")`;
   - an argument that is not a `u32` (for `FORWARD`) or not `LEFT`/`RIGHT` (for `TURN`) ->
     `BadArgument(argument as written)`;
   - any extra token after a complete command (also after `WAIT`) -> `TooManyArguments`.
2. Implement `Display` for `CommandError`: `"empty command"`, ``"unknown command `X`"``,
   ``"`FORWARD` needs an argument"``, ``"bad argument `X`"``, `"too many arguments"`.
3. `parse_program(text: &str) -> Result<Vec<Command>, String>`: skip blank lines and lines whose
   trimmed text starts with `'#'`; parse the rest. The error is `"line N: MESSAGE"` with the
   1-based line number of the first bad line.
4. `run(program: &[Command]) -> (i32, i32)`: the robot starts at `(0, 0)` facing north
   (`+y`). `Forward(n)` moves `n` steps, `Turn` rotates 90 degrees, `Wait` does nothing. Return
   the final position.

## Examples

```text
"forward 3".parse::<Command>()   -> Ok(Forward(3))
"Turn right".parse::<Command>()  -> Ok(Turn(Right))
"jump".parse::<Command>()        -> Err(UnknownCommand("jump"))
"FORWARD".parse::<Command>()     -> Err(MissingArgument("FORWARD"))
"FORWARD -1".parse::<Command>()  -> Err(BadArgument("-1"))
"WAIT 5".parse::<Command>()      -> Err(TooManyArguments)
parse_program("FORWARD 1\n\nTURN UP") -> Err("line 3: bad argument `UP`")
run([Forward(3), Turn(Left), Forward(2)]) -> (-2, 3)
```

## Constraints

- Programs have at most `10^3` lines and every `n <= 10^4`.

## Hints

1. `let mut tokens = s.split_whitespace();` then `tokens.next()` for each expected token, and
   finally `tokens.next().is_some()` detects extra tokens.
2. Compare keywords with `eq_ignore_ascii_case` or after `to_ascii_uppercase()`.
3. Facing `(dx, dy)`, a left turn gives `(-dy, dx)` and a right turn gives `(dy, -dx)`.

## Starter code

Write your code in [`src/practice/p0433_robot_command_parser.rs`](../src/practice/p0433_robot_command_parser.rs) (replace every `todo!(...)`, keep the tests unchanged), then run:

```bash
cargo test practice::p0433
```

```rust
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Command {
    Forward(u32),
    Turn(Direction),
    Wait,
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    Empty,
    UnknownCommand(String),
    MissingArgument(&'static str),
    BadArgument(String),
    TooManyArguments,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Write your solution here")
    }
}

impl Error for CommandError {}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!("Write your solution here")
    }
}

pub fn parse_program(text: &str) -> Result<Vec<Command>, String> {
    todo!("Write your solution here")
}

pub fn run(program: &[Command]) -> (i32, i32) {
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
    fn parse_valid_commands() {
        assert_eq!("forward 3".parse::<Command>(), Ok(Command::Forward(3)));
        assert_eq!("  FORWARD   0 ".parse::<Command>(), Ok(Command::Forward(0)));
        assert_eq!(
            "Turn right".parse::<Command>(),
            Ok(Command::Turn(Direction::Right))
        );
        assert_eq!(
            "TURN LEFT".parse::<Command>(),
            Ok(Command::Turn(Direction::Left))
        );
        assert_eq!("wait".parse::<Command>(), Ok(Command::Wait));
    }

    #[test]
    fn parse_errors() {
        assert_eq!("".parse::<Command>(), Err(CommandError::Empty));
        assert_eq!(
            "jump".parse::<Command>(),
            Err(CommandError::UnknownCommand("jump".to_string()))
        );
        assert_eq!(
            "FORWARD".parse::<Command>(),
            Err(CommandError::MissingArgument("FORWARD"))
        );
        assert_eq!(
            "turn".parse::<Command>(),
            Err(CommandError::MissingArgument("TURN"))
        );
        assert_eq!(
            "FORWARD -1".parse::<Command>(),
            Err(CommandError::BadArgument("-1".to_string()))
        );
        assert_eq!(
            "TURN up".parse::<Command>(),
            Err(CommandError::BadArgument("up".to_string()))
        );
        assert_eq!(
            "WAIT 5".parse::<Command>(),
            Err(CommandError::TooManyArguments)
        );
        assert_eq!(
            "forward 1 2".parse::<Command>(),
            Err(CommandError::TooManyArguments)
        );
    }

    #[test]
    fn display_messages() {
        assert_eq!(CommandError::Empty.to_string(), "empty command");
        assert_eq!(
            CommandError::UnknownCommand("jump".to_string()).to_string(),
            "unknown command `jump`"
        );
        assert_eq!(
            CommandError::MissingArgument("FORWARD").to_string(),
            "`FORWARD` needs an argument"
        );
        assert_eq!(
            CommandError::BadArgument("x".to_string()).to_string(),
            "bad argument `x`"
        );
        assert_eq!(
            CommandError::TooManyArguments.to_string(),
            "too many arguments"
        );
    }

    #[test]
    fn parse_whole_programs() {
        let text = "# square dance\nFORWARD 3\nTURN LEFT\n\nwait\nforward 2\n";
        assert_eq!(
            parse_program(text),
            Ok(vec![
                Command::Forward(3),
                Command::Turn(Direction::Left),
                Command::Wait,
                Command::Forward(2),
            ])
        );
        assert_eq!(
            parse_program("FORWARD 1\n\nTURN UP"),
            Err("line 3: bad argument `UP`".to_string())
        );
        assert_eq!(
            parse_program("  # only a comment\nfly"),
            Err("line 2: unknown command `fly`".to_string())
        );
    }

    #[test]
    fn run_programs() {
        assert_eq!(
            run(&[
                Command::Forward(3),
                Command::Turn(Direction::Left),
                Command::Forward(2)
            ]),
            (-2, 3)
        );
        assert_eq!(run(&[]), (0, 0));
        let square: Vec<Command> = (0..4)
            .flat_map(|_| [Command::Forward(5), Command::Turn(Direction::Right)])
            .collect();
        assert_eq!(run(&square), (0, 0));
        assert_eq!(
            run(&[
                Command::Turn(Direction::Right),
                Command::Wait,
                Command::Forward(10_000)
            ]),
            (10_000, 0)
        );
    }
}
```

</details>

## Answer

<details>
<summary>Show the answer</summary>

**Idea:** `FromStr` handles one line: it pulls tokens from a `split_whitespace` iterator and
turns every "expected token is missing / wrong" situation into a specific error variant.
`parse_program` adds line numbers with `map_err` and collects into a `Result`. `run` is a `fold`
whose state is `(position, heading)`.

**Rust concepts:**
- Pulling tokens manually with `next()` gives precise control over missing and extra tokens.
- `ok_or(...)?` converts "no next token" into the right error.
- `enumerate` before `filter` keeps the original line numbers.
- `fold` with a tuple state is a tiny interpreter: no mutable variables needed.

```rust
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Command {
    Forward(u32),
    Turn(Direction),
    Wait,
}

#[derive(Debug, PartialEq)]
pub enum CommandError {
    Empty,
    UnknownCommand(String),
    MissingArgument(&'static str),
    BadArgument(String),
    TooManyArguments,
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::Empty => write!(f, "empty command"),
            CommandError::UnknownCommand(c) => write!(f, "unknown command `{c}`"),
            CommandError::MissingArgument(c) => write!(f, "`{c}` needs an argument"),
            CommandError::BadArgument(a) => write!(f, "bad argument `{a}`"),
            CommandError::TooManyArguments => write!(f, "too many arguments"),
        }
    }
}

impl Error for CommandError {}

impl FromStr for Command {
    type Err = CommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tokens = s.split_whitespace();
        let keyword = tokens.next().ok_or(CommandError::Empty)?;
        let command = match keyword.to_ascii_uppercase().as_str() {
            "FORWARD" => {
                let arg = tokens
                    .next()
                    .ok_or(CommandError::MissingArgument("FORWARD"))?;
                let steps = arg
                    .parse::<u32>()
                    .map_err(|_| CommandError::BadArgument(arg.to_string()))?;
                Command::Forward(steps)
            }
            "TURN" => {
                let arg = tokens.next().ok_or(CommandError::MissingArgument("TURN"))?;
                let direction = match arg.to_ascii_uppercase().as_str() {
                    "LEFT" => Direction::Left,
                    "RIGHT" => Direction::Right,
                    _ => return Err(CommandError::BadArgument(arg.to_string())),
                };
                Command::Turn(direction)
            }
            "WAIT" => Command::Wait,
            _ => return Err(CommandError::UnknownCommand(keyword.to_string())),
        };
        if tokens.next().is_some() {
            return Err(CommandError::TooManyArguments);
        }
        Ok(command)
    }
}

pub fn parse_program(text: &str) -> Result<Vec<Command>, String> {
    text.lines()
        .enumerate()
        .filter(|(_, line)| {
            let line = line.trim();
            !line.is_empty() && !line.starts_with('#')
        })
        .map(|(i, line)| {
            line.parse::<Command>()
                .map_err(|e| format!("line {}: {e}", i + 1))
        })
        .collect()
}

pub fn run(program: &[Command]) -> (i32, i32) {
    let (position, _heading) = program.iter().fold(
        ((0, 0), (0, 1)),
        |((x, y), (dx, dy)), command| match command {
            // Steps are at most 10^4, so the cast is lossless.
            Command::Forward(n) => ((x + dx * *n as i32, y + dy * *n as i32), (dx, dy)),
            Command::Turn(Direction::Left) => ((x, y), (-dy, dx)),
            Command::Turn(Direction::Right) => ((x, y), (dy, -dx)),
            Command::Wait => ((x, y), (dx, dy)),
        },
    );
    position
}
```

Full tested solution: [`src/solutions/p0433_robot_command_parser.rs`](../src/solutions/p0433_robot_command_parser.rs)

</details>
