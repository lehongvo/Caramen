# Biến (Variables) trong Rust

> Tài liệu học tập về biến trong Rust: khái niệm, các loại biến, kiểu dữ liệu cơ bản, thực hành và bài tập có đáp án.

| Thông tin | Chi tiết |
|---|---|
| Chủ đề | Biến, tính bất biến, hằng số, shadowing, kiểu dữ liệu cơ bản |
| Môi trường | Rust 1.99.1 (stable), Cargo 1.99.1, edition 2024 |
| Dự án | `learn_03_variables` (thư mục này) |
| Nguồn tham khảo chính | *The Rust Programming Language*, chương 3 (mục 3.1 và 3.2) |

## Mục lục

1. [Mục tiêu bài học](#1-mục-tiêu-bài-học)
2. [Khái niệm](#2-khái-niệm)
3. [Các loại biến](#3-các-loại-biến)
4. [Kiểu dữ liệu](#4-kiểu-dữ-liệu)
5. [Thực hành](#5-thực-hành)
6. [Tìm hiểu chi tiết](#6-tìm-hiểu-chi-tiết)
7. [Lỗi thường gặp](#7-lỗi-thường-gặp)
8. [Bài tập](#8-bài-tập)
9. [Tóm tắt](#9-tóm-tắt)
10. [Tài liệu tham khảo](#10-tài-liệu-tham-khảo)

---

## 1. Mục tiêu bài học

Sau bài học này bạn có thể:

- [ ] Giải thích biến là gì và khai báo biến bằng `let`.
- [ ] Phân biệt `let`, `let mut`, `const`, `static` và shadowing.
- [ ] Chọn đúng kiểu dữ liệu cơ bản: số nguyên, số thực, `bool`, `char`, tuple, array.
- [ ] Giải thích vì sao Rust mặc định coi biến là bất biến.
- [ ] Đọc hiểu và sửa các lỗi biên dịch phổ biến liên quan đến biến.

**Cách dùng tài liệu.** Chép từng ví dụ vào `src/main.rs` rồi chạy:

```bash
cd learn_03_variables
cargo run
```

Các ví dụ được đánh dấu như sau:

- Ví dụ có chú thích `// ❌` **không biên dịch được** (cố ý, để minh hoạ lỗi).
- Ví dụ có chú thích `// ⚠️` biên dịch được nhưng sinh cảnh báo.
- Các ví dụ còn lại biên dịch sạch và cho đúng kết quả ghi bên dưới.

---

## 2. Khái niệm

### 2.1 Biến là gì

Biến là một **cái tên gắn với một giá trị** được lưu trong bộ nhớ. Thông qua tên đó, chương trình đọc giá trị và, nếu được phép, thay đổi nó. Trong tài liệu của Rust, khái niệm này còn được gọi là *variable binding* (ràng buộc giữa tên và giá trị).

### 2.2 Khai báo biến với `let`

Cú pháp:

```text
let <tên>: <kiểu> = <giá trị>;
```

Phần `: <kiểu>` có thể bỏ qua khi trình biên dịch tự suy luận được kiểu.

```rust
fn main() {
    let age = 20; // kiểu được suy luận (i32)
    let height: f64 = 1.75; // kiểu được ghi tường minh

    println!("Tuổi: {age}, chiều cao: {height} m");
}
```

**Kết quả:**

```text
Tuổi: 20, chiều cao: 1.75 m
```

> **Lưu ý:** `let` là câu lệnh, chỉ dùng được bên trong hàm hoặc khối `{ ... }`, không đặt được ở phạm vi toàn cục (ngoài mọi hàm).

### 2.3 Bất biến theo mặc định

Trong Rust, biến **bất biến (immutable) theo mặc định**: đã gán giá trị thì không thể gán lại. Muốn thay đổi, phải khai báo `mut` một cách tường minh (xem [mục 3.3](#33-biến-khả-biến-mut)).

Lý do của thiết kế này:

- **An toàn:** giá trị không bị thay đổi ngoài ý muốn ở một chỗ khác trong chương trình.
- **Dễ đọc:** thấy `let` không có `mut` là biết giá trị đó cố định trong suốt đời của biến.
- **Dễ suy luận khi chương trình lớn** hoặc khi nhiều luồng cùng đọc một dữ liệu.

### 2.4 Cú pháp in giá trị dùng trong tài liệu

| Cú pháp | Ý nghĩa | Ví dụ | Kết quả |
|---|---|---|---|
| `{}` | In giá trị theo dạng thông thường | `println!("{}", 5)` | `5` |
| `{x}` | In biến `x` ngay trong chuỗi | `println!("{x}")` | giá trị của `x` |
| `{:?}` | In dạng gỡ lỗi (Debug), dùng cho tuple, array, `Option`... | `println!("{:?}", [1, 2])` | `[1, 2]` |
| `{:.2}` | Số thực với 2 chữ số thập phân | `println!("{:.2}", 2.0 / 3.0)` | `0.67` |

`{x}` chỉ nhận **tên biến**. Với biểu thức, truyền riêng: `println!("{}", x + 1)`. Có thể kết hợp tên biến với định dạng, ví dụ `{average:.1}`.

---

## 3. Các loại biến

### 3.1 Tổng quan

| Loại | Cú pháp | Gán lại được | Bắt buộc ghi kiểu | Giá trị xác định lúc | Phạm vi khai báo |
|---|---|---|---|---|---|
| Biến bất biến | `let x = 5;` | Không | Không | Chạy | Trong hàm hoặc khối |
| Biến khả biến | `let mut x = 5;` | Có (cùng kiểu) | Không | Chạy | Trong hàm hoặc khối |
| Hằng số | `const X: u32 = 5;` | Không | Có | Biên dịch | Mọi phạm vi, kể cả toàn cục |
| Biến tĩnh | `static X: u32 = 5;` | Không | Có | Biên dịch | Mọi phạm vi, kể cả toàn cục |
| Shadowing | `let x = x + 1;` | Không (tạo biến mới) | Không | Chạy | Trong hàm hoặc khối |

> `static mut` cũng tồn tại nhưng phải truy cập trong khối `unsafe` và rất dễ gây lỗi, nên nằm ngoài phạm vi bài này.

### 3.2 Biến bất biến

Biến khai báo bằng `let` (không có `mut`) chỉ được gán giá trị **một lần**.

```rust
fn main() {
    let x = 5;
    println!("x = {x}");
    x = 6; // ❌ gán lại biến bất biến
    println!("x = {x}");
}
```

**Trình biên dịch báo:**

```text
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         - first assignment to `x`
3 |     println!("x = {x}");
4 |     x = 6; // ❌ gán lại biến bất biến
  |     ^^^^^ cannot assign twice to immutable variable
  |
help: consider making this binding mutable
  |
2 |     let mut x = 5;
  |         +++
```

Rust chặn lỗi này **ngay lúc biên dịch**, nên chương trình sai sẽ không bao giờ được chạy. Thông báo còn gợi ý sẵn cách sửa: thêm `mut`.

### 3.3 Biến khả biến mut

Thêm `mut` sau `let` để cho phép gán lại giá trị.

```rust
fn main() {
    let mut x = 5;
    println!("x = {x}");

    x = 6;
    println!("x = {x}");

    x += 1;
    println!("x = {x}");
}
```

**Kết quả:**

```text
x = 5
x = 6
x = 7
```

Quy tắc cần nhớ:

- `mut` cho phép đổi **giá trị**, nhưng **không đổi được kiểu** (xem [mục 3.7](#37-mut-hay-shadowing)).
- `mut` áp dụng cho từng biến. Chỉ khai báo `mut` khi thật sự cần đổi giá trị.
- Nếu khai báo `mut` mà không bao giờ gán lại, compiler cảnh báo `variable does not need to be mutable`.

### 3.4 Hằng số const

Hằng số là giá trị **không bao giờ thay đổi** và được tính ra từ lúc biên dịch.

```rust
const SECONDS_PER_DAY: u32 = 24 * 60 * 60;

fn main() {
    println!("Một ngày có {SECONDS_PER_DAY} giây");
}
```

**Kết quả:**

```text
Một ngày có 86400 giây
```

Quy tắc:

- **Bắt buộc ghi kiểu.** Viết `const MAX = 100;` sẽ báo lỗi `missing type for 'const' item`.
- **Không dùng được** `mut`**.** `const mut X: i32 = 1;` báo lỗi `const globals cannot be mutable`.
- **Giá trị phải là biểu thức hằng**, tức tính được lúc biên dịch (số, phép toán trên các hằng khác...). Không gán được kết quả phụ thuộc dữ liệu lúc chạy, ví dụ chuỗi người dùng nhập.
- **Khai báo ở bất kỳ phạm vi nào**, kể cả ngoài `fn main` để dùng chung cho cả tệp.
- **Đặt tên theo kiểu** `SCREAMING_SNAKE_CASE`**.**
- Hằng số có hiệu lực trong phạm vi khai báo suốt thời gian chương trình chạy.

Khi nào dùng: giá trị cố định, có ý nghĩa, được dùng ở nhiều nơi (tỉ lệ thuế, giới hạn, số giây trong ngày...). Đặt tên cho chúng thay cho "số ma thuật" nằm rải rác trong code giúp việc bảo trì dễ hơn.

### 3.5 Biến tĩnh static

```rust
static LANGUAGE: &str = "Rust";

fn main() {
    println!("Đang học {LANGUAGE}");
}
```

**Kết quả:**

```text
Đang học Rust
```

Khác biệt giữa `const` và `static`:

| | `const` | `static` |
|---|---|---|
| Trong bộ nhớ | Được chèn trực tiếp vào mọi chỗ dùng | Có **một vị trí cố định** duy nhất, mọi tham chiếu trỏ về đó |
| Địa chỉ | Không có địa chỉ cố định | Có địa chỉ cố định |
| Khi nào dùng | Mặc định, cho hầu hết giá trị cố định | Khi cần đúng một vùng nhớ dùng chung |

Người mới học nên ưu tiên `const`.

### 3.6 Shadowing

Dùng `let` để khai báo **biến mới cùng tên** với biến đã có. Từ điểm đó, tên này chỉ biến mới; biến cũ bị che (shadow) và không truy cập được qua tên đó nữa.

```rust
fn main() {
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("Trong khối: x = {x}");
    }

    println!("Ngoài khối: x = {x}");
}
```

**Kết quả:**

```text
Trong khối: x = 12
Ngoài khối: x = 6
```

Diễn giải:

1. `let x = 5;` tạo `x = 5`.
2. `let x = x + 1;` tạo `x` mới bằng 6, che `x` cũ.
3. Trong khối `{ ... }`, `let x = x * 2;` tạo `x` bằng 12, chỉ có hiệu lực đến hết khối.
4. Ra khỏi khối, tên `x` lại chỉ biến có giá trị 6.

Điểm mạnh của shadowing là **đổi được kiểu** mà vẫn dùng một cái tên có nghĩa:

```rust
fn main() {
    let spaces = "   "; // &str
    let spaces = spaces.len(); // usize

    println!("Số khoảng trắng: {spaces}");
}
```

**Kết quả:**

```text
Số khoảng trắng: 3
```

Đây cũng là kỹ thuật trong dòng `let guess: u32 = ...` của trò chơi đoán số: biến `guess` kiểu `String` được che bởi `guess` kiểu `u32`.

### 3.7 mut hay shadowing

| Tiêu chí | `let mut` | Shadowing (`let` lại) |
|---|---|---|
| Bản chất | Cùng một biến, giá trị bị ghi đè | Biến mới cùng tên, biến cũ bị che |
| Đổi kiểu | Không | Có |
| Sau khi thay đổi | Biến vẫn khả biến | Biến mới có thể là bất biến |
| Khi nào dùng | Giá trị thay đổi liên tục (bộ đếm, tổng tích luỹ) | Biến đổi giá trị qua từng bước (làm sạch, đổi kiểu) rồi giữ cố định |

Thử đổi kiểu bằng `mut` sẽ lỗi:

```rust
fn main() {
    let mut spaces = "   ";
    spaces = spaces.len(); // ❌ không đổi được kiểu của biến
}
```

**Trình biên dịch báo:**

```text
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len(); // ❌ không đổi được kiểu của biến
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`
```

---

## 4. Kiểu dữ liệu

Rust là ngôn ngữ **kiểu tĩnh**: mọi biến có kiểu xác định ngay lúc biên dịch. Kiểu dữ liệu chia thành hai nhóm:

- **Vô hướng (scalar):** biểu diễn một giá trị đơn, gồm số nguyên, số thực, `bool`, `char`.
- **Phức hợp (compound):** gom nhiều giá trị, gồm tuple và array.

### 4.1 Suy luận kiểu và chú thích kiểu

Thông thường compiler tự suy ra kiểu từ giá trị. Khi chưa có thông tin khác, số nguyên mặc định là `i32` và số thực mặc định là `f64`.

```rust
fn main() {
    let a = 5;
    let b = 2.5;
    let c = true;
    let d = 'z';
    let e = "chuỗi";
    let f = (1, 2.0);
    let g = [1, 2, 3];

    println!("a: {}", std::any::type_name_of_val(&a));
    println!("b: {}", std::any::type_name_of_val(&b));
    println!("c: {}", std::any::type_name_of_val(&c));
    println!("d: {}", std::any::type_name_of_val(&d));
    println!("e: {}", std::any::type_name_of_val(&e));
    println!("f: {}", std::any::type_name_of_val(&f));
    println!("g: {}", std::any::type_name_of_val(&g));
}
```

**Kết quả:**

```text
a: i32
b: f64
c: bool
d: char
e: &str
f: (i32, f64)
g: [i32; 3]
```

> `type_name_of_val` giúp quan sát kiểu khi học. Chuỗi nó trả về chỉ mang tính tham khảo, đừng dựa vào nó trong chương trình thật.

Khi có nhiều kiểu cùng hợp lệ, compiler yêu cầu bạn **chú thích kiểu**. Ví dụ điển hình là `parse`, vì một chuỗi có thể được đổi thành nhiều kiểu số khác nhau:

```rust
fn main() {
    let guess = "42".parse().expect("Not a number!"); // ❌ thiếu kiểu
}
```

**Trình biên dịch báo:**

```text
error[E0284]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!"); // ❌ thiếu kiểu
  |         ^^^^^        ----- type must be known at this point
  |
help: consider giving `guess` an explicit type
  |
2 |     let guess: /* Type */ = "42".parse().expect("Not a number!"); // ❌ thiếu kiểu
  |              ++++++++++++
```

Cách sửa: `let guess: u32 = ...` hoặc `"42".parse::<u32>()`.

### 4.2 Số nguyên

| Độ dài | Có dấu | Không dấu |
|---|---|---|
| 8 bit | `i8` | `u8` |
| 16 bit | `i16` | `u16` |
| 32 bit | `i32` | `u32` |
| 64 bit | `i64` | `u64` |
| 128 bit | `i128` | `u128` |
| Theo kiến trúc máy | `isize` | `usize` |

- Kiểu **có dấu** (`i`) lưu số âm và dương, phạm vi từ `-(2^(n-1))` đến `2^(n-1) - 1`.
- Kiểu **không dấu** (`u`) chỉ lưu số không âm, phạm vi từ `0` đến `2^n - 1`.
- `isize` và `usize` có kích thước bằng độ rộng con trỏ của máy (64 bit trên máy 64 bit). `usize` được dùng cho chỉ số và độ dài, ví dụ `.len()` trả về `usize`.
- Mặc định, số nguyên là `i32`.

```rust
fn main() {
    println!("i8:  {} .. {}", i8::MIN, i8::MAX);
    println!("u8:  {} .. {}", u8::MIN, u8::MAX);
    println!("i32: {} .. {}", i32::MIN, i32::MAX);
    println!("u32: {} .. {}", u32::MIN, u32::MAX);
    println!("i64: {} .. {}", i64::MIN, i64::MAX);
    println!("u64: {} .. {}", u64::MIN, u64::MAX);
}
```

**Kết quả:**

```text
i8:  -128 .. 127
u8:  0 .. 255
i32: -2147483648 .. 2147483647
u32: 0 .. 4294967295
i64: -9223372036854775808 .. 9223372036854775807
u64: 0 .. 18446744073709551615
```

**Các cách viết số nguyên:**

| Dạng | Ví dụ |
|---|---|
| Thập phân | `98_222` |
| Thập lục phân | `0xff` |
| Bát phân | `0o77` |
| Nhị phân | `0b1111_0000` |
| Byte (chỉ `u8`) | `b'A'` |

Dấu `_` chỉ để dễ đọc. Có thể thêm hậu tố để chỉ định kiểu, ví dụ `57u8`.

```rust
fn main() {
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    let typed = 57u8;

    println!("{decimal} {hex} {octal} {binary} {byte} {typed}");
}
```

**Kết quả:**

```text
98222 255 63 240 65 57
```

**Phép toán số nguyên:** `+`, `-`, `*`, `/`, `%`. Phép chia số nguyên **bỏ phần thập phân** (làm tròn về 0), và dấu của phần dư `%` theo số bị chia.

```rust
fn main() {
    println!("5 / 2  = {}", 5 / 2);
    println!("-5 / 3 = {}", -5 / 3);
    println!("43 % 5 = {}", 43 % 5);
    println!("-5 % 3 = {}", -5 % 3);
}
```

**Kết quả:**

```text
5 / 2  = 2
-5 / 3 = -1
43 % 5 = 3
-5 % 3 = -2
```

> Chuyện gì xảy ra khi kết quả vượt quá phạm vi của kiểu? Xem [mục 6.5](#65-tràn-số-nguyên).

### 4.3 Số thực

Rust có hai kiểu số thực theo chuẩn IEEE-754: `f32` (32 bit) và `f64` (64 bit). Mặc định là `f64`.

```rust
fn main() {
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("{x:?} {y:?}");
    println!("{}", 0.1 + 0.2);
    println!("{}", 7.0 / 2.0);
}
```

**Kết quả:**

```text
2.0 3.0
0.30000000000000004
3.5
```

Lưu ý:

- Số thực nhị phân **không biểu diễn chính xác** nhiều số thập phân, nên `0.1 + 0.2` không đúng bằng `0.3`. Đây là đặc tính của IEEE-754, không riêng Rust.
- Với `{}`, số thực có phần thập phân bằng 0 được in không có `.0` (ví dụ `2.0` in thành `2`). Dùng `{:?}` hoặc `{:.1}` để thấy `2.0`.
- Rust **không tự trộn** số nguyên và số thực: phải viết `9.0 / 5.0`, không viết `9 / 5` (kết quả của `9 / 5` giữa hai số nguyên là `1`).

### 4.4 Boolean và ký tự

- `bool` có hai giá trị `true` và `false`, chiếm 1 byte.
- `char` là một **giá trị vô hướng Unicode** (Unicode scalar value), chiếm 4 byte, viết trong **dấu nháy đơn** (`'a'`). Chuỗi `&str` mới dùng dấu nháy kép (`"a"`).

```rust
fn main() {
    let is_learning = true;
    let is_finished: bool = false;

    let letter = 'z';
    let math_symbol: char = 'ℤ';
    let emoji = '😻';

    println!("{is_learning} {is_finished}");
    println!("{letter} {math_symbol} {emoji}");
    println!("bool: {} byte, char: {} byte", size_of::<bool>(), size_of::<char>());
}
```

**Kết quả:**

```text
true false
z ℤ 😻
bool: 1 byte, char: 4 byte
```

### 4.5 Tuple

Tuple gom **nhiều giá trị có thể khác kiểu** thành một nhóm có **độ dài cố định**. Truy cập từng phần tử bằng chỉ số `.0`, `.1`... hoặc tách (destructuring) thành các biến.

```rust
fn main() {
    let person: (&str, u8, f64) = ("An", 20, 1.75);

    let (name, age, height) = person; // destructuring
    println!("{name} - {age} tuổi - {height} m");

    println!("Tên: {}", person.0);
    println!("{:?}", person);
}
```

**Kết quả:**

```text
An - 20 tuổi - 1.75 m
Tên: An
("An", 20, 1.75)
```

Tuple rỗng `()` được gọi là **unit**. Đây là giá trị mà biểu thức "không trả về gì" thực sự trả về, ví dụ hàm `main`.

### 4.6 Array

Array gom **nhiều giá trị cùng kiểu** với **độ dài cố định, biết trước lúc biên dịch**.

- Kiểu của array viết là `[kiểu; độ dài]`, ví dụ `[i32; 5]`.
- Cú pháp `[giá_trị; số_lần]` tạo array gồm `số_lần` phần tử giống nhau.
- Chỉ số bắt đầu từ `0`.
- Cần tập hợp thay đổi được độ dài thì dùng `Vec` (sẽ học sau).

```rust
fn main() {
    let scores: [i32; 5] = [8, 7, 9, 10, 6];
    let zeros = [0; 3];

    println!("Phần tử đầu: {}", scores[0]);
    println!("Phần tử cuối: {}", scores[4]);
    println!("Độ dài: {}", scores.len());
    println!("{:?} {:?}", scores, zeros);
}
```

**Kết quả:**

```text
Phần tử đầu: 8
Phần tử cuối: 6
Độ dài: 5
[8, 7, 9, 10, 6] [0, 0, 0]
```

Truy cập **ngoài phạm vi** không bao giờ đọc sang vùng nhớ khác:

- Nếu chỉ số là hằng số và vượt phạm vi, compiler báo lỗi ngay lúc biên dịch (`index out of bounds: the length is 3 but the index is 5`).
- Nếu chỉ số chỉ biết lúc chạy (ví dụ người dùng nhập), chương trình **panic** và dừng với thông báo `index out of bounds`.

### 4.7 Ép kiểu với as

Rust không tự chuyển kiểu số này sang kiểu số khác. Dùng `as` để ép kiểu một cách tường minh.

```rust
fn main() {
    let total: i32 = 7;
    let count: i32 = 2;
    let average = total as f64 / count as f64;
    println!("{average}");

    println!("{}", 3.99_f64 as i32);
    println!("{}", 300_i32 as u8);
    println!("{}", -1_i32 as u32);
    println!("{}", 'A' as u8);
    println!("{}", 97_u8 as char);
}
```

**Kết quả:**

```text
3.5
3
44
4294967295
65
a
```

| Biểu thức | Kết quả | Giải thích |
|---|---|---|
| `3.99_f64 as i32` | `3` | Cắt bỏ phần thập phân, không làm tròn |
| `300_i32 as u8` | `44` | Giữ lại 8 bit thấp: 300 mod 256 = 44 |
| `-1_i32 as u32` | `4294967295` | Diễn giải lại cùng dãy bit (bù hai) |
| `'A' as u8` | `65` | Mã của ký tự |
| `97_u8 as char` | `a` | Ký tự ứng với mã 97 |

> **Cẩn thận:** `as` có thể **làm mất dữ liệu mà không báo lỗi** (như `300_i32 as u8`). Khi ép số thực sang số nguyên, giá trị vượt phạm vi bị giữ ở biên của kiểu đích và `NaN` thành `0`. Cần kiểm tra an toàn thì dùng `try_from` hoặc `try_into` (sẽ học sau).

---

## 5. Thực hành

Hai bài thực hành có hướng dẫn, cùng xoay quanh một hoá đơn bán hàng nhỏ. Thực hiện trong `learn_03_variables/src/main.rs` và chạy bằng `cargo run`.

### Lab 1. Số lượng hàng: let và let mut

**Mục tiêu:** thấy rõ khác biệt giữa biến bất biến và biến khả biến.

**Bước 1.** Chép và chạy chương trình:

```rust
fn main() {
    let item = "Bút bi";
    let unit_price = 5_000;
    let quantity = 3;

    let subtotal = unit_price * quantity;
    println!("{item}: {quantity} x {unit_price} = {subtotal} đồng");
}
```

**Kết quả:**

```text
Bút bi: 3 x 5000 = 15000 đồng
```

**Bước 2.** Khách mua thêm 2 cái. Thêm dòng `quantity += 2;` ngay sau dòng `let quantity = 3;` rồi chạy lại. Quan sát lỗi `E0384: cannot assign twice to immutable variable 'quantity'` và dòng `help:` gợi ý thêm `mut`.

**Bước 3.** Sửa theo gợi ý:

```rust
fn main() {
    let item = "Bút bi";
    let unit_price = 5_000;
    let mut quantity = 3;

    quantity += 2; // khách mua thêm 2 cái

    let subtotal = unit_price * quantity;
    println!("{item}: {quantity} x {unit_price} = {subtotal} đồng");
}
```

**Kết quả:**

```text
Bút bi: 5 x 5000 = 25000 đồng
```

**Suy ngẫm:** vì sao `item` và `unit_price` không cần `mut`? Vì chúng không bao giờ đổi. Việc *không* có `mut` cho người đọc biết điều đó ngay.

### Lab 2. Hoá đơn có thuế: const, shadowing, kiểu dữ liệu

**Mục tiêu:** dùng `const` cho thuế suất cố định và shadowing để biến chuỗi nhập vào thành số.

```rust
const VAT_PERCENT: u32 = 8;

fn main() {
    let item = "Bút bi";
    let unit_price: u32 = 5_000;

    // Giả lập chuỗi người dùng nhập, giống kết quả của read_line
    let quantity = "  5\n";
    let quantity = quantity.trim(); // vẫn là &str, đã bỏ khoảng trắng và '\n'
    let quantity: u32 = quantity.parse().expect("Số lượng phải là số nguyên không âm");

    let subtotal = unit_price * quantity;
    let vat = subtotal * VAT_PERCENT / 100;
    let total = subtotal + vat;

    println!("{item}: {quantity} x {unit_price} = {subtotal} đồng");
    println!("VAT {VAT_PERCENT}%: {vat} đồng");
    println!("Tổng cộng: {total} đồng");
}
```

**Kết quả:**

```text
Bút bi: 5 x 5000 = 25000 đồng
VAT 8%: 2000 đồng
Tổng cộng: 27000 đồng
```

**Phân tích:**

- `quantity` được shadow **hai lần**: `&str` còn `\n` → `&str` đã `trim` → `u32`. Nhờ vậy ta giữ một cái tên có nghĩa thay vì `quantity_str`, `quantity_trimmed`, `quantity_num`.
- `VAT_PERCENT` là `const` vì là giá trị cố định, biết trước lúc biên dịch.
- `unit_price` và `quantity` cùng kiểu `u32` nên phép nhân hợp lệ. Trộn hai kiểu số khác nhau sẽ lỗi (xem [mục 4.7](#47-ép-kiểu-với-as)).

**Thử thêm:**

1. Đổi `VAT_PERCENT` thành `10` và dự đoán tổng cộng trước khi chạy (đáp án: `27500`).
2. Đổi chuỗi `"  5\n"` thành `"abc"`. Chương trình panic với thông báo `Số lượng phải là số nguyên không âm: ParseIntError { kind: InvalidDigit }`. Đây là nhánh lỗi của `Result` mà `expect` xử lý bằng cách dừng chương trình.

---

## 6. Tìm hiểu chi tiết

### 6.1 Phạm vi (scope)

Biến có hiệu lực **từ chỗ khai báo đến hết khối** `{ ... }` chứa nó. Ra khỏi khối, biến không còn dùng được.

```rust
fn main() {
    let outer = 1;

    {
        let inner = 2;
        println!("{outer} {inner}");
    }

    println!("{outer}");
}
```

**Kết quả:**

```text
1 2
1
```

Dùng `inner` ở ngoài khối sẽ lỗi:

```rust
fn main() {
    {
        let secret = 42;
    }
    println!("{secret}"); // ❌ secret đã hết phạm vi
}
```

**Trình biên dịch báo:**

```text
error[E0425]: cannot find value `secret` in this scope
 --> src/main.rs:5:16
  |
5 |     println!("{secret}"); // ❌ secret đã hết phạm vi
  |                ^^^^^^
  |
help: the binding `secret` is available in a different scope in the same function
 --> src/main.rs:3:13
  |
3 |         let secret = 42;
  |             ^^^^^^
```

Khi biến ra khỏi phạm vi, Rust tự giải phóng giá trị của nó. Cơ chế này gọi là *ownership* và sẽ được học ở chương sau.

### 6.2 Khai báo trước, gán sau

Biến có thể khai báo trước và gán giá trị sau, miễn là **mọi đường đi** của chương trình đều gán trước khi dùng. Biến bất biến vẫn chỉ được gán **một lần** trên mỗi đường đi.

```rust
fn main() {
    let discount: u32;
    let is_member = true;

    if is_member {
        discount = 10;
    } else {
        discount = 0;
    }

    println!("Giảm giá: {discount}%");
}
```

**Kết quả:**

```text
Giảm giá: 10%
```

Dùng biến khi chưa được gán sẽ lỗi:

```rust
fn main() {
    let n: i32;
    println!("{n}"); // ❌ n chưa có giá trị
}
```

**Trình biên dịch báo:**

```text
error[E0381]: used binding `n` isn't initialized
 --> src/main.rs:3:16
  |
2 |     let n: i32;
  |         - binding declared here but left uninitialized
3 |     println!("{n}"); // ❌ n chưa có giá trị
  |                ^ `n` used here but it isn't initialized
  |
help: consider assigning a value
  |
2 |     let n: i32 = 42;
  |                ++++
```

### 6.3 let với mẫu (pattern) và việc bỏ qua giá trị

`let` thật ra nhận một **mẫu** ở vế trái, nên có thể tách tuple hoặc array thành nhiều biến cùng lúc.

```rust
fn main() {
    let (x, y) = (3, 4);
    let [first, _, third] = [10, 20, 30];
    let (name, _) = ("An", 20);

    println!("{x} {y} {first} {third} {name}");
}
```

**Kết quả:**

```text
3 4 10 30 An
```

- `_` bỏ qua hoàn toàn một giá trị, không tạo biến.
- Tên bắt đầu bằng `_` (như `_unused`) **vẫn tạo biến** nhưng tắt cảnh báo "chưa dùng".

### 6.4 Quy ước đặt tên

| Đối tượng | Quy ước | Ví dụ |
|---|---|---|
| Biến, hàm | `snake_case` | `unit_price` |
| Hằng số, `static` | `SCREAMING_SNAKE_CASE` | `VAT_PERCENT` |
| Kiểu (struct, enum) | `UpperCamelCase` | `Student` |

Quy tắc về tên:

- Không bắt đầu bằng chữ số (xem lỗi khi chạy `cargo new 3variables`).
- Không trùng **từ khoá** của Rust. Từ edition 2024, `gen` cũng là từ khoá dành riêng, nên `let gen = 5;` bị lỗi `expected identifier, found reserved keyword 'gen'`. Nếu bắt buộc dùng, viết `r#gen` (raw identifier).
- Vi phạm quy ước đặt tên không gây lỗi nhưng compiler sẽ cảnh báo (xem [mục 7.3](#73-cảnh-báo-thường-gặp)).

### 6.5 Tràn số nguyên

Mỗi kiểu số nguyên có phạm vi giới hạn. Khi kết quả phép tính vượt phạm vi, ta gọi là **tràn số (overflow)**.

Nếu compiler thấy trước phép tính chắc chắn tràn, nó báo lỗi ngay lúc biên dịch:

```rust
fn main() {
    let x: u8 = 255;
    let y = x + 1; // ❌ u8 chỉ chứa tối đa 255
    println!("{y}");
}
```

**Trình biên dịch báo:**

```text
error: this arithmetic operation will overflow
 --> src/main.rs:3:13
  |
3 |     let y = x + 1; // ❌ u8 chỉ chứa tối đa 255
  |             ^^^^^ attempt to compute `u8::MAX + 1_u8`, which would overflow
  |
  = note: `#[deny(arithmetic_overflow)]` on by default
```

Khi giá trị chỉ biết lúc chạy (ví dụ người dùng nhập), hành vi phụ thuộc chế độ biên dịch:

| Chế độ | Lệnh | Khi tràn số |
|---|---|---|
| Debug | `cargo run`, `cargo build` | **Panic** với thông báo `attempt to add with overflow` |
| Release | `cargo run --release` | **Không panic**, giá trị quấn vòng (`255u8 + 1` thành `0`) |

Không nên dựa vào việc quấn vòng ngầm định. Hãy chọn phương thức thể hiện đúng ý định:

| Phương thức | Khi tràn số | Ví dụ `255u8 + 1` |
|---|---|---|
| `wrapping_add` | Quấn vòng | `0` |
| `checked_add` | Trả về `None` | `None` |
| `saturating_add` | Giữ ở giá trị biên | `255` |
| `overflowing_add` | Trả về cặp (giá trị quấn vòng, có tràn không) | `(0, true)` |

```rust
fn main() {
    let x: u8 = 255;

    println!("{}", x.wrapping_add(1));
    println!("{:?}", x.checked_add(1));
    println!("{}", x.saturating_add(1));
    println!("{:?}", x.overflowing_add(1));
}
```

**Kết quả:**

```text
0
None
255
(0, true)
```

Các phép trừ, nhân có phương thức tương ứng (`wrapping_sub`, `checked_mul`...).

### 6.6 Chọn công cụ nào?

| Tình huống | Chọn |
|---|---|
| Giá trị cố định, biết lúc biên dịch, dùng ở nhiều nơi | `const` |
| Giá trị tính ra một lần rồi không đổi | `let` |
| Giá trị cần thay đổi (bộ đếm, tổng tích luỹ) | `let mut` |
| Biến đổi giá trị qua nhiều bước hoặc cần đổi kiểu | Shadowing |
| Cần một vùng nhớ toàn cục có địa chỉ cố định (hiếm) | `static` |

---

## 7. Lỗi thường gặp

### 7.1 Cách đọc thông báo lỗi

Lấy lỗi E0384 ở [mục 3.2](#32-biến-bất-biến) làm ví dụ:

| Phần | Ý nghĩa |
|---|---|
| `error[E0384]` | Loại thông báo (`error`) và mã lỗi. Tra cứu bằng `rustc --explain E0384` |
| `cannot assign twice to immutable variable` | Mô tả ngắn gọn lỗi |
| `--> src/main.rs:4:5` | Vị trí lỗi: tệp, dòng 4, cột 5 |
| `^^^^^` và nhãn đi kèm | Đoạn mã gây lỗi và giải thích |
| `help:` | Gợi ý sửa cụ thể, thường có thể làm theo trực tiếp |

### 7.2 Lỗi biên dịch

| Mã lỗi | Thông báo | Nguyên nhân | Cách sửa |
|---|---|---|---|
| `E0384` | `cannot assign twice to immutable variable` | Gán lại biến không có `mut` | Thêm `mut`, hoặc dùng shadowing |
| `E0308` | `mismatched types` | Gán giá trị khác kiểu vào biến | Dùng đúng kiểu, dùng shadowing, hoặc ép kiểu bằng `as` |
| `E0381` | `used binding isn't initialized` | Dùng biến khi chưa gán giá trị | Gán giá trị trên mọi đường đi trước khi dùng |
| `E0425` | `cannot find value in this scope` | Dùng biến ngoài phạm vi hoặc sai tên | Khai báo đúng phạm vi, kiểm tra chính tả |
| `E0284` | `type annotations needed` | Compiler không suy ra được kiểu (ví dụ `parse`) | Ghi kiểu: `let n: u32 = ...` hoặc `parse::<u32>()` |
| `E0070` | `invalid left-hand side of assignment` | Gán giá trị cho hằng số | Hằng không đổi được; dùng `let mut` nếu cần đổi |
| Không có mã | `missing type for 'const' item` | `const` thiếu kiểu | Viết `const MAX: u32 = 100;` |
| Không có mã | `const globals cannot be mutable` | Viết `const mut` | Bỏ `mut` (hằng số không đổi được) |
| Không có mã | `expected identifier, found reserved keyword` | Đặt tên trùng từ khoá | Đổi tên, hoặc dùng `r#tên` |

> Mã lỗi cho cùng một tình huống có thể khác nhau giữa các phiên bản compiler. Ví dụ lỗi thiếu kiểu của `parse` có thể là `E0282` hoặc `E0284`. Bảng trên là kết quả với Rust 1.99.1.

### 7.3 Cảnh báo thường gặp

Cảnh báo (warning) không chặn việc biên dịch nhưng nên được sửa hết.

```rust
// ⚠️ Biên dịch được nhưng có 3 cảnh báo
fn main() {
    let unused = 1;
    let mut counter = 0;
    let myVar = 3;

    println!("{counter} {myVar}");
}
```

| Cảnh báo | Ý nghĩa | Cách xử lý |
|---|---|---|
| `unused variable: 'unused'` | Khai báo mà không dùng | Xoá đi, hoặc đổi thành `_unused` nếu cố ý |
| `variable does not need to be mutable` | Có `mut` nhưng không bao giờ gán lại | Bỏ `mut` |
| `variable 'myVar' should have a snake case name` | Sai quy ước đặt tên biến | Đổi thành `my_var` |
| `constant 'maxValue' should have an upper case name` | Sai quy ước đặt tên hằng | Đổi thành `MAX_VALUE` |

---

## 8. Bài tập

Hãy tự làm trước, chạy bằng `cargo run`, rồi mới mở phần đáp án. Mỗi bài có **kết quả mong đợi** để bạn tự kiểm tra.

### Cấp độ 1. Làm quen

#### Bài 1.1. Sửa lỗi gán lại biến

**Đề:** đoạn mã sau không biên dịch được. Sửa theo **hai cách khác nhau**: một cách dùng `mut`, một cách dùng shadowing.

```rust
fn main() {
    let score = 70;
    println!("Điểm ban đầu: {score}");
    score = 85; // ❌ gán lại biến bất biến
    println!("Điểm sau khi thi lại: {score}");
}
```

**Kết quả mong đợi:**

```text
Điểm ban đầu: 70
Điểm sau khi thi lại: 85
```

**Gợi ý:** đọc dòng `help:` trong thông báo lỗi.

Xem đáp án

**Cách 1:** dùng `mut`.

```rust
fn main() {
    let mut score = 70;
    println!("Điểm ban đầu: {score}");
    score = 85;
    println!("Điểm sau khi thi lại: {score}");
}
```

**Cách 2:** dùng shadowing.

```rust
fn main() {
    let score = 70;
    println!("Điểm ban đầu: {score}");
    let score = 85;
    println!("Điểm sau khi thi lại: {score}");
}
```

Cả hai cho cùng kết quả. Cách 1 ghi đè giá trị của cùng một biến, cách 2 tạo một biến mới che biến cũ.



#### Bài 1.2. Hằng số

**Đề:** khai báo hằng `SECONDS_PER_DAY` (kiểu `u32`) bằng `24 * 60 * 60`, và hằng `SECONDS_PER_WEEK` được **tính từ hằng đầu tiên**. In ra hai dòng như bên dưới.

**Kết quả mong đợi:**

```text
Một ngày có 86400 giây
Một tuần có 604800 giây
```

**Gợi ý:** hằng số có thể dùng hằng số khác trong biểu thức vì tính được lúc biên dịch.

Xem đáp án

```rust
const SECONDS_PER_DAY: u32 = 24 * 60 * 60;
const SECONDS_PER_WEEK: u32 = SECONDS_PER_DAY * 7;

fn main() {
    println!("Một ngày có {SECONDS_PER_DAY} giây");
    println!("Một tuần có {SECONDS_PER_WEEK} giây");
}
```



#### Bài 1.3. Chọn kiểu dữ liệu

**Đề:** khai báo bốn biến với kiểu được ghi tường minh, chọn kiểu **phù hợp nhất** cho từng giá trị: `age` (tuổi, 20), `height` (chiều cao tính bằng mét, 1.75), `is_student` (`true`), `grade` (xếp loại, `'A'`). In ra một dòng.

**Kết quả mong đợi:**

```text
age=20, height=1.75, is_student=true, grade=A
```

Xem đáp án

```rust
fn main() {
    let age: u8 = 20;
    let height: f64 = 1.75;
    let is_student: bool = true;
    let grade: char = 'A';

    println!("age={age}, height={height}, is_student={is_student}, grade={grade}");
}
```

Giải thích lựa chọn: tuổi không âm và nhỏ nên `u8` là đủ (các kiểu số nguyên khác như `u32` cũng chấp nhận được); chiều cao có phần thập phân nên cần `f64`; xếp loại là một ký tự nên dùng `char`, viết trong dấu nháy đơn.



### Cấp độ 2. Vận dụng

#### Bài 2.1. Dự đoán kết quả

**Đề:** đọc đoạn mã sau và **dự đoán** ba dòng in ra trước khi chạy.

```rust
fn main() {
    let x = 10;
    let x = x + 5;

    {
        let x = x * 2;
        println!("Trong khối: {x}");
    }

    println!("Ngoài khối: {x}");

    let x = "xong";
    println!("Cuối cùng: {x}");
}
```

Xem đáp án

**Kết quả:**

```text
Trong khối: 30
Ngoài khối: 15
Cuối cùng: xong
```

Giải thích: `x` lần lượt là 10, rồi 15. Trong khối, `x` bị che bởi giá trị `15 * 2 = 30`, nhưng chỉ đến hết khối. Ngoài khối, `x` vẫn là 15. Dòng cuối cùng shadow `x` thành kiểu `&str`, điều mà `mut` không làm được.



#### Bài 2.2. Shadowing để đổi kiểu

**Đề:** cho chuỗi `"  42\n"`. Chỉ dùng **một tên biến** `number` (nhờ shadowing), hãy cắt khoảng trắng, đổi thành `u32`, cộng thêm 8 rồi in.

**Kết quả mong đợi:**

```text
Kết quả: 50
```

**Gợi ý:** dùng `trim()` rồi `parse()`, nhớ chú thích kiểu `u32`.

Xem đáp án

```rust
fn main() {
    let number = "  42\n";
    let number = number.trim();
    let number: u32 = number.parse().expect("Không phải số");
    let number = number + 8;

    println!("Kết quả: {number}");
}
```



#### Bài 2.3. Tuple và destructuring

**Đề:** cho `let book = ("Lập trình Rust", 2024, 99.5);`. Tách tuple thành ba biến `title`, `year`, `price` rồi in theo mẫu.

**Kết quả mong đợi:**

```text
Sách: Lập trình Rust (2024) - giá 99.5 nghìn đồng
```

Xem đáp án

```rust
fn main() {
    let book = ("Lập trình Rust", 2024, 99.5);
    let (title, year, price) = book;

    println!("Sách: {title} ({year}) - giá {price} nghìn đồng");
}
```



#### Bài 2.4. Array và ép kiểu

**Đề:** cho `let scores = [8, 7, 9, 10, 5];`. Tính **tổng** và **trung bình** (làm tròn một chữ số thập phân). Chưa dùng vòng lặp, cứ cộng từng phần tử.

**Kết quả mong đợi:**

```text
Tổng: 39
Trung bình: 7.8
```

**Gợi ý:** `scores.len()` trả về `usize`; dùng `as f64` trước khi chia và `{average:.1}` để in.

Xem đáp án

```rust
fn main() {
    let scores = [8, 7, 9, 10, 5];

    let sum = scores[0] + scores[1] + scores[2] + scores[3] + scores[4];
    let average = sum as f64 / scores.len() as f64;

    println!("Tổng: {sum}");
    println!("Trung bình: {average:.1}");
}
```



#### Bài 2.5. Tràn số an toàn

**Đề:** cho `a: u8 = 250` và `b: u8 = 10`. Cộng chúng theo ba cách `wrapping_add`, `checked_add`, `saturating_add` và in kết quả.

**Kết quả mong đợi:**

```text
wrapping_add: 4
checked_add: None
saturating_add: 255
```

**Gợi ý:** `checked_add` trả về `Option`, in bằng `{:?}`.

Xem đáp án

```rust
fn main() {
    let a: u8 = 250;
    let b: u8 = 10;

    println!("wrapping_add: {}", a.wrapping_add(b));
    println!("checked_add: {:?}", a.checked_add(b));
    println!("saturating_add: {}", a.saturating_add(b));
}
```

Giải thích: `250 + 10 = 260`, vượt `255`. Quấn vòng cho `260 - 256 = 4`, `checked_add` báo `None`, còn `saturating_add` dừng ở giá trị lớn nhất của `u8` là `255`.



### Cấp độ 3. Thử thách

#### Bài 3.1. Chuyển đổi nhiệt độ

**Đề:** với `celsius = 36.6`, tính độ Fahrenheit theo công thức `F = C × 9 / 5 + 32`. Khai báo số `32.0` là một **hằng số** `FAHRENHEIT_OFFSET`. In kết quả với hai chữ số thập phân.

**Kết quả mong đợi:**

```text
36.6°C = 97.88°F
```

**Gợi ý:** viết `9.0 / 5.0`, không viết `9 / 5`. Phép chia hai số nguyên `9 / 5` cho kết quả `1`, và Rust không cho trộn số nguyên với số thực.

Xem đáp án

```rust
const FAHRENHEIT_OFFSET: f64 = 32.0;

fn main() {
    let celsius = 36.6;
    let fahrenheit = celsius * 9.0 / 5.0 + FAHRENHEIT_OFFSET;

    println!("{celsius}°C = {fahrenheit:.2}°F");
}
```



#### Bài 3.2. Bốn đoạn mã, bốn lỗi

**Đề:** mỗi đoạn mã dưới đây đều không biên dịch được. Với từng đoạn, hãy **dự đoán lỗi** trước khi chạy `cargo check`, rồi sửa.

**Đoạn (a):**

```rust
fn main() {
    let mut count = 0;
    count = "một"; // ❌
}
```

**Đoạn (b):**

```rust
fn main() {
    let total: i32;
    let doubled = total * 2; // ❌
    println!("{doubled}");
}
```

**Đoạn (c):**

```rust
const LIMIT = 10; // ❌

fn main() {
    println!("{LIMIT}");
}
```

**Đoạn (d):**

```rust
fn main() {
    {
        let secret = 42;
    }
    println!("{secret}"); // ❌
}
```

Xem đáp án

| Đoạn | Lỗi | Nguyên nhân | Cách sửa |
|---|---|---|---|
| (a) | `E0308` mismatched types | Biến `count` kiểu số nguyên, không nhận được chuỗi `"một"` | Gán số nguyên (`count = 1;`), hoặc dùng shadowing `let count = "một";` nếu muốn đổi kiểu |
| (b) | `E0381` used binding isn't initialized | `total` chưa có giá trị khi được dùng | Gán giá trị trước: `let total: i32 = 21;` |
| (c) | `missing type for 'const' item` | `const` bắt buộc phải ghi kiểu | `const LIMIT: u32 = 10;` |
| (d) | `E0425` cannot find value in this scope | `secret` chỉ tồn tại trong khối bên trong | Đưa `println!` vào cùng khối, hoặc khai báo `secret` ở ngoài khối |



#### Bài 3.3. Đổi giây sang giờ, phút, giây

**Đề:** cho `let total_seconds: u32 = 3725;`. Tính số giờ, số phút, số giây còn lại rồi in theo mẫu. Chỉ dùng phép chia `/` và phép lấy dư `%` trên số nguyên.

**Kết quả mong đợi:**

```text
1 giờ 2 phút 5 giây
```

**Gợi ý:** một giờ có 3600 giây. Số phút là phần dư sau khi bỏ đi các giờ trọn, chia cho 60.

Xem đáp án

```rust
fn main() {
    let total_seconds: u32 = 3725;

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    println!("{hours} giờ {minutes} phút {seconds} giây");
}
```

Giải thích: `3725 / 3600 = 1` giờ, phần dư `3725 % 3600 = 125` giây, `125 / 60 = 2` phút, và `3725 % 60 = 5` giây.



---

## 9. Tóm tắt

| Chủ đề | Ghi nhớ |
|---|---|
| Mặc định | Biến là **bất biến**; gán lại gây lỗi `E0384` |
| Cho phép đổi giá trị | `let mut`, nhưng **không đổi được kiểu** |
| Hằng số | `const TÊN: Kiểu = ...;`, bắt buộc ghi kiểu, giá trị tính lúc biên dịch, đặt tên `SCREAMING_SNAKE_CASE` |
| `static` | Một vùng nhớ cố định dùng chung; ưu tiên `const` khi mới học |
| Shadowing | `let` lại cùng tên, tạo biến mới, **đổi được kiểu** |
| Phạm vi | Biến sống đến hết khối `{ ... }` chứa nó |
| Số nguyên | Mặc định `i32`; có dấu `iN`, không dấu `uN`; `usize` cho chỉ số và độ dài |
| Số thực | Mặc định `f64`; không chính xác tuyệt đối (`0.1 + 0.2`) |
| Tràn số | Debug thì panic, release thì quấn vòng; dùng `checked_*`, `saturating_*`, `wrapping_*` để rõ ý |
| Tuple và array | Tuple: khác kiểu; array: cùng kiểu; cả hai có độ dài cố định |
| Ép kiểu | Dùng `as`, có thể làm mất dữ liệu |
| Đặt tên | Biến `snake_case`, hằng `SCREAMING_SNAKE_CASE`; không bắt đầu bằng chữ số |

---

## 10. Tài liệu tham khảo

- [The Rust Programming Language, 3.1 Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
- [The Rust Programming Language, 3.2 Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)
- [Rust By Example, Variable Bindings](https://doc.rust-lang.org/rust-by-example/variable_bindings.html)
- [The Rust Reference, Constant items](https://doc.rust-lang.org/reference/items/constant-items.html)
- [The Rust Reference, Static items](https://doc.rust-lang.org/reference/items/static-items.html)
- [Rust API Guidelines, Naming](https://rust-lang.github.io/api-guidelines/naming.html)
- [Tài liệu thư viện chuẩn, các phương thức của số nguyên (](https://doc.rust-lang.org/std/primitive.u8.html)`u8`[)](https://doc.rust-lang.org/std/primitive.u8.html)
- [Tài liệu thư viện chuẩn,](https://doc.rust-lang.org/std/fmt/index.html) `std::fmt` [(định dạng chuỗi)](https://doc.rust-lang.org/std/fmt/index.html)
- [Tra cứu mã lỗi, ví dụ E0384](https://doc.rust-lang.org/error_codes/E0384.html)
