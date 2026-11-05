# Sở hữu (Ownership) trong Rust

> Tài liệu học tập về ownership trong Rust: stack và heap, move, clone, copy, tham chiếu và mượn, slice, kèm thực hành và bài tập có đáp án.

| Thông tin | Chi tiết |
|---|---|
| Chủ đề | Ownership, move, clone, copy, tham chiếu và mượn (borrowing), slice |
| Môi trường | Rust 1.99.1 (stable), Cargo 1.99.1, edition 2024 |
| Dự án | `learn_05_ownership` (thư mục này) |
| Nguồn tham khảo chính | *The Rust Programming Language*, chương 4 (mục 4.1, 4.2 và 4.3) |
| Bài trước | [Hàm và điều khiển luồng](../learn_04_functions_control_flow/README.md) |

## Mục lục

1. [Mục tiêu bài học](#1-mục-tiêu-bài-học)
2. [Khái niệm](#2-khái-niệm)
3. [Move, Clone và Copy](#3-move-clone-và-copy)
4. [Ownership và hàm](#4-ownership-và-hàm)
5. [Tham chiếu và mượn](#5-tham-chiếu-và-mượn)
6. [Slice](#6-slice)
7. [Thực hành](#7-thực-hành)
8. [Tìm hiểu chi tiết](#8-tìm-hiểu-chi-tiết)
9. [Lỗi thường gặp](#9-lỗi-thường-gặp)
10. [Bài tập](#10-bài-tập)
11. [Tóm tắt](#11-tóm-tắt)
12. [Tài liệu tham khảo](#12-tài-liệu-tham-khảo)

---

## 1. Mục tiêu bài học

Sau bài học này bạn có thể:

- [ ] Giải thích ownership giải quyết vấn đề gì và vì sao Rust không cần bộ thu gom rác.
- [ ] Phân biệt stack và heap, và biết dữ liệu nào nằm ở đâu.
- [ ] Nêu ba quy tắc của ownership và áp dụng vào code.
- [ ] Phân biệt **move**, **clone** và **copy**.
- [ ] Dùng tham chiếu `&` và `&mut`, nắm quy tắc mượn.
- [ ] Giải thích dangling reference (tham chiếu treo) và vì sao Rust chặn nó.
- [ ] Dùng slice (`&str`, `&[T]`) và chọn đúng giữa `&str` và `String`.
- [ ] Đọc hiểu và sửa các lỗi `E0382`, `E0499`, `E0502`, `E0106`, `E0515`.

**Kiến thức cần có:** biết khai báo biến (`let`, `let mut`), kiểu dữ liệu cơ bản và cách viết hàm đơn giản. Nếu cần ôn lại, xem bài [Biến](../learn_03_variables/README.md) và bài [Hàm và điều khiển luồng](../learn_04_functions_control_flow/README.md).

**Cách dùng tài liệu.** Chép từng ví dụ vào `src/main.rs` rồi chạy:

```bash
cd learn_05_ownership
cargo run
```

Các ví dụ được đánh dấu như sau:

- Ví dụ có chú thích `// ❌` **không biên dịch được** (cố ý, để minh hoạ lỗi).
- Ví dụ có chú thích `// ⚠️` biên dịch được nhưng sinh cảnh báo.
- Các ví dụ còn lại biên dịch sạch và cho đúng kết quả ghi bên dưới.

> **Lời khuyên:** ownership là phần khiến nhiều người mới thấy "chống lại" compiler. Điều này bình thường. Hãy cố ý gây lỗi, đọc thông báo và sửa. Đó là cách học nhanh nhất.

---

## 2. Khái niệm

### 2.1 Ownership là gì và vì sao cần

Mọi chương trình đều phải quản lý bộ nhớ: xin bộ nhớ khi cần và trả lại khi hết dùng. Có ba cách tiếp cận phổ biến:

| Cách | Ví dụ | Ưu điểm | Nhược điểm |
|---|---|---|---|
| Bộ thu gom rác (garbage collector) | Java, Go, C# | Lập trình viên không phải lo giải phóng | Tốn tài nguyên lúc chạy, khó đoán thời điểm giải phóng |
| Giải phóng thủ công | C | Nhanh, kiểm soát hoàn toàn | Dễ quên giải phóng, giải phóng hai lần, hoặc dùng bộ nhớ đã giải phóng |
| **Ownership** | **Rust** | Nhanh và an toàn, không cần bộ thu gom rác | Phải học các quy tắc; compiler từ chối code vi phạm |

Ownership là **một tập quy tắc do compiler kiểm tra lúc biên dịch**. Code vi phạm sẽ không biên dịch được, còn code hợp lệ không tốn thêm chi phí nào lúc chạy cho việc quản lý bộ nhớ. Nhờ vậy các lỗi như dùng bộ nhớ đã giải phóng hay giải phóng hai lần bị chặn ngay từ đầu.

Ownership chủ yếu trả lời ba câu hỏi về dữ liệu nằm trên heap:

1. **Ai đang dùng** phần dữ liệu nào?
2. Làm sao **hạn chế bản sao** thừa?
3. **Khi nào dọn** dữ liệu không còn dùng?

**Hình dung bằng sách thư viện.** Một quyển sách luôn thuộc về **một người** (chủ sở hữu). Chủ có thể tặng sách cho người khác (chuyển quyền sở hữu), nhưng từ đó mình không còn sách. Chủ cũng có thể cho **mượn đọc** (nhiều người đọc cùng lúc cũng được), hoặc cho **mượn để viết** vào sách (lúc đó chỉ một người được cầm, không ai khác đọc hay viết). Khi chủ không còn nữa, quyển sách được dọn đi. Toàn bộ bài này xoay quanh hình ảnh đó.

### 2.2 Stack và heap

Chương trình dùng hai vùng nhớ chính:

| | Stack (ngăn xếp) | Heap (đống) |
|---|---|---|
| Cách sắp xếp | Chồng dữ liệu, thêm và bớt ở đỉnh (vào sau, ra trước) | Vùng nhớ rộng, cấp phát ở chỗ trống bất kỳ |
| Kích thước dữ liệu | Phải **biết trước lúc biên dịch** | Có thể **thay đổi lúc chạy** |
| Tốc độ | Rất nhanh | Chậm hơn (phải tìm chỗ trống, truy cập qua con trỏ) |
| Ví dụ | `i32`, `f64`, `bool`, `char`, tuple và array cố định | Nội dung của `String` và `Vec<T>` |

**Hình dung:** stack như một chồng đĩa, chỉ đặt và lấy ở trên cùng. Heap như nhà hàng: bạn báo số người, nhân viên tìm một bàn đủ chỗ rồi đưa cho bạn **số bàn**. Bạn cất số bàn trong túi (stack), còn cái bàn thật nằm ở nhà hàng (heap). Số bàn chính là **con trỏ** (pointer, tức địa chỉ).

`String` có thể dài ra hoặc ngắn lại lúc chạy, nên nội dung của nó phải nằm trên heap. Trên stack chỉ có ba thông tin nhỏ: con trỏ tới nội dung, độ dài hiện tại (`len`) và sức chứa (`capacity`).

```text
Stack                         Heap
┌──────────────┐              ┌───┬───┬───┬───┬───┐
│ ptr  ────────┼────────────► │ h │ e │ l │ l │ o │
│ len = 5      │              └───┴───┴───┴───┴───┘
│ capacity = 5 │
└──────────────┘
    s: String
```

```rust
fn main() {
    let s = String::from("hello");

    println!("len = {}", s.len());
    println!("String: {} byte", size_of::<String>());
    println!("&str:   {} byte", size_of::<&str>());
    println!("i32:    {} byte", size_of::<i32>());
}
```

**Kết quả:**

```text
len = 5
String: 24 byte
&str:   16 byte
i32:    4 byte
```

> Kích thước trên là của máy 64 bit: `String` gồm ba trường 8 byte (con trỏ, `len`, `capacity`). Nội dung chữ `hello` nằm ở heap và **không** được tính vào 24 byte này.

Điều này giải thích một sự thật quan trọng: sao chép `i32` chỉ là chép 4 byte trên stack, rẻ và hiển nhiên. Còn "sao chép" một `String` thì phải quyết định có chép cả vùng heap hay không. Ownership cho Rust câu trả lời rõ ràng (xem [mục 3](#3-move-clone-và-copy)).

### 2.3 Ba quy tắc ownership

1. **Mỗi giá trị trong Rust có một chủ sở hữu (owner)**, là một biến.
2. **Tại một thời điểm chỉ có đúng một owner.**
3. Khi owner **ra khỏi phạm vi** (scope), giá trị bị **huỷ** (*dropped*) và bộ nhớ được giải phóng.

Ba quy tắc này là toàn bộ nền tảng. Mọi thứ còn lại (move, clone, tham chiếu, slice) là hệ quả hoặc công cụ để sống chung với chúng.

### 2.4 Phạm vi và drop

Biến có hiệu lực từ chỗ khai báo đến hết khối `{ ... }` chứa nó. Khi chạm dấu `}` đóng khối, Rust **tự động gọi `drop`** để giải phóng giá trị mà biến đó sở hữu. Bạn không phải viết lệnh giải phóng.

```rust
fn main() {
    {
        let s = String::from("hello"); // s bắt đầu có hiệu lực
        println!("{s}");
    } // s ra khỏi phạm vi: Rust tự giải phóng bộ nhớ của s

    println!("đã ra khỏi khối");
}
```

**Kết quả:**

```text
hello
đã ra khỏi khối
```

Ví dụ trên dùng `String` thay vì chuỗi viết thẳng trong code. Hai thứ này khác nhau:

| | `"hello"` (`&str`) | `String::from("hello")` (`String`) |
|---|---|---|
| Nội dung | Cố định, nằm sẵn trong chương trình | Nằm trên heap, do chương trình tạo lúc chạy |
| Sửa được không | Không | Có (nếu khai báo `mut`) |
| Độ dài thay đổi | Không | Có |
| Có chủ sở hữu không | Không, chỉ là tham chiếu tới dữ liệu có sẵn | Có, biến giữ nó là owner |

```rust
fn main() {
    let literal = "hello"; // &str: chỉ đọc
    let mut owned = String::from(literal); // String: sở hữu dữ liệu riêng
    owned.push_str(", world");

    println!("{literal}");
    println!("{owned}");
}
```

**Kết quả:**

```text
hello
hello, world
```

`String` là kiểu dùng chính trong bài này, vì nó sở hữu dữ liệu trên heap nên cho thấy rõ ownership hoạt động thế nào.

---

## 3. Move, Clone và Copy

### 3.1 Move: chuyển quyền sở hữu

Với số nguyên, gán `let y = x;` cho ra hai biến độc lập. Với `String`, chuyện khác đi:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // quyền sở hữu chuyển từ s1 sang s2 (move)

    println!("{s2}");
}
```

**Kết quả:**

```text
hello
```

Điều thực sự xảy ra khi viết `let s2 = s1;`:

1. Rust chép **ba trường trên stack** (con trỏ, `len`, `capacity`) từ `s1` sang `s2`. Vùng heap chứa chữ `hello` **không** bị chép.
2. Rust coi `s1` là **không còn hợp lệ**. Chủ sở hữu bây giờ là `s2`.

```text
Trước khi move:                 Sau khi move:

s1 ┌──────────────┐             s1  (không còn hợp lệ)
   │ ptr ─────────┼──► hello
   │ len = 5      │             s2 ┌──────────────┐
   │ cap = 5      │                │ ptr ─────────┼──► hello
   └──────────────┘                │ len = 5      │
                                   │ cap = 5      │
                                   └──────────────┘
```

Vì sao `s1` phải mất hiệu lực? Nếu cả `s1` và `s2` cùng trỏ vào một vùng heap và cùng hợp lệ, khi ra khỏi phạm vi cả hai sẽ cố giải phóng vùng đó, gây lỗi **giải phóng hai lần** (*double free*), một lỗi bộ nhớ nguy hiểm. Rust giải quyết bằng quy tắc 2: chỉ một owner.

Dùng `s1` sau khi đã move sẽ bị chặn lúc biên dịch:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s1}, world!"); // ❌ s1 đã bị move sang s2
    println!("{s2}");
}
```

**Trình biên dịch báo:**

```text
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:16
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{s1}, world!"); // ❌ s1 đã bị move sang s2
  |                ^^ value borrowed here after move
  |
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++
```

Thông báo chỉ rõ ba điều: `s1` thuộc kiểu `String` (không `Copy`), quyền sở hữu bị chuyển ở dòng nào (`value moved here`), và chỗ dùng sai (`value borrowed here after move`).

> Trong các ngôn ngữ khác, thao tác "chép con trỏ" này gọi là *shallow copy* (sao chép nông). Rust gọi là **move** vì nó còn vô hiệu hoá biến cũ.

### 3.2 Clone: sao chép sâu

Muốn có **hai bản dữ liệu độc lập** trên heap, gọi `.clone()`:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // chép cả dữ liệu trên heap

    println!("s1 = {s1}, s2 = {s2}");
}
```

**Kết quả:**

```text
s1 = hello, s2 = hello
```

Rust không bao giờ tự ý sao chép sâu. Mọi bản sao có thể tốn kém đều phải viết rõ `.clone()`, nên đọc code là thấy chỗ nào đang tốn công.

> **Cẩn thận:** đừng thêm `.clone()` chỉ để "cho compiler im lặng". Trước tiên hãy thử **mượn** bằng tham chiếu (xem [mục 5](#5-tham-chiếu-và-mượn)). Chỉ dùng `clone` khi thật sự cần hai bản độc lập.

### 3.3 Copy: kiểu chép bằng bit

Kiểu có kích thước cố định, nằm hoàn toàn trên stack, thì việc chép rẻ và an toàn. Những kiểu này implement trait `Copy`: gán hay truyền đi đều **tự chép**, biến gốc vẫn dùng được.

```rust
fn main() {
    let x = 5;
    let y = x; // i32 là Copy: x vẫn còn dùng được

    println!("x = {x}, y = {y}");
}
```

**Kết quả:**

```text
x = 5, y = 5
```

Các kiểu `Copy` thường gặp:

- Mọi số nguyên (`i32`, `u8`, `usize`...) và số thực (`f32`, `f64`).
- `bool` và `char`.
- Tham chiếu bất biến `&T`.
- Tuple và array **chỉ chứa** kiểu `Copy`. Ví dụ `(i32, f64)` và `[i32; 3]` là `Copy`; `(i32, String)` thì không.

```rust
fn main() {
    let a = (1, 2.5); // (i32, f64): Copy
    let b = a;
    println!("{a:?} {b:?}");

    let c = (1, String::from("x")); // chứa String: không Copy, bị move
    let d = c;
    println!("{d:?}");
}
```

**Kết quả:**

```text
(1, 2.5) (1, 2.5)
(1, "x")
```

### 3.4 So sánh ba cách

| | Move | Clone | Copy |
|---|---|---|---|
| Cách viết | `let b = a;` (với kiểu không `Copy`) | `let b = a.clone();` | `let b = a;` (với kiểu `Copy`) |
| Điều gì xảy ra | Chuyển quyền sở hữu | Tạo bản sao sâu của dữ liệu, kể cả trên heap | Chép các bit trên stack |
| Chi phí | Rẻ (chép vài byte trên stack) | Có thể đắt (chép cả dữ liệu heap) | Rẻ |
| Biến gốc còn dùng được | **Không** | Có | Có |
| Kiểu điển hình | `String`, `Vec<T>`, `Box<T>` | Hầu hết kiểu có sở hữu dữ liệu | Số, `bool`, `char`, `&T` |

---

## 4. Ownership và hàm

### 4.1 Truyền giá trị vào hàm

Truyền một biến vào hàm cũng theo đúng luật gán: kiểu không `Copy` bị **move** vào hàm, kiểu `Copy` được **chép**.

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s bị move vào hàm, từ đây s không dùng được nữa

    let x = 5;
    makes_copy(x); // i32 là Copy: x vẫn dùng được
    println!("x = {x}");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // some_string ra khỏi phạm vi và bị drop tại đây

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
```

**Kết quả:**

```text
hello
5
x = 5
```

Dùng `s` sau lời gọi `takes_ownership(s)` sẽ lỗi:

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);

    println!("{s}"); // ❌ s đã bị move vào hàm
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
}
```

**Trình biên dịch báo:**

```text
error[E0382]: borrow of moved value: `s`
 --> src/main.rs:5:16
  |
2 |     let s = String::from("hello");
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
3 |     takes_ownership(s);
  |                     - value moved here
4 |
5 |     println!("{s}"); // ❌ s đã bị move vào hàm
  |                ^ value borrowed here after move
  |
note: consider changing this parameter type in function `takes_ownership` to borrow instead if owning the value isn't necessary
 --> src/main.rs:8:33
  |
8 | fn takes_ownership(some_string: String) {
  |    ---------------              ^^^^^^ this parameter takes ownership of the value
  |    |
  |    in this function
help: consider cloning the value if the performance cost is acceptable
  |
3 |     takes_ownership(s.clone());
  |                      ++++++++
```

Để ý đoạn `note:` của compiler: nó gợi ý **đổi tham số sang kiểu mượn** nếu hàm không thật sự cần sở hữu. Đó chính là hướng giải quyết ở [mục 5](#5-tham-chiếu-và-mượn).

### 4.2 Trả giá trị về từ hàm

Giá trị trả về cũng chuyển quyền sở hữu, lần này là từ hàm ra cho nơi gọi:

```rust
fn main() {
    let s1 = gives_ownership(); // hàm trao quyền sở hữu cho s1
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 vào hàm, rồi hàm trả về cho s3

    println!("{s1} {s3}");
}

fn gives_ownership() -> String {
    String::from("yours")
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```

**Kết quả:**

```text
yours hello
```

### 4.3 Vấn đề: muốn dùng mà không muốn mất quyền

Nếu một hàm chỉ cần **đọc** dữ liệu mà vẫn chuyển quyền sở hữu, ta phải trả lại nó để người gọi tiếp tục dùng:

```rust
fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("'{s2}' dài {len}");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // phải trả s về cùng kết quả
}
```

**Kết quả:**

```text
'hello' dài 5
```

Cách này chạy được nhưng rườm rà: mỗi hàm đều phải "trả lại đồ". Rust có cách gọn hơn: **cho mượn** bằng tham chiếu.

---

## 5. Tham chiếu và mượn

### 5.1 Tham chiếu bất biến `&`

**Tham chiếu** (*reference*) giống con trỏ: nó trỏ tới một giá trị nhưng **không sở hữu** giá trị đó. Tạo tham chiếu gọi là **mượn** (*borrowing*). Cú pháp: `&giá_trị`.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // cho hàm mượn s1

    println!("'{s1}' dài {len}"); // s1 vẫn thuộc về main, vẫn dùng được
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // s là tham chiếu, không sở hữu, nên không drop dữ liệu
```

**Kết quả:**

```text
'hello' dài 5
```

```text
s (tham chiếu)        s1 (chủ sở hữu)              Heap
┌─────────────┐       ┌──────────────┐
│ ptr ────────┼─────► │ ptr ─────────┼─────────► hello
└─────────────┘       │ len = 5      │
                      │ cap = 5      │
                      └──────────────┘
```

Tham chiếu `s` trỏ tới biến `s1`, còn `s1` mới trỏ tới dữ liệu trên heap. Khi `s` ra khỏi phạm vi, **không có gì bị drop**, vì `s` chưa bao giờ là chủ.

Mặc định tham chiếu `&` là **chỉ đọc**. Cố sửa dữ liệu qua nó sẽ lỗi:

```rust
fn main() {
    let s = String::from("hello");
    change(&s);

    println!("{s}");
}

fn change(some_string: &String) {
    some_string.push_str(", world"); // ❌ không sửa được qua tham chiếu &
}
```

**Trình biên dịch báo:**

```text
error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
 --> src/main.rs:9:5
  |
9 |     some_string.push_str(", world"); // ❌ không sửa được qua tham chiếu &
  |     ^^^^^^^^^^^ `some_string` is a `&` reference, so it cannot be borrowed as mutable
  |
help: consider changing this to be a mutable reference
  |
8 | fn change(some_string: &mut String) {
  |                         +++
```

### 5.2 Tham chiếu khả biến `&mut`

Muốn cho mượn **để sửa**, dùng `&mut`. Có hai điều kiện: biến gốc phải khai báo `mut`, và hàm nhận kiểu `&mut`.

```rust
fn main() {
    let mut s = String::from("hello"); // cần mut
    change(&mut s); // cho mượn để sửa

    println!("{s}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

**Kết quả:**

```text
hello, world
```

Quên `mut` ở biến gốc thì lỗi `E0596` (xem [mục 9.2](#92-lỗi-biên-dịch)).

### 5.3 Quy tắc mượn

Tại **mọi thời điểm**, bạn chỉ được chọn **một** trong hai:

| Lựa chọn | Số lượng |
|---|---|
| Tham chiếu khả biến `&mut` | **Đúng một** |
| Tham chiếu bất biến `&` | **Bao nhiêu cũng được** |

Ngoài ra, mọi tham chiếu phải **luôn hợp lệ** (không được trỏ tới dữ liệu đã bị huỷ, xem [mục 5.5](#55-dangling-reference-tham-chiếu-treo)).

Theo hình ảnh sách thư viện: nhiều người cùng **đọc** một quyển thì không sao. Nhưng khi có người đang **viết** vào sách, không ai khác được đọc hay viết cùng lúc, vì họ có thể thấy nội dung đang dở dang.

Nhiều tham chiếu `&` cùng lúc thì hợp lệ:

```rust
fn main() {
    let s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{r1} {r2}");
}
```

**Kết quả:**

```text
hello hello
```

Hai tham chiếu `&mut` cùng lúc thì không:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; // ❌ mượn khả biến lần thứ hai
    println!("{r1}, {r2}");
}
```

**Trình biên dịch báo:**

```text
error[E0499]: cannot borrow `s` as mutable more than once at a time
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here
5 |     let r2 = &mut s; // ❌ mượn khả biến lần thứ hai
  |              ^^^^^^ second mutable borrow occurs here
6 |     println!("{r1}, {r2}");
  |                -- first borrow later used here
```

Trộn `&` và `&mut` còn sống cùng lúc cũng không được:

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &mut s; // ❌ r1 (chỉ đọc) vẫn còn dùng ở dòng dưới
    println!("{r1}, {r2}");
}
```

**Trình biên dịch báo:**

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:14
  |
4 |     let r1 = &s;
  |              -- immutable borrow occurs here
5 |     let r2 = &mut s; // ❌ r1 (chỉ đọc) vẫn còn dùng ở dòng dưới
  |              ^^^^^^ mutable borrow occurs here
6 |     println!("{r1}, {r2}");
  |                -- immutable borrow later used here
```

Quy tắc này giúp Rust ngăn **data race** (tranh chấp dữ liệu) ngay lúc biên dịch: không thể có hai nơi cùng ghi, hoặc một nơi ghi trong lúc nơi khác đang đọc.

Có thể dùng nhiều `&mut` **lần lượt** (không chồng nhau) bằng cách để mỗi cái sống trong một khối riêng:

```rust
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" a");
    } // r1 ra khỏi phạm vi, kết thúc lần mượn thứ nhất

    let r2 = &mut s; // OK
    r2.push_str(" b");

    println!("{s}");
}
```

**Kết quả:**

```text
hello a b
```

### 5.4 Phạm vi của tham chiếu (NLL)

Một tham chiếu "sống" từ chỗ tạo ra đến **lần dùng cuối cùng** của nó, không phải đến hết khối. Compiler Rust tính điều này tự động (tính năng tên NLL, *non-lexical lifetimes*).

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{r1} {r2}"); // lần dùng cuối của r1 và r2

    let r3 = &mut s; // OK: r1 và r2 không còn được dùng nữa
    r3.push_str(" world");
    println!("{r3}");
}
```

**Kết quả:**

```text
hello hello
hello world
```

So với ví dụ lỗi `E0502` ở trên: ở đó `r1` còn được dùng **sau** khi `r2` được tạo, nên hai tham chiếu "sống" chồng nhau. Ở ví dụ này `r1` và `r2` đã hết dùng trước khi `r3` ra đời, nên không có xung đột.

### 5.5 Dangling reference (tham chiếu treo)

*Dangling reference* là tham chiếu trỏ tới dữ liệu **đã bị giải phóng**. Ngôn ngữ như C để lọt lỗi này, còn Rust **đảm bảo không bao giờ xảy ra**: compiler không cho tham chiếu sống lâu hơn dữ liệu mà nó trỏ tới.

```rust
fn main() {
    let r = dangle();
    println!("{r}");
}

fn dangle() -> &String { // ❌
    let s = String::from("hello");
    &s // trả về tham chiếu tới s, nhưng s sẽ bị drop khi hàm kết thúc
}
```

**Trình biên dịch báo:**

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:6:16
  |
6 | fn dangle() -> &String { // ❌
  |                ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime, but this is uncommon unless you're returning a borrowed value from a `const` or a `static`
  |
6 | fn dangle() -> &'static String { // ❌
  |                 +++++++
help: instead, you are more likely to want to return an owned value
  |
6 - fn dangle() -> &String { // ❌
6 + fn dangle() -> String { // ❌
  |
```

Vì `s` bị drop ở cuối hàm, tham chiếu `&s` sẽ trỏ vào chỗ trống. Thông báo gợi ý đúng hướng: hãy **trả về giá trị sở hữu** (`String`) thay vì tham chiếu. Lúc này quyền sở hữu được chuyển ra ngoài và không có gì bị huỷ:

```rust
fn main() {
    let s = no_dangle();
    println!("{s}");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s // trả về chính String: quyền sở hữu được chuyển ra ngoài
}
```

**Kết quả:**

```text
hello
```

Lỗi `E0106` liên quan tới *lifetime* (thời gian sống của tham chiếu) sẽ được học ở bài `generics_traits_lifetimes`. Hiện tại chỉ cần nhớ quy tắc thực hành: **không trả về tham chiếu tới biến cục bộ của hàm**.

Một dạng khác của cùng lỗi: chữ ký hàm hợp lệ, nhưng thân hàm trả về tham chiếu tới thứ do chính hàm tạo ra.

```rust
fn main() {
    let word = make_word("hello");
    println!("{word}");
}

fn make_word(text: &str) -> &str {
    let upper = text.to_uppercase();
    &upper // ❌ upper là biến cục bộ, sẽ bị drop khi hàm kết thúc
}
```

**Trình biên dịch báo:**

```text
error[E0515]: cannot return reference to local variable `upper`
 --> src/main.rs:8:5
  |
8 |     &upper // ❌ upper là biến cục bộ, sẽ bị drop khi hàm kết thúc
  |     ^^^^^^ returns a reference to data owned by the current function
```

Cách sửa vẫn là trả về `String`: `fn make_word(text: &str) -> String`.

---

## 6. Slice

### 6.1 String slice

**Slice** là tham chiếu tới **một đoạn liên tiếp** của dữ liệu, không sở hữu nó. Slice của chuỗi có kiểu `&str`. Cú pháp: `&s[bắt_đầu..kết_thúc]`, lấy từ vị trí `bắt_đầu` đến **trước** `kết_thúc`.

```rust
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{hello}|{world}");
}
```

**Kết quả:**

```text
hello|world
```

```text
s:  h e l l o   w o r l d
    0 1 2 3 4 5 6 7 8 9 10

hello = &s[0..5]  ──► "hello"
world = &s[6..11] ──► "world"
```

Bên trong, slice chỉ lưu hai thông tin: **con trỏ tới phần tử đầu** và **độ dài** (nên `&str` chiếm 16 byte, như đã thấy ở [mục 2.2](#22-stack-và-heap)).

Có thể viết tắt khi đoạn bắt đầu từ đầu hoặc kết thúc ở cuối:

```rust
fn main() {
    let s = String::from("hello world");

    println!("{}", &s[..5]); // từ đầu đến trước vị trí 5
    println!("{}", &s[6..]); // từ vị trí 6 đến hết
    println!("{}", &s[..]); // toàn bộ
    println!("{}", &s[0..=4]); // 0..=4 gồm cả vị trí 4
}
```

**Kết quả:**

```text
hello
world
hello world
hello
```

> **Lưu ý về tiếng Việt và Unicode.** Vị trí trong `&s[a..b]` là **vị trí byte**, không phải vị trí ký tự. Chuỗi trong Rust là UTF-8, nên ký tự có dấu như `à` chiếm 2 byte. Cắt giữa một ký tự thì chương trình **panic**, ví dụ `&"chào"[0..3]` dừng với thông báo `end byte index 3 is not a char boundary; it is inside 'à' (bytes 2..4 of string)`. Dùng `s.get(a..b)` để cắt an toàn: nó trả về `None` thay vì panic.

```rust
fn main() {
    let s = "chào"; // 'à' chiếm 2 byte

    println!("{} ký tự, {} byte", s.chars().count(), s.len());
    println!("{:?}", s.get(0..3)); // cắt giữa chữ 'à'
    println!("{:?}", s.get(0..4)); // cắt đúng biên ký tự
}
```

**Kết quả:**

```text
4 ký tự, 5 byte
None
Some("chà")
```

### 6.2 `&str` và `String`, bài học nối tiếp

Bây giờ có thể trả lời đầy đủ câu hỏi "`&str` và `String` khác nhau thế nào":

| | `&str` | `String` |
|---|---|---|
| Bản chất | **Tham chiếu** (slice) tới dữ liệu chuỗi có sẵn | **Chủ sở hữu** dữ liệu chuỗi trên heap |
| Sửa được không | Không | Có (nếu `mut`) |
| Chiếm trên stack | 16 byte (con trỏ, độ dài) | 24 byte (con trỏ, độ dài, sức chứa) |
| Chuỗi viết sẵn `"..."` | Chính là `&str` | Phải đổi: `String::from("...")` hoặc `"...".to_string()` |
| Dùng khi | Chỉ cần đọc, truyền vào hàm | Cần sở hữu, sửa, hoặc lưu lâu dài |

Chuỗi viết sẵn như `"hello"` là `&str` vì nó là một slice trỏ vào dữ liệu nằm sẵn trong chương trình.

**Quy tắc nên nhớ:** hàm chỉ cần đọc chuỗi thì nhận `&str`. Một hàm nhận `&str` dùng được với cả `&String` (Rust tự đổi) lẫn chuỗi viết sẵn:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes(); // chuỗi dưới dạng dãy byte

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // gặp dấu cách đầu tiên: trả về đoạn phía trước
        }
    }

    &s[..] // không có dấu cách: cả chuỗi là một từ
}

fn main() {
    let s = String::from("hello world");
    println!("Từ đầu tiên: {}", first_word(&s)); // &String tự đổi thành &str

    let literal = "good morning";
    println!("Từ đầu tiên: {}", first_word(literal)); // &str dùng trực tiếp
}
```

**Kết quả:**

```text
Từ đầu tiên: hello
Từ đầu tiên: good
```

Giải thích nhanh vòng lặp: `.as_bytes()` cho dãy byte của chuỗi, `.iter().enumerate()` trả về từng cặp (vị trí, giá trị), `&item` lấy giá trị ra khỏi tham chiếu, và `b' '` là byte của ký tự dấu cách.

Điểm hay của việc trả về **slice** thay vì một con số vị trí: slice **gắn chặt** với chuỗi gốc. Nếu sửa chuỗi gốc trong lúc slice còn dùng, compiler chặn lại:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    s.clear(); // ❌ clear cần &mut s, nhưng word đang mượn s
    println!("Từ đầu tiên: {word}");
}
```

**Trình biên dịch báo:**

```text
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
  --> src/main.rs:17:5
   |
16 |     let word = first_word(&s);
   |                           -- immutable borrow occurs here
17 |     s.clear(); // ❌ clear cần &mut s, nhưng word đang mượn s
   |     ^^^^^^^^^ mutable borrow occurs here
18 |     println!("Từ đầu tiên: {word}");
   |                             ---- immutable borrow later used here
```

Nếu `first_word` trả về một con số vị trí, chương trình vẫn biên dịch được, nhưng sau `s.clear()` con số đó vô nghĩa, tạo ra lỗi khó tìm. Với slice, lỗi bị phát hiện ngay lúc biên dịch.

### 6.3 Slice của array

Slice không chỉ có cho chuỗi. Mọi dãy liên tiếp đều có slice, kiểu `&[T]`:

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // kiểu &[i32]

    println!("{slice:?}");
    println!("{}", slice.len());
}
```

**Kết quả:**

```text
[2, 3]
2
```

Hàm nhận `&[i32]` dùng được với cả array lẫn một đoạn của nó:

```rust
fn sum(nums: &[i32]) -> i32 {
    nums.iter().sum()
}

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("{}", sum(&a)); // cả array
    println!("{}", sum(&a[..3])); // chỉ 3 phần tử đầu
}
```

**Kết quả:**

```text
15
6
```

Cũng như `&str` với `String`, `&[T]` là kiểu nên dùng cho tham số khi chỉ cần đọc một dãy phần tử.

---

## 7. Thực hành

Hai bài thực hành có hướng dẫn. Thực hiện trong `learn_05_ownership/src/main.rs` và chạy bằng `cargo run`.

### Lab 1. Ghi chú: từ move đến mượn

**Mục tiêu:** thấy ba cách truyền dữ liệu vào hàm: **move**, **mượn để đọc** (`&`) và **mượn để sửa** (`&mut`).

**Bước 1.** Chép đoạn mã dưới đây và chạy. Hàm `print_note` nhận `String` nên **lấy mất** quyền sở hữu:

```rust
fn main() {
    let note = String::from("Học Rust");

    print_note(note);
    print_note(note); // ❌ note đã bị move ở lời gọi trước
}

fn print_note(text: String) {
    println!("Ghi chú: {text}");
}
```

Quan sát lỗi `E0382: use of moved value: 'note'`, và đoạn `note:` gợi ý "consider changing this parameter type ... to borrow instead".

**Bước 2.** Làm theo gợi ý: đổi tham số sang `&str` và truyền `&note`.

```rust
fn main() {
    let note = String::from("Học Rust");

    print_note(&note);
    print_note(&note);
}

fn print_note(text: &str) {
    println!("Ghi chú: {text}");
}
```

**Kết quả:**

```text
Ghi chú: Học Rust
Ghi chú: Học Rust
```

Giờ gọi bao nhiêu lần cũng được, vì `main` vẫn là chủ của `note`.

**Bước 3.** Thêm hàm **sửa** ghi chú bằng `&mut`. Lưu ý `note` phải là `let mut`:

```rust
fn main() {
    let mut note = String::from("Học Rust");

    print_note(&note);
    add_signature(&mut note, "An");
    print_note(&note);

    println!("Độ dài: {} byte", note.len());
}

fn print_note(text: &str) {
    println!("Ghi chú: {text}");
}

fn add_signature(text: &mut String, author: &str) {
    text.push_str(" - ");
    text.push_str(author);
}
```

**Kết quả:**

```text
Ghi chú: Học Rust
Ghi chú: Học Rust - An
Độ dài: 15 byte
```

**Phân tích:**

- `print_note` chỉ đọc nên nhận `&str`: gọi được với `&String` lẫn chuỗi viết sẵn.
- `add_signature` cần sửa nên nhận `&mut String`. Đây là **kiểu duy nhất** cho phép `push_str`, vì `&str` không sửa được.
- `author: &str` chỉ cần đọc nên nhận `&str`, và truyền chuỗi viết sẵn `"An"` là đủ.
- Độ dài là **byte**, không phải số ký tự: `ọ` trong "Học" chiếm 3 byte.

**Thử thêm:** trong `main`, tạo `let r = &note;` rồi gọi `add_signature(&mut note, "An");` và in `r` ở cuối. Dự đoán lỗi (đáp án: `E0502`, vì `r` còn dùng sau khi `&mut note` được tạo).

### Lab 2. Thống kê điểm: slice của array

**Mục tiêu:** dùng `&[i32]` để viết hàm chạy được trên cả array lẫn một đoạn của array.

```rust
fn main() {
    let scores = [8, 7, 9, 10, 5, 6];

    let first_three = &scores[..3];
    let last_three = &scores[3..];

    println!("3 bài đầu: {first_three:?}, tổng {}", sum(first_three));
    println!("3 bài sau: {last_three:?}, tổng {}", sum(last_three));
    println!("Cả kỳ: tổng {}, cao nhất {}", sum(&scores), max(&scores));
}

fn sum(nums: &[i32]) -> i32 {
    let mut total = 0;
    for n in nums {
        total += n;
    }
    total
}

fn max(nums: &[i32]) -> i32 {
    let mut best = nums[0];
    for &n in nums {
        if n > best {
            best = n;
        }
    }
    best
}
```

**Kết quả:**

```text
3 bài đầu: [8, 7, 9], tổng 24
3 bài sau: [10, 5, 6], tổng 21
Cả kỳ: tổng 45, cao nhất 10
```

**Phân tích:**

- `first_three` và `last_three` là **slice**, không chép dữ liệu: chúng chỉ trỏ vào các đoạn của `scores`.
- `sum` và `max` nhận `&[i32]` nên dùng chung được cho cả array (`&scores`) lẫn mọi đoạn của nó.
- Trong `max`, `for &n in nums` lấy từng giá trị `i32` ra khỏi tham chiếu (`&n`) để so sánh trực tiếp. Còn ở `sum`, `total += n` cộng thẳng từ tham chiếu vì Rust hỗ trợ sẵn phép đó.
- `scores` vẫn thuộc về `main` suốt chương trình. Hai hàm chỉ **mượn**.

**Thử thêm:** thêm một hàm `min` tương tự `max` và in giá trị nhỏ nhất của `first_three` (đáp án: `7`).

---

## 8. Tìm hiểu chi tiết

### 8.1 Khi nào `drop` chạy

Giá trị bị drop khi **owner của nó ra khỏi phạm vi**. Để *nhìn thấy* khoảnh khắc đó, ta tạo một kiểu tự in thông báo khi bị drop. Phần `struct` và `impl` sẽ học kỹ ở bài sau, tạm thời chỉ cần hiểu: `drop` là hàm Rust tự gọi ngay trước khi giải phóng giá trị.

```rust
struct Noisy(u32);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn consume(n: Noisy) {
    println!("consume {}", n.0);
} // n bị drop ở cuối hàm

fn main() {
    let _a = Noisy(1);
    let _b = Noisy(2);

    {
        let _c = Noisy(3);
        println!("trong khối");
    } // _c bị drop ở đây

    consume(Noisy(4));

    let e = Noisy(5);
    drop(e); // drop sớm, không cần đợi hết phạm vi
    println!("e đã bị drop sớm");

    println!("cuối main");
} // _b rồi _a bị drop ở đây
```

**Kết quả:**

```text
trong khối
drop 3
consume 4
drop 4
drop 5
e đã bị drop sớm
cuối main
drop 2
drop 1
```

Rút ra:

- Biến bị drop **theo thứ tự ngược với thứ tự khai báo**: `_b` (khai báo sau) bị drop trước `_a`.
- Giá trị truyền vào hàm theo kiểu move bị drop **ở cuối hàm đó**, không phải ở `main`.
- `drop(e)` giải phóng sớm khi cần. Sau lời gọi này `e` đã bị move vào hàm `drop` nên không dùng lại được.

### 8.2 Biến đã bị move vẫn có thể được gán lại

Một biến `mut` đã bị move đi có thể nhận giá trị mới, và từ lúc đó lại hợp lệ:

```rust
fn main() {
    let mut s = String::from("cũ");
    let t = s; // s bị move sang t
    s = String::from("mới"); // gán giá trị mới: s hợp lệ trở lại

    println!("{s} {t}");
}
```

**Kết quả:**

```text
mới cũ
```

### 8.3 Kích thước của tham chiếu

Mọi tham chiếu đều là con trỏ. Tham chiếu tới kiểu có kích thước cố định chỉ cần lưu địa chỉ. Còn tham chiếu tới slice phải lưu thêm **độ dài**, nên lớn gấp đôi:

```rust
fn main() {
    println!("&i32:    {} byte", size_of::<&i32>());
    println!("&String: {} byte", size_of::<&String>());
    println!("&str:    {} byte", size_of::<&str>());
    println!("&[i32]:  {} byte", size_of::<&[i32]>());
}
```

**Kết quả:**

```text
&i32:    8 byte
&String: 8 byte
&str:    16 byte
&[i32]:  16 byte
```

(Số liệu cho máy 64 bit.) Dù vậy, tham chiếu luôn rất nhỏ so với dữ liệu nó trỏ tới. Truyền `&String` hay `&str` vào hàm rẻ hơn nhiều so với sao chép cả chuỗi.

### 8.4 Chọn kiểu tham số cho hàm

| Hàm cần làm gì | Kiểu tham số | Ví dụ |
|---|---|---|
| Chỉ đọc chuỗi | `&str` | `fn print_note(text: &str)` |
| Chỉ đọc một dãy phần tử | `&[T]` | `fn sum(nums: &[i32]) -> i32` |
| Sửa tại chỗ, người gọi vẫn giữ dữ liệu | `&mut T` | `fn add_signature(text: &mut String, ...)` |
| Lấy hẳn dữ liệu để cất giữ, biến đổi rồi trả lại | `T` (`String`...) | `fn into_upper(s: String) -> String` |
| Kiểu nhỏ, `Copy` (số, `bool`, `char`) | Truyền thẳng giá trị | `fn double(x: i32) -> i32` |

Tham số kiểu `&String` hầu như luôn nên đổi thành `&str`, vì `&str` nhận được nhiều loại đầu vào hơn.

### 8.5 Chọn công cụ nào?

| Tình huống | Chọn |
|---|---|
| Cần đọc dữ liệu của biến khác, không cần giữ | Mượn bằng `&` |
| Cần sửa dữ liệu của biến khác tại chỗ | Mượn bằng `&mut` |
| Cần hai bản dữ liệu **độc lập** | `.clone()` |
| Hàm tạo ra dữ liệu mới cho người gọi | Trả về giá trị sở hữu (`String`), không trả về `&` tới biến cục bộ |
| Cần cắt một đoạn của chuỗi hoặc dãy | Slice `&s[a..b]` |
| Kiểu `Copy` (số, `bool`, `char`) | Truyền giá trị, không cần tham chiếu |

---

## 9. Lỗi thường gặp

### 9.1 Cách đọc thông báo lỗi về ownership

Lấy lỗi `E0382` ở [mục 3.1](#31-move-chuyển-quyền-sở-hữu) làm ví dụ:

| Phần | Ý nghĩa |
|---|---|
| `error[E0382]: borrow of moved value: 's1'` | Mã lỗi và mô tả: dùng `s1` sau khi đã bị move |
| `move occurs because 's1' has type 'String', which does not implement the 'Copy' trait` | Giải thích vì sao là move: `String` không phải kiểu `Copy` |
| `value moved here` | Chỗ quyền sở hữu bị chuyển đi |
| `value borrowed here after move` | Chỗ dùng sai, sau khi đã move |
| `help: consider cloning the value ...` | Gợi ý sửa. Cân nhắc kỹ trước khi làm theo: có khi mượn tốt hơn clone |

Với lỗi mượn (`E0499`, `E0502`), thông báo luôn chỉ ra **ba chỗ**: nơi mượn lần đầu, nơi mượn xung đột, và nơi lần mượn đầu **còn được dùng sau đó**. Chỉ cần kết thúc lần mượn đầu sớm hơn (hoặc di chuyển dòng dùng lên trước) là hết lỗi.

### 9.2 Lỗi biên dịch

| Mã lỗi | Thông báo | Nguyên nhân | Cách sửa |
|---|---|---|---|
| `E0382` | `borrow of moved value` / `use of moved value` | Dùng biến sau khi quyền sở hữu đã chuyển đi | Mượn bằng `&`, hoặc `.clone()` nếu cần hai bản |
| `E0499` | `cannot borrow as mutable more than once at a time` | Hai `&mut` cùng sống một lúc | Dùng `&mut` lần lượt, hoặc tách thành các khối |
| `E0502` | `cannot borrow as mutable because it is also borrowed as immutable` | `&` còn dùng trong lúc tạo `&mut` | Dùng xong `&` rồi mới tạo `&mut` |
| `E0505` | `cannot move out of ... because it is borrowed` | Move biến khi đang có tham chiếu tới nó | Dùng xong tham chiếu trước khi move |
| `E0596` | `cannot borrow as mutable` | Sửa qua `&` hoặc qua biến không `mut` | Đổi sang `&mut` và khai báo `let mut` |
| `E0597` | `does not live long enough` | Tham chiếu sống lâu hơn dữ liệu nó trỏ tới | Cho dữ liệu sống lâu hơn, hoặc trả về giá trị sở hữu |
| `E0106` | `missing lifetime specifier` | Trả về `&` mà không có đầu vào nào để mượn từ đó | Trả về giá trị sở hữu (`String`) |
| `E0515` | `cannot return reference to local variable` | Trả về tham chiếu tới biến cục bộ | Trả về giá trị sở hữu (`String`) |

> Mã lỗi cho cùng một tình huống có thể khác nhau giữa các phiên bản compiler. Bảng trên là kết quả với Rust 1.99.1.

### 9.3 Quy trình sửa lỗi ownership

Khi gặp lỗi ownership, hãy thử lần lượt:

1. **Đọc dòng `note:` và `help:`** trước. Nhiều khi compiler đã gợi ý đúng hướng.
2. **Mượn trước, clone sau.** Hàm chỉ đọc thì đổi tham số thành `&T` hoặc `&str`.
3. **Rút ngắn đời của tham chiếu.** Dùng xong tham chiếu sớm, hoặc dời dòng dùng lên trước chỗ xung đột.
4. **Trả về giá trị sở hữu** thay vì tham chiếu tới biến cục bộ.
5. Chỉ khi thật sự cần hai bản độc lập mới dùng **`.clone()`**.

---

## 10. Bài tập

Hãy tự làm trước, chạy bằng `cargo run`, rồi mới mở phần đáp án. Mỗi bài có **kết quả mong đợi** để bạn tự kiểm tra.

### Cấp độ 1. Làm quen

#### Bài 1.1. Sửa lỗi dùng sau khi move

**Đề:** đoạn mã sau không biên dịch được. Sửa theo **hai cách khác nhau**: một cách dùng `.clone()`, một cách dùng tham chiếu `&`.

```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1;
    println!("{s1} {s2}"); // ❌ s1 đã bị move
}
```

**Kết quả mong đợi:**

```text
Rust Rust
```

**Gợi ý:** `let s2 = s1.clone();` hoặc `let s2 = &s1;`.

<details>
<summary>Xem đáp án</summary>

**Cách 1:** dùng `.clone()` để có hai bản độc lập.

```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = s1.clone();
    println!("{s1} {s2}");
}
```

**Cách 2:** dùng tham chiếu, `s2` chỉ mượn `s1`.

```rust
fn main() {
    let s1 = String::from("Rust");
    let s2 = &s1;
    println!("{s1} {s2}");
}
```

Cả hai cho cùng kết quả. Cách 1 tạo thêm một vùng heap, cách 2 không tạo thêm dữ liệu nào. Nếu chỉ cần **đọc**, cách 2 tiết kiệm hơn.

</details>

#### Bài 1.2. Hàm chỉ đọc

**Đề:** viết hàm `char_count` nhận một chuỗi và trả về số **ký tự** của nó (kiểu `usize`). Gọi hàm với một `String` và một chuỗi viết sẵn, mà **không** làm mất quyền sở hữu của `String`.

**Kết quả mong đợi:**

```text
chào: 4 ký tự
Rust: 4 ký tự
```

**Gợi ý:** tham số nhận `&str`; số ký tự là `text.chars().count()`.

<details>
<summary>Xem đáp án</summary>

```rust
fn char_count(text: &str) -> usize {
    text.chars().count()
}

fn main() {
    let owned = String::from("chào");
    let literal = "Rust";

    println!("{owned}: {} ký tự", char_count(&owned));
    println!("{literal}: {} ký tự", char_count(literal));
}
```

Nhờ nhận `&str`, hàm dùng được với cả `&String` lẫn chuỗi viết sẵn, và `owned` vẫn dùng được sau lời gọi. Dùng `chars().count()` thay vì `len()`, vì `len()` đếm **byte** (`chào` có 5 byte nhưng 4 ký tự).

</details>

#### Bài 1.3. Hàm sửa tại chỗ

**Đề:** viết hàm `shout` nhận một chuỗi và thêm dấu `!` vào cuối **chính chuỗi đó**. Gọi hàm hai lần trên `"Hello"`.

**Kết quả mong đợi:**

```text
Hello!!
```

**Gợi ý:** cần `&mut String` ở hàm, `let mut` ở biến, và `&mut` ở lời gọi.

<details>
<summary>Xem đáp án</summary>

```rust
fn shout(text: &mut String) {
    text.push('!');
}

fn main() {
    let mut s = String::from("Hello");

    shout(&mut s);
    shout(&mut s);

    println!("{s}");
}
```

Hai lời gọi `&mut s` nối tiếp nhau là hợp lệ vì lần mượn thứ nhất đã kết thúc khi hàm trả về.

</details>

### Cấp độ 2. Vận dụng

#### Bài 2.1. Dự đoán kết quả

**Đề:** đoạn mã sau có biên dịch được không? Nếu được, dự đoán dòng in ra. Sau cùng, biến nào đã bị move?

```rust
fn main() {
    let a = String::from("A");
    let b = a.clone();
    let c = b;

    println!("{a} {c}");
}
```

<details>
<summary>Xem đáp án</summary>

**Kết quả:**

```text
A A
```

Giải thích: `a.clone()` tạo `b` là bản sao độc lập, nên `a` vẫn dùng được. `let c = b;` **move** `b` sang `c`, nên từ đó chỉ còn `a` và `c` dùng được, `b` thì không.

</details>

#### Bài 2.2. Nhận rồi trả lại quyền sở hữu

**Đề:** viết hàm `add_exclamation` nhận một `String` (lấy hẳn quyền sở hữu), thêm `!` vào cuối rồi **trả lại** nó. Trong `main`, gán kết quả vào biến mới và in.

**Kết quả mong đợi:**

```text
Hi!
```

**Gợi ý:** tham số cần `mut` để sửa được: `fn add_exclamation(mut s: String) -> String`.

<details>
<summary>Xem đáp án</summary>

```rust
fn add_exclamation(mut s: String) -> String {
    s.push('!');
    s
}

fn main() {
    let greeting = add_exclamation(String::from("Hi"));
    println!("{greeting}");
}
```

Hàm sở hữu `s` nên được phép sửa nó (khai báo `mut` ngay ở tham số). Dòng cuối `s` không có dấu chấm phẩy, đó là giá trị trả về, quyền sở hữu chuyển ra cho nơi gọi.

</details>

#### Bài 2.3. Sửa hàm trả về tham chiếu treo

**Đề:** hàm `shout` dưới đây không biên dịch được. Sửa để chương trình chạy đúng.

```rust
fn shout(text: &str) -> &str {
    let upper = text.to_uppercase();
    &upper // ❌
}

fn main() {
    println!("{}", shout("rust is fast"));
}
```

**Kết quả mong đợi:**

```text
RUST IS FAST
```

**Gợi ý:** `upper` là biến cục bộ, sẽ bị drop khi hàm kết thúc. Hàm nên trả về kiểu gì?

<details>
<summary>Xem đáp án</summary>

```rust
fn shout(text: &str) -> String {
    text.to_uppercase()
}

fn main() {
    println!("{}", shout("rust is fast"));
}
```

`to_uppercase()` tạo một `String` mới, nên hàm phải **trả về nó (có sở hữu)**, không thể trả về tham chiếu tới biến cục bộ. Quyền sở hữu được chuyển ra cho `main`.

</details>

#### Bài 2.4. Tách trường đầu tiên bằng slice

**Đề:** viết hàm `first_field` nhận chuỗi dạng `"An,20,Hà Nội"` và trả về **slice** là phần trước dấu phẩy đầu tiên. Nếu không có dấu phẩy thì trả về cả chuỗi. Thử với hai chuỗi.

**Kết quả mong đợi:**

```text
An
Rust
```

**Gợi ý:** áp dụng đúng kiểu hàm `first_word` ở [mục 6.2](#62-str-và-string-bài-học-nối-tiếp), thay `b' '` bằng `b','`.

<details>
<summary>Xem đáp án</summary>

```rust
fn first_field(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b',' {
            return &s[..i];
        }
    }

    s
}

fn main() {
    println!("{}", first_field("An,20,Hà Nội"));
    println!("{}", first_field("Rust"));
}
```

Hàm trả về một **slice của chuỗi đầu vào**, không tạo chuỗi mới, nên không tốn thêm bộ nhớ. Vị trí `i` luôn là biên ký tự hợp lệ vì dấu phẩy chỉ chiếm 1 byte.

</details>

#### Bài 2.5. Sửa xung đột mượn

**Đề:** đoạn mã sau lỗi `E0502`. Sửa **chỉ bằng cách đổi thứ tự dòng** (không dùng `.clone()`) để in ra đúng hai dòng.

```rust
fn main() {
    let mut text = String::from("hello");

    let r1 = &text;
    text.push_str(" world"); // ❌ r1 còn được dùng ở dòng dưới
    println!("{r1}");
    println!("{text}");
}
```

**Kết quả mong đợi:**

```text
hello
hello world
```

**Gợi ý:** dùng `r1` xong rồi mới sửa `text`.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    let mut text = String::from("hello");

    let r1 = &text;
    println!("{r1}"); // lần dùng cuối của r1

    text.push_str(" world"); // OK: r1 không còn được dùng
    println!("{text}");
}
```

Nhờ NLL (xem [mục 5.4](#54-phạm-vi-của-tham-chiếu-nll)), lần mượn của `r1` kết thúc ngay sau lần dùng cuối. Dời `println!("{r1}")` lên trước `push_str` là đủ.

</details>

### Cấp độ 3. Thử thách

#### Bài 3.1. Bốn đoạn mã, bốn lỗi

**Đề:** mỗi đoạn mã dưới đây đều không biên dịch được. Với từng đoạn, hãy **dự đoán lỗi** trước khi chạy `cargo check`, rồi sửa.

**Đoạn (a):**

```rust
fn eat(s: String) {
    println!("{s}");
}

fn main() {
    let s = String::from("cơm");
    eat(s);
    eat(s); // ❌
}
```

**Đoạn (b):**

```rust
fn main() {
    let s = String::from("hi");
    s.push_str("!"); // ❌
    println!("{s}");
}
```

**Đoạn (c):**

```rust
fn main() {
    let r;
    {
        let s = String::from("hi");
        r = &s; // ❌
    }
    println!("{r}");
}
```

**Đoạn (d):**

```rust
fn main() {
    let s = String::from("hi");
    let r = &s;
    let t = s; // ❌
    println!("{r} {t}");
}
```

<details>
<summary>Xem đáp án</summary>

| Đoạn | Lỗi | Nguyên nhân | Cách sửa |
|---|---|---|---|
| (a) | `E0382` borrow of moved value | `eat(s)` lần đầu đã lấy quyền sở hữu `s` | Đổi `eat` sang nhận `&str` và gọi `eat(&s)`; hoặc `eat(s.clone())` |
| (b) | `E0596` cannot borrow as mutable | `s` không khai báo `mut` nên không sửa được | `let mut s = String::from("hi");` |
| (c) | `E0597` does not live long enough | `s` bị drop ở cuối khối trong, nhưng `r` còn dùng bên ngoài | Khai báo `s` bên ngoài khối để nó sống lâu hơn `r` |
| (d) | `E0505` cannot move out because it is borrowed | `let t = s;` move `s` trong khi `r` đang mượn và còn dùng ở dòng sau | Bỏ move (dùng `&s` hoặc `.clone()`), hoặc dùng `r` xong rồi mới move |

</details>

#### Bài 3.2. Tách họ và tên

**Đề:** viết hàm `split_name` nhận một chuỗi họ tên đầy đủ và trả về **cặp hai slice**: từ đầu tiên, và phần còn lại sau dấu cách đầu tiên. Nếu không có dấu cách thì phần còn lại là chuỗi rỗng. Thử với `"Nguyen Van An"` và `"Cher"`.

**Kết quả mong đợi:**

```text
Họ: Nguyen | Phần còn lại: Van An
Họ: Cher | Phần còn lại: 
```

**Gợi ý:** kiểu trả về là `(&str, &str)`. Tìm vị trí dấu cách như ở `first_word`, rồi trả về `(&s[..i], &s[i + 1..])`.

<details>
<summary>Xem đáp án</summary>

```rust
fn split_name(s: &str) -> (&str, &str) {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&s[..i], &s[i + 1..]);
        }
    }

    (s, "")
}

fn main() {
    let (first, rest) = split_name("Nguyen Van An");
    println!("Họ: {first} | Phần còn lại: {rest}");

    let (first, rest) = split_name("Cher");
    println!("Họ: {first} | Phần còn lại: {rest}");
}
```

Cả hai phần tử của cặp trả về đều **mượn từ chuỗi đầu vào**, nên không có dữ liệu nào được chép. Ở nhánh không có dấu cách, `""` là chuỗi rỗng viết sẵn, luôn hợp lệ.

</details>

#### Bài 3.3. Ba kiểu truyền tham số trong một chương trình

**Đề:** viết ba hàm và gọi chúng nối tiếp trong `main`, mỗi hàm dùng **một kiểu truyền** khác nhau:

- `append_world(s: ...)`: thêm `" world"` vào cuối chuỗi **tại chỗ**.
- `length(s: ...)`: trả về độ dài chuỗi, **chỉ đọc**.
- `into_upper(s: ...)`: **lấy hẳn** chuỗi, trả về bản chữ hoa.

Chuỗi ban đầu là `"hello"`.

**Kết quả mong đợi:**

```text
hello world (dài 11)
HELLO WORLD
```

**Gợi ý:** theo thứ tự trên, kiểu tham số lần lượt là `&mut String`, `&str`, `String`.

<details>
<summary>Xem đáp án</summary>

```rust
fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn length(s: &str) -> usize {
    s.len()
}

fn into_upper(s: String) -> String {
    s.to_uppercase()
}

fn main() {
    let mut text = String::from("hello");

    append_world(&mut text); // mượn để sửa
    println!("{text} (dài {})", length(&text)); // mượn để đọc

    let upper = into_upper(text); // move: text không dùng được nữa
    println!("{upper}");
}
```

Giải thích: `append_world` cần sửa nên dùng `&mut`. `length` chỉ đọc nên dùng `&str`. `into_upper` lấy hẳn quyền sở hữu, và vì `text` đã bị move vào đó, `main` chỉ còn dùng `upper`. Nếu thêm `println!("{text}")` sau dòng `into_upper(text)` sẽ gặp lỗi `E0382`.

</details>

---

## 11. Tóm tắt

| Chủ đề | Ghi nhớ |
|---|---|
| Mục đích | Quản lý bộ nhớ an toàn, kiểm tra lúc biên dịch, không cần bộ thu gom rác |
| Stack và heap | Stack: kích thước cố định, nhanh. Heap: kích thước linh hoạt, truy cập qua con trỏ. `String` có phần nội dung ở heap |
| Ba quy tắc | Mỗi giá trị có một owner; chỉ một owner tại một thời điểm; owner ra khỏi phạm vi thì giá trị bị drop |
| Move | Gán hoặc truyền kiểu không `Copy` thì chuyển quyền sở hữu; biến cũ không dùng được (`E0382`) |
| Clone | `.clone()` tạo bản sao sâu, tường minh và có thể tốn kém |
| Copy | Số, `bool`, `char`, `&T`, tuple/array chỉ chứa `Copy`: gán là chép, biến gốc vẫn dùng được |
| Tham chiếu | `&T` chỉ đọc, `&mut T` để sửa; mượn không làm mất quyền sở hữu |
| Quy tắc mượn | Một `&mut` **hoặc** nhiều `&`, không cùng lúc (`E0499`, `E0502`) |
| NLL | Tham chiếu sống đến lần dùng cuối cùng, không phải hết khối |
| Dangling | Không bao giờ có tham chiếu treo; không trả về `&` tới biến cục bộ (`E0106`, `E0515`) |
| Slice | `&str` và `&[T]` mượn một đoạn liên tiếp; vị trí cắt chuỗi tính theo byte |
| Tham số hàm | Chỉ đọc: `&str` / `&[T]`; sửa tại chỗ: `&mut T`; lấy hẳn: `T` |

---

## 12. Tài liệu tham khảo

- [The Rust Programming Language, 4.1 What is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
- [The Rust Programming Language, 4.2 References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [The Rust Programming Language, 4.3 The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
- [Rust By Example, Ownership and moves](https://doc.rust-lang.org/rust-by-example/scope/move.html)
- [Rust By Example, Borrowing](https://doc.rust-lang.org/rust-by-example/scope/borrow.html)
- [The Rust Reference, Destructors (drop)](https://doc.rust-lang.org/reference/destructors.html)
- [Tài liệu thư viện chuẩn, trait `Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html)
- [Tài liệu thư viện chuẩn, trait `Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html)
- [Tài liệu thư viện chuẩn, kiểu `String`](https://doc.rust-lang.org/std/string/struct.String.html)
- [Tra cứu mã lỗi, ví dụ E0382](https://doc.rust-lang.org/error_codes/E0382.html)
