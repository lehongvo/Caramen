# Hàm và điều khiển luồng (Functions & Control Flow) trong Rust

> Tài liệu học tập về hàm và điều khiển luồng trong Rust: khai báo hàm, tham số, giá trị trả về, rẽ nhánh `if`, vòng lặp `loop`, `while`, `for`, thực hành và bài tập có đáp án.

| Thông tin | Chi tiết |
|---|---|
| Chủ đề | Hàm, câu lệnh và biểu thức, `if/else`, `loop`, `while`, `for`, chú thích |
| Môi trường | Rust 1.99.1 (stable), Cargo 1.99.1, edition 2024 |
| Dự án | `learn_04_functions_control_flow` (thư mục này) |
| Bài trước | [Biến (Variables)](../learn_03_variables/README.md) |
| Nguồn tham khảo chính | *The Rust Programming Language*, chương 3 (mục 3.3, 3.4 và 3.5) |

## Mục lục

1. [Mục tiêu bài học](#1-mục-tiêu-bài-học)
2. [Khái niệm: hàm](#2-khái-niệm-hàm)
3. [Rẽ nhánh với if](#3-rẽ-nhánh-với-if)
4. [Vòng lặp](#4-vòng-lặp)
5. [Thực hành](#5-thực-hành)
6. [Tìm hiểu chi tiết](#6-tìm-hiểu-chi-tiết)
7. [Lỗi thường gặp](#7-lỗi-thường-gặp)
8. [Bài tập](#8-bài-tập)
9. [Tóm tắt](#9-tóm-tắt)
10. [Tài liệu tham khảo](#10-tài-liệu-tham-khảo)

---

## 1. Mục tiêu bài học

Sau bài học này bạn có thể:

- [ ] Khai báo và gọi hàm bằng `fn`, ghi đúng kiểu tham số và kiểu giá trị trả về.
- [ ] Phân biệt **câu lệnh** (statement) và **biểu thức** (expression), và hiểu vì sao dấu `;` ở cuối dòng làm đổi kiểu trả về của hàm.
- [ ] Dùng `if`, `else if`, `else`, kể cả `if` như một biểu thức cho ra giá trị.
- [ ] Chọn đúng vòng lặp `loop`, `while` hay `for`; dùng `break`, `continue` và nhãn vòng lặp.
- [ ] Duyệt dãy số và mảng bằng range, `.iter()`, `.enumerate()`, `.rev()`.
- [ ] Viết chú thích `//` và chú thích tài liệu `///`.
- [ ] Đọc hiểu và sửa các lỗi biên dịch phổ biến liên quan đến hàm và luồng điều khiển.

**Kiến thức cần có:** bài [Biến (Variables)](../learn_03_variables/README.md), gồm `let`, `let mut`, kiểu số nguyên, `bool`, tuple và array.

**Cách dùng tài liệu.** Chép từng ví dụ vào `src/main.rs` rồi chạy:

```bash
cd learn_04_functions_control_flow
cargo run
```

Các ví dụ được đánh dấu như sau:

- Ví dụ có chú thích `// ❌` **không biên dịch được** (cố ý, để minh hoạ lỗi).
- Ví dụ có chú thích `// ⚠️` biên dịch được nhưng sinh cảnh báo.
- Các ví dụ còn lại biên dịch sạch và cho đúng kết quả ghi bên dưới.

---

## 2. Khái niệm: hàm

### 2.1 Hàm là gì

Hàm là một **khối mã có tên**, nhận đầu vào (tham số) và có thể trả về một kết quả. Bạn đã dùng hàm từ bài đầu tiên: `main` là hàm đặc biệt, nơi chương trình bắt đầu chạy.

Tách chương trình thành các hàm giúp:

- **Tránh lặp lại:** viết một lần, gọi nhiều lần.
- **Dễ đọc:** tên hàm nói rõ đoạn mã làm gì.
- **Dễ kiểm tra và sửa:** mỗi hàm chỉ lo một việc.

### 2.2 Khai báo và gọi hàm

Cú pháp:

```text
fn <tên>(<tham số>: <kiểu>, ...) -> <kiểu trả về> {
    <thân hàm>
}
```

- `fn` là từ khoá khai báo hàm.
- Tên hàm đặt theo kiểu `snake_case` (xem [mục 6.6](#66-quy-ước-đặt-tên)).
- Phần `-> <kiểu trả về>` bỏ qua được nếu hàm không trả về giá trị nào.

```rust
fn main() {
    println!("Bắt đầu");
    say_hello();
    say_hello();
}

fn say_hello() {
    println!("Xin chào từ một hàm khác!");
}
```

**Kết quả:**

```text
Bắt đầu
Xin chào từ một hàm khác!
Xin chào từ một hàm khác!
```

Ghi chú:

- Rust **không quan tâm thứ tự**: `say_hello` được định nghĩa *sau* `main` nhưng vẫn gọi được. Không cần khai báo trước như C.
- `println!` có dấu `!` vì nó là **macro**, không phải hàm thường. Macro sẽ được giới thiệu ở các bài sau.

### 2.3 Tham số

Hàm nhận dữ liệu từ bên ngoài qua **tham số** (parameter). **Bắt buộc ghi kiểu** cho từng tham số.

```rust
fn main() {
    print_total(3, 5_000);
}

fn print_total(quantity: u32, unit_price: u32) {
    let total = quantity * unit_price;
    println!("{quantity} x {unit_price} = {total} đồng");
}
```

**Kết quả:**

```text
3 x 5000 = 15000 đồng
```

| Thuật ngữ | Nghĩa | Trong ví dụ |
|---|---|---|
| Tham số (parameter) | Tên khai báo trong chữ ký hàm | `quantity`, `unit_price` |
| Đối số (argument) | Giá trị cụ thể truyền vào khi gọi | `3`, `5_000` |

Quên ghi kiểu của tham số sẽ lỗi:

```rust
fn add(a, b) -> i32 { // ❌ tham số thiếu kiểu
    a + b
}

fn main() {
    println!("{}", add(1, 2));
}
```

**Trình biên dịch báo:**

```text
error: expected one of `:`, `@`, or `|`, found `,`
 --> src/main.rs:1:9
  |
1 | fn add(a, b) -> i32 { // ❌ tham số thiếu kiểu
  |         ^ expected one of `:`, `@`, or `|`
  |
help: if this is a parameter name, give it a type
  |
1 | fn add(a: TypeName, b) -> i32 { // ❌ tham số thiếu kiểu
  |         ++++++++++
help: if this is a type, explicitly ignore the parameter name
  |
1 | fn add(_: a, b) -> i32 { // ❌ tham số thiếu kiểu
  |        ++
```

Trình biên dịch báo hai lỗi, mỗi tham số một lỗi; trên đây là lỗi đầu tiên. Chữ ký hàm là "hợp đồng" giữa nơi gọi và thân hàm nên phải ghi tường minh. Trong thân hàm, biến cục bộ vẫn được suy luận kiểu như bình thường.

### 2.4 Giá trị trả về

Khai báo kiểu trả về sau mũi tên `->`. **Biểu thức cuối cùng của thân hàm, không có dấu `;`**, chính là giá trị trả về.

```rust
fn main() {
    let result = add(2, 3);
    println!("2 + 3 = {result}");
    println!("Bình phương của 7 là {}", square(7));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn square(x: i32) -> i32 {
    x * x
}
```

**Kết quả:**

```text
2 + 3 = 5
Bình phương của 7 là 49
```

Hàm không khai báo `->` trả về **unit** `()` (tuple rỗng), giá trị mà bạn đã gặp ở bài trước. Đây là kiểu trả về của `main` và của mọi hàm "chỉ làm việc, không cho kết quả".

```rust
fn say_hello() {
    println!("Hi");
}

fn main() {
    let nothing = say_hello();
    println!("{:?}", nothing);
}
```

**Kết quả:**

```text
Hi
()
```

**Lỗi kinh điển của người mới:** thêm dấu `;` vào biểu thức cuối.

```rust
fn plus_one(x: i32) -> i32 {
    x + 1; // ❌ dấu ; biến biểu thức thành câu lệnh
}

fn main() {
    println!("{}", plus_one(5));
}
```

**Trình biên dịch báo:**

```text
error[E0308]: mismatched types
 --> src/main.rs:1:24
  |
1 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
2 |     x + 1; // ❌ dấu ; biến biểu thức thành câu lệnh
  |          - help: remove this semicolon to return this value
```

Thông báo chỉ thẳng vấn đề: hàm hứa trả về `i32` nhưng thân hàm không có biểu thức cuối nên trả về `()`, và `help` gợi ý xoá dấu `;`. Lý do nằm ở [mục 2.5](#25-câu-lệnh-và-biểu-thức).

### 2.5 Câu lệnh và biểu thức

Rust là ngôn ngữ **dựa trên biểu thức**. Hai khái niệm cần phân biệt:

| | Câu lệnh (statement) | Biểu thức (expression) |
|---|---|---|
| Làm gì | Thực hiện một hành động | Tính ra một **giá trị** |
| Có giá trị trả về | Không | Có |
| Ví dụ | `let y = 6;` | `5 + 6`, `add(1, 2)`, `{ ... }`, `if ... else ...` |

Biểu thức có thể đứng ở vế phải của `let`, làm đối số, hay làm giá trị trả về. Câu lệnh thì không. Cả **khối `{ ... }`** cũng là biểu thức: giá trị của nó là biểu thức cuối cùng trong khối.

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // không có dấu ; nên đây là giá trị của cả khối
    };

    println!("y = {y}");
}
```

**Kết quả:**

```text
y = 4
```

Muốn dùng `let` như một giá trị thì không được:

```rust
fn main() {
    let y = (let x = 6); // ❌ let là câu lệnh, không có giá trị
}
```

**Trình biên dịch báo:**

```text
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let y = (let x = 6); // ❌ let là câu lệnh, không có giá trị
  |              ^^^
  |
  = note: only supported directly in conditions of `if` and `while` expressions
```

**Quy tắc then chốt:** thêm dấu `;` vào cuối một biểu thức sẽ biến nó thành câu lệnh và **bỏ đi giá trị** của nó. Vì vậy `x + 1` trả về một số, còn `x + 1;` thì không trả về gì (`()`). Đó là nguyên nhân của lỗi ở [mục 2.4](#24-giá-trị-trả-về).

### 2.6 return sớm

Dùng `return` để **thoát hàm giữa chừng**, thường để xử lý trường hợp đặc biệt trước.

```rust
fn main() {
    println!("{}", sign(-5));
    println!("{}", sign(0));
    println!("{}", sign(8));
}

fn sign(n: i32) -> i32 {
    if n < 0 {
        return -1;
    }
    if n == 0 {
        return 0;
    }
    1 // biểu thức cuối: giá trị trả về khi không gặp return nào
}
```

**Kết quả:**

```text
-1
0
1
```

Quy ước phong cách trong Rust:

- Kết thúc hàm bằng **biểu thức cuối** thay vì `return x;`.
- Chỉ dùng `return` khi cần **thoát sớm**.
- Hàm trả về `()` thoát sớm bằng `return;` (không có giá trị).

### 2.7 Chú thích

Chú thích là phần chữ trình biên dịch bỏ qua, dành cho người đọc.

| Cú pháp | Dùng cho | Ghi chú |
|---|---|---|
| `// ...` | Chú thích một dòng | Phổ biến nhất |
| `/* ... */` | Chú thích nhiều dòng | Ít dùng |
| `/// ...` | **Chú thích tài liệu** cho mục đứng ngay sau nó (hàm, struct...) | Viết bằng Markdown, `cargo doc` biến thành trang tài liệu |
| `//! ...` | Chú thích tài liệu cho mục bao ngoài (đầu tệp hoặc đầu crate) | Đặt ở đầu tệp |

```rust
/// Tính diện tích hình chữ nhật.
///
/// `width` và `height` tính bằng mét.
fn area(width: u32, height: u32) -> u32 {
    // Diện tích = dài x rộng
    width * height
}

fn main() {
    /* Chú thích
    nhiều dòng */
    println!("Diện tích: {} m2", area(4, 5));
}
```

**Kết quả:**

```text
Diện tích: 20 m2
```

Lời khuyên: chú thích nên giải thích **vì sao** làm vậy, không nhắc lại **làm gì** (code đã nói rồi). Chạy `cargo doc --open` để xem trang tài liệu sinh ra từ các dòng `///`.

---

## 3. Rẽ nhánh với if

### 3.1 Cú pháp

```text
if <điều kiện> {
    ...
} else if <điều kiện khác> {
    ...
} else {
    ...
}
```

- Các nhánh `else if` và `else` là tuỳ chọn.
- **Điều kiện phải là `bool`.** Rust không có khái niệm "truthy/falsy" như JavaScript hay Python.
- Điều kiện **không cần ngoặc tròn** (thêm vào sẽ bị cảnh báo, xem [mục 7.3](#73-cảnh-báo-thường-gặp)).
- Thân mỗi nhánh **luôn cần dấu ngoặc nhọn** `{ }`, dù chỉ một dòng.

```rust
fn main() {
    let number = 7;

    if number % 2 == 0 {
        println!("{number} là số chẵn");
    } else {
        println!("{number} là số lẻ");
    }
}
```

**Kết quả:**

```text
7 là số lẻ
```

### 3.2 else if

Dùng khi có nhiều trường hợp loại trừ nhau. Rust kiểm tra **từ trên xuống** và chỉ chạy **nhánh đầu tiên có điều kiện đúng**.

```rust
fn main() {
    let score = 8.5;

    if score >= 9.0 {
        println!("Xuất sắc");
    } else if score >= 8.0 {
        println!("Giỏi");
    } else if score >= 6.5 {
        println!("Khá");
    } else {
        println!("Trung bình trở xuống");
    }
}
```

**Kết quả:**

```text
Giỏi
```

`8.5` thoả cả `>= 8.0` và `>= 6.5`, nhưng chỉ nhánh `Giỏi` chạy vì nó đứng trước. Vì vậy **thứ tự các điều kiện rất quan trọng**.

> Khi có nhiều nhánh theo giá trị, Rust có `match` gọn và an toàn hơn chuỗi `else if` dài. Cấu trúc này sẽ học ở bài [enums_match](../learn_07_enums_match/README.md).

### 3.3 Điều kiện phải là bool

```rust
fn main() {
    let number = 3;

    if number { // ❌ number là số nguyên, không phải bool
        println!("khác 0");
    }
}
```

**Trình biên dịch báo:**

```text
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number { // ❌ number là số nguyên, không phải bool
  |        ^^^^^^ expected `bool`, found integer
```

Cách sửa: viết phép so sánh tường minh, ví dụ `if number != 0`.

### 3.4 if là biểu thức

`if` cho ra giá trị nên đặt được ở vế phải của `let`. Rust không có toán tử ba ngôi `điều_kiện ? a : b`, vì `if` đã đảm nhiệm vai trò đó.

```rust
fn main() {
    let score = 7;
    let result = if score >= 5 { "đậu" } else { "rớt" };
    println!("Kết quả thi: {result}");

    let a = 12;
    let b = 30;
    let larger = if a > b { a } else { b };
    println!("Số lớn hơn: {larger}");
}
```

**Kết quả:**

```text
Kết quả thi: đậu
Số lớn hơn: 30
```

Có hai quy tắc, đều xuất phát từ việc biến `result` phải có **một kiểu duy nhất** lúc biên dịch.

**Quy tắc 1: các nhánh phải cùng kiểu.**

```rust
fn main() {
    let ok = true;
    let value = if ok { 5 } else { "năm" }; // ❌ một nhánh số, một nhánh chuỗi
    println!("{value}");
}
```

**Trình biên dịch báo:**

```text
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:3:36
  |
3 |     let value = if ok { 5 } else { "năm" }; // ❌ một nhánh số, một nhánh chuỗi
  |                         -          ^^^^^ expected integer, found `&str`
  |                         |
  |                         expected because of this
```

**Quy tắc 2: dùng `if` làm giá trị thì phải có `else`.** `if` không có `else` luôn cho ra `()`, không khớp với kiểu của giá trị.

```rust
fn main() {
    let ok = true;
    let value = if ok { 5 }; // ❌ thiếu else
    println!("{value}");
}
```

**Trình biên dịch báo:**

```text
error[E0317]: `if` may be missing an `else` clause
 --> src/main.rs:3:17
  |
3 |     let value = if ok { 5 }; // ❌ thiếu else
  |                 ^^^^^^^^-^^
  |                 |       |
  |                 |       found here
  |                 expected integer, found `()`
  |
  = note: `if` expressions without `else` evaluate to `()`
  = help: consider adding an `else` block that evaluates to the expected type
```

---

## 4. Vòng lặp

### 4.1 Tổng quan

Rust có ba vòng lặp:

| Vòng lặp | Dùng khi | Dừng khi | Trả về giá trị |
|---|---|---|---|
| `loop` | Lặp mãi đến khi tự `break` | Gặp `break` | Có (`break giá_trị`) |
| `while` | Lặp khi điều kiện còn đúng | Điều kiện sai | Không |
| `for` | Duyệt từng phần tử của một dãy | Hết dãy | Không |

Hai từ khoá điều khiển dùng chung cho cả ba:

- `break`: **thoát hẳn** khỏi vòng lặp.
- `continue`: **bỏ qua** phần còn lại của lượt hiện tại, sang lượt kế tiếp.

### 4.2 loop

`loop` lặp vô hạn cho đến khi gặp `break`.

```rust
fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Lần {count}");

        if count == 3 {
            break;
        }
    }

    println!("Xong sau {count} lần");
}
```

**Kết quả:**

```text
Lần 1
Lần 2
Lần 3
Xong sau 3 lần
```

Trò chơi đoán số trong `learn_02_gessing_game` của bạn dùng đúng mẫu này: `loop` chạy mãi, `continue` khi người chơi nhập không phải số, và `break` khi đoán trúng.

### 4.3 break trả về giá trị

`loop` là biểu thức, nên `break` có thể **mang theo một giá trị** làm kết quả của cả vòng lặp.

```rust
fn main() {
    let mut counter = 0;

    let first = loop {
        counter += 1;
        if counter % 5 == 0 && counter % 7 == 0 {
            break counter; // trả về counter làm giá trị của cả vòng loop
        }
    };

    println!("Số nguyên dương nhỏ nhất chia hết cho cả 5 và 7: {first}");
}
```

**Kết quả:**

```text
Số nguyên dương nhỏ nhất chia hết cho cả 5 và 7: 35
```

Trong ba vòng lặp, chỉ `loop` cho phép điều này. Vòng `while` và `for` có thể kết thúc mà không chạy `break` (khi điều kiện sai hoặc hết dãy) nên không thể bảo đảm có giá trị để trả về:

```rust
fn main() {
    let found = for i in 0..5 {
        if i == 3 {
            break i; // ❌ for không trả về giá trị qua break
        }
    };
}
```

**Trình biên dịch báo:**

```text
error[E0571]: `break` with value from a `for` loop
 --> src/main.rs:4:13
  |
2 |     let found = for i in 0..5 {
  |                 ------------- you can't `break` with a value in a `for` loop
3 |         if i == 3 {
4 |             break i; // ❌ for không trả về giá trị qua break
  |             ^^^^^^^ can only break with a value inside `loop` or breakable block
  |
help: use `break` on its own without a value inside this `for` loop
  |
4 -             break i; // ❌ for không trả về giá trị qua break
4 +             break; // ❌ for không trả về giá trị qua break
  |
```

### 4.4 while

`while` kiểm tra điều kiện **trước mỗi lượt** và lặp khi điều kiện còn đúng.

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("Phóng!");
}
```

**Kết quả:**

```text
3!
2!
1!
Phóng!
```

Lưu ý:

- Phải có chỗ trong thân vòng lặp **làm điều kiện dần sai đi** (ở đây là `number -= 1`). Quên bước này sẽ gây vòng lặp vô hạn; bấm `Ctrl+C` để dừng.
- Muốn lặp vô hạn, viết `loop` thay vì `while true` (xem [mục 7.3](#73-cảnh-báo-thường-gặp)).

`while` cũng duyệt được mảng bằng chỉ số, nhưng dễ sai:

```rust
fn main() {
    let scores = [8, 7, 9, 10, 6];
    let mut index = 0;

    while index < scores.len() {
        println!("scores[{index}] = {}", scores[index]);
        index += 1;
    }
}
```

**Kết quả:**

```text
scores[0] = 8
scores[1] = 7
scores[2] = 9
scores[3] = 10
scores[4] = 6
```

Chỉ cần viết nhầm `<=` thay cho `<`, chương trình sẽ **panic** vì đọc ngoài phạm vi (`index out of bounds`), hoặc quên `index += 1` thì lặp mãi. Duyệt dãy nên dùng `for` (mục tiếp theo).

### 4.5 for

`for` duyệt **từng phần tử** của một dãy. Đây là vòng lặp được dùng nhiều nhất trong Rust vì an toàn (không có chỉ số để sai) và gọn.

```rust
fn main() {
    let scores = [8, 7, 9, 10, 6];

    for score in scores {
        println!("Điểm: {score}");
    }
}
```

**Kết quả:**

```text
Điểm: 8
Điểm: 7
Điểm: 9
Điểm: 10
Điểm: 6
```

### 4.6 Range: dãy số

Để lặp một số lần cố định, dùng **range** (khoảng):

| Cú pháp | Các giá trị | Ghi chú |
|---|---|---|
| `1..4` | 1, 2, 3 | **Không gồm** điểm cuối |
| `1..=4` | 1, 2, 3, 4 | `=` nghĩa là **gồm** điểm cuối |
| `(1..=4).rev()` | 4, 3, 2, 1 | Đảo ngược; cần bọc range trong ngoặc |
| `(0..10).step_by(3)` | 0, 3, 6, 9 | Nhảy cách 3 |

```rust
fn main() {
    for i in 1..4 {
        print!("{i} ");
    }
    println!();

    for i in 1..=4 {
        print!("{i} ");
    }
    println!();

    for i in (1..=4).rev() {
        print!("{i} ");
    }
    println!();

    for i in (0..10).step_by(3) {
        print!("{i} ");
    }
    println!();

    for _ in 0..3 {
        print!("*");
    }
    println!();
}
```

**Kết quả:**

```text
1 2 3
1 2 3 4
4 3 2 1
0 3 6 9
***
```

- `print!` giống `println!` nhưng **không xuống dòng**.
- `for _ in 0..3` dùng `_` khi chỉ cần lặp đúng 3 lần mà không dùng giá trị đếm.

### 4.7 Duyệt mảng: iter và enumerate

Hai phương thức hay dùng khi duyệt mảng:

- `.iter()` duyệt từng phần tử bằng cách **mượn** (tham chiếu), nên mảng vẫn dùng được sau vòng lặp.
- `.enumerate()` bọc mỗi phần tử thành cặp `(chỉ số, phần tử)`. Chỉ số có kiểu `usize`.

```rust
fn main() {
    let names = ["An", "Bình", "Chi"];

    for name in names.iter() {
        println!("Xin chào {name}");
    }

    for (index, name) in names.iter().enumerate() {
        println!("{index}: {name}");
    }
}
```

**Kết quả:**

```text
Xin chào An
Xin chào Bình
Xin chào Chi
0: An
1: Bình
2: Chi
```

Khi dùng `.iter()` trên mảng số, mỗi phần tử có kiểu `&i32` (tham chiếu tới số). `println!` in được như bình thường; còn muốn lấy giá trị để tính toán thì thêm `*` phía trước. Tham chiếu sẽ học kỹ ở bài [ownership](../learn_05_ownership/README.md).

```rust
fn main() {
    let scores = [8, 7, 9, 10, 6];
    let mut sum = 0;

    for score in scores.iter() {
        sum += *score;
    }

    println!("Tổng: {sum}");
}
```

**Kết quả:**

```text
Tổng: 40
```

### 4.8 continue và break trong vòng lặp

```rust
fn main() {
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // bỏ qua số chẵn, sang lượt kế
        }
        if i > 7 {
            break; // dừng hẳn khi i > 7
        }
        println!("{i}");
    }
}
```

**Kết quả:**

```text
1
3
5
7
```

`continue` ở các số chẵn nhảy thẳng đến lượt kế; `break` ở `i = 9` kết thúc cả vòng lặp (số `8` bị `continue` bỏ qua trước đó).

### 4.9 Nhãn vòng lặp

Mặc định `break` và `continue` chỉ tác động lên **vòng lặp trong cùng**. Với vòng lặp lồng nhau, đặt **nhãn** (bắt đầu bằng dấu nháy đơn, ví dụ `'outer`) để chỉ rõ vòng lặp cần thoát.

```rust
fn main() {
    'outer: for a in 1..10 {
        for b in 1..10 {
            if a + b == 10 && a * b == 21 {
                println!("Tìm thấy: {a} + {b} = 10 và {a} x {b} = 21");
                break 'outer; // thoát cả hai vòng for
            }
        }
    }

    println!("Kết thúc tìm kiếm");
}
```

**Kết quả:**

```text
Tìm thấy: 3 + 7 = 10 và 3 x 7 = 21
Kết thúc tìm kiếm
```

Nếu chỉ viết `break`, nó chỉ thoát vòng `for b` và vòng `for a` vẫn tiếp tục. Tương tự có `continue 'outer` để sang lượt kế của vòng ngoài. Nhãn dùng được với cả `loop`, `while` và `for`.

### 4.10 Chọn vòng lặp nào

| Tình huống | Chọn |
|---|---|
| Duyệt từng phần tử của mảng hay dãy số | `for` |
| Lặp đúng N lần | `for _ in 0..N` |
| Lặp khi điều kiện còn đúng, số lần không biết trước | `while` |
| Lặp đến khi sự kiện xảy ra rồi thoát, hoặc cần trả giá trị khi thoát (ví dụ nhập lại đến khi hợp lệ) | `loop` + `break` |

---


## 5. Thực hành

Hai bài thực hành có hướng dẫn. Lab 1 luyện hàm có tham số, `for` và `if/else if`. Lab 2 kết hợp hàm có giá trị trả về, `return` sớm, `while` và `for`. Thực hiện trong `learn_04_functions_control_flow/src/main.rs` và chạy bằng `cargo run`.

### Lab 1. Bảng cửu chương và FizzBuzz

**Mục tiêu:** viết hàm có tham số, dùng `for` với range và chuỗi `if/else if`.

**Bước 1.** In bảng cửu chương của một số bất kỳ. Số cần in là **tham số** của hàm:

```rust
fn main() {
    multiplication_table(7);
}

fn multiplication_table(n: u32) {
    for i in 1..=10 {
        println!("{n} x {i} = {}", n * i);
    }
}
```

**Kết quả:**

```text
7 x 1 = 7
7 x 2 = 14
7 x 3 = 21
7 x 4 = 28
7 x 5 = 35
7 x 6 = 42
7 x 7 = 49
7 x 8 = 56
7 x 9 = 63
7 x 10 = 70
```

Đổi `7` thành số khác để in bảng cửu chương khác mà không phải sửa thân hàm. Đó chính là lợi ích của tham số.

**Bước 2.** FizzBuzz, bài kinh điển để luyện rẽ nhánh. Với mỗi số từ 1 đến 15:

- Chia hết cho cả 3 và 5: in `FizzBuzz`.
- Chia hết cho 3: in `Fizz`.
- Chia hết cho 5: in `Buzz`.
- Còn lại: in chính số đó.

```rust
fn main() {
    for i in 1..=15 {
        if i % 15 == 0 {
            println!("FizzBuzz");
        } else if i % 3 == 0 {
            println!("Fizz");
        } else if i % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{i}");
        }
    }
}
```

**Kết quả:**

```text
1
2
Fizz
4
Buzz
Fizz
7
8
Fizz
Buzz
11
Fizz
13
14
FizzBuzz
```

**Phân tích:**

- Điều kiện `i % 15 == 0` (chia hết cho cả 3 và 5) phải đứng **đầu tiên**. Rust chỉ chạy nhánh đầu tiên đúng (xem [mục 3.2](#32-else-if)), nên nếu `i % 3 == 0` đứng trước thì số 15 sẽ in `Fizz` thay vì `FizzBuzz`.
- `i % 15 == 0` tương đương `i % 3 == 0 && i % 5 == 0`.

**Thử thêm:**

1. Đổi thứ tự: đặt nhánh `i % 3 == 0` lên đầu và quan sát dòng thứ 15.
2. Tách vòng lặp thành hàm `fizz_buzz(limit: u32)` rồi gọi `fizz_buzz(30)` từ `main`.

### Lab 2. Số nguyên tố và giai thừa

**Mục tiêu:** viết hàm trả về `bool` và `u64`, dùng `return` sớm, `while` và `for`.

```rust
fn main() {
    println!("Các số nguyên tố không vượt quá 30:");
    for n in 2..=30 {
        if is_prime(n) {
            print!("{n} ");
        }
    }
    println!();

    for n in [0, 1, 5, 10, 20] {
        println!("{n}! = {}", factorial(n));
    }
}

/// Kiểm tra số nguyên tố: thử chia cho 2, 3, 4, ... đến khi i * i > n.
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false; // 0 và 1 không phải số nguyên tố
    }

    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false; // tìm được ước số, thoát sớm
        }
        i += 1;
    }

    true
}

/// Tính n! = 1 x 2 x ... x n bằng vòng lặp.
fn factorial(n: u64) -> u64 {
    let mut result = 1;
    for i in 1..=n {
        result *= i;
    }
    result
}
```

**Kết quả:**

```text
Các số nguyên tố không vượt quá 30:
2 3 5 7 11 13 17 19 23 29
0! = 1
1! = 1
5! = 120
10! = 3628800
20! = 2432902008176640000
```

**Phân tích:**

- `is_prime` thoát sớm bằng `return false` ở hai chỗ. Nếu chạy hết vòng `while` mà không tìm được ước số, biểu thức cuối `true` được trả về.
- Điều kiện `i * i <= n` là một tối ưu: nếu `n` có ước số lớn hơn căn bậc hai của nó thì chắc chắn cũng có một ước số nhỏ hơn căn bậc hai, nên chỉ cần thử đến `i * i <= n`.
- `factorial` dùng `let mut result` làm **biến tích luỹ**. Với `n = 0`, range `1..=0` rỗng nên vòng lặp không chạy và kết quả là `1`, đúng theo quy ước `0! = 1`.
- Hàm `is_prime` chỉ dùng cho số nhỏ: với `n` rất lớn, gần `u32::MAX` (ví dụ `4294967291`), phép `i * i` tràn `u32` và chương trình ở chế độ debug sẽ panic với `attempt to multiply with overflow`.
- Kiểu của `n` ở hai vòng `for` trong `main` được suy luận từ tham số của hàm được gọi (`u32` cho `is_prime`, `u64` cho `factorial`).

**Thử thêm:**

1. Đổi `30` thành `100` để liệt kê các số nguyên tố đến 100 (có 25 số).
2. Gọi `factorial(21)`. Vì `21!` lớn hơn `u64::MAX`, chương trình ở chế độ debug sẽ panic với thông báo `attempt to multiply with overflow`. Hãy nhớ lại các phương thức `checked_mul`, `wrapping_mul` ở [bài Biến](../learn_03_variables/README.md#65-tràn-số-nguyên) để xử lý tràn số.

---

## 6. Tìm hiểu chi tiết

### 6.1 Phạm vi của biến trong hàm

Mỗi hàm có **phạm vi riêng**. Biến khai báo trong `main` không tồn tại bên trong hàm khác, dù hàm đó được `main` gọi.

```rust
fn show_total() {
    println!("Tổng: {total}"); // ❌ total là biến của main, không phải của hàm này
}

fn main() {
    let total = 100;
    show_total();
}
```

**Trình biên dịch báo:**

```text
error[E0425]: cannot find value `total` in this scope
 --> src/main.rs:2:22
  |
2 |     println!("Tổng: {total}"); // ❌ total là biến của main, không phải của hàm này
  |                      ^^^^^ not found in this scope
```

Cách đưa dữ liệu vào hàm là **truyền qua tham số**:

```rust
fn show_total(total: u32) {
    println!("Tổng: {total}");
}

fn main() {
    let total = 100;
    show_total(total);
}
```

**Kết quả:**

```text
Tổng: 100
```

### 6.2 Tham số là bản sao

Với các kiểu đơn giản (số, `bool`, `char`), giá trị truyền vào hàm được **sao chép**. Tham số mặc định **bất biến**, giống `let`. Thêm `mut` trước tên tham số để sửa **bản sao cục bộ** đó; biến gốc ở nơi gọi không bị ảnh hưởng.

```rust
fn main() {
    let n = 5;
    let m = add_one(n);
    println!("n = {n}, m = {m}");
}

fn add_one(mut n: i32) -> i32 {
    n += 1;
    n
}
```

**Kết quả:**

```text
n = 5, m = 6
```

`n` trong `main` vẫn là `5` vì `add_one` chỉ sửa bản sao của nó.

Với chuỗi, nên nhận tham số kiểu `&str`: hàm vừa nhận được chuỗi cố định vừa không "lấy mất" dữ liệu của nơi gọi.

```rust
fn main() {
    let name = "An";
    greet(name);
    greet("Bình");
}

fn greet(name: &str) {
    println!("Xin chào, {name}!");
}
```

**Kết quả:**

```text
Xin chào, An!
Xin chào, Bình!
```

> Với kiểu `String`, việc truyền vào hàm có thể **chuyển quyền sở hữu** (move) và khiến biến cũ không dùng được nữa. Đây là nội dung chính của bài [ownership](../learn_05_ownership/README.md).

### 6.3 Trả về nhiều giá trị bằng tuple

Hàm chỉ có một kiểu trả về, nhưng kiểu đó có thể là **tuple** để gom nhiều giá trị. Nơi gọi tách chúng bằng destructuring (xem [bài Biến, mục 4.5](../learn_03_variables/README.md#45-tuple)).

```rust
fn main() {
    let (quotient, remainder) = div_rem(17, 5);
    println!("17 = 5 x {quotient} + {remainder}");
}

fn div_rem(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}
```

**Kết quả:**

```text
17 = 5 x 3 + 2
```

### 6.4 return, break, continue có kiểu đặc biệt

Các biểu thức `return`, `break`, `continue` (và `panic!`) **không bao giờ cho ra giá trị** vì chúng chuyển luồng đi chỗ khác. Rust gán cho chúng kiểu đặc biệt `!` (đọc là *never*) và cho phép dùng thay cho **mọi kiểu**.

```rust
fn main() {
    describe(10);
    describe(7);
}

fn describe(n: i32) {
    let half = if n % 2 == 0 {
        n / 2
    } else {
        println!("{n} là số lẻ, bỏ qua");
        return; // kiểu !, nên nhánh else vẫn hợp lệ dù không cho ra i32
    };

    println!("Một nửa của {n} là {half}");
}
```

**Kết quả:**

```text
Một nửa của 10 là 5
7 là số lẻ, bỏ qua
```

Hai nhánh của `if` không cùng kiểu theo nghĩa thông thường (`i32` và `!`), nhưng vẫn hợp lệ nhờ quy tắc trên. Đây cũng là lý do trong trò chơi đoán số bạn viết được:

```text
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        println!("Please type a number!");
        continue; // kiểu !, nên nhánh này khớp với nhánh trả về u32
    }
};
```

### 6.5 Đệ quy

Hàm có thể **tự gọi chính nó**. Với giai thừa: `n! = n x (n - 1)!`, dừng ở `1! = 1`.

```rust
fn main() {
    println!("5! = {}", factorial(5));
}

fn factorial(n: u64) -> u64 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
```

**Kết quả:**

```text
5! = 120
```

Ở đây cả thân hàm là một biểu thức `if ... else ...` nên không cần `return`. Cần chú ý:

- Phải có **điều kiện dừng** (`n <= 1`), nếu không hàm gọi lại mãi.
- Mỗi lần gọi chiếm một phần bộ nhớ ngăn xếp (stack). Đệ quy quá sâu sẽ làm tràn stack và chương trình bị dừng. Rust không bảo đảm tối ưu hoá đệ quy đuôi, nên với số lần lặp lớn hãy dùng vòng lặp như ở [Lab 2](#lab-2-số-nguyên-tố-và-giai-thừa).

### 6.6 Quy ước đặt tên

| Đối tượng | Quy ước | Ví dụ |
|---|---|---|
| Hàm, biến, tham số | `snake_case` | `calculate_total`, `unit_price` |
| Hàm trả về `bool` | Thường bắt đầu bằng `is_`, `has_` | `is_prime`, `has_stock` |
| Hằng số | `SCREAMING_SNAKE_CASE` | `VAT_PERCENT` |

Tên hàm nên là **động từ** hoặc cụm động từ cho biết nó làm gì (`print_total`, `parse_number`). Đặt tên sai quy ước không gây lỗi nhưng compiler sẽ cảnh báo (xem [mục 7.3](#73-cảnh-báo-thường-gặp)).

### 6.7 Khi nào nên tách hàm

| Dấu hiệu | Nên làm |
|---|---|
| Cùng một đoạn mã xuất hiện ở nhiều nơi | Tách thành hàm và gọi lại |
| Một hàm làm nhiều việc khác nhau | Tách mỗi việc thành một hàm |
| Phải viết chú thích để giải thích một khối mã làm gì | Tách khối đó thành hàm có tên mô tả việc |
| Cần trả về nhiều giá trị | Dùng tuple (mục 6.3) |
| Hàm quá dài, khó đọc một lượt | Chia nhỏ theo từng bước |

---

## 7. Lỗi thường gặp

### 7.1 Cách đọc thông báo lỗi

Cách đọc một thông báo lỗi (mã lỗi, vị trí `-->`, nhãn `^^^`, dòng `help:`) đã được trình bày ở [bài Biến, mục 7.1](../learn_03_variables/README.md#71-cách-đọc-thông-báo-lỗi). Với lỗi về hàm và luồng điều khiển, hai mẫu câu thường gặp:

- `expected X, found Y`: trình biên dịch cần kiểu `X` nhưng bạn đưa kiểu `Y`.
- `help: remove this semicolon` hoặc `help: consider ...`: gợi ý sửa cụ thể, thường làm theo trực tiếp được.

### 7.2 Lỗi biên dịch

| Mã lỗi | Thông báo | Nguyên nhân | Cách sửa |
|---|---|---|---|
| Không có mã | `expected one of ':', '@', or '\|', found ','` | Tham số hàm thiếu kiểu | `fn add(a: i32, b: i32)` |
| `E0308` | `mismatched types` (`expected i32, found ()`) | Thêm `;` vào biểu thức cuối của hàm có kiểu trả về | Bỏ dấu `;` |
| `E0308` | `mismatched types` (`expected bool, found integer`) | Điều kiện `if` không phải `bool` | Viết phép so sánh: `if n != 0` |
| `E0308` | `'if' and 'else' have incompatible types` | Hai nhánh `if/else` khác kiểu | Cho hai nhánh cùng kiểu |
| `E0308` | `arguments to this function are incorrect` | Truyền đối số sai kiểu (ví dụ `2.5` cho `i32`) | Truyền đúng kiểu, hoặc ép kiểu bằng `as` |
| `E0317` | `'if' may be missing an 'else' clause` | Dùng `if` không có `else` làm giá trị | Thêm nhánh `else` cùng kiểu |
| `E0061` | `this function takes 2 arguments but 1 argument was supplied` | Gọi hàm thiếu hoặc thừa đối số | Truyền đủ số đối số |
| `E0425` | `cannot find value 'x' in this scope` | Hàm dùng biến của hàm khác | Truyền biến vào qua tham số |
| `E0384` | `cannot assign to immutable argument` | Gán lại tham số không có `mut` | `fn f(mut n: i32)` |
| `E0571` | `'break' with value from a 'for' loop` | `break giá_trị` trong `for` hoặc `while` | Dùng `loop`, hoặc gán vào biến ngoài vòng lặp |
| `E0268` | `'break' outside of a loop or labeled block` | `break` hay `continue` ngoài vòng lặp | Chỉ dùng bên trong vòng lặp |
| `E0426` | `use of undeclared label` | `break 'nhãn` mà nhãn chưa được khai báo | Đặt nhãn trước vòng lặp: `'outer: for ...` |
| `E0277` | `'{integer}' is not an iterator` | Viết `for i in 5` | Dùng range: `for i in 0..5` |
| `E0277` | `the type '[{integer}]' cannot be indexed by 'i32'` | Chỉ số mảng không phải `usize` | Dùng `usize`, hoặc `scores[i as usize]` |
| `E0277` | `can't compare '{integer}' with '{float}'` | So sánh số nguyên với số thực | Cho hai vế cùng kiểu |

> Mã lỗi cho cùng một tình huống có thể khác nhau giữa các phiên bản compiler. Bảng trên là kết quả với Rust 1.99.1.

### 7.3 Cảnh báo thường gặp

Cảnh báo (warning) không chặn việc biên dịch nhưng nên được sửa hết.

```rust
// ⚠️ Biên dịch được nhưng có 3 cảnh báo
fn addNumbers(a: i32, b: i32) -> i32 {
    a
}

fn main() {
    let x = 5;
    if (x > 3) {
        println!("{}", addNumbers(1, 2));
    }
}
```

| Cảnh báo | Ý nghĩa | Cách xử lý |
|---|---|---|
| `function 'addNumbers' should have a snake case name` | Tên hàm sai quy ước | Đổi thành `add_numbers` |
| `unused variable: 'b'` | Tham số khai báo mà không dùng | Dùng nó, hoặc đổi thành `_b` nếu cố ý |
| `unnecessary parentheses around 'if' condition` | Thừa ngoặc tròn quanh điều kiện | Viết `if x > 3 {` |
| `unreachable statement` | Có mã nằm sau `return` nên không bao giờ chạy | Xoá dòng đó hoặc sửa logic |
| `denote infinite loops with 'loop { ... }'` | Dùng `while true` | Viết `loop { ... }` |
| `unused doc comment` | Đặt `///` trước một câu lệnh như `let` | Đổi thành `//` |
| `variable does not need to be mutable` | Có `mut` nhưng không bao giờ gán lại | Bỏ `mut` |

---


## 8. Bài tập

Hãy tự làm trước, chạy bằng `cargo run`, rồi mới mở phần đáp án. Mỗi bài có **kết quả mong đợi** để bạn tự kiểm tra.

### Cấp độ 1. Làm quen

#### Bài 1.1. Hàm bình phương

**Đề:** viết hàm `square` nhận một số `i32` và trả về bình phương của nó. Trong `main`, in kết quả của `square(7)` và `square(-3)`.

**Kết quả mong đợi:**

```text
7 bình phương = 49
-3 bình phương = 9
```

**Gợi ý:** khai báo kiểu trả về bằng `-> i32` và **không** đặt dấu `;` sau biểu thức cuối.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    println!("7 bình phương = {}", square(7));
    println!("-3 bình phương = {}", square(-3));
}

fn square(x: i32) -> i32 {
    x * x
}
```

Biểu thức cuối `x * x` không có dấu `;` nên là giá trị trả về. Số âm bình phương lên thành số dương.

</details>

#### Bài 1.2. Sửa lỗi dấu chấm phẩy

**Đề:** đoạn mã sau không biên dịch được. Sửa theo **hai cách khác nhau**.

```rust
fn double(x: i32) -> i32 {
    x * 2; // ❌ dấu ; biến biểu thức thành câu lệnh
}

fn main() {
    println!("Gấp đôi 21 là {}", double(21));
}
```

**Kết quả mong đợi:**

```text
Gấp đôi 21 là 42
```

**Gợi ý:** đọc dòng `help:` trong thông báo lỗi, và nhớ lại `return`.

<details>
<summary>Xem đáp án</summary>

**Cách 1:** bỏ dấu `;` để `x * 2` thành biểu thức cuối.

```rust
fn double(x: i32) -> i32 {
    x * 2
}

fn main() {
    println!("Gấp đôi 21 là {}", double(21));
}
```

**Cách 2:** dùng `return` tường minh.

```rust
fn double(x: i32) -> i32 {
    return x * 2;
}

fn main() {
    println!("Gấp đôi 21 là {}", double(21));
}
```

Cả hai cho cùng kết quả. Cách 1 là phong cách thường dùng trong Rust; `return` ở cuối hàm là thừa nên chỉ dùng khi cần thoát sớm.

</details>

#### Bài 1.3. if làm biểu thức

**Đề:** cho `let score = 7;`. Dùng `if/else` **như một biểu thức** để gán cho biến `result` giá trị `"Đậu"` nếu `score >= 5`, ngược lại `"Rớt"`. Sau đó in kết quả.

**Kết quả mong đợi:**

```text
Điểm 7: Đậu
```

**Gợi ý:** `let result = if ... { ... } else { ... };` và hai nhánh phải cùng kiểu.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    let score = 7;
    let result = if score >= 5 { "Đậu" } else { "Rớt" };

    println!("Điểm {score}: {result}");
}
```

Đổi `score` thành `3` để thấy nhánh `else` chạy. Hai nhánh đều là `&str` nên hợp lệ; nếu một nhánh là số thì sẽ lỗi `E0308`.

</details>

### Cấp độ 2. Vận dụng

#### Bài 2.1. Dự đoán kết quả

**Đề:** đọc đoạn mã sau và **dự đoán** các dòng in ra trước khi chạy.

```rust
fn main() {
    let mut total = 0;

    for i in 1..=6 {
        if i % 3 == 0 {
            continue;
        }
        if i == 5 {
            break;
        }
        total += i;
        println!("i = {i}, total = {total}");
    }

    println!("Tổng cuối: {total}");
}
```

<details>
<summary>Xem đáp án</summary>

**Kết quả:**

```text
i = 1, total = 1
i = 2, total = 3
i = 4, total = 7
Tổng cuối: 7
```

Giải thích: `i = 3` chia hết cho 3 nên `continue` bỏ qua cả phần cộng và phần in. `i = 5` chạy `break` nên vòng lặp kết thúc, `i = 6` không bao giờ được xét.

</details>

#### Bài 2.2. Tổng từ 1 đến n bằng while

**Đề:** viết hàm `sum_to(n: u32) -> u32` tính tổng các số từ 1 đến `n` bằng vòng lặp `while`. In kết quả của `sum_to(100)`.

**Kết quả mong đợi:**

```text
Tổng 1..=100 = 5050
```

**Gợi ý:** cần hai biến `let mut`: một cho tổng, một cho bộ đếm.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    println!("Tổng 1..=100 = {}", sum_to(100));
}

fn sum_to(n: u32) -> u32 {
    let mut sum = 0;
    let mut i = 1;

    while i <= n {
        sum += i;
        i += 1;
    }

    sum
}
```

Cách ngắn hơn dùng range và `sum()` (tìm hiểu thêm sau, khi học về iterator):

```rust
fn main() {
    println!("Tổng 1..=100 = {}", sum_to(100));
}

fn sum_to(n: u32) -> u32 {
    (1..=n).sum()
}
```

</details>

#### Bài 2.3. Đếm ngược

**Đề:** dùng `for` và `.rev()` để in các số từ 5 xuống 1, mỗi số một dòng, rồi in `Phóng!`.

**Kết quả mong đợi:**

```text
5
4
3
2
1
Phóng!
```

**Gợi ý:** `.rev()` cần bọc range trong ngoặc: `(1..=5).rev()`.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    for i in (1..=5).rev() {
        println!("{i}");
    }
    println!("Phóng!");
}
```

</details>

#### Bài 2.4. Số lớn nhất và vị trí của nó

**Đề:** cho `let scores = [3, 9, 2, 7];`. Dùng `for` với `.iter().enumerate()` để tìm **giá trị lớn nhất** và **chỉ số** của nó.

**Kết quả mong đợi:**

```text
Lớn nhất: 9 tại vị trí 1
```

**Gợi ý:** dùng hai biến `let mut` là `max` và `max_index`. Mỗi `score` có kiểu `&i32`, nên so sánh và gán bằng `*score`.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    let scores = [3, 9, 2, 7];
    let mut max = scores[0];
    let mut max_index = 0;

    for (index, score) in scores.iter().enumerate() {
        if *score > max {
            max = *score;
            max_index = index;
        }
    }

    println!("Lớn nhất: {max} tại vị trí {max_index}");
}
```

Giải thích: bắt đầu giả định phần tử đầu là lớn nhất, rồi mỗi lần gặp phần tử lớn hơn thì cập nhật cả giá trị lẫn chỉ số.

</details>

#### Bài 2.5. Thoát vòng lặp lồng nhau

**Đề:** tìm cặp **đầu tiên** `(a, b)` với `a` và `b` đều thuộc `1..=10` sao cho `a * b == 42`. Duyệt `a` ở vòng ngoài, `b` ở vòng trong, và dùng **nhãn** để thoát cả hai vòng ngay khi tìm thấy.

**Kết quả mong đợi:**

```text
Tìm thấy: 6 x 7 = 42
```

**Gợi ý:** đặt nhãn `'outer:` trước vòng `for` ngoài và viết `break 'outer;`.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    'outer: for a in 1..=10 {
        for b in 1..=10 {
            if a * b == 42 {
                println!("Tìm thấy: {a} x {b} = 42");
                break 'outer;
            }
        }
    }
}
```

Nếu chỉ viết `break;`, nó chỉ thoát vòng `for b`, rồi vòng `for a` tiếp tục chạy và còn tìm thêm cặp `(7, 6)`.

</details>

### Cấp độ 3. Thử thách

#### Bài 3.1. Dãy Collatz

**Đề:** viết hàm `collatz_steps(mut n: u64) -> u32` đếm số bước để `n` về `1`, theo quy tắc: nếu `n` chẵn thì `n = n / 2`, nếu lẻ thì `n = 3 * n + 1`. In kết quả cho `6` và `27`.

**Kết quả mong đợi:**

```text
6 cần 8 bước
27 cần 111 bước
```

**Gợi ý:** tham số cần `mut` vì ta sửa `n` trong hàm; dùng `while n != 1` và `if` làm biểu thức.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    println!("6 cần {} bước", collatz_steps(6));
    println!("27 cần {} bước", collatz_steps(27));
}

fn collatz_steps(mut n: u64) -> u32 {
    let mut steps = 0;

    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }

    steps
}
```

Giải thích: với `6` dãy là `6 → 3 → 10 → 5 → 16 → 8 → 4 → 2 → 1`, tức 8 bước. Dòng `n = if ... { ... } else { ... };` dùng `if` như một biểu thức (xem [mục 3.4](#34-if-là-biểu-thức)).

</details>

#### Bài 3.2. Bốn đoạn mã, bốn lỗi

**Đề:** mỗi đoạn mã dưới đây đều không biên dịch được. Với từng đoạn, hãy **dự đoán lỗi** trước khi chạy `cargo check`, rồi sửa.

**Đoạn (a):**

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    println!("{}", area(4)); // ❌
}
```

**Đoạn (b):**

```rust
fn main() {
    let items = 3;
    if items { // ❌
        println!("Còn hàng");
    }
}
```

**Đoạn (c):**

```rust
fn main() {
    let stock = 0;
    let message = if stock > 0 { "Còn hàng" }; // ❌
    println!("{message}");
}
```

**Đoạn (d):**

```rust
fn show_total() {
    println!("Tổng: {total}"); // ❌
}

fn main() {
    let total = 100;
    show_total();
}
```

<details>
<summary>Xem đáp án</summary>

| Đoạn | Lỗi | Nguyên nhân | Cách sửa |
|---|---|---|---|
| (a) | `E0061` this function takes 2 arguments but 1 argument was supplied | Gọi `area` thiếu một đối số | `area(4, 5)` |
| (b) | `E0308` mismatched types (expected `bool`, found integer) | Điều kiện `if` phải là `bool`, không phải số nguyên | `if items > 0` |
| (c) | `E0317` `if` may be missing an `else` clause | `if` làm giá trị mà không có `else` | Thêm `else { "Hết hàng" }` |
| (d) | `E0425` cannot find value `total` in this scope | `total` là biến của `main`, hàm `show_total` không thấy | Thêm tham số: `fn show_total(total: u32)` và gọi `show_total(total)` |

</details>

#### Bài 3.3. Ước chung lớn nhất

**Đề:** viết hàm `gcd(a: u32, b: u32) -> u32` tính ước chung lớn nhất theo thuật toán Euclid: lặp lại việc thay `(a, b)` bằng `(b, a % b)` cho đến khi `b` bằng `0`; khi đó `a` là kết quả. In `gcd(48, 18)` và `gcd(17, 5)`.

**Kết quả mong đợi:**

```text
gcd(48, 18) = 6
gcd(17, 5) = 1
```

**Gợi ý:** dùng `while b != 0`; cả hai tham số đều cần `mut`; dùng một biến tạm để giữ `a % b`.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    println!("gcd(48, 18) = {}", gcd(48, 18));
    println!("gcd(17, 5) = {}", gcd(17, 5));
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
```

Giải thích với `(48, 18)`: `48 % 18 = 12` nên sang `(18, 12)`; `18 % 12 = 6` nên sang `(12, 6)`; `12 % 6 = 0` nên sang `(6, 0)`. `b = 0`, kết quả là `6`. Biến `remainder` giữ giá trị `a % b` trước khi `a` bị ghi đè.

</details>

---

## 9. Tóm tắt

| Chủ đề | Ghi nhớ |
|---|---|
| Khai báo hàm | `fn tên(tham_số: Kiểu) -> Kiểu { ... }`; `main` là điểm bắt đầu; không cần khai báo trước khi gọi |
| Tham số | **Bắt buộc ghi kiểu**; mặc định bất biến, thêm `mut` để sửa bản sao cục bộ |
| Giá trị trả về | Biểu thức cuối **không có `;`**; hàm không có `->` trả về `()` |
| Dấu `;` | Biến biểu thức thành câu lệnh và bỏ giá trị; thêm nhầm vào cuối hàm gây lỗi `E0308` |
| `return` | Thoát hàm sớm; ở cuối hàm nên dùng biểu thức cuối thay vì `return` |
| Nhiều giá trị | Trả về tuple và tách bằng destructuring |
| Phạm vi | Mỗi hàm có phạm vi riêng; đưa dữ liệu vào bằng tham số |
| `if` | Điều kiện phải là `bool`, không cần ngoặc tròn; là biểu thức, các nhánh cùng kiểu, cần `else` khi dùng làm giá trị |
| `loop` | Lặp đến khi `break`; `break giá_trị` trả về giá trị của cả vòng lặp |
| `while` | Lặp khi điều kiện còn đúng; phải cập nhật điều kiện để không lặp vô hạn |
| `for` | Duyệt dãy, an toàn và gọn nhất; `1..4` không gồm 4, `1..=4` gồm 4 |
| Duyệt mảng | `.iter()`, `.enumerate()` (chỉ số `usize`), `.rev()`, `.step_by(n)` |
| `break`, `continue` | Tác động vòng lặp trong cùng; dùng nhãn `'outer` để chọn vòng ngoài |
| Chú thích | `//` thường, `///` là tài liệu cho mục đứng ngay sau |
| Đặt tên | Hàm và biến `snake_case`; hàm trả về `bool` thường bắt đầu bằng `is_` hoặc `has_` |

---

## 10. Tài liệu tham khảo

- [The Rust Programming Language, 3.3 Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
- [The Rust Programming Language, 3.4 Comments](https://doc.rust-lang.org/book/ch03-04-comments.html)
- [The Rust Programming Language, 3.5 Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
- [Rust By Example, Functions](https://doc.rust-lang.org/rust-by-example/fn.html)
- [Rust By Example, Flow of Control](https://doc.rust-lang.org/rust-by-example/flow_control.html)
- [The Rust Reference, Statements and expressions](https://doc.rust-lang.org/reference/statements-and-expressions.html)
- [The Rust Reference, If and if let expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html)
- [The Rust Reference, Loop expressions](https://doc.rust-lang.org/reference/expressions/loop-expr.html)
- [The Rust Reference, Functions](https://doc.rust-lang.org/reference/items/functions.html)
- [Tài liệu thư viện chuẩn, `Range`](https://doc.rust-lang.org/std/ops/struct.Range.html) và [`RangeInclusive`](https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html)
- [Tài liệu thư viện chuẩn, trait `Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html) (`enumerate`, `rev`, `step_by`)
- [The rustdoc book, How to write documentation](https://doc.rust-lang.org/rustdoc/how-to-write-documentation.html)
- [Rust API Guidelines, Naming](https://rust-lang.github.io/api-guidelines/naming.html)
- [Tra cứu mã lỗi, E0308](https://doc.rust-lang.org/error_codes/E0308.html), [E0317](https://doc.rust-lang.org/error_codes/E0317.html), [E0061](https://doc.rust-lang.org/error_codes/E0061.html), [E0425](https://doc.rust-lang.org/error_codes/E0425.html)
