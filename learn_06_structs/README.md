# Struct và impl trong Rust

> Tài liệu học tập về `struct` trong Rust: định nghĩa kiểu dữ liệu riêng, gắn hành vi cho nó bằng `impl`, thực hành và bài tập có đáp án.

| Thông tin | Chi tiết |
|---|---|
| Chủ đề | `struct` (có tên trường, tuple struct, unit struct), `impl`, method, associated function, `#[derive(Debug)]` |
| Môi trường | Rust 1.99.1 (stable), Cargo 1.99.1, edition 2024 |
| Dự án | `learn_06_structs` (thư mục này) |
| Học trước | [Biến và kiểu dữ liệu](../learn_03_variables/README.md), [Ownership](../learn_05_ownership/README.md) |
| Nguồn tham khảo chính | *The Rust Programming Language*, chương 5 (mục 5.1, 5.2 và 5.3) |

## Mục lục

1. [Mục tiêu bài học](#1-mục-tiêu-bài-học)
2. [Khái niệm](#2-khái-niệm)
3. [Các loại struct](#3-các-loại-struct)
4. [Tạo và dùng struct](#4-tạo-và-dùng-struct)
5. [Method và associated function](#5-method-và-associated-function)
6. [Thực hành](#6-thực-hành)
7. [Tìm hiểu chi tiết](#7-tìm-hiểu-chi-tiết)
8. [Lỗi thường gặp](#8-lỗi-thường-gặp)
9. [Bài tập](#9-bài-tập)
10. [Tóm tắt](#10-tóm-tắt)
11. [Tài liệu tham khảo](#11-tài-liệu-tham-khảo)

---

## 1. Mục tiêu bài học

Sau bài học này bạn có thể:

- [ ] Giải thích `struct` là gì và khi nào nên dùng thay cho tuple.
- [ ] Định nghĩa và tạo giá trị cho ba loại struct: có tên trường, tuple struct, unit struct.
- [ ] Dùng cú pháp viết tắt trường (*field init shorthand*) và cú pháp cập nhật struct (`..other`), hiểu vì sao nó có thể làm "mất" dữ liệu của biến cũ.
- [ ] Giải thích vì sao muốn sửa một trường thì cả biến struct phải khai báo `mut`.
- [ ] In struct để gỡ lỗi bằng `#[derive(Debug)]`, `{:?}`, `{:#?}` và `dbg!`.
- [ ] Viết `impl` với method (`&self`, `&mut self`, `self`) và associated function (như `new`).
- [ ] Chọn `String` hay `&str` cho trường của struct.
- [ ] Đọc hiểu và sửa các lỗi biên dịch phổ biến liên quan đến struct.

**Cách dùng tài liệu.** Chép từng ví dụ vào `src/main.rs` rồi chạy:

```bash
cd learn_06_structs
cargo run
```

Các ví dụ được đánh dấu như sau:

- Ví dụ có chú thích `// ❌` **không biên dịch được** (cố ý, để minh hoạ lỗi).
- Ví dụ có chú thích `// ⚠️` biên dịch được nhưng sinh cảnh báo.
- Các ví dụ còn lại biên dịch sạch và cho đúng kết quả ghi bên dưới.

> **Gợi ý:** bài này dùng nhiều khái niệm của bài [Ownership](../learn_05_ownership/README.md) (chuyển quyền sở hữu, mượn `&` và `&mut`). Nếu thấy chỗ nào khó hiểu, hãy đọc lại bài đó.

---

## 2. Khái niệm

### 2.1 Struct là gì

`struct` (viết tắt của *structure*) là cách để **tự tạo một kiểu dữ liệu mới** bằng cách gom nhiều giá trị liên quan lại, và **đặt tên cho từng giá trị**. Mỗi phần có tên đó gọi là một **trường** (*field*).

Bài [Biến](../learn_03_variables/README.md) đã giới thiệu tuple: gom nhiều giá trị, nhưng phải nhớ `.0` là gì, `.1` là gì. Struct giải quyết đúng chỗ đó:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p_tuple = (3, 4);
    let p_struct = Point { x: 3, y: 4 };

    println!("tuple:  {} {}", p_tuple.0, p_tuple.1);
    println!("struct: {} {}", p_struct.x, p_struct.y);
}
```

**Kết quả:**

```text
tuple:  3 4
struct: 3 4
```

Cả hai chứa cùng dữ liệu, nhưng `p_struct.x` tự giải thích còn `p_tuple.0` thì không. Struct cũng cho phép bạn đặt **tên cho kiểu** (`Point`), nên hàm có thể khai báo rõ ràng "tôi cần một `Point`" thay vì "tôi cần một cặp số nào đó".

### 2.2 Định nghĩa và tạo giá trị

Cú pháp định nghĩa:

```text
struct <TênKiểu> {
    <tên_trường>: <kiểu>,
    <tên_trường>: <kiểu>,
}
```

Sau khi định nghĩa, tạo giá trị (còn gọi là *instance*, thể hiện) bằng cách ghi tên kiểu và cung cấp giá trị cho **từng** trường. Truy cập trường bằng dấu chấm.

```rust
struct Student {
    name: String,
    age: u8,
    is_active: bool,
}

fn main() {
    let student = Student {
        name: String::from("An"),
        age: 20,
        is_active: true,
    };

    println!("{} - {} tuổi - đang học: {}", student.name, student.age, student.is_active);
}
```

**Kết quả:**

```text
An - 20 tuổi - đang học: true
```

Quy tắc cần nhớ:

- Định nghĩa `struct` thường đặt **ngoài** mọi hàm, để cả tệp dùng chung.
- Các trường cách nhau bằng dấu phẩy. Dấu phẩy sau trường cuối được phép và là phong cách chuẩn của `rustfmt`.
- Khi tạo giá trị, thứ tự trường **không cần** giống thứ tự khai báo, nhưng phải có **đủ mọi trường**.
- Tên struct theo `UpperCamelCase` (`Student`), tên trường theo `snake_case` (`is_active`).

### 2.3 Struct sở hữu dữ liệu của nó

Một giá trị struct **sở hữu** các trường của nó. Khi biến struct ra khỏi phạm vi, mọi trường được giải phóng theo. Đó là lý do trường văn bản thường là `String` (kiểu sở hữu chuỗi) chứ không phải `&str` (xem [mục 7.6](#76-string-hay-str-trong-struct)).

Điều này cũng có nghĩa các quy tắc của [Ownership](../learn_05_ownership/README.md) áp dụng nguyên vẹn: gán struct sang biến khác thì **chuyển** quyền sở hữu (xem [mục 7.1](#71-move-và-copy-với-struct)); muốn đọc mà không lấy đi thì **mượn** bằng `&`.

---

## 3. Các loại struct

### 3.1 Tổng quan

| Loại | Cú pháp định nghĩa | Truy cập trường | Dùng khi |
|---|---|---|---|
| Struct có tên trường | `struct Point { x: i32, y: i32 }` | `p.x` | Trường hợp phổ biến nhất, mỗi trường có ý nghĩa riêng |
| Tuple struct | `struct Color(u8, u8, u8);` | `c.0` | Cần một kiểu riêng có tên nhưng các trường không cần tên |
| Unit struct | `struct Marker;` | Không có trường | Cần một kiểu không chứa dữ liệu (thường đi cùng trait) |

### 3.2 Struct có tên trường

Đây là loại đã dùng ở [mục 2](#2-khái-niệm): mỗi trường có tên và kiểu. Các trường có thể thuộc **kiểu khác nhau**, và cũng có thể là một struct khác (xem [mục 7.2](#72-struct-lồng-nhau)).

### 3.3 Tuple struct

Tuple struct là struct mà các trường **chỉ có kiểu, không có tên**. Truy cập bằng chỉ số `.0`, `.1`... giống tuple, hoặc tách bằng destructuring.

```rust
struct Color(u8, u8, u8);
struct Meters(f64);

fn main() {
    let red = Color(255, 0, 0);
    let height = Meters(1.75);

    println!("Đỏ: {} {} {}", red.0, red.1, red.2);
    println!("Chiều cao: {} m", height.0);

    let Color(r, g, b) = red;
    println!("{r} {g} {b}");
}
```

**Kết quả:**

```text
Đỏ: 255 0 0
Chiều cao: 1.75 m
255 0 0
```

Điểm khác tuple thường: **mỗi tuple struct là một kiểu riêng**, kể cả khi các trường giống hệt nhau. Nhờ vậy compiler giúp bạn không nhầm lẫn:

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn describe(color: &Color) {
    println!("RGB: {} {} {}", color.0, color.1, color.2);
}

fn main() {
    let origin = Point(0, 0, 0);
    describe(&origin); // ❌ Point không phải Color
}
```

**Trình biên dịch báo:**

```text
error[E0308]: mismatched types
  --> src/main.rs:10:14
   |
10 |     describe(&origin); // ❌ Point không phải Color
   |     -------- ^^^^^^^ expected `&Color`, found `&Point`
   |     |
   |     arguments to this function are incorrect
   |
   = note: expected reference `&Color`
              found reference `&Point`
note: function defined here
  --> src/main.rs:4:4
   |
 4 | fn describe(color: &Color) {
   |    ^^^^^^^^ -------------
```

Kỹ thuật bọc một giá trị trong tuple struct một trường (như `Meters(f64)`) để tạo kiểu riêng gọi là **newtype**. Ví dụ `Meters(f64)` và `Feet(f64)` đều chứa `f64`, nhưng compiler sẽ không cho cộng chúng lại với nhau nếu bạn chưa quy đổi.

### 3.4 Unit struct

Unit struct **không có trường nào**, viết tên kiểu rồi dấu `;`. Nó giống kiểu `()` (unit) nhưng có tên riêng.

```rust
struct Marker;

fn main() {
    let _m = Marker;
    println!("Marker chiếm {} byte", size_of::<Marker>());
}
```

**Kết quả:**

```text
Marker chiếm 0 byte
```

Unit struct không lưu dữ liệu nên chiếm 0 byte. Nó hữu ích khi bạn chỉ cần "một kiểu để gắn hành vi" mà không cần dữ liệu; ứng dụng chính nằm ở bài `generics_traits_lifetimes`, nên bây giờ chỉ cần biết nó tồn tại.

---

## 4. Tạo và dùng struct

### 4.1 Khởi tạo và truy cập trường

Ví dụ dưới đây tạo một `User` với các trường được liệt kê **theo thứ tự khác** với lúc định nghĩa. Việc này hợp lệ vì Rust ghép giá trị theo **tên trường**.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        email: String::from("an@example.com"),
        username: String::from("an123"),
        active: true,
        sign_in_count: 1,
    };

    println!(
        "{} <{}>: đăng nhập {} lần, active = {}",
        user.username, user.email, user.sign_in_count, user.active
    );
}
```

**Kết quả:**

```text
an123 <an@example.com>: đăng nhập 1 lần, active = true
```

Quên một trường là lỗi biên dịch. Rust không có "giá trị mặc định ngầm" cho trường chưa gán:

```rust
struct User {
    username: String,
    active: bool,
}

fn main() {
    let user = User {
        username: String::from("an123"),
    }; // ❌ thiếu trường active

    println!("{}", user.username);
}
```

**Trình biên dịch báo:**

```text
error[E0063]: missing field `active` in initializer of `User`
 --> src/main.rs:7:16
  |
7 |     let user = User {
  |                ^^^^ missing `active`
```

### 4.2 Viết tắt trường

Khi **tên biến trùng tên trường**, có thể viết một lần thay vì `username: username`. Cú pháp này gọi là *field init shorthand*, rất hay gặp trong hàm tạo.

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // viết tắt của username: username
        email,    // viết tắt của email: email
        active: true,
    }
}

fn main() {
    let user = build_user(String::from("an123"), String::from("an@example.com"));
    println!("{} {} {}", user.username, user.email, user.active);
}
```

**Kết quả:**

```text
an123 an@example.com true
```

### 4.3 Cập nhật struct từ struct cũ

Khi muốn tạo struct mới giống struct cũ, chỉ khác vài trường, dùng `..tên_struct_cũ` ở **cuối** danh sách trường. Các trường không ghi rõ sẽ được lấy từ struct cũ.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("an123"),
        email: String::from("an@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        email: String::from("binh@example.com"),
        ..user1 // username, sign_in_count, active lấy từ user1
    };

    println!("{} {} {} {}", user2.username, user2.email, user2.sign_in_count, user2.active);
    println!("user1.email vẫn dùng được: {}", user1.email);
}
```

**Kết quả:**

```text
an123 binh@example.com 1 true
user1.email vẫn dùng được: an@example.com
```

**Lưu ý quan trọng:** cú pháp `..` **di chuyển** (move) các trường được lấy từ struct cũ, giống phép gán `=` trong bài Ownership.

- `username` là `String` (không `Copy`), nên nó **chuyển** từ `user1` sang `user2`. Sau đó `user1.username` không dùng được nữa.
- `sign_in_count` và `active` là kiểu `Copy` (số, `bool`), nên chỉ được **sao chép**, `user1` vẫn giữ nguyên.
- `user1.email` không bị ảnh hưởng vì `user2` đã có giá trị `email` riêng.

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("an123"),
        email: String::from("an@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user2 = User {
        email: String::from("binh@example.com"),
        ..user1
    };

    println!("{} {} {} {}", user2.username, user2.email, user2.sign_in_count, user2.active);
    println!("{}", user1.username); // ❌ username đã chuyển sang user2
}
```

**Trình biên dịch báo:**

```text
error[E0382]: borrow of moved value: `user1.username`
  --> src/main.rs:22:20
   |
16 |       let user2 = User {
   |  _________________-
17 | |         email: String::from("binh@example.com"),
18 | |         ..user1
19 | |     };
   | |_____- value moved here
...
22 |       println!("{}", user1.username); // ❌ username đã chuyển sang user2
   |                      ^^^^^^^^^^^^^^ value borrowed here after move
   |
   = note: move occurs because `user1.username` has type `String`, which does not implement the `Copy` trait
```

Nếu bạn tự cung cấp **mọi trường kiểu không** `Copy` (ở đây là `username` và `email`) thì chỉ còn các trường `Copy` được lấy từ struct cũ, nên struct cũ vẫn dùng được nguyên vẹn:

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        username: String::from("an123"),
        email: String::from("an@example.com"),
        sign_in_count: 1,
        active: true,
    };

    let user3 = User {
        username: String::from("chi456"),
        email: String::from("chi@example.com"),
        ..user1 // chỉ lấy sign_in_count và active (kiểu Copy)
    };

    println!("{} {} {} {}", user1.username, user1.email, user1.sign_in_count, user1.active);
    println!("{} {} {} {}", user3.username, user3.email, user3.sign_in_count, user3.active);
}
```

**Kết quả:**

```text
an123 an@example.com 1 true
chi456 chi@example.com 1 true
```

### 4.4 Tính khả biến

Để **sửa** một trường, cả biến struct phải khai báo `mut`. Rust **không** cho đánh dấu `mut` riêng cho từng trường.

```rust
struct Student {
    name: String,
    age: u8,
}

fn main() {
    let mut student = Student {
        name: String::from("An"),
        age: 20,
    };

    student.age += 1;
    println!("{} có tuổi {}", student.name, student.age);
}
```

**Kết quả:**

```text
An có tuổi 21
```

Quên `mut` thì compiler chặn ngay lúc biên dịch:

```rust
struct Student {
    name: String,
    age: u8,
}

fn main() {
    let student = Student {
        name: String::from("An"),
        age: 20,
    };

    student.age = 21; // ❌ student không phải mut
    println!("{} có tuổi {}", student.name, student.age);
}
```

**Trình biên dịch báo:**

```text
error[E0594]: cannot assign to `student.age`, as `student` is not declared as mutable
  --> src/main.rs:12:5
   |
12 |     student.age = 21; // ❌ student không phải mut
   |     ^^^^^^^^^^^^^^^^ cannot assign
   |
help: consider changing this to be mutable
   |
 7 |     let mut student = Student {
   |         +++
```

Quy tắc này nhất quán với biến thường: `let` là bất biến, muốn đổi thì thêm `mut` (xem bài [Biến](../learn_03_variables/README.md#33-biến-khả-biến-mut)).

### 4.5 In struct để gỡ lỗi

Khác với số hay chuỗi, struct **không tự biết cách in ra**. Dùng `{:?}` với struct chưa hỗ trợ sẽ lỗi:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 4 };
    println!("{:?}", p); // ❌ Point chưa có Debug
}
```

**Trình biên dịch báo:**

```text
error[E0277]: `Point` doesn't implement `Debug`
 --> src/main.rs:8:22
  |
8 |     println!("{:?}", p); // ❌ Point chưa có Debug
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

Thông báo đã nói sẵn cách sửa: thêm `#[derive(Debug)]` ngay phía trên định nghĩa. Dòng này nhờ compiler **tự sinh code** để in struct (khái niệm *trait* sẽ được học ở bài `generics_traits_lifetimes`).

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 3, y: 4 };

    println!("{:?}", p);
    println!("{:#?}", p);
    println!("x = {}, y = {}", p.x, p.y);
}
```

**Kết quả:**

```text
Point { x: 3, y: 4 }
Point {
    x: 3,
    y: 4,
}
x = 3, y = 4
```

| Cách in | Kết quả |
|---|---|
| `{:?}` | Một dòng, gọn |
| `{:#?}` | Nhiều dòng, thụt lề (*pretty print*), dễ đọc khi struct lớn |

Macro `dbg!` cũng dùng `Debug` nhưng **ghi ra stderr** (không phải stdout) và kèm **tên tệp, số dòng, biểu thức**. Nó trả lại giá trị được truyền vào nên có thể đặt ngay giữa một biểu thức.

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(30 * scale), // in ra rồi trả lại 60
        height: 50,
    };

    dbg!(&rect);
    println!("{} x {}", rect.width, rect.height);
}
```

**Kết quả:**

```text
60 x 50
```

**Đầu ra của** `dbg!` **(ghi ra stderr, hiện ngay trong terminal):**

```text
[src/main.rs:10:16] 30 * scale = 60
[src/main.rs:14:5] &rect = Rectangle {
    width: 60,
    height: 50,
}
```

> **Cẩn thận:** `dbg!(giá_trị)` **lấy quyền sở hữu** giá trị rồi trả lại. Nếu bạn chỉ muốn xem mà không chuyển đi, hãy truyền tham chiếu như `dbg!(&rect)`.

---

## 5. Method và associated function

Struct chỉ **chứa dữ liệu**. Để gắn **hành vi** (những việc kiểu dữ liệu đó làm được), ta viết trong khối `impl` (viết tắt của *implementation*).

### 5.1 Khối `impl` và method

Bắt đầu với một hàm thường tính diện tích hình chữ nhật, rồi so sánh với phiên bản method:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

// Hàm thường: nhận Rectangle qua tham chiếu
fn area_fn(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

impl Rectangle {
    // Method: tham số đầu tiên luôn là self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("Hàm thường: {}", area_fn(&rect));
    println!("Method:     {}", rect.area());
}
```

**Kết quả:**

```text
Hàm thường: 1500
Method:     1500
```

Những điểm cần nhớ:

- `impl Rectangle { ... }` nghĩa là mọi thứ bên trong **thuộc về kiểu** `Rectangle`.
- **Method** là hàm có tham số đầu tiên tên `self`, đại diện cho giá trị đang gọi method. Gọi bằng dấu chấm: `rect.area()`.
- `&self` là cách viết ngắn của `self: &Self`. `Self` là **bí danh** của kiểu đang được `impl`, ở đây là `Rectangle`.
- Cách gọi `rect.area()` dễ đọc hơn `area_fn(&rect)`, và mọi hành vi của kiểu nằm cùng một chỗ, dễ tìm.

### 5.2 Ba dạng self

Dạng của tham số đầu tiên quyết định method **làm gì với dữ liệu**. Đây chính là ba cách dùng giá trị ở bài Ownership: đọc, sửa, lấy đi.

| Dạng | Ý nghĩa | Sau khi gọi, biến còn dùng được? | Dùng khi |
|---|---|---|---|
| `&self` | **Mượn để đọc** | Có | Chỉ cần đọc dữ liệu (phổ biến nhất) |
| `&mut self` | **Mượn để sửa** | Có (biến phải là `mut`) | Cần thay đổi dữ liệu |
| `self` | **Lấy quyền sở hữu** | **Không**, giá trị đã bị chuyển vào method | Biến đổi giá trị thành thứ khác, hoặc "dùng một lần rồi bỏ" |

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self: chỉ đọc
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // &mut self: sửa dữ liệu tại chỗ
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // self: lấy quyền sở hữu, trả về một Rectangle mới
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }
}

fn main() {
    let mut rect = Rectangle { width: 3, height: 4 };
    println!("Diện tích: {}", rect.area());

    rect.scale(2);
    println!("Sau khi phóng to: {} x {}", rect.width, rect.height);

    let square = rect.into_square();
    println!("Hình vuông: {} x {}", square.width, square.height);
}
```

**Kết quả:**

```text
Diện tích: 12
Sau khi phóng to: 6 x 8
Hình vuông: 8 x 8
```

Sau dòng `rect.into_square()`, biến `rect` **đã bị chuyển đi**. Dùng lại nó sẽ lỗi:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle { width: side, height: side }
    }
}

fn main() {
    let rect = Rectangle { width: 3, height: 4 };
    let square = rect.into_square();

    println!("{}", square.area());
    println!("{}", rect.area()); // ❌ rect đã bị chuyển vào into_square
}
```

**Trình biên dịch báo:**

```text
error[E0382]: borrow of moved value: `rect`
  --> src/main.rs:22:20
   |
18 |     let rect = Rectangle { width: 3, height: 4 };
   |         ---- move occurs because `rect` has type `Rectangle`, which does not implement the `Copy` trait
19 |     let square = rect.into_square();
   |                       ------------- `rect` moved due to this method call
...
22 |     println!("{}", rect.area()); // ❌ rect đã bị chuyển vào into_square
   |                    ^^^^ value borrowed here after move
   |
note: `Rectangle::into_square` takes ownership of the receiver `self`, which moves `rect`
  --> src/main.rs:11:20
   |
11 |     fn into_square(self) -> Rectangle {
   |                    ^^^^
```

> **Quy tắc chọn:** bắt đầu bằng `&self`. Chỉ nâng lên `&mut self` khi cần sửa, và chỉ dùng `self` khi method thật sự "tiêu thụ" giá trị.

### 5.3 Associated function

Hàm trong `impl` mà **không có tham số** `self` gọi là **associated function** (hàm liên kết). Nó thuộc về kiểu chứ không thuộc về một giá trị cụ thể, nên gọi bằng `Kiểu::tên_hàm(...)` (hai dấu hai chấm), không phải dấu chấm.

Cách dùng phổ biến nhất là viết **hàm tạo** (*constructor*). Rust không có từ khoá riêng cho hàm tạo; theo quy ước, hàm tạo được đặt tên là `new`.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let a = Rectangle::new(30, 50);
    let b = Rectangle::square(10);

    println!("a: {}", a.area());
    println!("b: {}", b.area());
}
```

**Kết quả:**

```text
a: 1500
b: 100
```

Bạn đã dùng associated function từ trước: `String::from("An")` và `String::new()` chính là associated function của kiểu `String`.

| | Method | Associated function |
|---|---|---|
| Tham số đầu tiên | `self`, `&self` hoặc `&mut self` | Không có `self` |
| Cách gọi | `giá_trị.tên()` | `Kiểu::tên()` |
| Ví dụ | `rect.area()` | `Rectangle::new(30, 50)` |
| Dùng để | Làm việc với một giá trị đã có | Tạo giá trị mới hoặc làm việc liên quan đến cả kiểu |

### 5.4 Method có thêm tham số

Ngoài `self`, method có thể nhận thêm tham số như hàm bình thường. Ví dụ kiểm tra hình chữ nhật này có chứa được hình kia không:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let big = Rectangle { width: 30, height: 50 };
    let small = Rectangle { width: 10, height: 40 };
    let wide = Rectangle { width: 60, height: 45 };

    println!("big chứa small? {}", big.can_hold(&small));
    println!("big chứa wide? {}", big.can_hold(&wide));
}
```

**Kết quả:**

```text
big chứa small? true
big chứa wide? false
```

`other` được truyền bằng `&` vì method chỉ cần **đọc** nó. Nhờ mượn, cả `big` và `small` vẫn dùng được sau lời gọi.

### 5.5 Method trùng tên với trường

Method được phép có cùng tên với một trường. Rust phân biệt bằng **dấu ngoặc**: `rect.width` là trường, `rect.width()` là method.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> u32 {
        self.width
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("trường: {}", rect.width);
    println!("method: {}", rect.width());
    println!("height: {}", rect.height);
}
```

**Kết quả:**

```text
trường: 30
method: 30
height: 50
```

Method kiểu này gọi là **getter**: trả về giá trị của một trường. Theo quy ước của Rust, getter đặt **cùng tên trường**, không thêm tiền tố `get_`. Getter có ích khi trường là riêng tư (xem [mục 7.5](#75-quyền-truy-cập-pub-và-trường-riêng-tư)).

### 5.6 Nhiều khối `impl`

Một kiểu có thể có **nhiều khối** `impl`. Chúng được gộp lại như một. Cách này hợp lệ nhưng ít cần thiết với struct đơn giản; về sau nó được dùng để nhóm method theo chức năng hoặc theo trait.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    println!("Diện tích: {}", rect.area());
    println!("Chu vi: {}", rect.perimeter());
}
```

**Kết quả:**

```text
Diện tích: 1500
Chu vi: 160
```

### 5.7 Tự động tham chiếu

Bạn có thể thắc mắc: `area` nhận `&self` mà ta viết `rect.area()`, không viết `(&rect).area()`. Đó là nhờ **tự động tham chiếu** (*automatic referencing and dereferencing*): khi gọi method bằng dấu chấm, Rust tự thêm `&`, `&mut` hoặc `*` cho khớp với kiểu của `self`.

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 3, height: 4 };
    let r = ▭ // r là tham chiếu tới rect

    println!("{}", rect.area());
    println!("{}", Rectangle::area(&rect)); // dạng viết đầy đủ
    println!("{}", (&rect).area());
    println!("{}", r.area()); // gọi qua tham chiếu vẫn dùng dấu chấm
}
```

**Kết quả:**

```text
12
12
12
12
```

Bốn dòng cho cùng một kết quả. Dạng `Rectangle::area(&rect)` cho thấy method thật ra chỉ là hàm có tham số đầu tiên là `self`; dấu chấm là cú pháp tiện lợi.

---

## 6. Thực hành

Hai bài thực hành có hướng dẫn: một hình chữ nhật (struct, `Debug`, method) và một tài khoản ngân hàng (`&mut self`, `self`, hàm tạo). Thực hiện trong `learn_06_structs/src/main.rs` và chạy bằng `cargo run`.

### Lab 1. Hình chữ nhật: struct, Debug và method

**Mục tiêu:** định nghĩa struct, in nó ra để kiểm tra, rồi gắn hành vi bằng `impl`.

**Bước 1.** Định nghĩa struct, in ra để kiểm tra dữ liệu. Chép và chạy:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("{:#?}", rect);
    println!("{} x {}", rect.width, rect.height);
}
```

