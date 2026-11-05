# `enum`, `match` và `Option` trong Rust

> Tài liệu học tập về kiểu liệt kê (`enum`), biểu thức `match`, `Option` và các cú pháp rút gọn như `if let`, `let else`: khái niệm, thực hành và bài tập có đáp án.

| Thông tin | Chi tiết |
|---|---|
| Chủ đề | `enum`, biến thể mang dữ liệu, `Option`, `match`, pattern, `if let`, `let else`, `while let`, `matches!`, giới thiệu `Result` |
| Môi trường | Rust 1.99.1 (stable), Cargo 1.99.1, edition 2024 |
| Dự án | `learn_07_enums_match` (thư mục này) |
| Bài trước | [structs](../learn_06_structs/README.md) |
| Nguồn tham khảo chính | *The Rust Programming Language*, chương 6 và chương 19 (mục 19.1, 19.3) |

## Mục lục

1. [Mục tiêu bài học](#1-mục-tiêu-bài-học)
2. [Khái niệm](#2-khái-niệm)
3. [Option và việc không có null](#3-option-và-việc-không-có-null)
4. [match và pattern](#4-match-và-pattern)
5. [Cú pháp ngắn gọn](#5-cú-pháp-ngắn-gọn)
6. [Giới thiệu Result](#6-giới-thiệu-result)
7. [Thực hành](#7-thực-hành)
8. [Tìm hiểu chi tiết](#8-tìm-hiểu-chi-tiết)
9. [Lỗi thường gặp](#9-lỗi-thường-gặp)
10. [Bài tập](#10-bài-tập)
11. [Tóm tắt](#11-tóm-tắt)
12. [Tài liệu tham khảo](#12-tài-liệu-tham-khảo)

---

## 1. Mục tiêu bài học

Sau bài học này bạn có thể:

- [ ] Định nghĩa `enum`, tạo giá trị của nó và phân biệt ba dạng biến thể (không dữ liệu, kiểu tuple, kiểu struct).
- [ ] Viết `impl` cho `enum` để gắn phương thức vào kiểu.
- [ ] Giải thích vì sao Rust không có `null` và dùng `Option<T>` thay thế.
- [ ] Viết `match` đầy đủ (exhaustive) với các loại pattern: giá trị, khoảng, `|`, điều kiện `if`, `@`, `_`, tách tuple và struct.
- [ ] Chọn đúng giữa `match`, `if let`, `let else`, `while let` và `matches!`.
- [ ] Hiểu `Result<T, E>` ở mức nhập môn.
- [ ] Đọc hiểu và sửa các lỗi biên dịch phổ biến liên quan đến `enum` và `match` (E0004, E0308, E0382...).

**Cách dùng tài liệu.** Chép từng ví dụ vào `src/main.rs` rồi chạy:

```bash
cd learn_07_enums_match
cargo run
```

Các ví dụ được đánh dấu như sau:

- Ví dụ có chú thích `// ❌` **không biên dịch được** (cố ý, để minh hoạ lỗi).
- Ví dụ có chú thích `// ⚠️` biên dịch được nhưng sinh cảnh báo.
- Các ví dụ còn lại biên dịch sạch và cho đúng kết quả ghi bên dưới.

---

## 2. Khái niệm

### 2.1 Enum là gì

`enum` (enumeration, kiểu liệt kê) cho phép định nghĩa một kiểu mà giá trị của nó là **đúng một** trong các **biến thể (variant)** được liệt kê sẵn.

Hãy so sánh với `struct` ở bài trước:

| | `struct` | `enum` |
|---|---|---|
| Ý nghĩa | Gom nhiều phần **và** nhau | Chọn **một trong** nhiều khả năng |
| Ví dụ đời thường | Một học sinh có tên **và** tuổi **và** điểm | Đèn giao thông là đỏ **hoặc** vàng **hoặc** xanh |
| Số giá trị tại một thời điểm | Tất cả các trường cùng có | Chỉ một biến thể |

Dùng `enum` khi một giá trị chỉ có thể rơi vào một số trường hợp đã biết trước: hướng đi, trạng thái đơn hàng, loại tiền xu, lệnh người dùng nhập...

### 2.2 Định nghĩa enum và tạo giá trị

```rust
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn turn_around(direction: Direction) -> Direction {
    match direction {
        Direction::North => Direction::South,
        Direction::South => Direction::North,
        Direction::East => Direction::West,
        Direction::West => Direction::East,
    }
}

fn main() {
    let heading = Direction::North;
    println!("Đang đi về hướng {:?}", heading);

    let back = turn_around(heading);
    println!("Quay đầu thì đi về hướng {:?}", back);
}
```

**Kết quả:**

```text
Đang đi về hướng North
Quay đầu thì đi về hướng South
```

Những điểm cần nhớ:

- Mỗi biến thể được viết dạng `TênEnum::TênBiếnThể`. Dấu `::` chỉ ra biến thể nằm trong enum.
- Tên enum và tên biến thể dùng `UpperCamelCase`.
- Mọi biến thể đều thuộc **cùng một kiểu** (`Direction`), nên có thể truyền chúng vào cùng một tham số hàm hoặc để chung trong một mảng.
- `#[derive(Debug)]` cho phép in bằng `{:?}`. Không có dòng này, `println!("{:?}", heading)` sẽ lỗi (xem [mục 9.3](#93-các-lỗi-khác)).
- Hàm `turn_around` dùng `match` để chọn nhánh theo biến thể. `match` được giải thích ở [mục 4](#4-match-và-pattern).

### 2.3 Biến thể mang dữ liệu

Biến thể có thể **mang theo dữ liệu**. Có ba dạng:

| Dạng | Cú pháp | Giống |
|---|---|---|
| Không dữ liệu (unit) | `Quit` | Một giá trị đơn |
| Kiểu tuple | `Write(String)`, `Color(u8, u8, u8)` | Tuple struct |
| Kiểu struct | `Move { x: i32, y: i32 }` | Struct có tên trường |

Ba dạng có thể trộn trong cùng một enum:

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    Color(u8, u8, u8),
}

fn describe(message: &Message) -> String {
    match message {
        Message::Quit => String::from("Thoát"),
        Message::Move { x, y } => format!("Di chuyển đến ({x}, {y})"),
        Message::Write(text) => format!("Viết: {text}"),
        Message::Color(r, g, b) => format!("Đổi màu thành rgb({r}, {g}, {b})"),
    }
}

fn main() {
    let messages = [
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("xin chào")),
        Message::Color(255, 0, 0),
    ];

    for message in &messages {
        println!("{:?} => {}", message, describe(message));
    }

    println!("{:#?}", messages[1]);
}
```

**Kết quả:**

```text
Quit => Thoát
Move { x: 10, y: 20 } => Di chuyển đến (10, 20)
Write("xin chào") => Viết: xin chào
Color(255, 0, 0) => Đổi màu thành rgb(255, 0, 0)
Move {
    x: 10,
    y: 20,
}
```

Lưu ý:

- Bốn biến thể mang dữ liệu khác nhau nhưng đều là `Message`. Nếu viết thành bốn `struct` riêng thì chúng là bốn kiểu khác nhau, không thể để chung trong một mảng hay một tham số hàm.
- `{:?}` in trên một dòng, còn `{:#?}` in nhiều dòng thụt lề (dạng "đẹp"), tiện khi dữ liệu lớn.
- `Message::Move` **không phải** một kiểu. Kiểu là `Message`; `Move` chỉ là một biến thể của nó.

### 2.4 Viết phương thức với `impl`

Giống `struct`, enum có thể có phương thức trong khối `impl`. Trong khối đó, `Self` là tên gọi ngắn của chính enum.

```rust
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn cents(&self) -> u32 {
        match self {
            Self::Penny => 1,
            Self::Nickel => 5,
            Self::Dime => 10,
            Self::Quarter => 25,
        }
    }
}

fn main() {
    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    let mut total = 0;

    for coin in &coins {
        println!("{:?} có giá trị {} xu", coin, coin.cents());
        total += coin.cents();
    }
    println!("Tổng cộng: {total} xu");
}
```

**Kết quả:**

```text
Penny có giá trị 1 xu
Nickel có giá trị 5 xu
Dime có giá trị 10 xu
Quarter có giá trị 25 xu
Tổng cộng: 41 xu
```

Viết `Self::Penny` hay `Coin::Penny` đều được. `&self` mượn giá trị để đọc, cùng quy tắc mượn đã học ở bài ownership.

---

## 3. Option và việc không có null

### 3.1 Vấn đề của null

Nhiều ngôn ngữ cho phép một biến mang giá trị `null` (hoặc `nil`, `None`...) nghĩa là "không có gì". Nếu bạn quên kiểm tra rồi dùng giá trị `null` như một giá trị thật, chương trình sẽ lỗi lúc chạy. Tony Hoare, người phát minh ra tham chiếu `null`, gọi đó là "sai lầm tỉ đô" của ông.

Rust **không có `null`**. Ý "có thể có hoặc không có giá trị" được viết ngay trong kiểu bằng `Option<T>`. Nhờ đó trình biên dịch biết giá trị nào có thể vắng mặt và **bắt bạn xử lý trường hợp vắng mặt** trước khi dùng.

### 3.2 Định nghĩa `Option<T>`

`Option` chỉ là một `enum` trong thư viện chuẩn:

```text
enum Option<T> {
    None,
    Some(T),
}
```

- `None`: không có giá trị.
- `Some(T)`: có giá trị kiểu `T`.
- `T` là kiểu bất kỳ (generic, sẽ học ở bài generics). `Option<i32>` là "có thể có một `i32`".
- `Some` và `None` luôn dùng được mà không cần `use` hay tiền tố `Option::`.

```rust
fn main() {
    let some_number = Some(5);
    let some_text = Some("xin chào");
    let absent: Option<i32> = None;

    println!("{:?} {:?} {:?}", some_number, some_text, absent);
}
```

**Kết quả:**

```text
Some(5) Some("xin chào") None
```

Với `Some(5)` compiler suy ra được kiểu `Option<i32>`. Với `None` thì không có giá trị nào để suy ra, nên bạn phải ghi kiểu (`Option<i32>`).

### 3.3 `Option<i32>` không phải `i32`

Đây là điểm mấu chốt: `Option<i32>` và `i32` là **hai kiểu khác nhau**, không trộn được.

```rust
fn main() {
    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    let sum = x + y; // ❌ không cộng được i32 với Option<i32>
    println!("{sum}");
}
```

**Trình biên dịch báo:**

```text
error[E0277]: cannot add `Option<i32>` to `i32`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y; // ❌ không cộng được i32 với Option<i32>
  |                 ^ no implementation for `i32 + Option<i32>`
  |
  = help: the trait `Add<Option<i32>>` is not implemented for `i32`
help: the following other types implement trait `Add<Rhs>`
...
```

Muốn dùng giá trị bên trong, bạn phải "mở hộp" `Option` và quyết định làm gì khi hộp rỗng. Ở các ngôn ngữ có `null`, bước này rất dễ bị quên.

### 3.4 Lấy giá trị ra khỏi Option

Có nhiều cách, từ đầy đủ nhất đến ngắn gọn nhất:

```rust
fn main() {
    let some = Some(4);
    let none: Option<i32> = None;

    // match: xử lý rõ ràng từng trường hợp
    match some {
        Some(n) => println!("Có giá trị {n}"),
        None => println!("Không có gì"),
    }

    // unwrap_or: nếu None thì dùng giá trị mặc định
    println!("{}", some.unwrap_or(0));
    println!("{}", none.unwrap_or(0));

    // map: biến đổi giá trị bên trong (nếu có), None vẫn là None
    println!("{:?}", some.map(|n| n * 2));
    println!("{:?}", none.map(|n| n * 2));

    // is_some, is_none: chỉ hỏi "có hay không"
    println!("{} {}", some.is_some(), none.is_none());
}
```

**Kết quả:**

```text
Có giá trị 4
4
0
Some(8)
None
true true
```

> `|n| n * 2` là **closure** (hàm ẩn danh): nhận `n`, trả về `n * 2`. Closure sẽ học sau, tạm hiểu đây là cách viết nhanh một hàm nhỏ để truyền vào `map`.

Hai phương thức nguy hiểm cần biết:

| Phương thức | Khi là `None` | Khi dùng |
|---|---|---|
| `unwrap()` | **Panic** với thông báo ``called `Option::unwrap()` on a `None` value`` | Chỉ trong thử nghiệm hoặc khi chắc chắn không thể là `None` |
| `expect("lời nhắn")` | **Panic** với lời nhắn của bạn | Như trên, nhưng lời nhắn giúp tìm lỗi nhanh hơn |

Trong chương trình thật, ưu tiên `match`, `unwrap_or` hoặc `if let` (xem [mục 5](#5-cú-pháp-ngắn-gọn)) thay vì `unwrap`.

### 3.5 Option trong thư viện chuẩn

Bạn đã gặp `Option` mà có thể chưa để ý: `checked_add` ở bài biến trả về `Option`. Nhiều hàm thư viện dùng `Option` để nói "có thể không có kết quả":

```rust
fn main() {
    let scores = [8, 7, 9];

    println!("{:?}", scores.get(1)); // phần tử có chỉ số 1
    println!("{:?}", scores.get(10)); // chỉ số vượt phạm vi: None, không panic
    println!("{:?}", scores.first());
    println!("{:?}", "rust".chars().next()); // ký tự đầu tiên
    println!("{:?}", 250u8.checked_add(10)); // tràn số: None
}
```

**Kết quả:**

```text
Some(7)
None
Some(8)
Some('r')
None
```

So với `scores[10]` (panic khi vượt phạm vi), `scores.get(10)` trả về `None` và để bạn quyết định xử lý thế nào.

---

## 4. match và pattern

### 4.1 Cú pháp match

```text
match <giá trị> {
    <pattern 1> => <biểu thức 1>,
    <pattern 2> => <biểu thức 2>,
    ...
}
```

- `match` so sánh giá trị lần lượt với từng **pattern** (mẫu) **từ trên xuống**. Nhánh đầu tiên khớp sẽ chạy.
- `match` là **biểu thức**: nó trả về giá trị của nhánh được chọn, nên mọi nhánh phải cùng kiểu.
- Nhánh có nhiều lệnh thì bọc trong `{ ... }`; giá trị của nhánh là biểu thức cuối cùng.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("May mắn!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coins = [Coin::Penny, Coin::Dime, Coin::Quarter, Coin::Nickel];
    let mut total = 0;

    for coin in coins {
        total += value_in_cents(coin);
    }
    println!("Tổng: {total} xu");
}
```

**Kết quả:**

```text
May mắn!
Tổng: 41 xu
```

Vì `match` là biểu thức nên có thể gán thẳng cho biến:

```rust
fn main() {
    let day = 6;

    let kind = match day {
        6 | 7 => "cuối tuần",
        _ => "ngày thường",
    };

    println!("Thứ {day} là {kind}");
}
```

**Kết quả:**

```text
Thứ 6 là cuối tuần
```

### 4.2 Lấy dữ liệu ra khỏi biến thể

Pattern có thể **tách** dữ liệu mà biến thể mang theo và gán vào biến mới. Đây là cách chính để dùng giá trị trong `Option` hoặc trong enum có dữ liệu.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("{:?}", plus_one(Some(5)));
    println!("{:?}", plus_one(None));
}
```

**Kết quả:**

```text
Some(6)
None
```

Với `plus_one(Some(5))`, pattern `Some(i)` khớp và `i` nhận giá trị `5`. Biến `i` chỉ có hiệu lực trong nhánh đó. Với `plus_one(None)`, nhánh `None` chạy.

### 4.3 match phải đầy đủ

Rust yêu cầu `match` phải **bao phủ mọi trường hợp có thể**. Quên một trường hợp là lỗi biên dịch, không phải lỗi lúc chạy.

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1), // ❌ thiếu nhánh None
    }
}

fn main() {
    println!("{:?}", plus_one(Some(1)));
}
```

**Trình biên dịch báo:**

```text
error[E0004]: non-exhaustive patterns: `None` not covered
 --> src/main.rs:2:11
  |
2 |     match x {
  |           ^ pattern `None` not covered
  |
...
  = note: the matched value is of type `Option<i32>`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
3 |         Some(i) => Some(i + 1), None => todo!(), // ❌ thiếu nhánh None
  |                               +++++++++++++++++
```

Thông báo chỉ rõ trường hợp bị bỏ sót (`None`) và gợi ý cách thêm nhánh. Đây là một trong những thế mạnh lớn nhất của `enum` + `match`: khi sau này bạn **thêm một biến thể mới** vào enum, compiler chỉ ra **mọi** `match` cần cập nhật, nên không thể quên.

Khi không muốn liệt kê hết, dùng nhánh **bắt tất cả** đặt ở cuối:

- `tên => ...`: bắt mọi giá trị còn lại và gán vào `tên` để dùng tiếp.
- `_ => ...`: bắt mọi giá trị còn lại nhưng **không** dùng giá trị đó.

```rust
fn main() {
    for roll in [3, 7, 5] {
        match roll {
            3 => println!("Trúng thưởng!"),
            7 => println!("Mất lượt"),
            other => println!("Đi {other} bước"),
        }
    }

    let answer = 'x';
    match answer {
        'y' => println!("Đồng ý"),
        'n' => println!("Từ chối"),
        _ => println!("Không hiểu"),
    }
}
```

**Kết quả:**

```text
Trúng thưởng!
Mất lượt
Đi 5 bước
Không hiểu
```

Với số nguyên và `char`, luôn cần nhánh bắt tất cả vì không thể liệt kê hết mọi giá trị.

### 4.4 Các loại pattern

| Pattern | Ví dụ | Khớp khi |
|---|---|---|
| Giá trị cụ thể | `0`, `'a'`, `true` | Bằng đúng giá trị đó |
| Nhiều giá trị | `1 \| 2 \| 3` | Bằng một trong các giá trị |
| Khoảng | `4..=99`, `'a'..='z'` | Nằm trong khoảng, gồm cả hai đầu (`0..10` loại trừ 10, dùng được từ Rust 1.80) |
| Đặt tên | `n`, `other` | Mọi giá trị; gán vào tên đó |
| Bỏ qua | `_` | Mọi giá trị; không gán |
| Điều kiện (guard) | `Some(n) if n > 0` | Khớp pattern **và** điều kiện đúng |
| Gán tên và kiểm tra | `n @ 1..=12` | Nằm trong khoảng; đồng thời gán vào `n` |
| Tách tuple | `(x, 0)` | Phần tử thứ hai bằng 0; lấy phần tử đầu |
| Tách struct | `Point { x, y: 0, .. }` | Trường `y` bằng 0; lấy `x` |
| Tách enum | `Some(n)`, `Message::Move { x, y }` | Đúng biến thể; lấy dữ liệu bên trong |
| Bỏ qua phần còn lại | `..` | Mọi trường (hoặc phần tử) còn lại |

**Giá trị, `|` và khoảng:**

```rust
fn classify(n: i32) -> &'static str {
    match n {
        i32::MIN..=-1 => "âm",
        0 => "không",
        1 | 2 | 3 => "nhỏ (1 đến 3)",
        4..=99 => "vừa (4 đến 99)",
        _ => "lớn (từ 100)",
    }
}

fn kind_of(c: char) -> &'static str {
    match c {
        'a'..='z' | 'A'..='Z' => "chữ cái",
        '0'..='9' => "chữ số",
        _ => "ký tự khác",
    }
}

fn main() {
    for n in [-5, 0, 2, 50, 100] {
        println!("{n}: {}", classify(n));
    }
    println!("{} {} {}", kind_of('R'), kind_of('7'), kind_of('#'));
}
```

**Kết quả:**

```text
-5: âm
0: không
2: nhỏ (1 đến 3)
50: vừa (4 đến 99)
100: lớn (từ 100)
chữ cái chữ số ký tự khác
```

> `&'static str` nghĩa là chuỗi nằm sẵn trong chương trình (như `"âm"`) và sống suốt thời gian chạy. Hàm không có tham số tham chiếu mà trả về `&str` thì cần ghi `'static`. Chi tiết ở bài lifetimes.

**Điều kiện `if` (guard):** thêm điều kiện sau pattern. Nhánh chỉ chạy khi cả pattern lẫn điều kiện đều đúng.

```rust
fn parity(x: Option<i32>) -> String {
    match x {
        Some(n) if n % 2 == 0 => format!("{n} là số chẵn"),
        Some(n) => format!("{n} là số lẻ"),
        None => String::from("không có số"),
    }
}

fn main() {
    println!("{}", parity(Some(4)));
    println!("{}", parity(Some(7)));
    println!("{}", parity(None));
}
```

**Kết quả:**

```text
4 là số chẵn
7 là số lẻ
không có số
```

Compiler **không** xét điều kiện của guard khi kiểm tra tính đầy đủ, nên vẫn cần nhánh `Some(n)` không có `if` để bao phủ phần còn lại.

**`@` (gán tên và kiểm tra khoảng):**

```rust
fn age_group(age: u32) -> String {
    match age {
        n @ 0..=12 => format!("{n} tuổi: trẻ em"),
        n @ 13..=19 => format!("{n} tuổi: thiếu niên"),
        n => format!("{n} tuổi: người lớn"),
    }
}

fn main() {
    println!("{}", age_group(8));
    println!("{}", age_group(15));
    println!("{}", age_group(30));
}
```

**Kết quả:**

```text
8 tuổi: trẻ em
15 tuổi: thiếu niên
30 tuổi: người lớn
```

**Tách tuple (kết hợp guard):**

```rust
fn describe_point(p: (i32, i32)) -> String {
    match p {
        (0, 0) => String::from("gốc tọa độ"),
        (x, 0) => format!("trên trục hoành, x = {x}"),
        (0, y) => format!("trên trục tung, y = {y}"),
        (x, y) if x == y => format!("trên đường chéo, x = y = {x}"),
        (x, y) => format!("điểm ({x}, {y})"),
    }
}

fn main() {
    for p in [(0, 0), (3, 0), (0, -2), (4, 4), (1, 2)] {
        println!("{:?}: {}", p, describe_point(p));
    }
}
```

**Kết quả:**

```text
(0, 0): gốc tọa độ
(3, 0): trên trục hoành, x = 3
(0, -2): trên trục tung, y = -2
(4, 4): trên đường chéo, x = y = 4
(1, 2): điểm (1, 2)
```

**Tách struct và bỏ qua phần còn lại với `..`:**

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn where_is(p: &Point) -> String {
    match p {
        Point { x: 0, y: 0, z: 0 } => String::from("gốc tọa độ"),
        Point { x, y: 0, z: 0 } => format!("trên trục x, x = {x}"),
        Point { x, .. } => format!("x = {x} (bỏ qua y và z)"),
    }
}

fn main() {
    println!("{}", where_is(&Point { x: 0, y: 0, z: 0 }));
    println!("{}", where_is(&Point { x: 5, y: 0, z: 0 }));
    println!("{}", where_is(&Point { x: 2, y: 8, z: 9 }));
}
```

**Kết quả:**

```text
gốc tọa độ
trên trục x, x = 5
x = 2 (bỏ qua y và z)
```

**Pattern lồng nhau:** pattern có thể đặt trong pattern, ví dụ `Some(Coin::Quarter)`.

```rust
enum Coin {
    Penny,
    Quarter,
}

fn describe(coin: Option<Coin>) -> &'static str {
    match coin {
        Some(Coin::Quarter) => "đồng 25 xu",
        Some(_) => "đồng xu khác",
        None => "không có đồng nào",
    }
}

fn main() {
    println!("{}", describe(Some(Coin::Quarter)));
    println!("{}", describe(Some(Coin::Penny)));
    println!("{}", describe(None));
}
```

**Kết quả:**

```text
đồng 25 xu
đồng xu khác
không có đồng nào
```

### 4.5 Thứ tự các nhánh

- Nhánh khớp **đầu tiên** (từ trên xuống) thắng; các nhánh sau không được xét.
- Nhánh bắt tất cả (`_` hoặc một tên) phải đặt **cuối cùng**. Đặt ở giữa thì các nhánh phía dưới không bao giờ chạy, và compiler cảnh báo `unreachable pattern` (xem [mục 9.4](#94-cảnh-báo-thường-gặp)).
- Nhánh có guard phải đặt **trước** nhánh chung cùng pattern, như `parity` ở trên: `Some(n) if ...` đứng trước `Some(n)`.

---

## 5. Cú pháp ngắn gọn

`match` rất mạnh nhưng dài dòng khi bạn chỉ quan tâm một trường hợp. Rust có vài cú pháp rút gọn.

### 5.1 `if let`

`if let` là `match` chỉ có **một** nhánh. Các trường hợp còn lại bị bỏ qua (hoặc đi vào `else`).

```rust
fn main() {
    let max: Option<u8> = Some(3);

    // Cách 1: match, buộc phải có nhánh _ dù không quan tâm
    match max {
        Some(m) => println!("match: lớn nhất là {m}"),
        _ => (),
    }

    // Cách 2: if let, gọn hơn
    if let Some(m) = max {
        println!("if let: lớn nhất là {m}");
    }

    // if let có thể có else
    let none: Option<u8> = None;
    if let Some(m) = none {
        println!("{m}");
    } else {
        println!("không có giá trị");
    }
}
```

**Kết quả:**

```text
match: lớn nhất là 3
if let: lớn nhất là 3
không có giá trị
```

Từ Rust 1.88 (edition 2024), có thể nối thêm điều kiện bằng `&&` ngay sau `if let`:

```rust
fn main() {
    let max: Option<u8> = Some(3);

    if let Some(m) = max && m > 2 {
        println!("{m} lớn hơn 2");
    }
}
```

**Kết quả:**

```text
3 lớn hơn 2
```

> **Đánh đổi:** `if let` không kiểm tra tính đầy đủ. Nếu sau này thêm biến thể mới vào enum, compiler sẽ **không** nhắc bạn xem lại `if let` như với `match`.

### 5.2 `let else`

`let else` lấy giá trị ra nếu pattern khớp, ngược lại chạy khối `else` và **phải thoát** (`return`, `continue`, `break` hoặc `panic!`). Nhờ vậy phần code chính phía sau không bị thụt vào nhiều tầng.

```rust
fn describe_age(input: Option<u32>) -> String {
    let Some(age) = input else {
        return String::from("không rõ tuổi");
    };

    // từ đây trở đi, age là u32 bình thường
    format!("{age} tuổi")
}

fn main() {
    println!("{}", describe_age(Some(20)));
    println!("{}", describe_age(None));
}
```

**Kết quả:**

```text
20 tuổi
không rõ tuổi
```

Khối `else` mà không thoát sẽ lỗi biên dịch (xem [mục 9.3](#93-các-lỗi-khác)).

### 5.3 `while let`

`while let` lặp **miễn là** pattern còn khớp. Nó hay đi với các hàm trả về `Option` như `next()`:

```rust
fn main() {
    let mut numbers = [10, 20, 30].into_iter();

    while let Some(n) = numbers.next() {
        println!("Lấy ra {n}");
    }
    println!("Hết");
}
```

**Kết quả:**

```text
Lấy ra 10
Lấy ra 20
Lấy ra 30
Hết
```

`next()` trả về `Some(giá_trị)` cho đến khi hết phần tử thì trả về `None`, lúc đó vòng lặp dừng. (Với `Vec`, mẫu `while let Some(top) = stack.pop()` rất phổ biến; sẽ gặp ở bài collections.)

### 5.4 `matches!`

`matches!` trả về `bool`: giá trị có khớp pattern hay không. Pattern có thể có `|` và guard.

```rust
fn main() {
    let c = 'e';
    let is_vowel = matches!(c, 'a' | 'e' | 'i' | 'o' | 'u');

    let x = Some(7);
    let is_big = matches!(x, Some(n) if n > 5);

    println!("{is_vowel} {is_big}");
}
```

**Kết quả:**

```text
true true
```

### 5.5 Chọn cú pháp nào?

| Tình huống | Chọn |
|---|---|
| Xử lý **mọi** trường hợp, muốn compiler kiểm tra đầy đủ | `match` |
| Chỉ quan tâm **một** pattern, bỏ qua phần còn lại | `if let` |
| Cần lấy giá trị ra để dùng tiếp, trường hợp khác thì **thoát sớm** | `let else` |
| Lặp cho đến khi pattern không còn khớp | `while let` |
| Chỉ cần câu trả lời đúng hoặc sai | `matches!` |
| Chỉ cần giá trị mặc định khi `None` | `unwrap_or` |

---

## 6. Giới thiệu Result

`Result` là enum thứ hai quan trọng của thư viện chuẩn, dùng cho thao tác **có thể thất bại**:

```text
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

| | `Option<T>` | `Result<T, E>` |
|---|---|---|
| Biến thể | `Some(T)` hoặc `None` | `Ok(T)` hoặc `Err(E)` |
| Ý nghĩa | Có hay không có giá trị | Thành công hay thất bại |
| Lý do khi vắng mặt | Không có | Có, nằm trong `Err(E)` |
| Ví dụ | `scores.get(10)` | `"abc".parse::<i32>()` |

Bạn đã dùng `Result` ở bài biến: `"42".parse()` trả về `Result`, và `.expect("...")` chính là một `match` dừng chương trình khi gặp `Err`. Giờ có thể tự viết `match`:

```rust
fn main() {
    for text in ["42", "abc", ""] {
        match text.parse::<i32>() {
            Ok(n) => println!("{text:?} -> số {n}"),
            Err(e) => println!("{text:?} -> lỗi: {e}"),
        }
    }
}
```

**Kết quả:**

```text
"42" -> số 42
"abc" -> lỗi: invalid digit found in string
"" -> lỗi: cannot parse integer from empty string
```

Mọi công cụ ở mục 5 dùng được cho `Result`. Ví dụ `let else` kết hợp `continue` để bỏ qua dữ liệu hỏng:

```rust
fn main() {
    let inputs = ["12", "abc", "7"];
    let mut total = 0;

    for text in inputs {
        let Ok(n) = text.parse::<u32>() else {
            println!("Bỏ qua {text:?}: không phải số");
            continue;
        };
        total += n;
    }

    println!("Tổng: {total}");
}
```

**Kết quả:**

```text
Bỏ qua "abc": không phải số
Tổng: 19
```

Cách xử lý lỗi bài bản (toán tử `?`, kiểu lỗi riêng...) được học ở bài [error_handling](../learn_09_error_handling/README.md).

---

## 7. Thực hành

Ba bài thực hành có hướng dẫn. Thực hiện trong `learn_07_enums_match/src/main.rs` và chạy bằng `cargo run`.

### Lab 1. Trạng thái đơn hàng: enum có dữ liệu, impl, match

**Mục tiêu:** mô hình hoá vòng đời đơn hàng bằng enum, và thấy compiler nhắc bạn cập nhật `match` khi thêm biến thể.

**Bước 1.** Chép và chạy chương trình:

```rust
enum OrderStatus {
    Pending,
    Paid { amount: u32 },
    Shipped { carrier: String },
    Cancelled(String),
}

impl OrderStatus {
    fn describe(&self) -> String {
        match self {
            OrderStatus::Pending => String::from("Chờ thanh toán"),
            OrderStatus::Paid { amount } => format!("Đã thanh toán {amount} đồng"),
            OrderStatus::Shipped { carrier } => format!("Đang giao bởi {carrier}"),
            OrderStatus::Cancelled(reason) => format!("Đã hủy: {reason}"),
        }
    }

    fn is_open(&self) -> bool {
        !matches!(self, OrderStatus::Cancelled(_))
    }
}

fn main() {
    let orders = [
        OrderStatus::Pending,
        OrderStatus::Paid { amount: 150_000 },
        OrderStatus::Shipped {
            carrier: String::from("GHN"),
        },
        OrderStatus::Cancelled(String::from("khách đổi ý")),
    ];

    for order in &orders {
        println!("{} (còn hiệu lực: {})", order.describe(), order.is_open());
    }
}
```

**Kết quả:**

```text
Chờ thanh toán (còn hiệu lực: true)
Đã thanh toán 150000 đồng (còn hiệu lực: true)
Đang giao bởi GHN (còn hiệu lực: true)
Đã hủy: khách đổi ý (còn hiệu lực: false)
```

**Bước 2.** Thêm một biến thể mới vào cuối enum, **chưa** sửa gì khác:

```text
    Cancelled(String),
    Refunded { amount: u32 },   // thêm dòng này
}
```

Chạy lại `cargo run`. Compiler dừng với lỗi `E0004: non-exhaustive patterns: &OrderStatus::Refunded { .. } not covered` trỏ đúng vào dòng `match self` trong `describe`. Bạn không thể quên xử lý biến thể mới.

**Bước 3.** Thêm nhánh cho `Refunded` trong `describe` và đưa một đơn hoàn tiền vào danh sách:

```rust
enum OrderStatus {
    Pending,
    Paid { amount: u32 },
    Shipped { carrier: String },
    Cancelled(String),
    Refunded { amount: u32 },
}

impl OrderStatus {
    fn describe(&self) -> String {
        match self {
            OrderStatus::Pending => String::from("Chờ thanh toán"),
            OrderStatus::Paid { amount } => format!("Đã thanh toán {amount} đồng"),
            OrderStatus::Shipped { carrier } => format!("Đang giao bởi {carrier}"),
            OrderStatus::Cancelled(reason) => format!("Đã hủy: {reason}"),
            OrderStatus::Refunded { amount } => format!("Đã hoàn {amount} đồng"),
        }
    }

    fn is_open(&self) -> bool {
        !matches!(
            self,
            OrderStatus::Cancelled(_) | OrderStatus::Refunded { .. }
        )
    }
}

fn main() {
    let orders = [
        OrderStatus::Pending,
        OrderStatus::Paid { amount: 150_000 },
        OrderStatus::Shipped {
            carrier: String::from("GHN"),
        },
        OrderStatus::Cancelled(String::from("khách đổi ý")),
        OrderStatus::Refunded { amount: 150_000 },
    ];

    for order in &orders {
        println!("{} (còn hiệu lực: {})", order.describe(), order.is_open());
    }
}
```

**Kết quả:**

```text
Chờ thanh toán (còn hiệu lực: true)
Đã thanh toán 150000 đồng (còn hiệu lực: true)
Đang giao bởi GHN (còn hiệu lực: true)
Đã hủy: khách đổi ý (còn hiệu lực: false)
Đã hoàn 150000 đồng (còn hiệu lực: false)
```

**Phân tích:**

- Mỗi biến thể mang dữ liệu khác nhau (`amount`, `carrier`, lý do hủy), nhưng cả danh sách vẫn là một mảng `OrderStatus`.
- `match self` trong `describe` là `match` **đầy đủ**: compiler bắt lỗi ở Bước 2.
- `is_open` dùng `matches!` cho gọn. Đổi lại, nó **không** nhắc bạn khi có biến thể mới: nếu thêm `Returned`, `is_open` vẫn biên dịch và coi nó là "còn hiệu lực". Khi quy tắc quan trọng, hãy viết bằng `match` đầy đủ.

**Thử thêm:** thêm biến thể `Delivered { days: u32 }` ("giao xong sau `days` ngày") và để compiler dẫn bạn đến chỗ cần sửa.

### Lab 2. Tìm kiếm trả về Option

**Mục tiêu:** viết hàm có thể "không tìm thấy" và dùng kết quả theo bốn cách: `match`, `if let`, `unwrap_or`, `map`.

```rust
fn find_index(items: &[i32], target: i32) -> Option<usize> {
    let mut i = 0;
    while i < items.len() {
        if items[i] == target {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}

fn main() {
    let numbers = [4, 8, 15, 16, 23, 42];

    // 1. match
    match find_index(&numbers, 15) {
        Some(index) => println!("15 nằm ở vị trí {index}"),
        None => println!("Không tìm thấy 15"),
    }

    // 2. if let
    if let Some(index) = find_index(&numbers, 99) {
        println!("99 nằm ở vị trí {index}");
    } else {
        println!("Không tìm thấy 99");
    }

    // 3. unwrap_or
    let position = find_index(&numbers, 7).unwrap_or(0);
    println!("Vị trí của 7 (mặc định 0): {position}");

    // 4. map
    let doubled = safe_divide(20, 4).map(|q| q * 2);
    println!("{:?} {:?}", doubled, safe_divide(1, 0));
}
```

**Kết quả:**

```text
15 nằm ở vị trí 2
Không tìm thấy 99
Vị trí của 7 (mặc định 0): 0
Some(10) None
```

**Phân tích:**

- Hàm trả về `Option<usize>` thay vì một "số đặc biệt" như `-1` để báo "không tìm thấy". Kiểu dữ liệu tự nói lên rằng kết quả có thể vắng mặt, và compiler buộc người gọi xử lý.
- `usize` vì giá trị là **chỉ số** trong mảng.
- `items: &[i32]` là **slice** (mượn một dãy `i32`), nên hàm nhận được mảng bất kỳ độ dài mà không lấy quyền sở hữu.
- `safe_divide(1, 0)` trả về `None` thay vì làm chương trình panic vì chia cho 0.

**Thử thêm:**

1. Đổi `safe_divide(20, 4)` thành `safe_divide(20, 0)`. `map` bỏ qua closure và kết quả là `None`.
2. Viết hàm `first_even(items: &[i32]) -> Option<i32>` trả về số chẵn đầu tiên trong dãy, hoặc `None` nếu không có.

### Lab 3. Máy bán nước: match trên cặp (trạng thái, sự kiện)

**Mục tiêu:** dùng `enum` mô tả trạng thái và sự kiện, `match` trên tuple để quyết định trạng thái tiếp theo.

```rust
#[derive(Debug, Clone, Copy)]
enum State {
    Idle,
    HasMoney(u32),
    Dispensing,
}

#[derive(Debug)]
enum Event {
    InsertCoin(u32),
    Select { price: u32 },
    Cancel,
    Take,
}

fn next(state: State, event: Event) -> State {
    match (state, event) {
        (State::Idle, Event::InsertCoin(coin)) => State::HasMoney(coin),
        (State::HasMoney(money), Event::InsertCoin(coin)) => State::HasMoney(money + coin),
        (State::HasMoney(money), Event::Select { price }) if money >= price => State::Dispensing,
        (State::HasMoney(_), Event::Cancel) => State::Idle,
        (State::Dispensing, Event::Take) => State::Idle,
        (current, _) => current, // sự kiện không hợp lệ: giữ nguyên trạng thái
    }
}

fn main() {
    let events = [
        Event::Take,
        Event::InsertCoin(5_000),
        Event::Cancel,
        Event::InsertCoin(10_000),
        Event::InsertCoin(10_000),
        Event::Select { price: 25_000 },
        Event::Select { price: 15_000 },
        Event::Take,
    ];

    let mut state = State::Idle;
    for event in events {
        let label = format!("{:?}", event);
        let after = next(state, event);
        println!("{:?} --[{}]--> {:?}", state, label, after);
        state = after;
    }
}
```

**Kết quả:**

```text
Idle --[Take]--> Idle
Idle --[InsertCoin(5000)]--> HasMoney(5000)
HasMoney(5000) --[Cancel]--> Idle
Idle --[InsertCoin(10000)]--> HasMoney(10000)
HasMoney(10000) --[InsertCoin(10000)]--> HasMoney(20000)
HasMoney(20000) --[Select { price: 25000 }]--> HasMoney(20000)
HasMoney(20000) --[Select { price: 15000 }]--> Dispensing
Dispensing --[Take]--> Idle
```

**Phân tích:**

- `match (state, event)` ghép hai giá trị thành một tuple rồi khớp **cả cặp** cùng lúc, nên mỗi nhánh là một "luật chuyển trạng thái".
- Nhánh thứ ba có **guard** `if money >= price`: khi tiền không đủ (`15000 < 25000`), nhánh này bị bỏ qua và rơi xuống nhánh cuối.
- Nhánh cuối `(current, _) => current` bắt mọi cặp còn lại, như bấm `Take` khi máy đang rảnh. Nếu xoá nhánh này, compiler báo `E0004` liệt kê các cặp chưa xử lý (`(State::Idle, Event::Select { .. })`, `(State::Idle, Event::Cancel)`, ... and 4 more not covered).
- `State` có `Clone, Copy` nên dùng `state` nhiều lần mà không bị move.

**Thử thêm:** nhánh `(current, _)` là nhánh bắt tất cả, nên khi thêm trạng thái mới (ví dụ `OutOfOrder`) compiler **không** nhắc bạn. Hãy cân nhắc liệt kê rõ từng cặp hợp lệ nếu muốn compiler kiểm tra giúp.

---

## 8. Tìm hiểu chi tiết

### 8.1 Enum trong bộ nhớ

Một enum phải nhớ nó đang là biến thể nào, nên thường kèm một **nhãn (tag)** nhỏ. Vì vậy `Option<u32>` to hơn `u32`. Nhưng với một số kiểu, compiler tận dụng "chỗ trống" có sẵn nên `Option` không tốn thêm byte nào (tối ưu niche).

```rust
use std::mem::size_of;

fn main() {
    println!("bool: {} byte", size_of::<bool>());
    println!("Option<bool>: {} byte", size_of::<Option<bool>>());
    println!("u32: {} byte", size_of::<u32>());
    println!("Option<u32>: {} byte", size_of::<Option<u32>>());
    println!("&u8: {} byte", size_of::<&u8>());
    println!("Option<&u8>: {} byte", size_of::<Option<&u8>>());
}
```

**Kết quả (trên máy 64 bit):**

```text
bool: 1 byte
Option<bool>: 1 byte
u32: 4 byte
Option<u32>: 8 byte
&u8: 8 byte
Option<&u8>: 8 byte
```

- `u32` dùng hết mọi giá trị của 4 byte nên `Option<u32>` cần thêm chỗ cho nhãn (8 byte do căn chỉnh bộ nhớ).
- `bool` chỉ dùng giá trị 0 và 1, còn nhiều giá trị trống để biểu diễn `None`. Tham chiếu `&u8` không bao giờ bằng 0, nên `None` dùng chính giá trị 0 đó.

Bạn không cần tối ưu thủ công gì. Chỉ cần biết dùng `Option<&T>` gần như không tốn thêm chi phí so với con trỏ thường.

### 8.2 Giá trị số của biến thể

Enum **không mang dữ liệu** có thể ép sang số nguyên bằng `as`. Mặc định giá trị bắt đầu từ `0` và tăng dần 1; bạn có thể gán giá trị tường minh, các biến thể sau đó tiếp tục tăng từ đó.

```rust
#[derive(Debug, Clone, Copy)]
enum Level {
    Low,
    Mid = 5,
    High,
}

fn main() {
    let levels = [Level::Low, Level::Mid, Level::High];

    for level in levels {
        println!("{:?} = {}", level, level as i32);
    }
}
```

**Kết quả:**

```text
Low = 0
Mid = 5
High = 6
```

Chiều ngược lại (từ số sang enum) **không có** phép `as`; bạn phải viết `match` hoặc dùng `TryFrom` (sẽ học sau). Điều này tránh việc tạo ra giá trị enum không hợp lệ.

### 8.3 derive và so sánh enum

Muốn `==`, `!=` với enum phải khai báo `PartialEq`; muốn gán mà không bị move thì thêm `Clone, Copy`.

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum Light {
    Red,
    Green,
}

fn main() {
    let a = Light::Red;
    let b = a; // Copy: a vẫn dùng được

    println!("{}", a == b);
    println!("{}", a == Light::Green);
    println!("{:?} {:?}", a, b);
}
```

**Kết quả:**

```text
true
false
Red Red
```

| `derive` | Cho phép |
|---|---|
| `Debug` | In bằng `{:?}` và `{:#?}` |
| `Clone` | Gọi `.clone()` để sao chép |
| `Copy` | Gán và truyền vào hàm mà không bị move (chỉ khi mọi dữ liệu bên trong cũng `Copy`) |
| `PartialEq` | So sánh bằng `==` và `!=` |

Enum có biến thể chứa `String` thì **không** `Copy` được. Khi chỉ cần kiểm tra biến thể, `matches!(x, Light::Red)` không đòi `PartialEq`.

### 8.4 match và quyền sở hữu

`match` trên một giá trị sở hữu dữ liệu (như `Option<String>`) có thể **chuyển (move)** dữ liệu ra khỏi nó:

```rust
fn main() {
    let name: Option<String> = Some(String::from("An"));

    match name {
        Some(n) => println!("Xin chào {n}"),
        None => println!("Không có tên"),
    }

    println!("{:?}", name); // ❌ String bên trong đã bị move vào n
}
```

**Trình biên dịch báo:**

```text
error[E0382]: borrow of partially moved value: `name`
 --> src/main.rs:9:22
  |
5 |         Some(n) => println!("Xin chào {n}"),
  |              - value partially moved here
...
9 |     println!("{:?}", name); // ❌ String bên trong đã bị move vào n
  |                      ^^^^ value borrowed here after partial move
  |
  = note: partial move occurs because value has type `String`, which does not implement the `Copy` trait
help: borrow this binding in the pattern to avoid moving the value
  |
5 |         Some(ref n) => println!("Xin chào {n}"),
  |              +++
```

Cách sửa: `match` trên **tham chiếu** `&name` để chỉ mượn.

```rust
fn main() {
    let name: Option<String> = Some(String::from("An"));

    match &name {
        Some(n) => println!("Xin chào {n}"),
        None => println!("Không có tên"),
    }

    println!("{:?}", name);
}
```

**Kết quả:**

```text
Xin chào An
Some("An")
```

Với `match &name`, `n` có kiểu `&String` (một tham chiếu), nên `name` vẫn còn nguyên. Hai cách tương đương: `if let Some(n) = &name` hoặc `name.as_ref()` (biến `Option<String>` thành `Option<&String>`).

### 8.5 Các phương thức hay dùng của Option

```rust
fn main() {
    let some: Option<i32> = Some(6);
    let none: Option<i32> = None;

    println!("{:?}", some.map(|n| n + 1));
    println!("{:?}", some.and_then(|n| n.checked_mul(1_000_000_000)));
    println!("{:?}", some.filter(|n| n % 2 == 1));
    println!("{}", none.unwrap_or(-1));
    println!("{}", none.unwrap_or_default());
    println!("{}", none.unwrap_or_else(|| 10 * 10));
    println!("{:?}", some.ok_or("không có giá trị"));
    println!("{:?}", none.ok_or("không có giá trị"));
    println!("{:?}", none.or(Some(1)));
}
```

**Kết quả:**

```text
Some(7)
None
None
-1
0
100
Ok(6)
Err("không có giá trị")
Some(1)
```

| Phương thức | Việc làm | Dùng khi |
|---|---|---|
| `map(f)` | Biến đổi giá trị bên trong; `None` giữ nguyên | Muốn xử lý giá trị nếu có |
| `and_then(f)` | Như `map` nhưng `f` tự trả về `Option` | Chuỗi nhiều bước, mỗi bước có thể thất bại |
| `filter(f)` | Giữ `Some` nếu thoả điều kiện, ngược lại `None` | Lọc giá trị |
| `unwrap_or(v)` | Giá trị bên trong, hoặc `v` nếu `None` | Có giá trị mặc định rẻ |
| `unwrap_or_default()` | Giá trị bên trong, hoặc mặc định của kiểu (`0`, `""`...) | Mặc định của kiểu là đủ |
| `unwrap_or_else(f)` | Như `unwrap_or` nhưng chỉ tính `f()` khi cần | Mặc định tốn kém để tính |
| `ok_or(e)` | Đổi `Option` thành `Result`: `Some(v)` → `Ok(v)`, `None` → `Err(e)` | Muốn kèm lý do khi vắng mặt |
| `or(opt)` | Dùng `opt` nếu là `None` | Có phương án dự phòng |
| `is_some()`, `is_none()` | Trả về `bool` | Chỉ cần biết có hay không |

Ở ví dụ trên, `6 * 1_000_000_000` vượt phạm vi `i32` nên `checked_mul` trả về `None`, và `and_then` cho ra `None`.

### 8.6 Đặt tên, use và biến thể

| Đối tượng | Quy ước | Ví dụ |
|---|---|---|
| Tên enum | `UpperCamelCase` | `OrderStatus` |
| Tên biến thể | `UpperCamelCase` | `Pending`, `Paid` |
| Phương thức | `snake_case` | `is_open` |

Có thể đưa các biến thể vào phạm vi bằng `use` để khỏi gõ tiền tố `Light::`:

```rust
enum Light {
    Red,
    Yellow,
    Green,
}

use Light::*;

fn wait_seconds(light: Light) -> u32 {
    match light {
        Red => 30,
        Yellow => 3,
        Green => 0,
    }
}

fn main() {
    println!("{} {} {}", wait_seconds(Red), wait_seconds(Yellow), wait_seconds(Green));
}
```

**Kết quả:**

```text
30 3 0
```

> **Cẩn thận:** nếu quên `use` hoặc gõ sai tên biến thể trong pattern, Rust hiểu đó là **tên biến mới** bắt mọi giá trị, chứ không phải biến thể. Compiler có bắt lỗi này (xem lỗi `E0170` ở [mục 9.2](#92-lỗi-biên-dịch-hay-gặp-nhất)). Người mới nên giữ tiền tố `Light::Red` cho rõ ràng.

### 8.7 Chọn enum, struct, Option hay Result?

| Dữ liệu của bạn | Chọn |
|---|---|
| Luôn có **đủ mọi phần** (tên và tuổi và điểm) | `struct` |
| Là **một trong vài dạng**, mỗi dạng có thể mang dữ liệu khác nhau | `enum` |
| Một trạng thái đổi theo thời gian (chờ, đang chạy, xong) | `enum` |
| Có hoặc không có giá trị | `Option<T>` |
| Thành công hoặc thất bại, kèm lý do | `Result<T, E>` |

---

## 9. Lỗi thường gặp

### 9.1 Cách đọc thông báo lỗi liên quan đến match

Cách đọc chung (mã lỗi, vị trí, dấu `^^^`, `help:`) đã nêu ở [bài biến](../learn_03_variables/README.md#71-cách-đọc-thông-báo-lỗi). Với `enum` và `match`, các cụm từ sau xuất hiện nhiều nhất:

| Cụm từ trong thông báo | Nghĩa |
|---|---|
| `non-exhaustive patterns: ... not covered` | `match` thiếu một số trường hợp (liệt kê ngay sau dấu hai chấm) |
| `` `match` arms have incompatible types `` | Các nhánh trả về kiểu khác nhau |
| `pattern binding ... is named the same as one of the variants` | Quên tiền tố `TênEnum::`, nên pattern bị hiểu là tên biến |
| `borrow of partially moved value` | `match` đã chuyển (move) dữ liệu ra khỏi giá trị gốc |
| ``expected `Option<T>`, found ...`` | Quên bọc giá trị trong `Some(...)` |

### 9.2 Lỗi biên dịch hay gặp nhất

**1. `E0004`: `match` thiếu trường hợp.** Với số nguyên, ký tự hay chuỗi luôn cần nhánh bắt tất cả:

```rust
fn main() {
    let n: u8 = 3;

    let name = match n { // ❌ thiếu nhánh bắt tất cả
        1 => "một",
        2 => "hai",
    };

    println!("{name}");
}
```

**Trình biên dịch báo:**

```text
error[E0004]: non-exhaustive patterns: `0_u8` and `3_u8..=u8::MAX` not covered
 --> src/main.rs:4:22
  |
4 |     let name = match n { // ❌ thiếu nhánh bắt tất cả
  |                      ^ patterns `0_u8` and `3_u8..=u8::MAX` not covered
  |
  = note: the matched value is of type `u8`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern, a match arm with multiple or-patterns as shown, or multiple match arms
  |
6 ~         2 => "hai",
7 ~         0_u8 | 3_u8..=u8::MAX => todo!(),
  |
```

Cách sửa: thêm `_ => "nhiều"` ở cuối.

**2. `E0308`: các nhánh `match` trả về kiểu khác nhau.** `match` là biểu thức nên mọi nhánh phải cùng kiểu:

```rust
fn main() {
    let n = 3;

    let text = match n {
        1 => "một",
        2 => 2, // ❌ nhánh này là số, các nhánh khác là chuỗi
        _ => "nhiều",
    };

    println!("{text}");
}
```

**Trình biên dịch báo:**

```text
error[E0308]: `match` arms have incompatible types
 --> src/main.rs:6:14
  |
4 |       let text = match n {
  |  ________________-
5 | |         1 => "một",
  | |              ----- this is found to be of type `&str`
6 | |         2 => 2, // ❌ nhánh này là số, các nhánh khác là chuỗi
  | |              ^ expected `&str`, found integer
7 | |         _ => "nhiều",
8 | |     };
  | |_____- `match` arms have incompatible types
```

Cách sửa: đổi `2` thành `"hai"` (hoặc cho mọi nhánh cùng một kiểu khác).

**3. `E0308`: quên bọc giá trị trong `Some`.**

```rust
fn main() {
    let x: Option<i32> = 5; // ❌ 5 là i32, không phải Option<i32>
    println!("{:?}", x);
}
```

**Trình biên dịch báo:**

```text
error[E0308]: mismatched types
 --> src/main.rs:2:26
  |
2 |     let x: Option<i32> = 5; // ❌ 5 là i32, không phải Option<i32>
  |            -----------   ^ expected `Option<i32>`, found integer
  |            |
  |            expected due to this
  |
  = note: expected enum `Option<i32>`
             found type `{integer}`
help: try wrapping the expression in `Some`
  |
2 |     let x: Option<i32> = Some(5); // ❌ 5 là i32, không phải Option<i32>
  |                          +++++ +
```

Cách sửa: viết `Some(5)`, đúng như gợi ý `help:`.

**4. `E0170`: quên tiền tố, pattern thành tên biến.** Đây là lỗi dễ gây bối rối nhất:

```rust
enum Direction {
    North,
    South,
}

fn name(direction: Direction) -> &'static str {
    match direction {
        North => "bắc", // ❌ thiếu Direction::
        South => "nam",
    }
}

fn main() {
    println!("{}", name(Direction::South));
    let _ = Direction::North;
}
```

**Trình biên dịch báo:**

```text
error[E0170]: pattern binding `North` is named the same as one of the variants of the type `Direction`
 --> src/main.rs:8:9
  |
8 |         North => "bắc", // ❌ thiếu Direction::
  |         ^^^^^ help: to match on the variant, qualify the path: `Direction::North`
  |
  = note: `#[deny(bindings_with_variant_name)]` on by default
...
```

Vì `North` không có tiền tố và chưa được `use`, Rust hiểu đó là **tên biến mới** khớp mọi giá trị. Compiler báo cùng lỗi cho `South`, và còn kèm cảnh báo `unreachable pattern` vì `North` đã nuốt mọi giá trị nên nhánh phía dưới không bao giờ chạy (phần còn lại của thông báo được lược bớt). Cách sửa: viết `Direction::North` và `Direction::South`.

### 9.3 Các lỗi khác

| Mã lỗi | Thông báo (rút gọn) | Nguyên nhân | Cách sửa |
|---|---|---|---|
| `E0277` | `` cannot add `Option<i32>` to `i32` `` | Cộng `i32` với `Option<i32>` ([mục 3.3](#33-optioni32-không-phải-i32)); đảo vế (`Option<i32> + 1`) thì báo `E0369` | `match`, `unwrap_or`... lấy giá trị ra trước |
| `E0282` | `` type annotations needed for `Option<_>` `` | `let x = None;` không cho compiler biết kiểu bên trong | `let x: Option<i32> = None;` |
| `E0599` | `` no variant, associated function, or constant named `Nort` found for enum `Direction` `` | Gõ sai tên biến thể | Sửa chính tả; compiler gợi ý tên gần giống |
| `E0023` | `this pattern has 2 fields, but the corresponding tuple variant has 3 fields` | Pattern không đủ số trường của biến thể | Ghi đủ trường, dùng `_` hoặc `..` cho trường bỏ qua |
| `E0369` | `` binary operation `==` cannot be applied to type `Light` `` | Enum chưa có `PartialEq` | Thêm `#[derive(PartialEq)]`, hoặc dùng `matches!` |
| `E0277` | `` `Direction` doesn't implement `Debug` `` | In bằng `{:?}` mà enum chưa có `Debug` | Thêm `#[derive(Debug)]` |
| `E0308` | `` `else` clause of `let...else` does not diverge `` | Khối `else` của `let else` không thoát | Thêm `return`, `continue`, `break` hoặc `panic!` |
| `E0382` | `` borrow of partially moved value: `name` `` | `match` chuyển dữ liệu ra khỏi giá trị gốc ([mục 8.4](#84-match-và-quyền-sở-hữu)) | `match &name`, hoặc `name.as_ref()` |

> Các thông báo trên được lấy từ Rust 1.99.1. Câu chữ và mã lỗi có thể khác chút giữa các phiên bản compiler.

### 9.4 Cảnh báo thường gặp

Cảnh báo không chặn việc biên dịch nhưng nên được sửa hết.

```rust
// ⚠️ Biên dịch được nhưng có 4 cảnh báo
enum shape {
    Circle(f64),
    Unused,
}

fn main() {
    let s = shape::Circle(1.0);

    match s {
        shape::Circle(radius) => println!("hình tròn"),
        _ => println!("hình khác"),
        shape::Unused => println!("không bao giờ tới đây"),
    }
}
```

| Cảnh báo | Ý nghĩa | Cách xử lý |
|---|---|---|
| `unreachable pattern` | Nhánh nằm sau nhánh bắt tất cả (`_`) nên không bao giờ chạy | Đổi thứ tự (nhánh cụ thể trước, `_` sau cùng) hoặc xoá nhánh thừa |
| `` unused variable: `radius` `` | Tên đặt trong pattern nhưng không dùng | Dùng nó, hoặc viết `_radius` / `_` |
| ``variant `Unused` is never constructed`` | Biến thể không được tạo ở đâu trong chương trình | Xoá, hoặc tạo giá trị đó ở chỗ cần |
| ``type `shape` should have an upper camel case name`` | Tên enum sai quy ước | Đổi thành `Shape` |

---

## 10. Bài tập

Hãy tự làm trước, chạy bằng `cargo run`, rồi mới mở phần đáp án. Mỗi bài có **kết quả mong đợi** để bạn tự kiểm tra.

### Cấp độ 1. Làm quen

#### Bài 1.1. Đèn giao thông

**Đề:** định nghĩa enum `TrafficLight` gồm `Red`, `Yellow`, `Green`. Viết hàm `action(light: TrafficLight) -> &'static str` trả về `"dừng lại"`, `"chuẩn bị"`, `"đi"` tương ứng. Trong `main`, in ra ba dòng như bên dưới.

**Kết quả mong đợi:**

```text
Đỏ: dừng lại
Vàng: chuẩn bị
Xanh: đi
```

**Gợi ý:** dùng `match` trên `light`, mỗi biến thể một nhánh.

<details>
<summary>Xem đáp án</summary>

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TrafficLight) -> &'static str {
    match light {
        TrafficLight::Red => "dừng lại",
        TrafficLight::Yellow => "chuẩn bị",
        TrafficLight::Green => "đi",
    }
}

fn main() {
    println!("Đỏ: {}", action(TrafficLight::Red));
    println!("Vàng: {}", action(TrafficLight::Yellow));
    println!("Xanh: {}", action(TrafficLight::Green));
}
```

</details>

#### Bài 1.2. Sửa lỗi match thiếu nhánh

**Đề:** đoạn mã sau không biên dịch được. Đọc thông báo lỗi để biết thiếu gì rồi sửa.

```rust
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

fn describe(season: Season) -> &'static str {
    match season { // ❌ thiếu một nhánh
        Season::Spring => "ấm áp",
        Season::Summer => "nóng",
        Season::Autumn => "mát",
    }
}

fn main() {
    println!("Xuân: {}", describe(Season::Spring));
    println!("Hạ: {}", describe(Season::Summer));
    println!("Thu: {}", describe(Season::Autumn));
    println!("Đông: {}", describe(Season::Winter));
}
```

**Kết quả mong đợi:**

```text
Xuân: ấm áp
Hạ: nóng
Thu: mát
Đông: lạnh
```

**Gợi ý:** thông báo `E0004` cho biết biến thể nào chưa được xử lý.

<details>
<summary>Xem đáp án</summary>

Thêm nhánh cho `Season::Winter`:

```rust
enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

fn describe(season: Season) -> &'static str {
    match season {
        Season::Spring => "ấm áp",
        Season::Summer => "nóng",
        Season::Autumn => "mát",
        Season::Winter => "lạnh",
    }
}

fn main() {
    println!("Xuân: {}", describe(Season::Spring));
    println!("Hạ: {}", describe(Season::Summer));
    println!("Thu: {}", describe(Season::Autumn));
    println!("Đông: {}", describe(Season::Winter));
}
```

Cũng có thể sửa bằng nhánh `_ => "lạnh"`, nhưng nhánh liệt kê rõ `Season::Winter` tốt hơn: sau này thêm mùa mới, compiler vẫn nhắc bạn.

</details>

#### Bài 1.3. Option cơ bản

**Đề:** cho `nickname: Option<&str> = Some("Bi")` và `no_nickname: Option<&str> = None`. In lời chào dùng biệt danh nếu có, ngược lại gọi là `"bạn"`.

**Kết quả mong đợi:**

```text
Chào Bi
Chào bạn
```

**Gợi ý:** `unwrap_or` cho giá trị mặc định khi là `None`.

<details>
<summary>Xem đáp án</summary>

**Cách 1:** `unwrap_or`.

```rust
fn main() {
    let nickname: Option<&str> = Some("Bi");
    let no_nickname: Option<&str> = None;

    println!("Chào {}", nickname.unwrap_or("bạn"));
    println!("Chào {}", no_nickname.unwrap_or("bạn"));
}
```

**Cách 2:** `match`.

```rust
fn greet(name: Option<&str>) {
    match name {
        Some(n) => println!("Chào {n}"),
        None => println!("Chào bạn"),
    }
}

fn main() {
    greet(Some("Bi"));
    greet(None);
}
```

Cả hai cho cùng kết quả. `unwrap_or` ngắn hơn khi chỉ cần một giá trị mặc định.

</details>

### Cấp độ 2. Vận dụng

#### Bài 2.1. Diện tích hình

**Đề:** định nghĩa enum `Shape` gồm `Circle(f64)` (bán kính), `Rectangle { width: f64, height: f64 }` và `Square(f64)` (cạnh), có `#[derive(Debug)]`. Viết phương thức `area(&self) -> f64` và in diện tích của ba hình `Circle(2.0)`, `Rectangle { width: 3.0, height: 4.0 }`, `Square(5.0)`. Mỗi dòng theo mẫu `{:?}: diện tích {:.2}` (tên hình dạng Debug, diện tích làm tròn hai chữ số thập phân).

**Kết quả mong đợi:**

```text
Circle(2.0): diện tích 12.57
Rectangle { width: 3.0, height: 4.0 }: diện tích 12.00
Square(5.0): diện tích 25.00
```

**Gợi ý:** số π là `std::f64::consts::PI`. Trong `match self`, các biến lấy từ tham chiếu có kiểu `&f64`; dấu `*` lấy giá trị bên trong.

<details>
<summary>Xem đáp án</summary>

```rust
use std::f64::consts::PI;

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Square(f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Square(side) => side * side,
        }
    }
}

fn main() {
    let shapes = [
        Shape::Circle(2.0),
        Shape::Rectangle {
            width: 3.0,
            height: 4.0,
        },
        Shape::Square(5.0),
    ];

    for shape in &shapes {
        println!("{:?}: diện tích {:.2}", shape, shape.area());
    }
}
```

Giải thích: thư viện chuẩn có sẵn phép nhân giữa `f64` và `&f64`, nên `PI * radius * radius` chạy mà không cần `*`. Trường hợp cần chính giá trị (ví dụ `Celsius(c) => *c` ở Bài 2.4) thì dùng `*`.

</details>

#### Bài 2.2. Số chẵn đầu tiên

**Đề:** viết hàm `first_even(items: &[i32]) -> Option<i32>` trả về số chẵn đầu tiên trong dãy, hoặc `None` nếu không có. Thử với `[3, 5, 4, 8]` và `[1, 3, 5]`, dùng `match` để in `Số chẵn đầu tiên: <n>` hoặc `Không có số chẵn nào`.

**Kết quả mong đợi:**

```text
Số chẵn đầu tiên: 4
Không có số chẵn nào
```

**Gợi ý:** `for &item in items` lấy từng phần tử `i32` (pattern `&item` tách tham chiếu). Gặp số chẵn thì `return Some(item)`; hết vòng lặp thì `None`.

<details>
<summary>Xem đáp án</summary>

```rust
fn first_even(items: &[i32]) -> Option<i32> {
    for &item in items {
        if item % 2 == 0 {
            return Some(item);
        }
    }
    None
}

fn main() {
    let a = [3, 5, 4, 8];
    let b = [1, 3, 5];

    match first_even(&a) {
        Some(n) => println!("Số chẵn đầu tiên: {n}"),
        None => println!("Không có số chẵn nào"),
    }

    match first_even(&b) {
        Some(n) => println!("Số chẵn đầu tiên: {n}"),
        None => println!("Không có số chẵn nào"),
    }
}
```

</details>

#### Bài 2.3. Xếp loại điểm

**Đề:** viết hàm `grade(score: u32) -> &'static str` theo bảng: `90..=100` là `"Giỏi"`, `80..=89` là `"Khá"`, `50..=79` là `"Trung bình"`, `0..=49` là `"Yếu"`, còn lại là `"Không hợp lệ"`. Thử với `95, 85, 60, 30, 101`, mỗi dòng in theo mẫu `điểm: xếp loại`.

**Kết quả mong đợi:**

```text
95: Giỏi
85: Khá
60: Trung bình
30: Yếu
101: Không hợp lệ
```

**Gợi ý:** dùng pattern khoảng `a..=b` và nhánh `_` cuối cùng.

<details>
<summary>Xem đáp án</summary>

```rust
fn grade(score: u32) -> &'static str {
    match score {
        90..=100 => "Giỏi",
        80..=89 => "Khá",
        50..=79 => "Trung bình",
        0..=49 => "Yếu",
        _ => "Không hợp lệ",
    }
}

fn main() {
    for score in [95, 85, 60, 30, 101] {
        println!("{score}: {}", grade(score));
    }
}
```

Giải thích: `101` không rơi vào khoảng nào nên vào nhánh `_`. Với `u32`, không cần xét số âm vì kiểu không dấu đã loại chúng.

</details>

#### Bài 2.4. Enum có phương thức

**Đề:** định nghĩa enum `Temperature` gồm `Celsius(f64)` và `Fahrenheit(f64)`. Viết phương thức `to_celsius(&self) -> f64` (công thức `C = (F - 32) × 5 / 9`) rồi in hai nhiệt độ `Celsius(36.6)` và `Fahrenheit(98.6)` quy về độ C, mỗi dòng theo mẫu `<số> độ C` với hai chữ số thập phân.

**Kết quả mong đợi:**

```text
36.60 độ C
37.00 độ C
```

**Gợi ý:** trong `match self`, biến lấy ra có kiểu `&f64`. Với nhánh `Celsius(c)` cần trả `f64` nên viết `*c`.

<details>
<summary>Xem đáp án</summary>

```rust
enum Temperature {
    Celsius(f64),
    Fahrenheit(f64),
}

impl Temperature {
    fn to_celsius(&self) -> f64 {
        match self {
            Temperature::Celsius(c) => *c,
            Temperature::Fahrenheit(f) => (f - 32.0) * 5.0 / 9.0,
        }
    }
}

fn main() {
    let readings = [Temperature::Celsius(36.6), Temperature::Fahrenheit(98.6)];

    for reading in &readings {
        println!("{:.2} độ C", reading.to_celsius());
    }
}
```

</details>

#### Bài 2.5. let else

**Đề:** viết hàm `double(text: &str) -> String`. Nếu `text` parse được thành `i32` thì trả về `"<text> gấp đôi là <kết quả>"`, ngược lại trả về `"<text>: không phải số"`. Dùng `let else`. Thử với `"21"` và `"abc"`.

**Kết quả mong đợi:**

```text
21 gấp đôi là 42
abc: không phải số
```

**Gợi ý:** `let Ok(n) = text.parse::<i32>() else { return ...; };`

<details>
<summary>Xem đáp án</summary>

```rust
fn double(text: &str) -> String {
    let Ok(n) = text.parse::<i32>() else {
        return format!("{text}: không phải số");
    };

    format!("{text} gấp đôi là {}", n * 2)
}

fn main() {
    println!("{}", double("21"));
    println!("{}", double("abc"));
}
```

Giải thích: nếu `parse` trả về `Ok(n)` thì `n` dùng được ngay ở các dòng sau; nếu `Err` thì khối `else` chạy và `return` thoát hàm.

</details>

### Cấp độ 3. Thử thách

#### Bài 3.1. Máy tính nhỏ

**Đề:** định nghĩa enum `Op` gồm `Add`, `Sub`, `Mul`, `Div` (có `Clone, Copy`). Viết hai phương thức: `symbol(self) -> char` (ký tự `+ - * /`) và `apply(self, a: f64, b: f64) -> Option<f64>` (phép chia cho `0` trả về `None`). Với `a = 12.0`, thử lần lượt `b` bằng `5.0` cho cả bốn phép, rồi thử thêm `12 / 0`. Mỗi dòng theo mẫu `a <ký hiệu> b = <kết quả>`, hoặc `không tính được` khi là `None`.

**Kết quả mong đợi:**

```text
12 + 5 = 17
12 - 5 = 7
12 * 5 = 60
12 / 5 = 2.4
12 / 0 = không tính được
```

**Gợi ý:** trong `apply`, đặt nhánh `Op::Div if b == 0.0 => None` **trước** nhánh `Op::Div` thường. Dùng `for (op, b) in cases` để tách tuple.

<details>
<summary>Xem đáp án</summary>

```rust
#[derive(Clone, Copy)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Op {
    fn symbol(self) -> char {
        match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        }
    }

    fn apply(self, a: f64, b: f64) -> Option<f64> {
        match self {
            Op::Add => Some(a + b),
            Op::Sub => Some(a - b),
            Op::Mul => Some(a * b),
            Op::Div if b == 0.0 => None,
            Op::Div => Some(a / b),
        }
    }
}

fn main() {
    let a = 12.0;
    let cases = [
        (Op::Add, 5.0),
        (Op::Sub, 5.0),
        (Op::Mul, 5.0),
        (Op::Div, 5.0),
        (Op::Div, 0.0),
    ];

    for (op, b) in cases {
        match op.apply(a, b) {
            Some(result) => println!("{a} {} {b} = {result}", op.symbol()),
            None => println!("{a} {} {b} = không tính được", op.symbol()),
        }
    }
}
```

Giải thích: `Op` có `Copy` nên `op` dùng được hai lần (`apply` rồi `symbol`) mà không bị move. Với `{}`, số thực `17.0` được in là `17`.

</details>

#### Bài 3.2. Bốn đoạn mã, bốn lỗi

**Đề:** mỗi đoạn mã dưới đây đều không biên dịch được. Với từng đoạn, hãy **dự đoán lỗi** trước khi chạy `cargo check`, rồi sửa.

**Đoạn (a):**

```rust
enum Status {
    Active,
    Inactive,
    Banned,
}

fn label(status: Status) -> &'static str {
    match status { // ❌
        Status::Active => "hoạt động",
        Status::Inactive => "tạm ngưng",
    }
}

fn main() {
    println!("{}", label(Status::Active));
    println!("{}", label(Status::Inactive));
    println!("{}", label(Status::Banned));
}
```

**Đoạn (b):**

```rust
fn main() {
    let n: Option<i32> = Some(3);
    let m = n + 1; // ❌
    println!("{:?}", m);
}
```

**Đoạn (c):**

```rust
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

fn area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(w) => w, // ❌
    }
}

fn main() {
    println!("{}", area(Shape::Circle(1.0)));
    println!("{}", area(Shape::Rectangle(2.0, 3.0)));
}
```

**Đoạn (d):**

```rust
fn main() {
    let name: Option<String> = Some(String::from("An"));

    if let Some(n) = name {
        println!("Xin chào {n}");
    }

    println!("{:?}", name); // ❌
}
```

<details>
<summary>Xem đáp án</summary>

| Đoạn | Lỗi | Nguyên nhân | Cách sửa |
|---|---|---|---|
| (a) | `E0004` non-exhaustive patterns | `match` thiếu biến thể `Banned` | Thêm `Status::Banned => "bị cấm"` |
| (b) | `E0369` cannot add `{integer}` to `Option<i32>` | `Option<i32>` không cộng được với số | Lấy giá trị ra trước: `n.map(\|x\| x + 1)`, hoặc `match`/`unwrap_or` |
| (c) | `E0023` this pattern has 1 field, but the corresponding tuple variant has 2 fields | Pattern `Rectangle(w)` thiếu một trường | `Shape::Rectangle(w, h) => w * h` |
| (d) | `E0382` borrow of partially moved value | `if let Some(n) = name` chuyển `String` vào `n` | `if let Some(n) = &name` để chỉ mượn |

</details>

#### Bài 3.3. Phân tích lệnh

**Đề:** định nghĩa enum `Command` gồm `Go(u32)`, `Say(String)`, `Stop`. Viết hàm `parse(input: &str) -> Option<Command>` hiểu các lệnh `"go <số>"`, `"say <nội dung>"` và `"stop"`; mọi trường hợp khác (kể cả `"go abc"`) trả về `None`. Thử với `"go 5"`, `"say hello world"`, `"stop"`, `"go abc"`, `"jump"`. Mỗi dòng gồm chuỗi nhập (in bằng `{:?}`), mũi tên `->`, rồi mô tả: `đi <n> bước`, `nói: <nội dung>`, `dừng` hoặc `lệnh không hợp lệ`.

**Kết quả mong đợi:**

```text
"go 5" -> đi 5 bước
"say hello world" -> nói: hello world
"stop" -> dừng
"go abc" -> lệnh không hợp lệ
"jump" -> lệnh không hợp lệ
```

**Gợi ý:** `input.split_once(' ')` trả về `Option<(&str, &str)>`, tách chuỗi ở dấu cách đầu tiên. Có thể khớp thẳng pattern như `Some(("go", steps))`. Để đổi `steps` thành số: `steps.parse().ok()` cho `Option<u32>`, rồi `.map(Command::Go)`.

<details>
<summary>Xem đáp án</summary>

```rust
enum Command {
    Go(u32),
    Say(String),
    Stop,
}

fn parse(input: &str) -> Option<Command> {
    match input.split_once(' ') {
        None if input == "stop" => Some(Command::Stop),
        Some(("go", steps)) => steps.parse().ok().map(Command::Go),
        Some(("say", text)) => Some(Command::Say(text.to_string())),
        _ => None,
    }
}

fn main() {
    for input in ["go 5", "say hello world", "stop", "go abc", "jump"] {
        match parse(input) {
            Some(Command::Go(steps)) => println!("{input:?} -> đi {steps} bước"),
            Some(Command::Say(text)) => println!("{input:?} -> nói: {text}"),
            Some(Command::Stop) => println!("{input:?} -> dừng"),
            None => println!("{input:?} -> lệnh không hợp lệ"),
        }
    }
}
```

Giải thích:

- `split_once(' ')` cho `None` với `"stop"` và `"jump"` (không có dấu cách), nên nhánh `None if input == "stop"` chỉ bắt đúng `"stop"`.
- `Some(("go", steps))` là pattern lồng nhau: khớp `Some` chứa một tuple có phần tử đầu đúng bằng `"go"`.
- `steps.parse().ok()` đổi `Result` thành `Option`: `"abc"` cho `None`, nên `map` bỏ qua và kết quả là `None`.
- `Command::Go` dùng như một hàm `u32 -> Command`, nên `.map(Command::Go)` gọn hơn `.map(|n| Command::Go(n))`.

</details>

---

## 11. Tóm tắt

| Chủ đề | Ghi nhớ |
|---|---|
| `enum` | Kiểu có giá trị là **một trong** các biến thể; viết `TênEnum::TênBiếnThể` |
| Biến thể mang dữ liệu | Ba dạng: không dữ liệu `Quit`, kiểu tuple `Write(String)`, kiểu struct `Move { x, y }` |
| `impl` cho enum | Gắn phương thức vào enum; `Self` là tên ngắn của enum |
| Không có `null` | Dùng `Option<T>`: `Some(T)` hoặc `None`; `Option<i32>` khác `i32` |
| Lấy giá trị từ `Option` | `match`, `if let`, `unwrap_or`, `map`; tránh `unwrap()` trong chương trình thật |
| `match` | Là biểu thức; nhánh khớp đầu tiên thắng; **phải đầy đủ** (`E0004`) |
| Pattern | Giá trị, `\|`, khoảng `a..=b`, `_`, guard `if`, `n @ ...`, tách tuple/struct/enum, `..` |
| `if let` | `match` một nhánh; không kiểm tra tính đầy đủ |
| `let else` | Lấy giá trị ra hoặc **thoát sớm**; khối `else` phải `return`/`continue`/`break`/`panic!` |
| `while let` | Lặp khi pattern còn khớp, hay đi với `next()` |
| `matches!` | Trả về `bool` cho một pattern |
| `Result<T, E>` | `Ok(T)` hoặc `Err(E)`; có lý do khi thất bại; học kỹ ở bài error_handling |
| Quyền sở hữu | `match name` có thể move dữ liệu; dùng `match &name` để chỉ mượn |
| Quy ước | Enum và biến thể `UpperCamelCase`; phương thức `snake_case` |

---

## 12. Tài liệu tham khảo

- [The Rust Programming Language, 6.1 Defining an Enum](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [The Rust Programming Language, 6.2 The match Control Flow Construct](https://doc.rust-lang.org/book/ch06-02-match.html)
- [The Rust Programming Language, 6.3 Concise Control Flow with if let and let...else](https://doc.rust-lang.org/book/ch06-03-if-let.html)
- [The Rust Programming Language, 19.1 All the Places Patterns Can Be Used](https://doc.rust-lang.org/book/ch19-01-all-the-places-for-patterns.html)
- [The Rust Programming Language, 19.3 Pattern Syntax](https://doc.rust-lang.org/book/ch19-03-pattern-syntax.html)
- [The Rust Programming Language, 9.2 Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [Rust By Example, Enums](https://doc.rust-lang.org/rust-by-example/custom_types/enum.html)
- [Rust By Example, match](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
- [The Rust Reference, Patterns](https://doc.rust-lang.org/reference/patterns.html)
- [Tài liệu thư viện chuẩn, `Option`](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Tài liệu thư viện chuẩn, `Result`](https://doc.rust-lang.org/std/result/enum.Result.html)
- [Tài liệu thư viện chuẩn, macro `matches!`](https://doc.rust-lang.org/std/macro.matches.html)
- [Tra cứu mã lỗi, E0004 (match không đầy đủ)](https://doc.rust-lang.org/error_codes/E0004.html)
- [Tra cứu mã lỗi, E0382 (dùng giá trị đã bị move)](https://doc.rust-lang.org/error_codes/E0382.html)
