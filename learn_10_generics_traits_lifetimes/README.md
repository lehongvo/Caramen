# Generics, Trait và Lifetime trong Rust

> Tài liệu học tập về generics (kiểu tổng quát), trait và lifetime trong Rust: khái niệm, cú pháp, thực hành và bài tập có đáp án.

| Thông tin | Chi tiết |
|---|---|
| Chủ đề | Generics, trait, trait bound, `derive`, trait object (`dyn`), lifetime |
| Môi trường | Rust 1.99.1 (stable), Cargo 1.99.1, edition 2024 |
| Dự án | `learn_10_generics_traits_lifetimes` (thư mục này) |
| Bài trước | [error_handling](../learn_09_error_handling/README.md) |
| Nguồn tham khảo chính | *The Rust Programming Language*, chương 10 và mục 18.2 (trait object) |

## Mục lục

1. [Mục tiêu bài học](#1-mục-tiêu-bài-học)
2. [Khái niệm](#2-khái-niệm)
3. [Generics](#3-generics)
4. [Trait](#4-trait)
5. [Lifetime](#5-lifetime)
6. [Thực hành](#6-thực-hành)
7. [Tìm hiểu chi tiết](#7-tìm-hiểu-chi-tiết)
8. [Lỗi thường gặp](#8-lỗi-thường-gặp)
9. [Bài tập](#9-bài-tập)
10. [Tóm tắt](#10-tóm-tắt)
11. [Tài liệu tham khảo](#11-tài-liệu-tham-khảo)

---

## 1. Mục tiêu bài học

Sau bài học này bạn có thể:

- [ ] Viết hàm, `struct`, `enum` và khối `impl` generic với `<T>`.
- [ ] Giải thích vì sao generics không làm chương trình chậm đi (monomorphization).
- [ ] Định nghĩa `trait`, triển khai bằng `impl Trait for Type` và viết phương thức có sẵn nội dung.
- [ ] Dùng trait bound (`T: Trait`, `where`, `impl Trait`) để nói rõ kiểu tổng quát cần làm được gì.
- [ ] Dùng `#[derive]` cho các trait thường gặp: `Debug`, `Clone`, `Copy`, `PartialEq`, `Default`.
- [ ] Phân biệt generics (quyết định lúc biên dịch) với `dyn Trait` (quyết định lúc chạy).
- [ ] Giải thích lifetime là gì, đọc và viết chú thích `'a`, áp dụng ba quy tắc lược bỏ lifetime.
- [ ] Đọc hiểu và sửa các lỗi biên dịch phổ biến: `E0369`, `E0277`, `E0106`, `E0597`...

**Cách dùng tài liệu.** Mỗi ví dụ là một chương trình hoàn chỉnh (có `fn main`). Chép từng ví dụ vào `src/main.rs` rồi chạy:

```bash
cd learn_10_generics_traits_lifetimes
cargo run
```

Các ví dụ được đánh dấu như sau:

- Ví dụ có chú thích `// ❌` **không biên dịch được** (cố ý, để minh hoạ lỗi).
- Ví dụ có chú thích `// ⚠️` biên dịch được nhưng sinh cảnh báo.
- Các ví dụ còn lại biên dịch sạch và cho đúng kết quả ghi bên dưới.

**Liên hệ với các bài trước.** Bài này dùng lại nhiều thứ bạn đã gặp:

- `Option<T>` và `Result<T, E>` (bài [error_handling](../learn_09_error_handling/README.md)) chính là hai `enum` generic. Hôm nay bạn sẽ hiểu chữ `<T>` trong đó nghĩa là gì.
- Tham chiếu `&`, mượn (borrow) và việc giá trị bị huỷ khi ra khỏi phạm vi (bài [ownership](../learn_05_ownership/README.md)) là nền tảng của lifetime.
- Ở bài [structs](../learn_06_structs/README.md) ta đã lưu ý: trường của `struct` nên dùng `String` thay vì `&str`. Lý do nằm ở mục [5.6](#56-struct-chứa-tham-chiếu) của bài này.

---

## 2. Khái niệm

### 2.1 Ba công cụ, ba câu hỏi

Ba chủ đề của bài này đều xoay quanh một mục tiêu: **viết code một lần, dùng cho nhiều trường hợp mà vẫn an toàn**. Mỗi công cụ trả lời một câu hỏi khác nhau:

| Công cụ | Câu hỏi nó trả lời | Dấu hiệu nhận biết | Ví dụ quen thuộc |
|---|---|---|---|
| **Generics** | Code này dùng được cho **kiểu nào**? | `<T>` | `Vec<T>`, `Option<T>` |
| **Trait** | Một kiểu **làm được gì**? | `trait`, `impl ... for ...`, `T: Trait` | `Display`, `Clone` |
| **Lifetime** | Một tham chiếu **hợp lệ trong bao lâu**? | `'a` | `&'a str` |

Ví dụ sau dùng cả ba trong một chữ ký hàm:

```rust
use std::fmt::Display;

// <'a, T: Display> gom cả ba ý tưởng của bài học:
//   T         generics: một kiểu bất kỳ
//   : Display trait bound: kiểu đó phải in ra được
//   'a        lifetime: kết quả không sống lâu hơn x và y
fn longest_with_note<'a, T: Display>(x: &'a str, y: &'a str, note: T) -> &'a str {
    println!("Ghi chú: {note}");
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let result = longest_with_note("Rust", "Cargo", 2024);
    println!("Dài nhất: {result}");
}
```

**Kết quả:**

```text
Ghi chú: 2024
Dài nhất: Cargo
```

Cách đọc `fn longest_with_note<'a, T: Display>(x: &'a str, y: &'a str, note: T) -> &'a str`:

- `'a`: một lifetime có tên là `a`. Lifetime luôn được khai báo **trước** tham số kiểu trong `<...>`.
- `T: Display`: `T` là kiểu nào cũng được, miễn là in ra được bằng `{}`.
- `&'a str`: tham chiếu tới chuỗi, hợp lệ trong lifetime `'a`. Kết quả trả về cũng thuộc `'a`, nghĩa là **không sống lâu hơn** `x` và `y`.

Đừng lo nếu chưa hiểu hết. Các mục sau tách từng phần ra.

### 2.2 Vì sao cần

| Công cụ | Không có nó thì sao | Có nó thì được gì |
|---|---|---|
| Generics | Viết lại cùng một hàm cho `i32`, `f64`, `char`... | Một hàm duy nhất cho mọi kiểu phù hợp |
| Trait | Không có cách chung để nói "kiểu này in được, so sánh được, vẽ được" | Quy ước hành vi chung, code dùng được cho mọi kiểu có hành vi đó |
| Lifetime | Tham chiếu có thể trỏ vào vùng nhớ đã bị huỷ (lỗi *dangling reference*), thường chỉ phát hiện lúc chạy | Compiler chặn lỗi này ngay lúc biên dịch, không cần bộ gom rác |

Cả ba đều được kiểm tra **lúc biên dịch**. Chương trình đã biên dịch thì không còn các lỗi tương ứng.

### 2.3 Bảng ký hiệu

| Ký hiệu | Đọc là | Ví dụ |
|---|---|---|
| `<T>` | Kiểu tổng quát tên `T` | `fn largest<T>(list: &[T])` |
| `T: Display` | `T` phải có trait `Display` (trait bound) | `fn show<T: Display>(x: T)` |
| `where T: Display` | Cách viết bound khác, dễ đọc khi có nhiều bound | `fn f<T>(x: T) where T: Display` |
| `impl Trait` | "Một kiểu nào đó có trait này" | `fn f(x: &impl Display)` |
| `dyn Trait` | Trait object: kiểu cụ thể chỉ biết lúc chạy | `Box<dyn Display>` |
| `'a` | Tên một lifetime | `&'a str` |
| `'static` | Lifetime kéo dài suốt chương trình | `&'static str` |

Quy ước đặt tên: tham số kiểu thường là một chữ in hoa (`T` là *Type*, `U`, `V` là các kiểu tiếp theo, `E` là *Error*). Lifetime là chữ thường ngắn: `'a`, `'b`.

---

## 3. Generics

### 3.1 Vấn đề: lặp code

Hai hàm dưới đây tìm phần tử lớn nhất trong một slice. Thân hàm **giống hệt nhau**, chỉ khác kiểu:

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    println!("{}", largest_i32(&[34, 50, 25, 100, 65]));
    println!("{}", largest_char(&['y', 'm', 'a', 'q']));
}
```

**Kết quả:**

```text
100
y
```

Nếu cần thêm `f64`, `u8`, `String`... ta phải chép thêm hàm nữa. Generics giải quyết đúng chuyện này.

### 3.2 Hàm generic

Khai báo tham số kiểu trong `<...>` ngay sau tên hàm, rồi dùng nó như một kiểu bình thường:

```text
fn largest<T>(list: &[T]) -> &T
```

Đọc: "`largest` làm việc với kiểu `T` nào đó; nhận một slice các `T` và trả về tham chiếu tới một `T`".

Thử viết thân hàm giống hai hàm ở trên:

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { // ❌ chưa biết T có so sánh được không
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = [34, 50, 25, 100, 65];
    println!("Số lớn nhất: {}", largest(&numbers));
}
```

**Trình biên dịch báo:**

```text
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:4:17
  |
4 |         if item > largest { // ❌ chưa biết T có so sánh được không
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T` with trait `PartialOrd`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++
```

Lỗi cho biết compiler **chưa biết `T` có so sánh được bằng `>` hay không**. `T` có thể là số, nhưng cũng có thể là một kiểu không có thứ tự. Rust kiểm tra thân hàm generic **một lần duy nhất** dựa trên những gì ta cam kết về `T`, chứ không đợi đến lúc gọi hàm. Dòng `help:` gợi ý đúng cách sửa: thêm **trait bound** `T: PartialOrd` (mục [4.4](#44-trait-làm-tham-số-và-trait-bound)).

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = [34, 50, 25, 100, 65];
    let letters = ['y', 'm', 'a', 'q'];
    let words = ["táo", "cam", "bưởi"];

    println!("Số lớn nhất: {}", largest(&numbers));
    println!("Ký tự lớn nhất: {}", largest(&letters));
    println!("Chuỗi lớn nhất: {}", largest(&words));
}
```

**Kết quả:**

```text
Số lớn nhất: 100
Ký tự lớn nhất: y
Chuỗi lớn nhất: táo
```

Lưu ý:

- `PartialOrd` là trait của các kiểu so sánh được thứ tự (`<`, `>`, `<=`, `>=`). Số, `char` và `&str` đều có.
- Hàm trả về `&T` (tham chiếu) nên không cần sao chép phần tử. Mục [8.2](#82-lỗi-biên-dịch) giải thích vì sao trả về `T` sẽ lỗi.
- Slice rỗng làm `list[0]` panic. Trong code thật, nên kiểm tra `list.is_empty()` hoặc trả về `Option<&T>`.

### 3.3 Struct generic

`struct` cũng có thể có tham số kiểu:

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.5, y: 4.0 };

    println!("x = {}, y = {}", integer.x, integer.y);
    println!("{:?}", float);
}
```

**Kết quả:**

```text
x = 5, y = 10
Point { x: 1.5, y: 4.0 }
```

`Point<T>` dùng **cùng một** `T` cho cả `x` và `y`. Khi `x` là số nguyên thì `y` cũng phải là số nguyên:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let mixed = Point { x: 5, y: 4.0 }; // ❌ x là số nguyên, y là số thực
}
```

**Trình biên dịch báo:**

```text
error[E0308]: mismatched types
 --> src/main.rs:7:34
  |
7 |     let mixed = Point { x: 5, y: 4.0 }; // ❌ x là số nguyên, y là số thực
  |                                  ^^^ expected integer, found floating-point number
```

Thông báo cho biết: `T` đã được chốt là số nguyên từ `x: 5`, nên `y: 4.0` (số thực) sai kiểu. Muốn hai trường khác kiểu, khai báo hai tham số:

```rust
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let mixed = Point { x: 5, y: 4.0 };
    let text = Point { x: "một", y: 'z' };

    println!("x = {}, y = {:?}", mixed.x, mixed.y);
    println!("{:?}", text);
}
```

**Kết quả:**

```text
x = 5, y = 4.0
Point { x: "một", y: 'z' }
```

### 3.4 Enum generic

`enum` cũng generic được. Hai `enum` bạn dùng hằng ngày trong thư viện chuẩn được định nghĩa như sau:

```text
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`Option<T>` mô tả "có một `T` hoặc không có gì". `Result<T, E>` mô tả "thành công với `T` hoặc thất bại với `E`". Ta cũng tự viết được:

```rust
#[derive(Debug)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

fn main() {
    let a: Either<i32, String> = Either::Left(5);
    let b: Either<i32, String> = Either::Right(String::from("năm"));
    println!("{:?} {:?}", a, b);

    let c: Option<i32> = Some(5);
    let d: Result<u32, String> = Ok(7);
    println!("{:?} {:?}", c, d);
}
```

**Kết quả:**

```text
Left(5) Right("năm")
Some(5) Ok(7)
```

### 3.5 Phương thức generic

Khối `impl` cho kiểu generic phải khai báo lại `T`: viết `impl<T> Point<T>`. Ta cũng có thể viết khối `impl` **chỉ cho một kiểu cụ thể**, ví dụ `impl Point<f64>`:

```rust
struct Point<T> { x: T, y: T }

impl<T> Point<T> {
    fn x(&self) -> &T { &self.x }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.0, y: 4.0 };
    println!("x = {}", p.x());
    println!("Khoảng cách đến gốc: {:.1}", p.distance_from_origin());

    let q = Point { x: 5, y: 10 };
    println!("x = {}", q.x());
}
```

**Kết quả:**

```text
x = 3
Khoảng cách đến gốc: 5.0
x = 5
```

- `impl<T> Point<T>`: phương thức `x` có cho **mọi** `Point<T>`.
- `impl Point<f64>`: phương thức `distance_from_origin` chỉ có cho `Point<f64>`, vì chỉ số thực mới có `powi` và `sqrt`.

Gọi `distance_from_origin` trên `Point<i32>` báo `E0599`: `no method named 'distance_from_origin' found for struct 'Point<{integer}>'`, kèm ghi chú `the method was found for 'Point<f64>'`.

### 3.6 Monomorphization

Dùng generics có làm chương trình chậm không? **Không.** Lúc biên dịch, Rust tạo ra một bản code riêng cho mỗi kiểu cụ thể được dùng. Quá trình này gọi là *monomorphization*.

```text
// Bạn viết:
let a = Some(5);
let b = Some(2.5);

// Compiler tạo ra (chỉ là minh hoạ ý tưởng, không phải code thật):
enum Option_i32 { Some(i32), None }
enum Option_f64 { Some(f64), None }
let a = Option_i32::Some(5);
let b = Option_f64::Some(2.5);
```

Hàm `type_name` cho thấy mỗi lời gọi `describe` làm việc với một kiểu `T` cụ thể khác nhau (mỗi kiểu ứng với một bản riêng do compiler tạo ra):

```rust
fn describe<T>(_value: T) -> &'static str {
    std::any::type_name::<T>()
}

fn main() {
    println!("{}", describe(5));
    println!("{}", describe(2.5));
    println!("{}", describe("chữ"));
}
```

**Kết quả:**

```text
i32
f64
&str
```

Mỗi `Option<...>` là một kiểu riêng, có kích thước riêng:

```rust
fn main() {
    println!("Option<u8>:  {} byte", size_of::<Option<u8>>());
    println!("Option<u32>: {} byte", size_of::<Option<u32>>());
    println!("Option<u64>: {} byte", size_of::<Option<u64>>());
}
```

**Kết quả:**

```text
Option<u8>:  2 byte
Option<u32>: 8 byte
Option<u64>: 16 byte
```

Đánh đổi của cách làm này:

| | Tác động |
|---|---|
| Tốc độ chạy | Giống hệt việc tự viết từng hàm riêng (*zero-cost abstraction*) |
| Thời gian biên dịch | Lâu hơn một chút |
| Kích thước file chạy | Có thể lớn hơn khi dùng một hàm generic với rất nhiều kiểu |

---

## 4. Trait

### 4.1 Trait là gì

Trait định nghĩa **một nhóm hành vi chung** mà nhiều kiểu có thể cùng có. Nếu bạn từng dùng `interface` trong Java, TypeScript hay Go thì ý tưởng khá giống: trait là một "hợp đồng". Kiểu nào ký hợp đồng (`impl Trait for Type`) thì cam kết có đủ các phương thức trong đó.

Trait kết hợp với generics để viết code nói được kiểu: "hàm này nhận bất kỳ kiểu nào, miễn là kiểu đó làm được việc X".

### 4.2 Định nghĩa và triển khai trait

```text
trait TênTrait {
    fn phương_thức(&self) -> KiểuTrảVề;   // chỉ có chữ ký, kết thúc bằng ;
}

impl TênTrait for TênKiểu {
    fn phương_thức(&self) -> KiểuTrảVề { ... }
}
```

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article { title: String, author: String }
struct Tweet { username: String, content: String }

impl Summary for Article {
    fn summarize(&self) -> String { format!("{}, bởi {}", self.title, self.author) }
}

impl Summary for Tweet {
    fn summarize(&self) -> String { format!("@{}: {}", self.username, self.content) }
}

fn main() {
    let article = Article { title: String::from("Học Rust"), author: String::from("An") };
    let tweet = Tweet { username: String::from("rustlang"), content: String::from("Rust 2024 ra mắt!") };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}
```

**Kết quả:**

```text
Học Rust, bởi An
@rustlang: Rust 2024 ra mắt!
```

Hai kiểu `Article` và `Tweet` hoàn toàn khác nhau, nhưng cùng có phương thức `summarize` nhờ cùng triển khai `Summary`.

Thiếu một phương thức bắt buộc là lỗi `E0046`: `not all trait items implemented, missing: 'summarize'`.

### 4.3 Phương thức có sẵn nội dung

Trait có thể cung cấp **cài đặt mặc định** cho phương thức. Kiểu nào không muốn thì dùng luôn bản mặc định, muốn thì tự ghi đè:

```rust
trait Summary {
    fn author(&self) -> String;

    // Có sẵn nội dung: dùng luôn, hoặc tự ghi đè
    fn summarize(&self) -> String {
        format!("(Đọc thêm từ {}...)", self.author())
    }
}

struct Article { title: String, author: String }
struct Tweet { username: String }

impl Summary for Article {
    fn author(&self) -> String { self.author.clone() }
    fn summarize(&self) -> String { format!("{}, bởi {}", self.title, self.author) }
}

impl Summary for Tweet {
    fn author(&self) -> String { format!("@{}", self.username) }
}

fn main() {
    let article = Article { title: String::from("Học Rust"), author: String::from("An") };
    let tweet = Tweet { username: String::from("rustlang") };

    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}
```

**Kết quả:**

```text
Học Rust, bởi An
(Đọc thêm từ @rustlang...)
```

- `Article` ghi đè `summarize`.
- `Tweet` không viết `summarize` nên dùng bản mặc định, và bản đó gọi lại phương thức `author` mà `Tweet` đã cài đặt.

Phương thức mặc định được phép gọi các phương thức khác của cùng trait, kể cả phương thức chưa có nội dung.

### 4.4 Trait làm tham số và trait bound

Có hai cách viết hàm nhận "bất kỳ kiểu nào có trait `Summary`":

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article;
struct Tweet;

impl Summary for Article {
    fn summarize(&self) -> String { String::from("Bài báo") }
}

impl Summary for Tweet {
    fn summarize(&self) -> String { String::from("Tweet") }
}

// Cách 1: cú pháp ngắn
fn notify(item: &impl Summary) {
    println!("Tin mới! {}", item.summarize());
}

// Cách 2: trait bound, viết đầy đủ
fn notify_generic<T: Summary>(item: &T) {
    println!("Tin mới! {}", item.summarize());
}

fn main() {
    notify(&Article);
    notify_generic(&Tweet);
}
```

**Kết quả:**

```text
Tin mới! Bài báo
Tin mới! Tweet
```

| Cách viết | Tên gọi | Ghi chú |
|---|---|---|
| `item: &impl Summary` | Cú pháp `impl Trait` | Ngắn gọn, đủ cho hầu hết trường hợp |
| `<T: Summary>(item: &T)` | Trait bound | Đầy đủ, cần khi phải nhắc lại cùng một kiểu `T` ở nhiều chỗ (mục [7.1](#71-generic-impl-trait-hay-dyn-trait)) |

Khi cần **nhiều trait cùng lúc**, nối bằng dấu `+`. Khi chữ ký dài, chuyển bound xuống mệnh đề `where` cho dễ đọc:

```rust
use std::fmt::{Debug, Display};

// Nhiều bound nối bằng dấu +
fn show_larger<T: Display + PartialOrd>(a: T, b: T) {
    if a >= b {
        println!("Lớn hơn: {a}");
    } else {
        println!("Lớn hơn: {b}");
    }
}

// where giúp chữ ký dễ đọc khi có nhiều bound
fn describe_both<T, U>(t: &T, u: &U) -> String
where
    T: Display + Clone,
    U: Clone + Debug,
{
    format!("{t} / {u:?}")
}

fn main() {
    show_larger(3, 8);
    show_larger("cam", "bưởi");
    println!("{}", describe_both(&"Rust", &vec![1, 2]));
}
```

**Kết quả:**

```text
Lớn hơn: 8
Lớn hơn: cam
Rust / [1, 2]
```

### 4.5 Trả về impl Trait

`impl Trait` cũng dùng được ở vị trí kiểu trả về:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Tweet;

impl Summary for Tweet {
    fn summarize(&self) -> String { String::from("Tweet của @rustlang") }
}

// Trả về "một kiểu nào đó có Summary", không nói rõ là kiểu nào
fn make_summary() -> impl Summary {
    Tweet
}

fn main() {
    println!("{}", make_summary().summarize());
}
```

**Kết quả:**

```text
Tweet của @rustlang
```

Người gọi chỉ biết kết quả có `Summary`, không biết đó là `Tweet`. Điều kiện: hàm chỉ được trả về **một kiểu cụ thể duy nhất**. Nếu `if` trả về `Article` còn `else` trả về `Tweet`, compiler báo `E0308` (`'if' and 'else' have incompatible types`) và gợi ý đổi kiểu trả về thành `Box<dyn Summary>`, tức trait object ở mục [4.10](#410-dyn-trait).

### 4.6 Bound có điều kiện cho phương thức

Ta có thể viết khối `impl` chỉ áp dụng khi `T` thoả bound. `Pair<T>` nào cũng có `new`, nhưng `show_largest` chỉ có khi `T` vừa in được vừa so sánh được:

```rust
use std::fmt::Display;

struct Pair<T> { x: T, y: T }

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { Self { x, y } }
}

// Chỉ Pair<T> nào có T in được và so sánh được mới có phương thức này
impl<T: Display + PartialOrd> Pair<T> {
    fn show_largest(&self) {
        if self.x >= self.y {
            println!("Lớn nhất là x = {}", self.x);
        } else {
            println!("Lớn nhất là y = {}", self.y);
        }
    }
}

fn main() {
    Pair::new(3, 8).show_largest();
    Pair::new("cam", "bưởi").show_largest();
}
```

**Kết quả:**

```text
Lớn nhất là y = 8
Lớn nhất là x = cam
```

### 4.7 Quy tắc orphan

Bạn chỉ được triển khai một trait cho một kiểu khi **ít nhất một trong hai thuộc về crate của bạn**. Quy tắc này (*orphan rule*) ngăn hai thư viện khác nhau cùng triển khai một trait cho cùng một kiểu rồi xung đột.

| Trait | Kiểu | Được không |
|---|---|---|
| Của bạn | Của bạn | Được |
| Của bạn | Của thư viện chuẩn (vd `Vec<i32>`) | Được |
| Của thư viện chuẩn (vd `Display`) | Của bạn | Được |
| Của thư viện chuẩn | Của thư viện chuẩn | **Không** |

```rust
use std::fmt;

impl fmt::Display for Vec<i32> { // ❌ cả trait lẫn kiểu đều thuộc thư viện chuẩn
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} số", self.len())
    }
}

fn main() {}
```

**Trình biên dịch báo:**

```text
error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
 --> src/main.rs:3:1
  |
3 | impl fmt::Display for Vec<i32> { // ❌ cả trait lẫn kiểu đều thuộc thư viện chuẩn
  | ^^^^^^^^^^^^^^^^^^^^^^--------
  |                       |
  |                       `Vec` is not defined in the current crate
  |
  = note: impl doesn't have any local type before any uncovered type parameters
  = note: for more information see https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
  = note: define and implement a trait or new type instead
```

Cách vòng: bọc kiểu có sẵn trong một kiểu của riêng bạn (mẫu *newtype*):

```rust
use std::fmt;

// Kiểu mới của riêng ta, bọc lấy Vec<String>
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("một"), String::from("hai")]);
    println!("{w}");
}
```

**Kết quả:**

```text
[một, hai]
```

### 4.8 Tự động triển khai với derive

Một số trait rất thường gặp có cách cài đặt "chuẩn". Thay vì viết tay, thêm `#[derive(...)]` trước định nghĩa và compiler tự sinh code:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a; // Copy: a vẫn dùng được sau khi gán
    println!("{:?} {:?}", a, b);
    println!("a == b: {}", a == b);
    println!("Mặc định: {:?}", Point::default());
    println!("{:#?}", a.clone());
}
```

**Kết quả:**

```text
Point { x: 1, y: 2 } Point { x: 1, y: 2 }
a == b: true
Mặc định: Point { x: 0, y: 0 }
Point {
    x: 1,
    y: 2,
}
```

| Trait | Cho phép | Điều kiện để `derive` |
|---|---|---|
| `Debug` | In bằng `{:?}` và `{:#?}` | Mọi trường đều `Debug` |
| `Clone` | Gọi `.clone()` để tạo bản sao | Mọi trường đều `Clone` |
| `Copy` | Gán và truyền tham số bằng cách **sao chép** thay vì chuyển quyền sở hữu | Có `Clone`, mọi trường đều `Copy` |
| `PartialEq` | So sánh `==`, `!=` | Mọi trường đều `PartialEq` |
| `Eq` | Đánh dấu "so sánh bằng hoàn chỉnh" (`f64` chỉ có `PartialEq`, vì `NaN != NaN`) | Có `PartialEq`, mọi trường đều `Eq` |
| `PartialOrd` | So sánh `<`, `>`; với `derive`, so từng trường theo thứ tự khai báo | Có `PartialEq`, mọi trường đều `PartialOrd` |
| `Default` | `Kiểu::default()` cho giá trị mặc định (số là 0, chuỗi rỗng...) | Mọi trường đều `Default` |
| `Hash` | Dùng làm khoá của `HashMap`, `HashSet` (bài [collections](../learn_08_collections/README.md)) | Mọi trường đều `Hash` |

Thiếu `derive` thì dùng tính năng tương ứng sẽ lỗi. Dùng `{:?}` với kiểu chưa có `Debug`:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p); // ❌ Point chưa derive Debug
}
```

**Trình biên dịch báo:**

```text
error[E0277]: `Point` doesn't implement `Debug`
 --> src/main.rs:8:22
  |
8 |     println!("{:?}", p); // ❌ Point chưa derive Debug
  |               ----   ^ `Point` cannot be formatted using `{:?}` because it doesn't implement `Debug`
  |               |
  |               required by this formatting parameter
  |
  = help: the trait `Debug` is not implemented for `Point`
help: consider annotating `Point` with `#[derive(Debug)]`
  |
1 + #[derive(Debug)]
2 | struct Point {
  |
```

Tương tự, `==` trên kiểu chưa có `PartialEq` báo `E0369` (mục [8.2](#82-lỗi-biên-dịch)).

`Copy` nối với ownership. Nếu `Point` **không** có `Copy` thì `let b = a;` **chuyển** `a` đi, và dùng lại `a` báo `E0382` (`borrow of moved value`). Muốn `derive(Copy)`, mọi trường phải `Copy`. Một `String` thì không (nó sở hữu dữ liệu trên heap), và `derive(Copy)` trên `struct` có trường `String` báo `E0204`.

### 4.9 Trait thường gặp và Display

| Trait | Dùng để |
|---|---|
| `Display` | In ra cho người dùng bằng `{}` (phải tự viết, không `derive` được) |
| `Debug` | In ra cho lập trình viên bằng `{:?}` |
| `Clone`, `Copy` | Sao chép giá trị |
| `PartialEq`, `PartialOrd` | So sánh bằng và so sánh thứ tự |
| `Default` | Giá trị mặc định |
| `Iterator` | Duyệt qua một chuỗi giá trị (`for`) |
| `From`, `Into` | Chuyển đổi giữa các kiểu |
| `Drop` | Chạy code khi giá trị bị huỷ |

Tự cài đặt `Display` cho kiểu của mình:

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 3, y: -1 };
    println!("{p}");

    let text: String = p.to_string(); // to_string() có sẵn nhờ Display
    println!("{} ký tự", text.len());
}
```

**Kết quả:**

```text
(3, -1)
7 ký tự
```

Nhờ có `Display`, kiểu `Point` tự có thêm `to_string()`. Thư viện chuẩn khai báo một lần `impl<T: Display> ToString for T`, tức *mọi* kiểu có `Display` đều có `ToString`. Đây là ví dụ của trait bound áp dụng cho cả họ kiểu.

### 4.10 dyn Trait

Generics và `impl Trait` yêu cầu compiler biết **một kiểu cụ thể** ở mỗi chỗ dùng. Đôi khi ta cần một danh sách chứa **nhiều kiểu khác nhau** mà chỉ biết chúng đều có chung một trait. Khi đó dùng *trait object*, viết `dyn Trait` và đặt sau một con trỏ như `&dyn Trait` hoặc `Box<dyn Trait>`:

```rust
trait Animal {
    fn name(&self) -> String;
    fn speak(&self) -> String;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn name(&self) -> String { String::from("Chó") }
    fn speak(&self) -> String { String::from("Gâu gâu") }
}

impl Animal for Cat {
    fn name(&self) -> String { String::from("Mèo") }
    fn speak(&self) -> String { String::from("Meo meo") }
}

fn introduce(animal: &dyn Animal) {
    println!("{} kêu: {}", animal.name(), animal.speak());
}

fn main() {
    // Một Vec chứa nhiều kiểu khác nhau, miễn đều là Animal
    let zoo: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in &zoo {
        introduce(animal.as_ref());
    }
}
```

**Kết quả:**

```text
Chó kêu: Gâu gâu
Mèo kêu: Meo meo
```

- `dyn Animal` là kiểu **không biết trước kích thước** lúc biên dịch (mỗi kiểu cụ thể như `Dog`, `Cat` có kích thước riêng), nên không đặt trực tiếp vào `Vec` được. Ta đặt giá trị lên heap bằng `Box` và để `Vec` giữ các con trỏ, vì con trỏ luôn có kích thước cố định. (`Box` sẽ được học kỹ hơn ở bài sau.)
- `as_ref()` đổi `&Box<dyn Animal>` thành `&dyn Animal`.
- Thiếu từ khoá `dyn` (viết `Box<Animal>`) báo `E0782` (`expected a type, found a trait`), kèm gợi ý thêm `dyn`.

So sánh hai cách dùng trait:

| | Generics / `impl Trait` | `dyn Trait` |
|---|---|---|
| Tên gọi | Static dispatch (gọi tĩnh) | Dynamic dispatch (gọi động) |
| Chọn hàm cần gọi | Lúc biên dịch | Lúc chạy, qua vtable |
| Tốc độ | Nhanh nhất, có thể inline | Chậm hơn một chút (gọi gián tiếp) |
| Một danh sách nhiều kiểu | Không | **Có** |
| Kích thước file chạy | Có thể tăng (monomorphization) | Nhỏ hơn |

Quy tắc chọn: mặc định dùng generics. Chỉ dùng `dyn` khi thật sự cần trộn nhiều kiểu trong cùng một tập hợp hoặc cần quyết định kiểu lúc chạy.

---

## 5. Lifetime

### 5.1 Vấn đề: tham chiếu treo

Ở bài [ownership](../learn_05_ownership/README.md), bạn đã biết một giá trị bị huỷ khi biến sở hữu nó ra khỏi phạm vi. Nếu một tham chiếu vẫn trỏ vào giá trị đã huỷ, ta có **tham chiếu treo** (*dangling reference*), nguồn gốc của nhiều lỗi bộ nhớ nghiêm trọng ở C và C++. Rust chặn chúng bằng **borrow checker**, thành phần so sánh "thời gian sống" của giá trị với "thời gian sống" của các tham chiếu tới nó.

```rust
fn main() {
    let r;
    {
        let x = 5;
        r = &x; // ❌ x bị huỷ ở cuối khối này
    }
    println!("r: {r}");
}
```

**Trình biên dịch báo:**

```text
error[E0597]: `x` does not live long enough
 --> src/main.rs:5:13
  |
4 |         let x = 5;
  |             - binding `x` declared here
5 |         r = &x; // ❌ x bị huỷ ở cuối khối này
  |             ^^ borrowed value does not live long enough
6 |     }
  |     - `x` dropped here while still borrowed
7 |     println!("r: {r}");
  |                   - borrow later used here
```

Có thể hình dung bằng cách đánh dấu thời gian sống:

```text
fn main() {
    let r;                // ---------+-- 'a  (r sống ở đây)
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |      (x chỉ sống trong khối này)
        r = &x;           //  |       |
    }                     // -+       |      (x bị huỷ)
                          //          |
    println!("r: {r}");   //          |      (r dùng x đã huỷ)
}                         // ---------+
```

`r` sống lâu (`'a`), còn `x` mà nó mượn lại chỉ sống ngắn (`'b`). Tham chiếu không được sống lâu hơn thứ nó trỏ tới. Cách sửa là cho `x` sống ít nhất bằng `r`:

```rust
fn main() {
    let x = 5;
    let r = &x;
    println!("r: {r}");
}
```

**Kết quả:**

```text
r: 5
```

**Lifetime** chính là tên gọi cho "đoạn thời gian mà một tham chiếu hợp lệ". Phần lớn thời gian compiler tự suy ra được. Ta chỉ phải viết ra khi có nhiều tham chiếu và compiler không đoán được quan hệ giữa chúng.

### 5.2 Chú thích lifetime

Hàm `longest` trả về chuỗi dài hơn trong hai chuỗi. Viết ngây thơ sẽ bị từ chối:

```rust
fn longest(x: &str, y: &str) -> &str { // ❌ thiếu lifetime
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = String::from("abcd");
    let b = "xyz";
    println!("Dài nhất: {}", longest(a.as_str(), b));
}
```

**Trình biên dịch báo:**

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str { // ❌ thiếu lifetime
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
1 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { // ❌ thiếu lifetime
  |           ++++     ++          ++          ++
```

Thông báo nói rõ: kiểu trả về là tham chiếu, nhưng chữ ký **không cho biết nó mượn từ `x` hay từ `y`**. Compiler không thể kiểm tra thời gian sống nếu không biết điều đó. Ta nói cho nó biết bằng **chú thích lifetime**:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let a = String::from("abcd");
    let b = "xyz";
    println!("Dài nhất: {}", longest(a.as_str(), b));
}
```

**Kết quả:**

```text
Dài nhất: abcd
```

Cú pháp:

| Cú pháp | Ý nghĩa |
|---|---|
| `&i32` | Tham chiếu, lifetime do compiler tự suy ra |
| `&'a i32` | Tham chiếu có lifetime tên `'a` |
| `&'a mut i32` | Tham chiếu khả biến có lifetime tên `'a` |
| `fn longest<'a>(...)` | Khai báo tham số lifetime `'a`, giống khai báo `<T>` |

Điều quan trọng cần nhớ:

- **Chú thích lifetime không làm giá trị sống lâu hơn.** Nó chỉ *mô tả* quan hệ giữa các tham chiếu để compiler kiểm tra.
- Chữ ký `longest<'a>(x: &'a str, y: &'a str) -> &'a str` có nghĩa: "kết quả không được sống lâu hơn **tham số có thời gian sống ngắn hơn** trong `x` và `y`".
- Ta viết lifetime ở **chữ ký hàm**. Compiler chỉ cần chữ ký (không cần đọc thân hàm) để kiểm tra các nơi gọi hàm.

### 5.3 Lifetime và phạm vi sử dụng

Vì kết quả bị buộc không sống lâu hơn tham số ngắn nhất, ta chỉ được dùng nó trong phạm vi mà **cả hai** tham số còn sống:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("một chuỗi dài");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // ❌ string2 sống ngắn hơn result
    }
    println!("Dài nhất: {result}");
}
```

**Trình biên dịch báo:**

```text
error[E0597]: `string2` does not live long enough
  --> src/main.rs:10:44
   |
 9 |         let string2 = String::from("xyz");
   |             ------- binding `string2` declared here
10 |         result = longest(string1.as_str(), string2.as_str()); // ❌ string2 sống ngắn hơn result
   |                                            ^^^^^^^ borrowed value does not live long enough
11 |     }
   |     - `string2` dropped here while still borrowed
12 |     println!("Dài nhất: {result}");
   |                          ------ borrow later used here
```

`result` có thể trỏ vào `string2`, mà `string2` đã bị huỷ ở cuối khối trong. Dùng `result` ngay trong khối thì hợp lệ:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("một chuỗi dài");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("Dài nhất: {result}");
    }
}
```

**Kết quả:**

```text
Dài nhất: một chuỗi dài
```

Tham số nào **không liên quan** đến kết quả thì không cần dính vào `'a`. Ở ví dụ dưới, kết quả chỉ mượn từ `x`, nên `b` được huỷ sớm cũng không sao:

```rust
// y có lifetime riêng, không liên quan đến kết quả
fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn main() {
    let a = String::from("một");
    let result;
    {
        let b = String::from("hai");
        result = first(a.as_str(), b.as_str());
    }
    println!("{result}");
}
```

**Kết quả:**

```text
một
```

### 5.4 Không trả về tham chiếu tới biến cục bộ

Một lỗi lifetime kinh điển: trả về tham chiếu tới thứ được tạo ra ngay trong hàm.

```rust
fn greeting<'a>() -> &'a str {
    let text = String::from("xin chào");
    text.as_str() // ❌ trả về tham chiếu tới biến cục bộ sắp bị huỷ
}

fn main() {
    println!("{}", greeting());
}
```

**Trình biên dịch báo:**

```text
error[E0515]: cannot return value referencing local variable `text`
 --> src/main.rs:3:5
  |
3 |     text.as_str() // ❌ trả về tham chiếu tới biến cục bộ sắp bị huỷ
  |     ----^^^^^^^^^
  |     |
  |     returns a value referencing data owned by the current function
  |     `text` is borrowed here
```

`text` bị huỷ khi hàm kết thúc, nên tham chiếu trả về sẽ trỏ vào vùng nhớ đã huỷ. Không có chú thích lifetime nào cứu được trường hợp này, vì vấn đề không nằm ở chú thích. Cách sửa là **trả về chính giá trị** (chuyển quyền sở hữu cho người gọi):

```rust
fn greeting() -> String {
    String::from("xin chào") // trả về chính giá trị, quyền sở hữu chuyển cho người gọi
}

fn main() {
    println!("{}", greeting());
}
```

**Kết quả:**

```text
xin chào
```

### 5.5 Quy tắc lược bỏ lifetime

Bạn đã viết rất nhiều hàm nhận tham chiếu mà không cần `'a`. Đó là nhờ ba **quy tắc lược bỏ lifetime** (*lifetime elision*): compiler tự điền theo đúng quy tắc, và chỉ báo lỗi khi các quy tắc không đủ.

| Quy tắc | Nội dung |
|---|---|
| 1 | Mỗi tham số là tham chiếu nhận **một lifetime riêng** |
| 2 | Nếu có **đúng một** lifetime đầu vào, nó được gán cho **mọi** lifetime đầu ra |
| 3 | Nếu có `&self` hoặc `&mut self`, lifetime của `self` được gán cho mọi lifetime đầu ra |

```rust
fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[..i],
        None => s,
    }
}

fn main() {
    let sentence = String::from("hello wide world");
    println!("{}", first_word(&sentence));
}
```

**Kết quả:**

```text
hello
```

Compiler suy ra như sau:

```text
fn first_word(s: &str) -> &str

Quy tắc 1:  fn first_word<'a>(s: &'a str) -> &str
Quy tắc 2:  fn first_word<'a>(s: &'a str) -> &'a str     // xong, đủ thông tin
```

Với `longest` thì các quy tắc không đủ:

```text
fn longest(x: &str, y: &str) -> &str

Quy tắc 1:  fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str
Quy tắc 2:  không áp dụng được (có hai lifetime đầu vào)
Quy tắc 3:  không áp dụng được (không có &self)
=> lifetime của kết quả vẫn chưa xác định: lỗi E0106, ta phải tự chú thích
```

Chỉ cần **chú thích khi compiler yêu cầu**. Đừng viết `'a` vào mọi nơi.

### 5.6 Struct chứa tham chiếu

`struct` có thể chứa tham chiếu, nhưng phải khai báo lifetime cho nó:

```rust
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Gọi tôi là Ishmael. Nhiều năm trước...");
    let first_sentence = novel.split('.').next().expect("Không có dấu chấm");
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!("{}", excerpt.part);
}
```

**Kết quả:**

```text
Gọi tôi là Ishmael
```

`Excerpt<'a>` nghĩa là: một `Excerpt` **không được sống lâu hơn** chuỗi mà `part` đang mượn. Thiếu chú thích là lỗi:

```rust
struct Excerpt {
    part: &str, // ❌ thiếu lifetime
}

fn main() {
    let excerpt = Excerpt { part: "Gọi tôi là Ishmael" };
    println!("{}", excerpt.part);
}
```

**Trình biên dịch báo:**

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:11
  |
2 |     part: &str, // ❌ thiếu lifetime
  |           ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct Excerpt<'a> {
2 ~     part: &'a str, // ❌ thiếu lifetime
  |
```

Nếu dữ liệu gốc bị huỷ trước `Excerpt` thì compiler cũng chặn bằng `E0597`, giống ví dụ `dangling` ở mục [5.1](#51-vấn-đề-tham-chiếu-treo).

Đây chính là lý do ở bài [structs](../learn_06_structs/README.md) ta khuyên dùng `String` cho trường của `struct`: `String` **sở hữu** dữ liệu nên `struct` không phụ thuộc vào ai, không cần lifetime. Dùng `&'a str` khi bạn muốn tránh sao chép chuỗi lớn và chấp nhận `struct` bị ràng buộc với dữ liệu gốc.

### 5.7 Lifetime trong khối impl

Lifetime của `struct` là một phần của kiểu, nên khối `impl` phải khai báo lại: `impl<'a> Excerpt<'a>`.

```rust
struct Excerpt<'a> { part: &'a str }

impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    // Quy tắc 3: kết quả mang lifetime của &self
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Chú ý! {announcement}");
        self.part
    }
}

fn main() {
    let text = String::from("Một. Hai. Ba.");
    let excerpt = Excerpt { part: text.as_str() };

    println!("level = {}", excerpt.level());
    println!("{}", excerpt.announce_and_return("sắp in"));
}
```

**Kết quả:**

```text
level = 3
Chú ý! sắp in
Một. Hai. Ba.
```

Ở `announce_and_return`, có hai tham chiếu đầu vào (`&self` và `announcement`) nhưng không cần chú thích, nhờ **quy tắc 3**: kết quả nhận lifetime của `&self`. Phương thức `level` không trả về tham chiếu nên không có gì để chú thích.

### 5.8 Lifetime static

`'static` là lifetime đặc biệt: tham chiếu có thể sống **suốt chương trình**. Mọi chuỗi viết trực tiếp trong code (`"Rust"`) có kiểu `&'static str`, vì chúng được lưu sẵn trong chính file chạy.

```rust
fn language() -> &'static str {
    "Rust"
}

fn main() {
    let s: &'static str = "Chuỗi này sống suốt chương trình";
    println!("{s}");
    println!("{}", language());
}
```

**Kết quả:**

```text
Chuỗi này sống suốt chương trình
Rust
```

Lưu ý:

- `'static` nghĩa là *có thể* sống đến hết chương trình, không có nghĩa dữ liệu bị giữ lại mãi mãi.
- Khi compiler báo lỗi lifetime, **đừng vội thêm `'static` cho hết lỗi**. Thường vấn đề thật là tham chiếu trỏ vào dữ liệu sống quá ngắn. Cách sửa đúng là đổi phạm vi, trả về giá trị sở hữu (`String`) hoặc xem lại thiết kế.

---

## 6. Thực hành

Ba bài thực hành có hướng dẫn, mỗi bài ứng với một chủ đề: generics, trait và lifetime. Thực hiện trong `learn_10_generics_traits_lifetimes/src/main.rs` và chạy bằng `cargo run`.

### Lab 1. Hàm largest cho nhiều kiểu

**Mục tiêu:** viết hàm generic có trait bound và thấy bound áp dụng cho cả kiểu do bạn tự định nghĩa.

**Bước 1.** Hàm `largest<T: PartialOrd>` hoạt động với số, nhưng thử gọi với `Version`, một `struct` tự định nghĩa chưa có `PartialOrd`:

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Version {
    major: u32,
    minor: u32,
}

fn main() {
    let versions = [Version { major: 1, minor: 9 }, Version { major: 2, minor: 0 }];
    let newest = largest(&versions); // ❌ Version chưa có PartialOrd
    println!("{}.{}", newest.major, newest.minor);
}
```

**Trình biên dịch báo:**

```text
error[E0277]: can't compare `Version` with `Version`
  --> src/main.rs:18:26
   |
18 |     let newest = largest(&versions); // ❌ Version chưa có PartialOrd
   |                  ------- ^^^^^^^^^ no implementation for `Version < Version` and `Version > Version`
   |                  |
   |                  required by a bound introduced by this call
   |
   = help: the trait `PartialOrd` is not implemented for `Version`
note: required by a bound in `largest`
  --> src/main.rs:1:15
   |
 1 | fn largest<T: PartialOrd>(list: &[T]) -> &T {
   |               ^^^^^^^^^^ required by this bound in `largest`
help: consider annotating `Version` with `#[derive(PartialOrd)]`
...
```

Lỗi chỉ ra đúng chỗ: `Version` không thoả bound `PartialOrd` mà `largest` yêu cầu, và cho gợi ý `#[derive(PartialOrd)]`.

**Bước 2.** Thêm `#[derive(Debug, PartialEq, PartialOrd)]` cho `Version` (`PartialOrd` yêu cầu có sẵn `PartialEq`). Lúc này `largest` dùng được cho số nguyên, số thực, ký tự và cả `Version`:

```rust
#[derive(Debug, PartialEq, PartialOrd)]
struct Version {
    major: u32,
    minor: u32,
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = [34, 50, 25, 100, 65];
    let temperatures = [36.6, 39.2, 37.0];
    let letters = ['y', 'm', 'a', 'q'];
    let versions = [
        Version { major: 1, minor: 9 },
        Version { major: 2, minor: 0 },
        Version { major: 1, minor: 12 },
    ];

    println!("Số lớn nhất: {}", largest(&numbers));
    println!("Nhiệt độ cao nhất: {}", largest(&temperatures));
    println!("Ký tự lớn nhất: {}", largest(&letters));

    let newest = largest(&versions);
    println!("Phiên bản mới nhất: {}.{}", newest.major, newest.minor);
}
```

**Kết quả:**

```text
Số lớn nhất: 100
Nhiệt độ cao nhất: 39.2
Ký tự lớn nhất: y
Phiên bản mới nhất: 2.0
```

**Phân tích:**

- Một hàm `largest` phục vụ bốn kiểu khác nhau. Compiler sinh ra bốn bản riêng (mục [3.6](#36-monomorphization)).
- Với `Version`, `derive(PartialOrd)` so sánh **từng trường theo thứ tự khai báo**: so `major` trước, bằng nhau mới so `minor`. Vì vậy `2.0` lớn hơn `1.12`.
- Ta chỉ đọc trường `major` và `minor` ở `main` để in kết quả. Nếu chỉ in bằng `{:?}`, compiler cảnh báo các trường "never read" (mục [8.3](#83-cảnh-báo-thường-gặp)).

**Thử thêm:**

1. Đổi chỗ hai trường trong `struct Version` (đặt `minor` lên trước `major`) và dự đoán phiên bản mới nhất (đáp án: `1.12`, vì lúc này `minor` được so trước).
2. Gọi `largest` với một mảng rỗng, ví dụ `let empty: [i32; 0] = [];`. Chương trình panic với `index out of bounds: the len is 0 but the index is 0`. Hãy sửa `largest` để trả về `Option<&T>`.

### Lab 2. Trait Shape và các hình

**Mục tiêu:** định nghĩa trait có phương thức mặc định, triển khai cho nhiều kiểu, rồi dùng cả generics lẫn `dyn Trait`.

Chương trình gồm hai bước, cùng nằm trong một tệp:

- **Bước 1.** Định nghĩa trait `Shape`, hai kiểu `Circle` và `Rectangle`, rồi hàm `print_info` dùng generics (gọi tĩnh).
- **Bước 2.** Thêm `Triangle`, rồi đặt cả ba hình vào **cùng một** `Vec<Box<dyn Shape>>` để tính tổng diện tích (gọi động).

```rust
use std::f64::consts::PI;

trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> f64;

    // Phương thức có sẵn: dùng lại name() và area() của từng hình
    fn describe(&self) -> String {
        format!("{} có diện tích {:.2}", self.name(), self.area())
    }
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }
struct Triangle { base: f64, height: f64 }

impl Shape for Circle {
    fn name(&self) -> String { String::from("Hình tròn") }
    fn area(&self) -> f64 { PI * self.radius * self.radius }
}

impl Shape for Rectangle {
    fn name(&self) -> String { String::from("Hình chữ nhật") }
    fn area(&self) -> f64 { self.width * self.height }
}

impl Shape for Triangle {
    fn name(&self) -> String { String::from("Tam giác") }
    fn area(&self) -> f64 { 0.5 * self.base * self.height }
}

// Bước 1: generics, nhận bất kỳ kiểu nào là Shape (gọi tĩnh)
fn print_info<T: Shape>(shape: &T) {
    println!("{}", shape.describe());
}

fn main() {
    // Bước 1
    print_info(&Circle { radius: 2.0 });
    print_info(&Rectangle { width: 3.0, height: 4.5 });

    // Bước 2: một danh sách chứa nhiều kiểu hình khác nhau (gọi động)
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle { width: 3.0, height: 4.5 }),
        Box::new(Triangle { base: 6.0, height: 2.5 }),
    ];

    let mut total = 0.0;
    for shape in &shapes {
        total += shape.area();
    }
    println!("Tổng diện tích {} hình: {:.2}", shapes.len(), total);
}
```

**Kết quả:**

```text
Hình tròn có diện tích 12.57
Hình chữ nhật có diện tích 13.50
Tổng diện tích 3 hình: 33.57
```

**Phân tích:**

- `describe` là phương thức mặc định: được viết một lần trong trait, mọi hình đều dùng được, và nó gọi `name()` và `area()` của từng hình cụ thể.
- Bước 1 dùng generics: `print_info::<Circle>` và `print_info::<Rectangle>` là hai bản riêng, việc chọn hàm nào được quyết định lúc biên dịch.
- Bước 2 phải dùng `dyn`: `Vec<Circle>` chỉ chứa được `Circle`. Để trộn ba loại hình, ta cần `Box<dyn Shape>`.
- `Triangle` không được dùng trong bước 1 mà vẫn nằm chung danh sách ở bước 2, vì cả ba cùng `impl Shape`.
- `{:.2}` làm tròn kết quả thực đến hai chữ số thập phân.

**Thử thêm:** thêm hình `Square` và phương thức `perimeter(&self) -> f64` vào trait `Shape`. Compiler sẽ bắt bạn bổ sung `perimeter` cho **mọi** kiểu đã `impl Shape` (lỗi `E0046`). Đây chính là sức mạnh của "hợp đồng".

### Lab 3. Bộ thống kê văn bản mượn dữ liệu

**Mục tiêu:** viết `struct` chứa tham chiếu và quan sát borrow checker bảo vệ dữ liệu gốc.

**Bước 1.** `TextStats` mượn một đoạn văn bản (không sao chép) và cung cấp hai phép thống kê:

```rust
struct TextStats<'a> { text: &'a str }

impl<'a> TextStats<'a> {
    fn new(text: &'a str) -> Self { TextStats { text } }

    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }

    fn longest_word(&self) -> &'a str {
        let mut longest = "";
        for word in self.text.split_whitespace() {
            if word.len() > longest.len() {
                longest = word;
            }
        }
        longest
    }
}

fn main() {
    let text = String::from("ownership makes Rust safe and fast");
    let stats = TextStats::new(&text);

    println!("Số từ: {}", stats.word_count());
    println!("Từ dài nhất: {}", stats.longest_word());
}
```

**Kết quả:**

```text
Số từ: 6
Từ dài nhất: ownership
```

**Bước 2.** Huỷ chuỗi gốc trong khi `stats` còn dùng:

```rust
struct TextStats<'a> {
    text: &'a str,
}

impl<'a> TextStats<'a> {
    fn word_count(&self) -> usize {
        self.text.split_whitespace().count()
    }
}

fn main() {
    let text = String::from("ownership makes Rust safe and fast");
    let stats = TextStats { text: &text };
    drop(text); // ❌ text đang bị stats mượn
    println!("Số từ: {}", stats.word_count());
}
```

**Trình biên dịch báo:**

```text
error[E0505]: cannot move out of `text` because it is borrowed
  --> src/main.rs:14:10
   |
12 |     let text = String::from("ownership makes Rust safe and fast");
   |         ---- binding `text` declared here
13 |     let stats = TextStats { text: &text };
   |                                   ----- borrow of `text` occurs here
14 |     drop(text); // ❌ text đang bị stats mượn
   |          ^^^^ move out of `text` occurs here
15 |     println!("Số từ: {}", stats.word_count());
   |                           ----- borrow later used here
   |
help: consider cloning the value if the performance cost is acceptable
   |
13 |     let stats = TextStats { text: &text.clone() };
   |                                        ++++++++
```

**Phân tích:**

- `TextStats<'a>` giữ `&'a str`, nên nó **mượn** `text` suốt thời gian `stats` còn được dùng.
- `drop(text)` cần chuyển quyền sở hữu `text` đi, trong khi vẫn đang bị mượn. Compiler từ chối, và nhờ đó không bao giờ có chuyện `stats` trỏ vào chuỗi đã huỷ.
- `longest_word` trả về `&'a str` (gắn với dữ liệu gốc), không phải gắn với `&self`. Khác biệt này được giải thích ở mục [7.3](#73-lifetime-của-kết-quả-trong-phương-thức).

**Thử thêm:** xoá dòng `drop(text);` rồi chạy lại để thấy chương trình hợp lệ trở lại.

---

## 7. Tìm hiểu chi tiết

### 7.1 Generic, impl Trait hay dyn Trait

`<T: Trait>` buộc các tham số cùng dùng `T` phải **cùng một kiểu**:

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct Article;
struct Tweet;

impl Summary for Article {
    fn summarize(&self) -> String { String::from("bài báo") }
}

impl Summary for Tweet {
    fn summarize(&self) -> String { String::from("tweet") }
}

// a và b phải là CÙNG một kiểu T
fn compare<T: Summary>(a: &T, b: &T) {
    println!("{} và {}", a.summarize(), b.summarize());
}

fn main() {
    compare(&Article, &Tweet); // ❌ hai kiểu khác nhau
}
```

**Trình biên dịch báo:**

```text
error[E0308]: mismatched types
  --> src/main.rs:22:23
   |
22 |     compare(&Article, &Tweet); // ❌ hai kiểu khác nhau
   |     -------           ^^^^^^ expected `&Article`, found `&Tweet`
   |     |
   |     arguments to this function are incorrect
   |
   = note: expected reference `&Article`
              found reference `&Tweet`
note: function defined here
  --> src/main.rs:17:4
   |
17 | fn compare<T: Summary>(a: &T, b: &T) {
   |    ^^^^^^^                    -----
```

Dùng `impl Trait` cho từng tham số thì mỗi tham số là một kiểu riêng, nên có thể khác nhau. Chỉ cần đổi chữ ký:

```text
fn compare(a: &impl Summary, b: &impl Summary)
```

Với chữ ký này, lời gọi `compare(&Article, &Tweet)` biên dịch và in `bài báo và tweet`.

| Tình huống | Chọn |
|---|---|
| Nhiều tham số **phải cùng kiểu** | `<T: Trait>(a: &T, b: &T)` |
| Nhiều tham số **được phép khác kiểu** | `&impl Trait` cho từng tham số |
| Cần nhắc tên kiểu ở nhiều chỗ (bound `where`, kiểu trả về...) | `<T: Trait>` |
| Chỉ cần ngắn gọn cho một tham số hoặc kiểu trả về | `impl Trait` |
| Trộn nhiều kiểu trong cùng một `Vec` hoặc quyết định kiểu lúc chạy | `Box<dyn Trait>` hoặc `&dyn Trait` |

### 7.2 Trait dyn compatible

Không phải trait nào cũng dùng được với `dyn`. Trait phải **dyn compatible** (trước đây gọi là *object safe*). Ví dụ một phương thức trả về `Self` làm trait mất khả năng này, vì với `dyn` compiler không biết `Self` là kiểu gì:

```rust
trait Duplicate {
    fn duplicate(&self) -> Self; // trả về Self
}

struct Dog;

impl Duplicate for Dog {
    fn duplicate(&self) -> Self {
        Dog
    }
}

fn main() {
    let _d: Box<dyn Duplicate> = Box::new(Dog); // ❌ trait này không dùng được với dyn
}
```

**Trình biên dịch báo:**

```text
error[E0038]: the trait `Duplicate` is not dyn compatible
  --> src/main.rs:14:21
   |
14 |     let _d: Box<dyn Duplicate> = Box::new(Dog); // ❌ trait này không dùng được với dyn
   |                     ^^^^^^^^^ `Duplicate` is not dyn compatible
   |
note: for a trait to be dyn compatible it needs to allow building a vtable
      for more information, visit <https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility>
  --> src/main.rs:2:28
   |
 1 | trait Duplicate {
   |       --------- this trait is not dyn compatible...
 2 |     fn duplicate(&self) -> Self; // trả về Self
   |                            ^^^^ ...because method `duplicate` references the `Self` type in its return type
...
```

Hai quy tắc chính cho phương thức của trait dyn compatible:

- Không trả về `Self` (hoặc dùng `Self` ngoài vai trò người nhận).
- Không có tham số kiểu generic riêng (ví dụ `fn f<T>(&self, x: T)`).

Trait không dyn compatible vẫn dùng bình thường với generics và `impl Trait`, chỉ không dùng được `dyn`. Danh sách đầy đủ có trong [The Rust Reference](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility).

### 7.3 Lifetime của kết quả trong phương thức

Trong khối `impl<'a>`, kết quả của phương thức có thể gắn với **hai** thứ khác nhau:

| Kiểu trả về | Gắn với | Ý nghĩa |
|---|---|---|
| `&'a str` | Dữ liệu gốc mà struct đang mượn | Kết quả có thể sống lâu hơn struct |
| `&str` (theo quy tắc 3) | `&self` | Kết quả chỉ sống bằng struct |

Ví dụ dưới đây dùng phương thức `first_word`. Với `&'a str`, kết quả vẫn dùng được sau khi `stats` đã bị huỷ, miễn là `text` còn sống:

```rust
struct TextStats<'a> { text: &'a str }

impl<'a> TextStats<'a> {
    // Kết quả gắn với 'a (dữ liệu gốc), không gắn với &self
    fn first_word(&self) -> &'a str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}

fn main() {
    let text = String::from("ownership makes Rust safe");
    let word;
    {
        let stats = TextStats { text: text.as_str() };
        word = stats.first_word();
    }
    println!("{word}");
}
```

**Kết quả:**

```text
ownership
```

Nếu viết `-> &str`, quy tắc 3 buộc kết quả sống không lâu hơn `stats`, và đoạn code trên bị từ chối:

```rust
struct TextStats<'a> { text: &'a str }

impl<'a> TextStats<'a> {
    // Quy tắc 3: kết quả gắn với &self
    fn first_word(&self) -> &str {
        self.text.split_whitespace().next().unwrap_or("")
    }
}

fn main() {
    let text = String::from("ownership makes Rust safe");
    let word;
    {
        let stats = TextStats { text: text.as_str() };
        word = stats.first_word(); // ❌ kết quả bị buộc sống ngắn bằng stats
    }
    println!("{word}");
}
```

**Trình biên dịch báo:**

```text
error[E0597]: `stats` does not live long enough
  --> src/main.rs:15:16
   |
14 |         let stats = TextStats { text: text.as_str() };
   |             ----- binding `stats` declared here
15 |         word = stats.first_word(); // ❌ kết quả bị buộc sống ngắn bằng stats
   |                ^^^^^ borrowed value does not live long enough
16 |     }
   |     - `stats` dropped here while still borrowed
17 |     println!("{word}");
   |                ---- borrow later used here
```

Hãy dùng `&'a str` khi kết quả thật sự trỏ vào dữ liệu gốc và bạn muốn nó độc lập với struct. Nếu không cần điều đó, `-> &str` ngắn gọn hơn.

### 7.4 Chọn công cụ nào

| Tình huống | Chọn |
|---|---|
| Cùng một thuật toán cho nhiều kiểu | Hàm hoặc `struct` generic `<T>` |
| Nói rõ kiểu cần làm được việc gì | Trait bound `T: Trait` hoặc `impl Trait` |
| Gom nhiều kiểu khác nhau vào một tập hợp | `Box<dyn Trait>` |
| In, so sánh, sao chép... theo cách chuẩn | `#[derive(...)]` |
| In theo định dạng riêng cho người dùng | Tự `impl Display` |
| Hàm nhận nhiều tham chiếu và trả về một tham chiếu | Chú thích `'a` (khi quy tắc lược bỏ không đủ) |
| `struct` giữ tham chiếu | `struct S<'a>` và `impl<'a> S<'a>` |
| Không muốn dính lifetime | Dùng kiểu sở hữu (`String`, `Vec<T>`) thay cho tham chiếu |

---

## 8. Lỗi thường gặp

### 8.1 Cách đọc lỗi liên quan đến trait và lifetime

Cách đọc một thông báo lỗi (mã lỗi, vị trí, `^^^^`, `help:`) giống ở bài [variables](../learn_03_variables/README.md). Với generics, trait và lifetime, có thêm vài cụm từ hay gặp:

| Cụm từ trong thông báo | Ý nghĩa | Việc nên làm |
|---|---|---|
| `consider restricting type parameter` | `T` thiếu một trait bound | Thêm bound vào `<T: ...>` theo gợi ý |
| `required by a bound in ...` | Hàm bạn gọi đòi một trait mà kiểu của bạn chưa có | Thêm `derive` hoặc `impl` cho kiểu đó |
| `consider annotating ... with #[derive(...)]` | Chỉ cần `derive` là đủ | Thêm dòng `derive` được gợi ý |
| `missing lifetime specifier` | Compiler không suy ra được lifetime | Thêm chú thích `'a` |
| `does not live long enough` | Dữ liệu bị huỷ trước khi tham chiếu dùng xong | Kéo dài phạm vi của dữ liệu, hoặc dùng xong tham chiếu sớm hơn |

### 8.2 Lỗi biên dịch

| Mã lỗi | Thông báo | Nguyên nhân | Cách sửa |
|---|---|---|---|
| `E0369` | `binary operation '>' cannot be applied to type '&T'` | Dùng toán tử trên `T` khi chưa có bound | Thêm `T: PartialOrd` |
| `E0277` | `'T' doesn't implement 'std::fmt::Display'` | Dùng `{}` với `T` chưa có `Display` | Thêm `T: Display` |
| `E0277` | `can't compare 'Version' with 'Version'` | Gọi hàm đòi `PartialOrd` với kiểu chưa có | `#[derive(PartialOrd)]` hoặc tự `impl` |
| `E0277` | `the trait bound '{integer}: Summary' is not satisfied` | Truyền giá trị của kiểu chưa `impl` trait vào hàm đòi trait đó | `impl Summary for ...` cho kiểu đó, hoặc đổi đối số |
| `E0277` | `'Point' doesn't implement 'Debug'` | In `{:?}` kiểu chưa có `Debug` | `#[derive(Debug)]` |
| `E0369` | `binary operation '==' cannot be applied to type 'Point'` | So sánh `==` kiểu chưa có `PartialEq` | `#[derive(PartialEq)]` |
| `E0599` | `no method named 'distance_from_origin' found for struct 'Point<{integer}>'` | Phương thức chỉ có cho kiểu cụ thể khác (hoặc chưa thoả bound) | Dùng đúng kiểu, hoặc bổ sung bound |
| `E0308` | `mismatched types` | Một tham số kiểu `T` bị gán hai kiểu khác nhau | Dùng hai tham số `<T, U>`, hoặc `impl Trait` cho từng tham số |
| `E0308` | `'if' and 'else' have incompatible types` | Hàm `-> impl Trait` trả về hai kiểu khác nhau | Dùng `Box<dyn Trait>` |
| `E0046` | `not all trait items implemented, missing: 'summarize'` | `impl Trait` thiếu phương thức bắt buộc | Viết đủ các phương thức |
| `E0117` | `only traits defined in the current crate can be implemented for types defined outside of the crate` | Vi phạm quy tắc orphan | Bọc kiểu trong newtype |
| `E0204` | `the trait 'Copy' cannot be implemented for this type` | `derive(Copy)` khi có trường không `Copy` (như `String`) | Bỏ `Copy`, dùng `Clone` |
| `E0382` | `borrow of moved value: 'a'` | Kiểu không `Copy` bị chuyển đi rồi vẫn dùng | `.clone()`, mượn bằng `&`, hoặc `derive(Copy)` nếu phù hợp |
| `E0508` | `cannot move out of type '[T]', a non-copy slice` | Lấy phần tử `T` ra khỏi slice theo giá trị | Trả về `&T`, hoặc thêm bound `Copy` hay `Clone` |
| `E0782` | `expected a type, found a trait` | Thiếu từ khoá `dyn` | Viết `dyn Trait` |
| `E0038` | `the trait 'Duplicate' is not dyn compatible` | Dùng `dyn` với trait không dyn compatible | Bỏ phương thức trả về `Self`, hoặc dùng generics |
| `E0106` | `missing lifetime specifier` | Hàm trả về tham chiếu mà không rõ mượn từ đâu, hoặc `struct` chứa tham chiếu | Thêm chú thích `'a` |
| `E0597` | `'x' does not live long enough` | Tham chiếu được dùng sau khi dữ liệu đã bị huỷ | Kéo dài phạm vi dữ liệu, hoặc dùng tham chiếu trong phạm vi hợp lệ |
| `E0505` | `cannot move out of 'text' because it is borrowed` | Chuyển hoặc huỷ giá trị khi còn tham chiếu đang dùng | Dùng xong tham chiếu trước, hoặc sao chép |
| `E0515` | `cannot return value referencing local variable 'text'` | Trả về tham chiếu tới biến cục bộ | Trả về giá trị sở hữu (`String`...) |

Hai lỗi dưới đây hay làm người mới bối rối nên xem kỹ. Lỗi thứ nhất: quên bound khi dùng `{}`.

```rust
fn show<T>(value: T) {
    println!("Giá trị: {value}"); // ❌ T chưa có bound Display
}

fn main() {
    show(5);
}
```

**Trình biên dịch báo:**

```text
error[E0277]: `T` doesn't implement `std::fmt::Display`
 --> src/main.rs:2:24
  |
2 |     println!("Giá trị: {value}"); // ❌ T chưa có bound Display
  |                        ^^^^^^^ `T` cannot be formatted with the default formatter
  |
  = note: in format strings you may be able to use `{:?}` (or `{:#?}` for pretty-print) instead
help: consider restricting type parameter `T` with trait `Display`
  |
1 | fn show<T: std::fmt::Display>(value: T) {
  |          +++++++++++++++++++
```

Lỗi thứ hai: cố trả về `T` bằng cách lấy phần tử ra khỏi slice. Slice chỉ **mượn** các phần tử, nên không thể chuyển quyền sở hữu một phần tử ra ngoài:

```rust
fn first<T>(list: &[T]) -> T {
    list[0] // ❌ không "lấy" phần tử ra khỏi slice được
}

fn main() {
    println!("{}", first(&[1, 2, 3]));
}
```

**Trình biên dịch báo:**

```text
error[E0508]: cannot move out of type `[T]`, a non-copy slice
 --> src/main.rs:2:5
  |
2 |     list[0] // ❌ không "lấy" phần tử ra khỏi slice được
  |     ^^^^^^^
  |     |
  |     cannot move out of here
  |     move occurs because `list[_]` has type `T`, which does not implement the `Copy` trait
  |
help: if `T` implemented `Clone`, you could clone the value
 --> src/main.rs:1:10
  |
1 | fn first<T>(list: &[T]) -> T {
  |          ^ consider constraining this type parameter with `Clone`
2 |     list[0] // ❌ không "lấy" phần tử ra khỏi slice được
  |     ------- you could clone this value
```

Đây là lý do `largest` ở mục [3.2](#32-hàm-generic) trả về `&T` chứ không phải `T`. Trả về `T` chỉ làm được nếu thêm bound `T: Copy` (hoặc `Clone` rồi gọi `.clone()`).

> Mã lỗi và câu chữ có thể khác nhau giữa các phiên bản compiler. Bảng trên là kết quả với Rust 1.99.1. Cụm "dyn compatible" trước đây được gọi là "object safe".

### 8.3 Cảnh báo thường gặp

Cảnh báo (warning) không chặn việc biên dịch nhưng nên được sửa hết. Hai cảnh báo sau hay xuất hiện khi học trait và lifetime.

**Trường chỉ được dùng qua `derive(Debug)`:**

```rust
// ⚠️ Biên dịch được nhưng có 1 cảnh báo
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);
}
```

**Giấu lifetime trong kiểu trả về:**

```rust
// ⚠️ Biên dịch được nhưng có 1 cảnh báo
struct Excerpt<'a> {
    part: &'a str,
}

fn make(text: &str) -> Excerpt {
    Excerpt { part: text }
}

fn main() {
    let e = make("Gọi tôi là Ishmael");
    println!("{}", e.part);
}
```

| Cảnh báo | Ý nghĩa | Cách xử lý |
|---|---|---|
| `fields 'x' and 'y' are never read` | Compiler không tính `derive(Debug)` là "đọc" trường. Nếu không có code nào khác đọc, trường bị coi là thừa | Đọc trường ở đâu đó có ích, hoặc xoá trường đó |
| `hiding a lifetime that's elided elsewhere is confusing` | Kiểu trả về `Excerpt` giấu lifetime mà tham số `&str` đã lược bỏ, nên khó thấy hai thứ liên quan nhau | Viết `Excerpt<'_>` cho rõ |

---

## 9. Bài tập

Hãy tự làm trước, chạy bằng `cargo run`, rồi mới mở phần đáp án. Mỗi bài có **kết quả mong đợi** để bạn tự kiểm tra.

### Cấp độ 1. Làm quen

#### Bài 1.1. Thêm trait bound

**Đề:** hàm `print_boxed` dưới đây không biên dịch được. Sửa để nó chạy cho cả số lẫn chuỗi.

```rust
fn print_boxed<T>(value: T) {
    println!("[ {value} ]"); // ❌
}

fn main() {
    print_boxed(42);
    print_boxed("Rust");
}
```

**Kết quả mong đợi:**

```text
[ 42 ]
[ Rust ]
```

**Gợi ý:** đọc dòng `help:` trong thông báo lỗi.

<details>
<summary>Xem đáp án</summary>

```rust
use std::fmt::Display;

fn print_boxed<T: Display>(value: T) {
    println!("[ {value} ]");
}

fn main() {
    print_boxed(42);
    print_boxed("Rust");
}
```

Giải thích: `{value}` cần `T` có trait `Display`. Compiler không được phép giả định điều đó cho một `T` bất kỳ, nên ta cam kết bằng bound `T: Display`.

</details>

#### Bài 1.2. Struct generic

**Đề:** định nghĩa `struct Pair<T>` có hai trường `first` và `second` cùng kiểu `T`, và phương thức `swap(self) -> Pair<T>` trả về một `Pair` mới với hai phần tử đổi chỗ. Thử với số nguyên và với chuỗi `&str`.

**Kết quả mong đợi:**

```text
2 1
hai một
```

**Gợi ý:** khối `impl` bắt đầu bằng `impl<T> Pair<T>`.

<details>
<summary>Xem đáp án</summary>

```rust
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn swap(self) -> Pair<T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

fn main() {
    let numbers = Pair { first: 1, second: 2 };
    let swapped = numbers.swap();
    println!("{} {}", swapped.first, swapped.second);

    let words = Pair {
        first: "một",
        second: "hai",
    };
    let swapped = words.swap();
    println!("{} {}", swapped.first, swapped.second);
}
```

Giải thích: `swap` nhận `self` (lấy quyền sở hữu) nên chỉ cần **di chuyển** hai giá trị sang vị trí mới, không cần so sánh hay sao chép. Vì thế `T` không cần bound nào. Compiler tạo hai bản `Pair<i32>` và `Pair<&str>` riêng.

</details>

#### Bài 1.3. Định nghĩa và triển khai trait

**Đề:** định nghĩa trait `Greet` có phương thức `hello(&self) -> String`. Tạo hai kiểu rỗng `Vietnamese` và `English`, mỗi kiểu triển khai `Greet` để trả về lời chào tương ứng.

**Kết quả mong đợi:**

```text
Xin chào
Hello
```

<details>
<summary>Xem đáp án</summary>

```rust
trait Greet {
    fn hello(&self) -> String;
}

struct Vietnamese;
struct English;

impl Greet for Vietnamese {
    fn hello(&self) -> String { String::from("Xin chào") }
}

impl Greet for English {
    fn hello(&self) -> String { String::from("Hello") }
}

fn main() {
    println!("{}", Vietnamese.hello());
    println!("{}", English.hello());
}
```

Giải thích: `struct Vietnamese;` là *unit struct* (không có trường). Gọi `Vietnamese.hello()` tạo giá trị rồi gọi phương thức ngay.

</details>

### Cấp độ 2. Vận dụng

#### Bài 2.1. Dự đoán kết quả

**Đề:** đọc đoạn mã sau và **dự đoán** năm dòng in ra trước khi chạy.

```rust
fn pick<T: PartialOrd + Copy>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    println!("{}", pick(3, 9));
    println!("{}", pick(2.5, 1.5));
    println!("{}", pick('a', 'z'));
    println!("{}", longest("abc", "xyz"));
    println!("{}", longest("hi", "hello"));
}
```

<details>
<summary>Xem đáp án</summary>

**Kết quả:**

```text
9
2.5
z
abc
hello
```

Giải thích:

- `pick` trả về giá trị lớn hơn: `9`, `2.5`, `'z'`. Một hàm, ba kiểu.
- `longest("abc", "xyz")`: hai chuỗi **bằng độ dài** nên điều kiện `>=` đúng và hàm trả về `x`, tức `abc`.
- `longest("hi", "hello")`: `hello` dài hơn.

</details>

#### Bài 2.2. Phương thức mặc định

**Đề:** định nghĩa trait `Describe` có phương thức `name(&self) -> String` (bắt buộc) và `describe(&self) -> String` có sẵn nội dung, trả về `"Đây là <tên>"`. Tạo `Student` (có `name`) dùng bản mặc định, và `Teacher` (có `name`, `subject`) **ghi đè** `describe` thành `"Đây là cô <tên>, dạy <môn>"`.

**Kết quả mong đợi:**

```text
Đây là An
Đây là cô Lan, dạy Toán
```

**Gợi ý:** `Student` chỉ cần viết `name`.

<details>
<summary>Xem đáp án</summary>

```rust
trait Describe {
    fn name(&self) -> String;

    fn describe(&self) -> String {
        format!("Đây là {}", self.name())
    }
}

struct Student { name: String }
struct Teacher { name: String, subject: String }

impl Describe for Student {
    fn name(&self) -> String { self.name.clone() }
}

impl Describe for Teacher {
    fn name(&self) -> String { self.name.clone() }

    fn describe(&self) -> String {
        format!("Đây là cô {}, dạy {}", self.name, self.subject)
    }
}

fn main() {
    let student = Student { name: String::from("An") };
    let teacher = Teacher { name: String::from("Lan"), subject: String::from("Toán") };

    println!("{}", student.describe());
    println!("{}", teacher.describe());
}
```

Giải thích: `Student` không viết `describe` nên dùng bản mặc định, bản đó gọi `self.name()`. `Teacher` viết lại `describe` nên bản mặc định bị thay thế.

</details>

#### Bài 2.3. impl Trait ở tham số và kiểu trả về

**Đề:** cho trait `Shape` có `area(&self) -> f64`, các kiểu `Rectangle { width, height }` và `Square(f64)`. Viết hàm `total_area` nhận **hai hình có thể thuộc hai kiểu khác nhau** và trả về tổng diện tích, cùng hàm `unit_square()` trả về `impl Shape` là hình vuông cạnh 1.

**Kết quả mong đợi:**

```text
Tổng: 16.0
Hình vuông đơn vị: 1.0
```

**Gợi ý:** dùng `&impl Shape` cho từng tham số.

<details>
<summary>Xem đáp án</summary>

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Rectangle { width: f64, height: f64 }
struct Square(f64);

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
}

impl Shape for Square {
    fn area(&self) -> f64 { self.0 * self.0 }
}

// Hai tham số có thể thuộc hai kiểu khác nhau
fn total_area(a: &impl Shape, b: &impl Shape) -> f64 {
    a.area() + b.area()
}

// Trả về "một Shape nào đó"
fn unit_square() -> impl Shape {
    Square(1.0)
}

fn main() {
    let rect = Rectangle { width: 3.0, height: 4.0 };
    let square = Square(2.0);

    println!("Tổng: {:.1}", total_area(&rect, &square));
    println!("Hình vuông đơn vị: {:.1}", unit_square().area());
}
```

Giải thích: mỗi `impl Shape` ở tham số là một kiểu riêng nên `Rectangle` và `Square` truyền chung được (mục [7.1](#71-generic-impl-trait-hay-dyn-trait)). `unit_square` trả về đúng một kiểu cụ thể (`Square`), nhưng người gọi chỉ biết nó là một `Shape`.

</details>

#### Bài 2.4. Sửa lỗi lifetime

**Đề:** hàm `shorter` trả về chuỗi ngắn hơn trong hai chuỗi, nhưng không biên dịch được. Sửa lỗi.

```rust
fn shorter(x: &str, y: &str) -> &str { // ❌
    if x.len() <= y.len() { x } else { y }
}

fn main() {
    println!("{}", shorter("Rust", "Go"));
    println!("{}", shorter("cargo", "rustc"));
}
```

**Kết quả mong đợi:**

```text
Go
cargo
```

**Gợi ý:** hàm có hai tham chiếu đầu vào và trả về một tham chiếu, đúng tình huống của `longest` ở mục [5.2](#52-chú-thích-lifetime).

<details>
<summary>Xem đáp án</summary>

```rust
fn shorter<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() <= y.len() { x } else { y }
}

fn main() {
    println!("{}", shorter("Rust", "Go"));
    println!("{}", shorter("cargo", "rustc"));
}
```

Giải thích: thêm tham số lifetime `'a` và gắn vào cả hai tham số lẫn kiểu trả về. Với hai chuỗi bằng độ dài (`cargo` và `rustc`), điều kiện `<=` trả về `x`, tức `cargo`.

</details>

### Cấp độ 3. Thử thách

#### Bài 3.1. Ngăn xếp generic

**Đề:** cài đặt `struct Stack<T>` bọc một `Vec<T>` với các phương thức `new`, `push`, `pop` (trả về `Option<T>`), `peek` (trả về `Option<&T>`), `len` và `is_empty`. Thử với `Stack<i32>` và `Stack<&str>`.

**Kết quả mong đợi:**

```text
Đỉnh: Some(3)
Lấy ra: Some(3)
Còn lại: 2
Some("một") true
None
```

**Gợi ý:** `Vec` đã có sẵn `push`, `pop`, `last`, `len`, `is_empty` (bài [collections](../learn_08_collections/README.md)). `peek` chỉ mượn nên trả về `Option<&T>`.

<details>
<summary>Xem đáp án</summary>

```rust
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self { Stack { items: Vec::new() } }
    fn push(&mut self, item: T) { self.items.push(item); }
    fn pop(&mut self) -> Option<T> { self.items.pop() }
    fn peek(&self) -> Option<&T> { self.items.last() }
    fn len(&self) -> usize { self.items.len() }
    fn is_empty(&self) -> bool { self.items.is_empty() }
}

fn main() {
    let mut numbers = Stack::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("Đỉnh: {:?}", numbers.peek());
    println!("Lấy ra: {:?}", numbers.pop());
    println!("Còn lại: {}", numbers.len());

    let mut words = Stack::new();
    words.push("một");
    println!("{:?} {}", words.pop(), words.is_empty());
    println!("{:?}", words.pop());
}
```

Giải thích: `Stack<T>` không đòi bound nào vì chỉ cất và lấy giá trị, không so sánh hay in. `pop` trả về `Option<T>` (chuyển quyền sở hữu phần tử ra), còn `peek` trả về `Option<&T>` (chỉ mượn). `Stack::new()` tự suy ra kiểu `T` từ lần `push` đầu tiên.

</details>

#### Bài 3.2. Bốn đoạn mã, bốn lỗi

**Đề:** mỗi đoạn mã dưới đây đều không biên dịch được. Với từng đoạn, hãy **dự đoán lỗi** trước khi chạy `cargo check`, rồi sửa.

**Đoạn (a):**

```rust
fn bigger<T>(a: T, b: T) -> T {
    if a > b { a } else { b } // ❌
}

fn main() {
    println!("{}", bigger(3, 7));
}
```

**Đoạn (b):**

```rust
fn pick(x: &str, y: &str) -> &str { // ❌
    if x.is_empty() { y } else { x }
}

fn main() {
    println!("{}", pick("", "Rust"));
}
```

**Đoạn (c):**

```rust
fn main() {
    let r;
    {
        let word = String::from("Rust");
        r = word.as_str(); // ❌
    }
    println!("{r}");
}
```

**Đoạn (d):**

```rust
trait Greet {
    fn hello(&self) -> String;
}

fn welcome(who: &impl Greet) {
    println!("{}", who.hello());
}

fn main() {
    welcome(&"An"); // ❌
}
```

<details>
<summary>Xem đáp án</summary>

| Đoạn | Lỗi | Nguyên nhân | Cách sửa |
|---|---|---|---|
| (a) | `E0369` binary operation `>` cannot be applied to type `T` | `T` chưa có bound để so sánh | `fn bigger<T: PartialOrd>(a: T, b: T) -> T` |
| (b) | `E0106` missing lifetime specifier | Hai tham chiếu đầu vào, một tham chiếu đầu ra, quy tắc lược bỏ không đủ | `fn pick<'a>(x: &'a str, y: &'a str) -> &'a str` |
| (c) | `E0597` `word` does not live long enough | `r` được dùng sau khi `word` bị huỷ ở cuối khối | Khai báo `word` ngoài khối, hoặc `println!` trong khối |
| (d) | `E0277` the trait bound `&str: Greet` is not satisfied | `welcome` đòi kiểu có `Greet`, mà `&str` chưa `impl Greet` | Tạo kiểu của bạn (như `struct Person`), `impl Greet for Person` rồi truyền nó vào |

</details>

#### Bài 3.3. Tìm hình lớn nhất

**Đề:** với trait `Shape` có `name(&self) -> String` và `area(&self) -> f64`, cùng `Circle { radius }` và `Rectangle { width, height }`, viết hàm `largest_shape(shapes: &[Box<dyn Shape>]) -> &dyn Shape` trả về hình có diện tích lớn nhất. Thử với một hình tròn bán kính 2 và hai hình chữ nhật 2x3 và 1x1.

**Kết quả mong đợi:**

```text
Lớn nhất: Hình tròn (12.57)
```

**Gợi ý:** hàm chỉ có một tham số tham chiếu nên không cần chú thích lifetime. Dùng `as_ref()` để lấy `&dyn Shape` từ `Box<dyn Shape>`.

<details>
<summary>Xem đáp án</summary>

```rust
use std::f64::consts::PI;

trait Shape {
    fn name(&self) -> String;
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }

impl Shape for Circle {
    fn name(&self) -> String { String::from("Hình tròn") }
    fn area(&self) -> f64 { PI * self.radius * self.radius }
}

impl Shape for Rectangle {
    fn name(&self) -> String { String::from("Hình chữ nhật") }
    fn area(&self) -> f64 { self.width * self.height }
}

fn largest_shape(shapes: &[Box<dyn Shape>]) -> &dyn Shape {
    let mut best = shapes[0].as_ref();
    for shape in shapes {
        if shape.area() > best.area() {
            best = shape.as_ref();
        }
    }
    best
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { width: 2.0, height: 3.0 }),
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle { width: 1.0, height: 1.0 }),
    ];

    let best = largest_shape(&shapes);
    println!("Lớn nhất: {} ({:.2})", best.name(), best.area());
}
```

Giải thích: danh sách chứa nhiều kiểu khác nhau nên phải dùng `Box<dyn Shape>`. Kết quả mượn từ `shapes`, và theo quy tắc lược bỏ số 2, lifetime của kết quả chính là lifetime của `shapes`, nên không cần viết `'a`.

</details>

---

## 10. Tóm tắt

| Chủ đề | Ghi nhớ |
|---|---|
| Generics | `<T>` cho phép viết một lần dùng cho nhiều kiểu. Compiler tạo bản riêng cho từng kiểu (monomorphization), không tốn chi phí chạy |
| Trait | "Hợp đồng" hành vi. `trait` khai báo, `impl Trait for Type` triển khai, có thể có phương thức mặc định |
| Trait bound | `T: Trait`, `T: A + B`, `where`. Không có bound thì không được dùng toán tử hay phương thức của trait trên `T` |
| `impl Trait` | Cú pháp ngắn ở tham số, và dùng ở kiểu trả về cho **một** kiểu cụ thể |
| `derive` | Tự sinh `Debug`, `Clone`, `Copy`, `PartialEq`, `PartialOrd`, `Default`, `Hash`. Mọi trường phải hỗ trợ trait đó |
| Orphan rule | Chỉ `impl` trait cho kiểu khi trait hoặc kiểu thuộc crate của bạn. Dùng newtype để vòng |
| `dyn Trait` | Quyết định lúc chạy, cho phép trộn nhiều kiểu (`Vec<Box<dyn Trait>>`). Mặc định ưu tiên generics |
| Lifetime | Mô tả thời gian sống của tham chiếu. `'a` không kéo dài giá trị, chỉ mô tả quan hệ |
| Lược bỏ lifetime | Ba quy tắc: mỗi tham chiếu đầu vào một lifetime; một đầu vào thì gán cho đầu ra; `&self` thì gán cho đầu ra |
| `struct` chứa tham chiếu | `struct S<'a>` và `impl<'a> S<'a>`. Muốn tránh lifetime thì dùng kiểu sở hữu như `String` |
| `'static` | Tham chiếu có thể sống suốt chương trình (như chuỗi viết trực tiếp). Đừng thêm chỉ để hết lỗi |
| Không trả về tham chiếu cục bộ | Trả về giá trị sở hữu thay vì tham chiếu tới biến sắp bị huỷ |

> **Bước tiếp theo gợi ý:** module và crate, closure và iterator, viết test, và smart pointer như `Box`, `Rc`. Tất cả đều dựa trên generics, trait và lifetime mà bạn vừa học.

---

## 11. Tài liệu tham khảo

- [The Rust Programming Language, 10.0 Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)
- [The Rust Programming Language, 10.1 Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [The Rust Programming Language, 10.2 Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [The Rust Programming Language, 10.3 Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)
- [The Rust Programming Language, 18.2 Using Trait Objects](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- [Rust By Example, Generics](https://doc.rust-lang.org/rust-by-example/generics.html)
- [Rust By Example, Traits](https://doc.rust-lang.org/rust-by-example/trait.html)
- [Rust By Example, Derive](https://doc.rust-lang.org/rust-by-example/trait/derive.html)
- [Rust By Example, Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
- [The Rust Reference, Lifetime elision](https://doc.rust-lang.org/reference/lifetime-elision.html)
- [The Rust Reference, Trait bounds](https://doc.rust-lang.org/reference/trait-bounds.html)
- [The Rust Reference, Traits và dyn compatibility](https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility)
- [The Rust Reference, Implementations và orphan rules](https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules)
- [The Rustonomicon, Lifetimes](https://doc.rust-lang.org/nomicon/lifetimes.html)
- [Tài liệu thư viện chuẩn, trait `Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)
- [Tài liệu thư viện chuẩn, trait `Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [Tra cứu mã lỗi, ví dụ E0277](https://doc.rust-lang.org/error_codes/E0277.html)
- [Tra cứu mã lỗi, ví dụ E0106](https://doc.rust-lang.org/error_codes/E0106.html)
- [Tra cứu mã lỗi, ví dụ E0597](https://doc.rust-lang.org/error_codes/E0597.html)