**Kết quả:**

```text
Rectangle {
    width: 30,
    height: 50,
}
30 x 50
```

**Bước 2.** Thêm khối `impl` với hàm tạo `square`, các method `area`, `perimeter`, `can_hold`, rồi thay `main` để dùng chúng:

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self { width: size, height: size }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle::square(60);

    println!("rect1: {:?}", rect1);
    println!("Diện tích rect1: {}", rect1.area());
    println!("Chu vi rect1: {}", rect1.perimeter());
    println!("rect1 chứa được rect2? {}", rect1.can_hold(&rect2));
    println!("rect1 chứa được rect3? {}", rect1.can_hold(&rect3));
}
```

**Kết quả:**

```text
rect1: Rectangle { width: 30, height: 50 }
Diện tích rect1: 1500
Chu vi rect1: 160
rect1 chứa được rect2? true
rect1 chứa được rect3? false
```

**Phân tích:**

- `square` là associated function (không có `self`), gọi bằng `Rectangle::square(60)`.
- `area`, `perimeter`, `can_hold` chỉ đọc dữ liệu nên dùng `&self`. Vì vậy `rect1` vẫn dùng được sau mỗi lần gọi.
- `can_hold` nhận `other: &Rectangle`, tức là mượn `rect2` và `rect3` chứ không lấy đi. Nhờ vậy sau đó ta vẫn dùng được chúng.
- `#[derive(Debug)]` cho phép in `rect1` bằng `{:?}`.

**Thử thêm:**

1. Thêm method `is_square(&self) -> bool` trả về `self.width == self.height`, rồi kiểm tra `rect3.is_square()` (đáp án: `true`).
2. Tạo `rect4` bằng `Rectangle { height: 10, ..rect1 }` và in ra. Các trường `u32` là `Copy` nên `rect1` vẫn dùng được (đáp án: `rect4` là `30 x 10`).

### Lab 2. Tài khoản ngân hàng: &mut self, self và hàm tạo

**Mục tiêu:** thấy ba dạng `self` trong một chương trình có ý nghĩa thực tế.

```rust
struct BankAccount {
    owner: String,
    balance: u64,
}

impl BankAccount {
    // Hàm tạo: associated function, không có self
    fn new(owner: &str) -> Self {
        Self {
            owner: String::from(owner),
            balance: 0,
        }
    }

    // &mut self: thay đổi số dư
    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    // &mut self: chỉ trừ khi đủ tiền, trả về true/false
    fn withdraw(&mut self, amount: u64) -> bool {
        if amount > self.balance {
            false
        } else {
            self.balance -= amount;
            true
        }
    }

    // &self: chỉ đọc
    fn summary(&self) -> String {
        format!("{}: {} đồng", self.owner, self.balance)
    }

    // self: đóng tài khoản, lấy đi quyền sở hữu và trả về số dư còn lại
    fn close(self) -> u64 {
        println!("Đã đóng tài khoản của {}", self.owner);
        self.balance
    }
}

fn main() {
    let mut account = BankAccount::new("An");
    println!("{}", account.summary());

    account.deposit(500_000);
    println!("{}", account.summary());

    let ok = account.withdraw(200_000);
    println!("Rút 200000: {ok}");
    let ok = account.withdraw(1_000_000);
    println!("Rút 1000000: {ok}");
    println!("{}", account.summary());

    let money = account.close();
    println!("Nhận lại {money} đồng");
}
```

**Kết quả:**

```text
An: 0 đồng
An: 500000 đồng
Rút 200000: true
Rút 1000000: false
An: 300000 đồng
Đã đóng tài khoản của An
Nhận lại 300000 đồng
```

**Phân tích:**

- `BankAccount::new("An")` nhận `&str` (đọc chuỗi) nhưng lưu vào trường kiểu `String` bằng `String::from`. Struct cần **sở hữu** tên của nó (xem [mục 7.6](#76-string-hay-str-trong-struct)).
- `deposit` và `withdraw` dùng `&mut self` nên biến phải khai báo `let mut account`.
- `withdraw` kiểm tra `amount > self.balance` trước khi trừ. `balance` là `u64` (không âm), nên trừ quá số dư sẽ tràn số (xem [mục 6.5 của bài Biến](../learn_03_variables/README.md#65-tràn-số-nguyên)).
- `summary` chỉ đọc nên dùng `&self`, gọi bao nhiêu lần cũng được.
- `close` dùng `self`: sau khi gọi, `account` **không còn tồn tại**. Điều này hợp lý, vì tài khoản đã đóng thì không nên dùng tiếp.

**Thử thêm:**

1. Thêm dòng `println!("{}", account.summary());` ngay sau `account.close()`. Quan sát lỗi `E0382: borrow of moved value: 'account'`.
2. Xoá đoạn kiểm tra trong `withdraw`, chỉ để `self.balance -= amount;` rồi rút 1_000_000. Ở chế độ debug chương trình panic với `attempt to subtract with overflow`.
3. `withdraw` trả về `bool` nên chưa nói được **vì sao** thất bại. Bài `error_handling` sẽ dùng `Result` để làm việc đó tốt hơn.

---

## 7. Tìm hiểu chi tiết

### 7.1 Move và Copy với struct

Mặc định, gán struct sang biến khác là **chuyển quyền sở hữu** (move), giống `String`. Biến cũ không dùng được nữa:

```rust
#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
}

fn main() {
    let a = Student { name: String::from("An"), age: 20 };
    let b = a;
    println!("{:?} {:?}", a, b); // ❌ a đã bị chuyển sang b
}
```

**Trình biên dịch báo:**

```text
error[E0382]: borrow of moved value: `a`
  --> src/main.rs:10:27
   |
 8 |     let a = Student { name: String::from("An"), age: 20 };
   |         - move occurs because `a` has type `Student`, which does not implement the `Copy` trait
 9 |     let b = a;
   |             - value moved here
10 |     println!("{:?} {:?}", a, b); // ❌ a đã bị chuyển sang b
   |                           ^ value borrowed here after move
   |
note: if `Student` implemented `Clone`, you could clone the value
  --> src/main.rs:2:1
   |
 2 | struct Student {
   | ^^^^^^^^^^^^^^ consider implementing `Clone` for this type
...
 9 |     let b = a;
   |             - you could clone this value
```

Có hai cách để giữ lại cả hai bản. Cách thứ nhất là `Clone`: tạo bản sao tường minh bằng `.clone()`.

```rust
#[derive(Debug, Clone)]
struct Student {
    name: String,
    age: u8,
}

fn main() {
    let a = Student { name: String::from("An"), age: 20 };
    let b = a.clone();

    println!("{:?} {:?}", a, b);
    println!("{} {}", a.name, b.age);
}
```

**Kết quả:**

```text
Student { name: "An", age: 20 } Student { name: "An", age: 20 }
An 20
```

Cách thứ hai là `Copy`: gán sẽ tự **sao chép** thay vì chuyển, không cần gọi `.clone()`. Chỉ dùng được khi **mọi trường đều là** `Copy` (số, `bool`, `char`...).

```rust
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let a = Point { x: 1, y: 2 };
    let b = a; // sao chép, a vẫn dùng được

    println!("{:?} {:?}", a, b);
    println!("{}", a.x + b.y);
}
```

**Kết quả:**

```text
Point { x: 1, y: 2 } Point { x: 1, y: 2 }
3
```

Struct có trường `String` (không `Copy`) thì không thể `derive(Copy)`:

```rust
#[derive(Clone, Copy)] // ❌ String không phải Copy
struct Student {
    name: String,
    age: u8,
}

fn main() {
    let s = Student { name: String::from("An"), age: 20 };
    println!("{} {}", s.name, s.age);
}
```

**Trình biên dịch báo:**

```text
error[E0204]: the trait `Copy` cannot be implemented for this type
 --> src/main.rs:2:8
  |
1 | #[derive(Clone, Copy)] // ❌ String không phải Copy
  |                 ---- in this derive macro expansion
2 | struct Student {
  |        ^^^^^^^
3 |     name: String,
  |     ------------ this field does not implement `Copy`
```

| Kiểu struct | Nên dùng | Lý do |
|---|---|---|
| Nhỏ, toàn số/`bool`/`char` (toạ độ, màu) | `Copy` (kèm `Clone`) | Sao chép rẻ, tiện dùng |
| Có `String`, `Vec`... | `Clone` khi cần bản sao | Sao chép tốn kém nên phải gọi tường minh |

### 7.2 Struct lồng nhau

Trường của struct có thể là một struct khác. Truy cập nối tiếp bằng dấu chấm: `c.center.x`.

```rust
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let c = Circle {
        center: Point { x: 1.0, y: 2.0 },
        radius: 3.0,
    };

    println!("Tâm: ({:.1}, {:.1})", c.center.x, c.center.y);
    println!("Diện tích: {:.2}", c.area());
    println!("{:?}", c);
}
```

**Kết quả:**

```text
Tâm: (1.0, 2.0)
Diện tích: 28.27
Circle { center: Point { x: 1.0, y: 2.0 }, radius: 3.0 }
```

### 7.3 Tách struct (destructuring)

Giống tuple, struct có thể được tách thành các biến bằng một **mẫu** (*pattern*) ở vế trái của `let`. Dùng `..` để bỏ qua các trường còn lại, và `tên_trường: tên_biến` để đổi tên.

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let p = Point { x: 1, y: 2, z: 3 };

    let Point { x, y, .. } = p; // bỏ qua z
    println!("{x} {y}");

    let Point { x: a, y: b, z: c } = p; // đổi tên trường thành biến a, b, c
    println!("{a} {b} {c}");
}
```

**Kết quả:**

```text
1 2
1 2 3
```

### 7.4 Struct nằm ở đâu trong bộ nhớ

Các trường của struct được xếp **liền nhau** trong một vùng nhớ (có thể có thêm vài byte đệm để căn chỉnh). Nếu biến là biến cục bộ thì vùng nhớ đó nằm trên **stack**. Riêng trường `String` chỉ giữ một "tay cầm" gồm con trỏ, độ dài và dung lượng (24 byte trên máy 64 bit); phần chữ thật nằm trên **heap**.

```rust
#![allow(dead_code)] // tắt cảnh báo "trường chưa dùng" vì ví dụ này chỉ đo kích thước

struct Point {
    x: f64,
    y: f64,
}

struct Student {
    name: String,
    age: u8,
    is_active: bool,
}

fn main() {
    println!("Point:   {} byte", size_of::<Point>());
    println!("String:  {} byte", size_of::<String>());
    println!("Student: {} byte", size_of::<Student>());
}
```

**Kết quả:**

```text
Point:   16 byte
String:  24 byte
Student: 32 byte
```

`Student` chiếm 32 byte dù tên dài hay ngắn: 24 byte cho `String`, 1 byte cho `u8`, 1 byte cho `bool`, cộng phần đệm để tổng căn theo 8 byte. Con số này có thể khác trên kiến trúc khác, và Rust được phép sắp xếp lại thứ tự trường để tiết kiệm bộ nhớ.

### 7.5 Quyền truy cập: `pub` và trường riêng tư

Trong Rust, mọi thứ mặc định là **riêng tư** (*private*) đối với code bên ngoài module chứa nó. Muốn cho bên ngoài dùng thì thêm `pub`. Với struct, `pub` được đặt **riêng cho từng trường**.

Ví dụ này dùng `mod` để tạo một module. Module được học kỹ ở chương riêng của giáo trình; ở đây bạn chỉ cần biết `mod bank { ... }` tạo ra một ranh giới mà bên ngoài không nhìn xuyên qua được trừ phần `pub`.

```rust
mod bank {
    pub struct Account {
        pub owner: String,
        balance: u64, // riêng tư: chỉ code trong mod bank thấy
    }

    impl Account {
        pub fn new(owner: &str, balance: u64) -> Account {
            Account {
                owner: String::from(owner),
                balance,
            }
        }

        pub fn balance(&self) -> u64 {
            self.balance
        }
    }
}

fn main() {
    let acc = bank::Account::new("An", 500_000);
    println!("{} có {} đồng", acc.owner, acc.balance());
}
```

**Kết quả:**

```text
An có 500000 đồng
```

Truy cập trực tiếp trường riêng tư từ ngoài module sẽ lỗi:

```rust
mod bank {
    pub struct Account {
        pub owner: String,
        balance: u64,
    }

    impl Account {
        pub fn new(owner: &str, balance: u64) -> Account {
            Account {
                owner: String::from(owner),
                balance,
            }
        }

        pub fn balance(&self) -> u64 {
            self.balance
        }
    }
}

fn main() {
    let acc = bank::Account::new("An", 500_000);
    println!("{} có {} đồng", acc.owner, acc.balance); // ❌ balance là riêng tư
}
```

**Trình biên dịch báo:**

```text
error[E0616]: field `balance` of struct `Account` is private
  --> src/main.rs:23:46
   |
23 |     println!("{} có {} đồng", acc.owner, acc.balance); // ❌ balance là riêng tư
   |                                              ^^^^^^^ private field
   |
help: a method `balance` also exists, call it with parentheses
   |
23 |     println!("{} có {} đồng", acc.owner, acc.balance()); // ❌ balance là riêng tư
   |                                                     ++
```

Đây là lý do getter (`balance()`) hữu ích: bên ngoài **đọc** được số dư nhưng không **tự gán** được giá trị tuỳ ý, mọi thay đổi phải đi qua các method (`deposit`, `withdraw`...) do bạn kiểm soát.

### 7.6 `String` hay `&str` trong struct

Cho trường văn bản, hãy dùng `String`: struct sở hữu dữ liệu của nó, đơn giản và an toàn. Dùng `&str` trực tiếp sẽ lỗi:

```rust
struct User {
    name: &str, // ❌ thiếu lifetime
}

fn main() {
    let user = User { name: "An" };
    println!("{}", user.name);
}
```

**Trình biên dịch báo:**

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:2:11
  |
2 |     name: &str, // ❌ thiếu lifetime
  |           ^ expected named lifetime parameter
  |
help: consider introducing a named lifetime parameter
  |
1 ~ struct User<'a> {
2 ~     name: &'a str, // ❌ thiếu lifetime
  |
```

Một tham chiếu (`&`) trong struct luôn **mượn** dữ liệu của ai đó, và compiler cần biết dữ liệu được mượn sống **bao lâu** để chắc chắn struct không tồn tại lâu hơn nó. Thời gian sống này gọi là **lifetime**, viết là `'a`. Compiler đã gợi ý cách sửa:

```rust
struct User<'a> {
    name: &'a str,
}

fn main() {
    let user = User { name: "An" };
    println!("{}", user.name);
}
```

**Kết quả:**

```text
An
```

Đừng lo nếu cú pháp `<'a>` còn lạ. Lifetime được giải thích ở bài [generics_traits_lifetimes](../learn_10_generics_traits_lifetimes/README.md). Hiện tại hãy theo quy tắc:

| Trường là | Dùng | Khi nào |
|---|---|---|
| Văn bản mà struct **sở hữu** | `String` | Hầu hết trường hợp |
| Văn bản **mượn** từ nơi khác | `&'a str` | Khi đã học lifetime và muốn tránh sao chép |

Với hàm tạo, nhận `&str` rồi chuyển thành `String` (như `BankAccount::new(owner: &str)` ở Lab 2) là cách vừa linh hoạt cho người gọi vừa giữ struct đơn giản.

### 7.7 Tuple, tuple struct hay struct

| Cần | Chọn |
|---|---|
| Trả về hai ba giá trị tạm thời từ hàm | Tuple `(a, b)` |
| Một kiểu riêng có tên, một hai trường, ý nghĩa đã rõ (đơn vị, ID) | Tuple struct, như `Meters(f64)` |
| Nhiều trường, mỗi trường có ý nghĩa riêng | Struct có tên trường |
| Một kiểu không có dữ liệu | Unit struct |

Quy tắc ngón tay cái: nếu bạn phải viết chú thích để nhớ `.2` là gì, hãy chuyển sang struct có tên trường.

### 7.8 Quy ước đặt tên

| Đối tượng | Quy ước | Ví dụ |
|---|---|---|
| Struct, tuple struct, unit struct | `UpperCamelCase` | `BankAccount` |
| Trường, method, hàm | `snake_case` | `is_active`, `can_hold` |
| Hàm tạo | `new` | `Rectangle::new(30, 50)` |
| Getter | Cùng tên trường, không có `get_` | `account.balance()` |
| Method trả về `bool` | Thường bắt đầu bằng `is_` hoặc `has_` | `is_square()` |

### 7.9 Hàm thường, method hay associated function

| Tình huống | Chọn |
|---|---|
| Việc không gắn với kiểu nào cụ thể | Hàm thường |
| Đọc dữ liệu của một giá trị đã có | Method `&self` |
| Sửa dữ liệu của một giá trị đã có | Method `&mut self` |
| Biến đổi, tiêu thụ giá trị | Method `self` |
| Tạo giá trị mới của kiểu | Associated function (`new`...) |

---

## 8. Lỗi thường gặp

### 8.1 Cách đọc thông báo lỗi

Cách đọc một thông báo lỗi (mã lỗi, vị trí `-->`, nhãn `^^^^`, `help:`) đã được giải thích ở [mục 7.1 của bài Biến](../learn_03_variables/README.md#71-cách-đọc-thông-báo-lỗi). Với struct, có thêm hai mẹo:

- Phần `note:` thường **trỏ tới chỗ định nghĩa struct hoặc method** liên quan, giúp bạn thấy ngay trường hay chữ ký nào đang bị dùng sai.
- Phần `help:` hay đã viết sẵn đoạn code cần thêm (ví dụ `#[derive(Debug)]`, `mut`, `&'a`).

### 8.2 Lỗi biên dịch

Những lỗi sau đã xuất hiện ở các mục trước: thiếu trường (E0063, [mục 4.1](#41-khởi-tạo-và-truy-cập-trường)), dùng trường sau khi bị chuyển (E0382, [mục 4.3](#43-cập-nhật-struct-từ-struct-cũ) và [mục 5.2](#52-ba-dạng-self)), sửa trường của biến bất biến (E0594, [mục 4.4](#44-tính-khả-biến)), thiếu `Debug` (E0277, [mục 4.5](#45-in-struct-để-gỡ-lỗi)). Dưới đây là năm lỗi khác rất hay gặp.

**Sai tên trường khi tạo struct (gõ nhầm):**

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        heigth: 50, // ❌ sai chính tả: heigth
    };

    println!("{} {}", rect.width, rect.height);
}
```

**Trình biên dịch báo:**

```text
error[E0560]: struct `Rectangle` has no field named `heigth`
 --> src/main.rs:9:9
  |
9 |         heigth: 50, // ❌ sai chính tả: heigth
  |         ^^^^^^ unknown field
  |
help: a field with a similar name exists
  |
9 -         heigth: 50, // ❌ sai chính tả: heigth
9 +         height: 50, // ❌ sai chính tả: heigth
  |
```

**Sai tên trường khi truy cập:**

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("{} {}", rect.width, rect.height);
    println!("{}", rect.lenght); // ❌ không có trường lenght
}
```

**Trình biên dịch báo:**

```text
error[E0609]: no field `lenght` on type `Rectangle`
  --> src/main.rs:10:25
   |
10 |     println!("{}", rect.lenght); // ❌ không có trường lenght
   |                         ^^^^^^ unknown field
   |
help: a field with a similar name exists
   |
10 -     println!("{}", rect.lenght); // ❌ không có trường lenght
10 +     println!("{}", rect.height); // ❌ không có trường lenght
   |
```

**Quên** `self` **trong method:**

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(width: u32, height: u32) -> u32 {
        width * height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };

    println!("{} {}", rect.width, rect.height);
    println!("{}", rect.area()); // ❌ area không có tham số self
}
```

**Trình biên dịch báo:**

```text
error[E0599]: no method named `area` found for struct `Rectangle` in the current scope
  --> src/main.rs:16:25
   |
 1 | struct Rectangle {
   | ---------------- method `area` not found for this struct
...
16 |     println!("{}", rect.area()); // ❌ area không có tham số self
   |                         ^^^^ this is an associated function, not a method
   |
   = note: found the following associated functions; to be used as methods, functions must have a `self` parameter
note: the candidate is defined in an impl for the type `Rectangle`
  --> src/main.rs:7:5
   |
 7 |     fn area(width: u32, height: u32) -> u32 {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: use associated function syntax instead
   |
16 -     println!("{}", rect.area()); // ❌ area không có tham số self
16 +     println!("{}", Rectangle::area()); // ❌ area không có tham số self
   |
```

Hàm không có `self` là associated function nên phải gọi bằng `Rectangle::area(30, 50)`. Muốn gọi bằng dấu chấm, hãy đổi chữ ký thành `fn area(&self) -> u32`.

**Quên** `mut` **khi gọi method** `&mut self`**:**

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }
}

fn main() {
    let counter = Counter::new();
    counter.increment(); // ❌ counter không phải mut
    println!("{}", counter.count);
}
```

**Trình biên dịch báo:**

```text
error[E0596]: cannot borrow `counter` as mutable, as it is not declared as mutable
  --> src/main.rs:17:5
   |
17 |     counter.increment(); // ❌ counter không phải mut
   |     ^^^^^^^ cannot borrow as mutable
   |
help: consider changing this to be mutable
   |
16 |     let mut counter = Counter::new();
   |         +++
```

**Trả về** `String` **từ trường khi chỉ có** `&self`**:**

```rust
struct Student {
    name: String,
}

impl Student {
    fn name(&self) -> String {
        self.name // ❌ không thể chuyển trường ra khỏi &self
    }
}

fn main() {
    let student = Student { name: String::from("An") };
    println!("{}", student.name());
}
```

**Trình biên dịch báo:**

```text
error[E0507]: cannot move out of `self.name` which is behind a shared reference
 --> src/main.rs:7:9
  |
7 |         self.name // ❌ không thể chuyển trường ra khỏi &self
  |         ^^^^^^^^^ move occurs because `self.name` has type `String`, which does not implement the `Copy` trait
  |
help: consider cloning the value if the performance cost is acceptable
  |
7 |         self.name.clone() // ❌ không thể chuyển trường ra khỏi &self
  |                  ++++++++
```

`&self` chỉ **mượn** struct, nên method không được lấy đi (move) một trường của nó. Có hai cách sửa, tuỳ nhu cầu:

```rust
struct Student {
    name: String,
}

impl Student {
    // Cách 1: trả về tham chiếu, không tốn bản sao (thường là lựa chọn tốt nhất)
    fn name(&self) -> &str {
        &self.name
    }

    // Cách 2: trả về bản sao riêng
    fn name_owned(&self) -> String {
        self.name.clone()
    }
}

fn main() {
    let student = Student { name: String::from("An") };
    println!("{}", student.name());
    println!("{}", student.name_owned());
}
```

**Kết quả:**

```text
An
An
```

**Bảng tổng hợp** (mã lỗi và thông báo là kết quả với Rust 1.99.1):

| Mã lỗi | Thông báo | Nguyên nhân | Cách sửa |
|---|---|---|---|
| `E0063` | `missing field 'x' in initializer of 'S'` | Tạo struct thiếu trường | Cung cấp đủ mọi trường (hoặc dùng `..other`) |
| `E0560` | `struct 'S' has no field named 'x'` | Ghi sai tên trường khi tạo | Sửa chính tả theo định nghĩa struct |
| `E0609` | `no field 'x' on type 'S'` | Truy cập trường không tồn tại | Sửa chính tả; hoặc dùng `()` nếu đó là method |
| `E0599` | `no method named 'x' found for struct 'S'` | Gọi method không tồn tại, hoặc gọi hàm không có `self` bằng dấu chấm | Thêm `&self`, hoặc gọi bằng `S::x()` |
| `E0594` | `cannot assign to 's.x', as 's' is not declared as mutable` | Gán vào trường của biến bất biến | Khai báo `let mut s` |
| `E0596` | `cannot borrow 's' as mutable, as it is not declared as mutable` | Gọi method `&mut self` trên biến bất biến | Khai báo `let mut s` |
| `E0382` | `borrow of moved value` | Dùng struct (hoặc trường) sau khi đã chuyển đi | Mượn bằng `&`, hoặc `.clone()`, hoặc cho kiểu `Copy` |
| `E0507` | `cannot move out of 'self.x' which is behind a shared reference` | Method `&self` cố chuyển trường ra ngoài | Trả về `&` tham chiếu hoặc `.clone()` |
| `E0277` | `'S' doesn't implement 'Debug'` | In bằng `{:?}` khi chưa `derive(Debug)` | Thêm `#[derive(Debug)]` |
| `E0106` | `missing lifetime specifier` | Trường `&str` mà không có lifetime | Dùng `String`, hoặc thêm `<'a>` |
| `E0616` | `field 'x' of struct 'S' is private` | Truy cập trực tiếp trường riêng tư | Dùng getter, hoặc thêm `pub` nếu hợp lý |
| `E0204` | `the trait 'Copy' cannot be implemented for this type` | `derive(Copy)` khi có trường không `Copy` | Bỏ `Copy`, dùng `Clone` |
| `E0308` | `mismatched types` | Giá trị sai kiểu so với trường | Dùng đúng kiểu đã khai báo |

### 8.3 Cảnh báo thường gặp

Cảnh báo không chặn biên dịch nhưng nên được sửa hết. Với struct, compiler hay cảnh báo về trường hoặc hàm **được định nghĩa mà chưa dùng**, và về tên kiểu sai quy ước.

```rust
// ⚠️ Biên dịch được nhưng có cảnh báo
struct my_point {
    x: i32,
    y: i32,
}

impl my_point {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }
}

fn main() {
    let p = my_point { x: 1, y: 2 };
    println!("{}", p.x);
}
```

**Trình biên dịch cảnh báo:**

```text
warning: type `my_point` should have an upper camel case name
 --> src/main.rs:2:8
  |
2 | struct my_point {
  |        ^^^^^^^^ help: convert the identifier to upper camel case: `MyPoint`
  |
  = note: `#[warn(non_camel_case_types)]` (part of `#[warn(nonstandard_style)]`) on by default

warning: field `y` is never read
 --> src/main.rs:4:5
  |
2 | struct my_point {
  |        -------- field in this struct
3 |     x: i32,
4 |     y: i32,
  |     ^
  |
  = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: associated function `origin` is never used
 --> src/main.rs:8:8
  |
7 | impl my_point {
  | ------------- associated function in this implementation
8 |     fn origin() -> Self {
  |        ^^^^^^
```

| Cảnh báo | Ý nghĩa | Cách xử lý |
|---|---|---|
| `type 'my_point' should have an upper camel case name` | Tên struct sai quy ước | Đổi thành `MyPoint` |
| `field 'y' is never read` | Trường được gán nhưng không bao giờ đọc | Dùng nó, hoặc xoá, hoặc đổi tên `_y` nếu cố ý |
| `associated function 'origin' is never used` | Viết hàm trong `impl` mà không gọi | Gọi nó hoặc xoá |

> **Lưu ý:** `#[derive(Debug)]` **không** được tính là "đã đọc" trường. Một struct chỉ được in bằng `{:?}` mà không code nào đọc trường vẫn bị cảnh báo `never read`. Compiler cố ý bỏ qua `Debug` khi kiểm tra điều này.

---

## 9. Bài tập

Hãy tự làm trước, chạy bằng `cargo run`, rồi mới mở phần đáp án. Mỗi bài có **kết quả mong đợi** để bạn tự kiểm tra.

### Cấp độ 1. Làm quen

#### Bài 1.1. Định nghĩa struct

**Đề:** khai báo struct `Book` gồm `title` (`String`), `pages` (`u32`) và `is_read` (`bool`). Tạo một cuốn sách "Lập trình Rust", 550 trang, chưa đọc, rồi in theo mẫu.

**Kết quả mong đợi:**

```text
Sách: Lập trình Rust, 550 trang, đã đọc: false
```

**Gợi ý:** trường văn bản dùng `String`, tạo bằng `String::from("...")`.

Xem đáp án

```rust
struct Book {
    title: String,
    pages: u32,
    is_read: bool,
}

fn main() {
    let book = Book {
        title: String::from("Lập trình Rust"),
        pages: 550,
        is_read: false,
    };

    println!("Sách: {}, {} trang, đã đọc: {}", book.title, book.pages, book.is_read);
}
```



#### Bài 1.2. Sửa lỗi khả biến

**Đề:** đoạn mã sau không biên dịch được. Hãy sửa để chạy đúng kết quả mong đợi.

```rust
struct Student {
    name: String,
    age: u8,
}

fn main() {
    let student = Student {
        name: String::from("An"),
        age: 20,
    };
    println!("{} có tuổi {}", student.name, student.age);

    student.age += 1; // ❌ student không phải mut
    println!("Năm sau {} có tuổi {}", student.name, student.age);
}
```

**Kết quả mong đợi:**

```text
An có tuổi 20
Năm sau An có tuổi 21
```

**Gợi ý:** đọc dòng `help:` trong thông báo lỗi. Có thể chỉ cho trường `age` là `mut` không?

Xem đáp án

```rust
struct Student {
    name: String,
    age: u8,
}

fn main() {
    let mut student = Student {
        name: String::from("An"),
        age: 20,
    };
    println!("{} có tuổi {}", student.name, student.age);

    student.age += 1;
    println!("Năm sau {} có tuổi {}", student.name, student.age);
}
```

Phải khai báo `mut` cho **cả biến** `student`. Rust không có cách đánh dấu `mut` riêng cho một trường.



#### Bài 1.3. Tuple struct

**Đề:** khai báo tuple struct `Rgb(u8, u8, u8)`. Tạo màu cam `Rgb(255, 165, 0)`, tách nó thành ba biến `r`, `g`, `b` rồi in cả dạng thập phân và dạng mã hex.

**Kết quả mong đợi:**

```text
rgb(255, 165, 0) = #FFA500
```

**Gợi ý:** `{:02X}` in số dưới dạng hex chữ hoa, đủ ít nhất 2 chữ số (thêm số 0 phía trước nếu thiếu). Có thể viết kèm tên biến: `{r:02X}`.

Xem đáp án

```rust
struct Rgb(u8, u8, u8);

fn main() {
    let orange = Rgb(255, 165, 0);
    let Rgb(r, g, b) = orange;

    println!("rgb({r}, {g}, {b}) = #{r:02X}{g:02X}{b:02X}");
}
```



### Cấp độ 2. Vận dụng

#### Bài 2.1. Dự đoán kết quả

**Đề:** đọc đoạn mã sau và **dự đoán** ba dòng in ra trước khi chạy. Chương trình có biên dịch được không?

```rust
struct Settings {
    volume: u8,
    brightness: u8,
    muted: bool,
}

fn main() {
    let base = Settings { volume: 50, brightness: 70, muted: false };
    let night = Settings { brightness: 20, ..base };
    let silent = Settings { muted: true, ..night };

    println!("{} {} {}", base.volume, base.brightness, base.muted);
    println!("{} {} {}", night.volume, night.brightness, night.muted);
    println!("{} {} {}", silent.volume, silent.brightness, silent.muted);
}
```

Xem đáp án

**Kết quả:**

```text
50 70 false
50 20 false
50 20 true
```

Giải thích: chương trình biên dịch được. Mọi trường của `Settings` đều là kiểu `Copy` (`u8`, `bool`), nên `..base` và `..night` chỉ **sao chép** giá trị chứ không chuyển đi. Vì vậy `base` và `night` vẫn dùng được nguyên vẹn sau đó. Nếu `Settings` có thêm một trường `String` được lấy từ `..base` thì `base` sẽ bị chuyển đi (xem [mục 4.3](#43-cập-nhật-struct-từ-struct-cũ)).



#### Bài 2.2. Method trên hình tròn

**Đề:** định nghĩa struct `Circle { radius: f64 }` với hàm tạo `new(radius: f64)` và hai method `area(&self)`, `circumference(&self)`. Tạo `Circle::new(2.5)` và in theo mẫu (hai chữ số thập phân).

**Kết quả mong đợi:**

```text
Bán kính 2.5: diện tích 19.63, chu vi 15.71
```

**Gợi ý:** dùng `std::f64::consts::PI`. Diện tích là `π × r²`, chu vi là `2 × π × r`.

Xem đáp án

```rust
use std::f64::consts::PI;

struct Circle {
    radius: f64,
}

impl Circle {
    fn new(radius: f64) -> Self {
        Self { radius }
    }

    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn circumference(&self) -> f64 {
        2.0 * PI * self.radius
    }
}

fn main() {
    let c = Circle::new(2.5);

    println!(
        "Bán kính {}: diện tích {:.2}, chu vi {:.2}",
        c.radius,
        c.area(),
        c.circumference()
    );
}
```



#### Bài 2.3. Tuple struct với method

**Đề:** khai báo tuple struct `Celsius(f64)` và method `to_fahrenheit(&self) -> f64` theo công thức `F = C × 9 / 5 + 32`. Với `Celsius(36.6)` in kết quả theo mẫu.

**Kết quả mong đợi:**

```text
36.6°C = 97.88°F
```

**Gợi ý:** trong method, giá trị bên trong tuple struct là `self.0`. Viết `9.0 / 5.0`, không viết `9 / 5`.

Xem đáp án

```rust
struct Celsius(f64);

impl Celsius {
    fn to_fahrenheit(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

fn main() {
    let temp = Celsius(36.6);
    println!("{}°C = {:.2}°F", temp.0, temp.to_fahrenheit());
}
```



#### Bài 2.4. Bộ đếm với `&mut self`

**Đề:** đoạn `main` dưới đây dùng struct `Counter` chưa có `impl`. Viết `impl Counter` gồm `new()` (bắt đầu từ 0), `increment(&mut self)` (tăng 1), `reset(&mut self)` (về 0) và `value(&self) -> u32`.

```rust
struct Counter {
    count: u32,
}

fn main() {
    let mut counter = Counter::new(); // ❌ chưa có impl Counter
    counter.increment();
    counter.increment();
    counter.increment();
    println!("Sau 3 lần tăng: {}", counter.value());

    counter.reset();
    println!("Sau khi reset: {}", counter.value());
}
```

**Kết quả mong đợi:**

```text
Sau 3 lần tăng: 3
Sau khi reset: 0
```

**Gợi ý:** `increment` và `reset` sửa dữ liệu, nên dùng `&mut self`; `value` chỉ đọc, nên dùng `&self`.

Xem đáp án

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Self {
        Self { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn reset(&mut self) {
        self.count = 0;
    }

    fn value(&self) -> u32 {
        self.count
    }
}

fn main() {
    let mut counter = Counter::new();
    counter.increment();
    counter.increment();
    counter.increment();
    println!("Sau 3 lần tăng: {}", counter.value());

    counter.reset();
    println!("Sau khi reset: {}", counter.value());
}
```



#### Bài 2.5. Sửa lỗi dùng sau khi chuyển

**Đề:** đoạn mã sau không biên dịch được. Sửa **method** (không sửa `main`) để gọi `describe` hai lần được.

```rust
struct Ticket {
    id: u32,
    owner: String,
}

impl Ticket {
    fn describe(self) -> String {
        format!("Vé {} của {}", self.id, self.owner)
    }
}

fn main() {
    let ticket = Ticket { id: 7, owner: String::from("An") };
    println!("{}", ticket.describe());
    println!("{}", ticket.describe()); // ❌ ticket đã bị chuyển vào lần gọi đầu
}
```

**Kết quả mong đợi:**

```text
Vé 7 của An
Vé 7 của An
```

**Gợi ý:** method chỉ cần **đọc** dữ liệu. Dạng `self` nào phù hợp?

Xem đáp án

```rust
struct Ticket {
    id: u32,
    owner: String,
}

impl Ticket {
    fn describe(&self) -> String {
        format!("Vé {} của {}", self.id, self.owner)
    }
}

fn main() {
    let ticket = Ticket { id: 7, owner: String::from("An") };
    println!("{}", ticket.describe());
    println!("{}", ticket.describe());
}
```

Đổi `self` thành `&self`: method chỉ mượn, nên `ticket` vẫn còn sau mỗi lần gọi. Chỉ dùng `self` khi method thật sự cần tiêu thụ giá trị.



### Cấp độ 3. Thử thách

#### Bài 3.1. Struct lồng nhau

**Đề:** định nghĩa `Point { x: f64, y: f64 }` và `Line { start: Point, end: Point }`. Viết method `length(&self) -> f64` cho `Line` trả về độ dài đoạn thẳng. Với đoạn từ `(0, 0)` đến `(3, 4)` in kết quả theo mẫu.

**Kết quả mong đợi:**

```text
Đường từ (0.0, 0.0) đến (3.0, 4.0) dài 5.00
```

**Gợi ý:** độ dài là `√(dx² + dy²)` với `dx = end.x - start.x`. Số thực có method `.sqrt()`.

Xem đáp án

```rust
struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn length(&self) -> f64 {
        let dx = self.end.x - self.start.x;
        let dy = self.end.y - self.start.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let line = Line {
        start: Point { x: 0.0, y: 0.0 },
        end: Point { x: 3.0, y: 4.0 },
    };

    println!(
        "Đường từ ({:.1}, {:.1}) đến ({:.1}, {:.1}) dài {:.2}",
        line.start.x,
        line.start.y,
        line.end.x,
        line.end.y,
        line.length()
    );
}
```

Giải thích: `dx = 3`, `dy = 4`, `√(9 + 16) = √25 = 5`.



#### Bài 3.2. Bốn đoạn mã, bốn lỗi

**Đề:** mỗi đoạn mã dưới đây đều không biên dịch được. Với từng đoạn, hãy **dự đoán lỗi** trước khi chạy `cargo check`, rồi sửa.

**Đoạn (a):**

```rust
struct Book {
    title: String,
    pages: u32,
    is_read: bool,
}

fn main() {
    let book = Book {
        title: String::from("Lập trình Rust"),
        pages: 550,
    }; // ❌

    println!("{} {}", book.title, book.pages);
}
```

**Đoạn (b):**

```rust
struct Student {
    name: String,
    age: u8,
}

fn main() {
    let student = Student {
        name: String::from("An"),
        age: "20", // ❌
    };

    println!("{} {}", student.name, student.age);
}
```

**Đoạn (c):**

```rust
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let p = Point::new(1, 2);
    let q = p.new(3, 4); // ❌

    println!("{} {} {} {}", p.x, p.y, q.x, q.y);
}
```

**Đoạn (d):**

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 1, y: 2 };

    println!("{}", p); // ❌
    println!("{} {}", p.x, p.y);
}
```

Xem đáp án

| Đoạn | Lỗi | Nguyên nhân | Cách sửa |
|---|---|---|---|
| (a) | `E0063` missing field `is_read` | Tạo struct thiếu trường `is_read` | Thêm `is_read: false,` |
| (b) | `E0308` mismatched types | `age` là `u8`, nhưng `"20"` là chuỗi `&str` | Viết `age: 20` |
| (c) | `E0599` no method named `new` found | `new` là associated function (không có `self`), không gọi được bằng dấu chấm trên giá trị | Gọi `Point::new(3, 4)` |
| (d) | `E0277` `Point` doesn't implement `std::fmt::Display` | `{}` cần kiểu có `Display`, struct không tự có | In từng trường (`p.x`, `p.y`), hoặc `#[derive(Debug)]` và dùng `{:?}` |



#### Bài 3.3. Chuyển tiền giữa hai tài khoản

**Đề:** dùng lại `BankAccount` của [Lab 2](#lab-2-tài-khoản-ngân-hàng-mut-self-self-và-hàm-tạo) (các hàm `new`, `deposit`, `withdraw`, `summary`). Viết **hàm thường** `transfer(from: &mut BankAccount, to: &mut BankAccount, amount: u64) -> bool` chuyển tiền từ `from` sang `to`, trả về `true` nếu thành công và `false` nếu `from` không đủ tiền (khi đó không ai bị thay đổi số dư).

```rust
fn main() {
    let mut an = BankAccount::new("An"); // ❌ chưa có BankAccount và transfer
    let mut binh = BankAccount::new("Binh");
    an.deposit(500_000);

    println!("Chuyển 300000: {}", transfer(&mut an, &mut binh, 300_000));
    println!("Chuyển 900000: {}", transfer(&mut an, &mut binh, 900_000));
    println!("{}", an.summary());
    println!("{}", binh.summary());
}
```

**Kết quả mong đợi:**

```text
Chuyển 300000: true
Chuyển 900000: false
An: 200000 đồng
Binh: 300000 đồng
```

**Gợi ý:** `withdraw` đã trả về `bool` cho biết có trừ được tiền hay không. Hãy dựa vào nó để quyết định có `deposit` vào `to` hay không.

Xem đáp án

```rust
struct BankAccount {
    owner: String,
    balance: u64,
}

impl BankAccount {
    fn new(owner: &str) -> Self {
        Self {
            owner: String::from(owner),
            balance: 0,
        }
    }

    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u64) -> bool {
        if amount > self.balance {
            false
        } else {
            self.balance -= amount;
            true
        }
    }

    fn summary(&self) -> String {
        format!("{}: {} đồng", self.owner, self.balance)
    }
}

fn transfer(from: &mut BankAccount, to: &mut BankAccount, amount: u64) -> bool {
    if from.withdraw(amount) {
        to.deposit(amount);
        true
    } else {
        false
    }
}

fn main() {
    let mut an = BankAccount::new("An");
    let mut binh = BankAccount::new("Binh");
    an.deposit(500_000);

    println!("Chuyển 300000: {}", transfer(&mut an, &mut binh, 300_000));
    println!("Chuyển 900000: {}", transfer(&mut an, &mut binh, 900_000));
    println!("{}", an.summary());
    println!("{}", binh.summary());
}
```

Giải thích:

- `transfer` nhận **hai tham chiếu** `&mut` tới hai tài khoản khác nhau nên được phép. Gọi `transfer(&mut an, &mut an, ...)` (cùng một biến hai lần) sẽ bị cấm, vì không được có hai tham chiếu `&mut` tới cùng một giá trị cùng lúc (quy tắc mượn của [Ownership](../learn_05_ownership/README.md)).
- `from.withdraw(amount)` vừa **thực hiện** việc trừ tiền vừa **báo** kết quả, nên `if` quyết định luôn có cộng tiền cho `to` hay không. Lần chuyển thứ hai thất bại (900000 > 200000) và không ai bị đổi số dư.



---

## 10. Tóm tắt

| Chủ đề | Ghi nhớ |
|---|---|
| Struct | Gom các giá trị liên quan thành một kiểu mới, **mỗi trường có tên** |
| Ba loại | Có tên trường `S { x: i32 }`, tuple struct `S(i32)`, unit struct `S;` |
| Tạo giá trị | Ghi `Tên { trường: giá_trị, ... }`, đủ mọi trường, thứ tự tuỳ ý |
| Viết tắt | `S { x, y }` khi tên biến trùng tên trường |
| Cập nhật | `..cũ` đặt cuối; **di chuyển** các trường không `Copy` từ struct cũ |
| Khả biến | Cả biến phải `mut` để sửa trường; không có `mut` riêng cho từng trường |
| In để gỡ lỗi | `#[derive(Debug)]`, rồi `{:?}` hoặc `{:#?}`; `dbg!` in ra stderr |
| `impl` | Khối chứa method và associated function của kiểu |
| `&self` / `&mut self` / `self` | Đọc / sửa / lấy quyền sở hữu; bắt đầu bằng `&self` |
| Associated function | Không có `self`, gọi bằng `Kiểu::tên()`; `new` là quy ước hàm tạo |
| `Self` | Bí danh của kiểu đang `impl` |
| Tự động tham chiếu | `rect.area()` tự thêm `&`/`&mut`/`*` cho khớp `self` |
| Move và Copy | Gán struct mặc định là **chuyển**; `Clone` để tạo bản sao, `Copy` khi mọi trường đều `Copy` |
| Trường văn bản | Dùng `String`; `&str` cần lifetime (bài `generics_traits_lifetimes`) |
| Riêng tư | Trường mặc định riêng tư ngoài module; dùng `pub` và getter |
| Đặt tên | Kiểu `UpperCamelCase`; trường, method `snake_case`; hàm tạo `new` |

Bài tiếp theo: [Enum và match](../learn_07_enums_match/README.md).

---

## 11. Tài liệu tham khảo

- [The Rust Programming Language, 5.1 Defining and Instantiating Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
- [The Rust Programming Language, 5.2 An Example Program Using Structs](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
- [The Rust Programming Language, 5.3 Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html)
- [Rust By Example, Structures](https://doc.rust-lang.org/rust-by-example/custom_types/structs.html)
- [Rust By Example, Methods](https://doc.rust-lang.org/rust-by-example/fn/methods.html)
- [The Rust Reference, Structs](https://doc.rust-lang.org/reference/items/structs.html)
- [The Rust Reference, Implementations](https://doc.rust-lang.org/reference/items/implementations.html)
- [Tài liệu thư viện chuẩn, macro](https://doc.rust-lang.org/std/macro.dbg.html) `dbg!`
- [Tài liệu thư viện chuẩn, trait](https://doc.rust-lang.org/std/fmt/trait.Debug.html) `Debug`
- [Rust API Guidelines, Naming](https://rust-lang.github.io/api-guidelines/naming.html)
- [Tra cứu mã lỗi, ví dụ E0063](https://doc.rust-lang.org/error_codes/E0063.html)
