# Xử lý lỗi (Error Handling) trong Rust

> Tài liệu học tập về xử lý lỗi trong Rust: `panic!`, `Result`, `Option`, toán tử `?`, kiểu lỗi tự định nghĩa, thực hành và bài tập có đáp án.

| Thông tin | Chi tiết |
|---|---|
| Chủ đề | Lỗi không khôi phục được (`panic!`), lỗi khôi phục được (`Result`, `Option`), toán tử `?`, enum lỗi, `Box<dyn Error>` |
| Môi trường | Rust 1.99.1 (stable), Cargo 1.99.1, edition 2024 |
| Dự án | `error_handling` (thư mục này) |
| Nguồn tham khảo chính | *The Rust Programming Language*, chương 9 |
| Bài trước | [collections](../collections/README.md) |
| Bài sau | [generics_traits_lifetimes](../generics_traits_lifetimes/README.md) |

## Mục lục

1. [Mục tiêu bài học](#1-mục-tiêu-bài-học)
2. [Khái niệm](#2-khái-niệm)
3. [Panic: lỗi không thể khôi phục](#3-panic-lỗi-không-thể-khôi-phục)
4. [Result: lỗi có thể khôi phục](#4-result-lỗi-có-thể-khôi-phục)
5. [Lan truyền lỗi với dấu hỏi chấm](#5-lan-truyền-lỗi-với-dấu-hỏi-chấm)
6. [Kiểu lỗi của riêng bạn](#6-kiểu-lỗi-của-riêng-bạn)
7. [Thực hành](#7-thực-hành)
8. [Tìm hiểu chi tiết](#8-tìm-hiểu-chi-tiết)
9. [Lỗi thường gặp](#9-lỗi-thường-gặp)
10. [Bài tập](#10-bài-tập)
11. [Tóm tắt](#11-tóm-tắt)
12. [Tài liệu tham khảo](#12-tài-liệu-tham-khảo)

---

## 1. Mục tiêu bài học

Sau bài học này bạn có thể:

- [ ] Phân biệt lỗi **khôi phục được** và **không khôi phục được**, từ đó chọn `Result` hay `panic!`.
- [ ] Đọc hiểu thông báo panic và bật `RUST_BACKTRACE` để lần ra nơi gây lỗi.
- [ ] Dùng `Result<T, E>` và `Option<T>` với `match`, `if let`, `unwrap`, `expect`, `unwrap_or`...
- [ ] Dùng toán tử `?` để chuyển lỗi lên cho nơi gọi; viết `main` trả về `Result`.
- [ ] Tự định nghĩa enum lỗi, dùng `From` để `?` tự đổi kiểu lỗi, biết khi nào dùng `Box<dyn Error>`.
- [ ] Đọc hiểu và sửa các lỗi biên dịch phổ biến liên quan đến xử lý lỗi.

**Kiến thức cần có.** Bạn đã biết `enum`, `match` (bài [enums_match](../enums_match/README.md)) và `Vec` (bài [collections](../collections/README.md)). Từ mục 6 trở đi có vài dòng `impl ... for ...`. Đó là *trait*, sẽ học kỹ ở bài sau; bài này chỉ yêu cầu chép đúng theo mẫu.

**Cách dùng tài liệu.** Chép từng ví dụ vào `src/main.rs` rồi chạy:

```bash
cd error_handling
cargo run
```

Các ví dụ được đánh dấu như sau:

- Ví dụ có chú thích `// ❌` **không biên dịch được** (cố ý, để minh hoạ lỗi).
- Ví dụ có chú thích `// ⚠️` biên dịch được nhưng sinh cảnh báo.
- Ví dụ có chú thích `// 💥` biên dịch được nhưng **dừng giữa chừng lúc chạy** (panic, hoặc `main` trả về lỗi). Đây là chủ ý. Phần **Khi chạy** bên dưới ghi lại đúng những gì in ra và mã thoát của chương trình.
- Các ví dụ còn lại biên dịch sạch và cho đúng kết quả ghi bên dưới.

> **Về thông báo panic.** Dòng `thread 'main' (12345) panicked at src/main.rs:3:5:` có hai con số cần lưu ý. Số trong ngoặc tròn là ID của luồng, **mỗi lần chạy một khác**, nên tài liệu ghi chung là `(12345)`. Cặp `3:5` là dòng và cột trong đúng đoạn mã được chép ở tài liệu.

---

## 2. Khái niệm

### 2.1 Hai loại lỗi

Chương trình nào cũng gặp lỗi: người dùng nhập chữ thay vì số, tệp không tồn tại, mạng bị ngắt. Rust chia lỗi làm hai loại và cho mỗi loại một công cụ riêng.

| | Khôi phục được (recoverable) | Không khôi phục được (unrecoverable) |
|---|---|---|
| Ví dụ | Người dùng nhập `"abc"` thay vì số, tệp không tồn tại, mạng quá thời gian chờ | Chỉ số mảng ngoài phạm vi, chia số nguyên cho 0, một điều kiện "không thể sai" lại sai |
| Bản chất | Tình huống bình thường, có thể xảy ra | Thường là **lỗi lập trình** (bug) |
| Công cụ của Rust | Kiểu `Result<T, E>` | Macro `panic!` |
| Chương trình làm gì | Nơi gọi tự quyết: báo lỗi, thử lại, dùng giá trị mặc định... | Dừng ngay |

Rust **không có ngoại lệ** (exception) với `try/catch` như Java hay Python. Thay vào đó, khả năng thất bại được ghi ngay trong **kiểu trả về** của hàm. Vì vậy trình biên dịch buộc bạn nhìn thấy và xử lý lỗi, thay vì để nó trôi qua im lặng.

### 2.2 Option và Result

Hai enum này nằm sẵn trong thư viện chuẩn, không cần `use`:

```text
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}
```

`T` và `E` là "chỗ trống" cho kiểu dữ liệu (*generics*, học ở bài sau). Ví dụ `Result<u32, ParseIntError>` nghĩa là: thành công thì chứa một `u32`, thất bại thì chứa một `ParseIntError`.

| | `Option<T>` | `Result<T, E>` |
|---|---|---|
| Ý nghĩa | **Có hoặc không có** giá trị | **Thành công hoặc thất bại** |
| Hai dạng | `Some(giá_trị)` / `None` | `Ok(giá_trị)` / `Err(lỗi)` |
| Có cho biết lý do không | Không | Có, nằm trong `Err` |
| Ví dụ | Phần tử đầu tiên của danh sách rỗng | Đổi `"abc"` thành số |

Các dạng `Ok`, `Err`, `Some`, `None` dùng trực tiếp, không cần viết `Result::Ok`.

### 2.3 Bản đồ công cụ

| Tôi muốn... | Dùng | Xem mục |
|---|---|---|
| Dừng chương trình vì lỗi lập trình | `panic!`, `unwrap`, `expect` | [3](#3-panic-lỗi-không-thể-khôi-phục), [4.5](#45-các-phương-thức-tiện-lợi) |
| Xử lý từng trường hợp thành công hoặc thất bại | `match`, `if let` | [4.2](#42-match-trên-result), [4.4](#44-if-let) |
| Chuyển lỗi lên cho hàm gọi mình | `?` | [5](#5-lan-truyền-lỗi-với-dấu-hỏi-chấm) |
| Hàm có nhiều nguồn lỗi khác nhau | enum lỗi, `Box<dyn Error>` | [6](#6-kiểu-lỗi-của-riêng-bạn) |
| Quyết định nên `panic!` hay trả `Result` | Bảng cân nhắc | [8.1](#81-chọn-panic-hay-result) |

---

## 3. Panic: lỗi không thể khôi phục

### 3.1 Macro panic

`panic!` in thông báo lỗi rồi **dừng chương trình**. Mọi dòng phía sau không bao giờ chạy.

```rs
fn main() {
    println!("Bắt đầu");
    panic!("Có lỗi nghiêm trọng!"); // 💥
}
```

**Khi chạy (mã thoát 101):**

```text
Bắt đầu

thread 'main' (12345) panicked at src/main.rs:3:5:
Có lỗi nghiêm trọng!
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Dòng `Bắt đầu` in ra bình thường vì nó chạy trước khi panic. Chương trình kết thúc với **mã thoát 101**, mã mà Rust dùng cho panic (xem [mục 8.5](#85-mã-thoát-của-chương-trình)).

### 3.2 Panic do chương trình tự sinh

Bạn không cần gọi `panic!` mới gặp panic. Rất nhiều thao tác tự gây ra panic khi vi phạm điều kiện:

| Nguyên nhân | Thông báo |
|---|---|
| Chỉ số ngoài phạm vi, chỉ biết lúc chạy (`v[5]` khi `v` có 3 phần tử) | `index out of bounds: the len is 3 but the index is 5` |
| Chia số nguyên cho 0, chỉ biết lúc chạy | `attempt to divide by zero` |
| Tràn số nguyên ở chế độ debug (`cargo run`) | `attempt to add with overflow` |
| `unwrap()` trên `None` | ``called `Option::unwrap()` on a `None` value`` |
| `unwrap()` trên `Err` | ``called `Result::unwrap()` on an `Err` value: ...`` |
| `expect("...")` trên `None` hoặc `Err` | Thông điệp của bạn, kèm giá trị lỗi nếu là `Err` |

Ví dụ chỉ số mảng:

```rs
fn main() {
    let v = vec![1, 2, 3];
    let i = 5;
    println!("{}", v[i]); // 💥
}
```

**Khi chạy (mã thoát 101):**

```text
thread 'main' (12345) panicked at src/main.rs:4:21:
index out of bounds: the len is 3 but the index is 5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Rust dừng chương trình thay vì đọc sang vùng nhớ khác. Đây là điểm an toàn của Rust so với C/C++, nơi truy cập ngoài phạm vi có thể âm thầm đọc hoặc ghi sai dữ liệu.

Phép chia cho 0 cũng vậy. Khi trình biên dịch **thấy trước** giá trị 0, nó chặn ngay lúc biên dịch:

```rust
fn main() {
    let b = 0;
    println!("{}", 10 / b); // ❌ compiler thấy trước phép chia cho 0
}
```

**Trình biên dịch báo:**

```text
error: this operation will panic at runtime
 --> src/main.rs:3:20
  |
3 |     println!("{}", 10 / b); // ❌ compiler thấy trước phép chia cho 0
  |                    ^^^^^^ attempt to divide `10_i32` by zero
  |
  = note: `#[deny(unconditional_panic)]` on by default
```

Còn khi số 0 đến từ nơi compiler không theo dõi được (tham số hàm, dữ liệu nhập vào), lỗi chỉ lộ ra lúc chạy:

```rs
fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {
    println!("{}", divide(10, 0)); // 💥
}
```

**Khi chạy (mã thoát 101):**

```text
thread 'main' (12345) panicked at src/main.rs:2:5:
attempt to divide by zero
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

### 3.3 Đọc thông báo panic và backtrace

Thông báo panic gồm các phần sau:

| Phần | Ý nghĩa |
|---|---|
| `thread 'main'` | Luồng bị panic. Chương trình đơn giản chỉ có luồng `main` |
| `(12345)` | ID của luồng, thay đổi mỗi lần chạy |
| `panicked at src/main.rs:2:5:` | Vị trí panic: tệp, dòng 2, cột 5 |
| Dòng kế tiếp | Thông điệp của panic |
| `note: run with RUST_BACKTRACE=1 ...` | Gợi ý bật backtrace để xem thêm |

**Backtrace** (dấu vết ngăn xếp) liệt kê chuỗi hàm đang gọi nhau dẫn tới chỗ panic. Nó có ích khi panic xảy ra sâu trong một hàm mà bạn không biết ai đã gọi. Bật bằng biến môi trường `RUST_BACKTRACE` khi chạy lệnh trong terminal (đây không phải là code Rust):

```bash
RUST_BACKTRACE=1 cargo run
```

Cú pháp `TÊN=giá_trị lệnh` đặt biến môi trường chỉ cho riêng lệnh đó. Thử với đoạn mã sau (chạy ở chế độ debug bằng `cargo run`, trong dự án `error_handling`):

```rs
fn chia(a: i32, b: i32) -> i32 {
    a / b // 💥
}

fn main() {
    println!("Kết quả: {}", chia(10, 0));
}
```

**Khi chạy với RUST_BACKTRACE=1 (mã thoát 101):**

```text
thread 'main' (12345) panicked at src/main.rs:2:5:
attempt to divide by zero
stack backtrace:
   ...
   3: error_handling::chia
             at ./src/main.rs:2:5
   4: error_handling::main
             at ./src/main.rs:6:33
   ...
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

Dấu `...` là các dòng đã lược bớt. Cách đọc: khung số 3 là `chia`, nơi panic xảy ra (dòng 2); nó được gọi bởi khung số 4 là `main` (dòng 6). Các khung còn lại là mã nội bộ của thư viện chuẩn, bạn có thể bỏ qua. Hãy tìm những dòng có tên dự án và `src/main.rs`. Số hiệu khung và đường dẫn tới thư viện chuẩn có thể khác giữa các máy và phiên bản Rust.

### 3.4 Khi panic xảy ra

- Mặc định, Rust **đi ngược lại các hàm đang gọi** (*unwinding*) và dọn dẹp dữ liệu của từng hàm trước khi dừng hẳn. Việc này tốn công và làm tệp thực thi lớn hơn.
- Nếu muốn dừng ngay và để hệ điều hành dọn dẹp (*abort*), thêm vào `Cargo.toml` để áp dụng cho bản `release`:

```toml
[profile.release]
panic = "abort"
```

Người mới chưa cần đổi cấu hình này.

---

## 4. Result: lỗi có thể khôi phục

### 4.1 Result trong thực tế

Bạn đã gặp `Result` mỗi khi gọi `parse`: đổi chuỗi thành số có thể thất bại nên `parse` trả về `Result`, không phải số.

```rust
fn main() {
    let ok = "42".parse::<u32>();
    let err = "abc".parse::<u32>();

    println!("{:?}", ok);
    println!("{:?}", err);
}
```

**Kết quả:**

```text
Ok(42)
Err(ParseIntError { kind: InvalidDigit })
```

Vì kết quả là `Result`, bạn **không thể** dùng nó như một số:

```rust
fn main() {
    let n: u32 = "5".parse::<u32>(); // ❌ vế phải là Result, chưa phải u32
    println!("{n}");
}
```

**Trình biên dịch báo:**

```text
error[E0308]: mismatched types
 --> src/main.rs:2:18
  |
2 |     let n: u32 = "5".parse::<u32>(); // ❌ vế phải là Result, chưa phải u32
  |            ---   ^^^^^^^^^^^^^^^^^^ expected `u32`, found `Result<u32, ParseIntError>`
  |            |
  |            expected due to this
  |
  = note: expected type `u32`
             found enum `Result<u32, ParseIntError>`
help: consider using `Result::expect` to unwrap the `Result<u32, ParseIntError>` value, panicking if the value is a `Result::Err`
  |
2 |     let n: u32 = "5".parse::<u32>().expect("REASON"); // ❌ vế phải là Result, chưa phải u32
  |                                    +++++++++++++++++
```

Đây là ý đồ của Rust: bạn phải **mở "chiếc hộp" `Result`** và quyết định làm gì với trường hợp `Err` trước khi chạm vào giá trị bên trong.

### 4.2 match trên Result

Cách rõ ràng nhất để mở `Result` là `match`, xử lý cả hai nhánh `Ok` và `Err`:

```rust
fn main() {
    for text in ["42", "abc", "-7", "", " 5"] {
        match text.parse::<u32>() {
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
"-7" -> lỗi: invalid digit found in string
"" -> lỗi: cannot parse integer from empty string
" 5" -> lỗi: invalid digit found in string
```

Nhận xét: `"-7"` lỗi vì `u32` không âm, còn `" 5"` lỗi vì có khoảng trắng đầu. Đó là lý do ta luôn `trim()` trước khi `parse()`.

Đây chính là kỹ thuật trong trò chơi đoán số của bạn: nhánh `Err` báo lỗi rồi `continue` để hỏi lại, thay vì làm chương trình sập.

```rust
fn main() {
    for text in ["50", "abc", " 7 "] {
        let guess: u32 = match text.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{text:?} không phải số, thử lại");
                continue;
            }
        };
        println!("Bạn đoán: {guess}");
    }
}
```

**Kết quả:**

```text
Bạn đoán: 50
"abc" không phải số, thử lại
Bạn đoán: 7
```

`Err(_)` dùng `_` vì nhánh này không cần xem nội dung lỗi.

### 4.3 Tự viết hàm trả về Result

Hàm của bạn cũng có thể trả `Result` để báo "tôi có thể thất bại". Trả `Ok(giá_trị)` khi thành công và `Err(lỗi)` khi thất bại.

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("không thể chia cho 0"))
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("{:?}", divide(10.0, 4.0));
    println!("{:?}", divide(10.0, 0.0));

    match divide(7.0, 2.0) {
        Ok(value) => println!("7 / 2 = {value}"),
        Err(message) => println!("Lỗi: {message}"),
    }
}
```

**Kết quả:**

```text
Ok(2.5)
Err("không thể chia cho 0")
7 / 2 = 3.5
```

Kiểu `Result<f64, String>` đọc là: thành công thì có một `f64`, thất bại thì có một `String` giải thích lý do. Dùng `String` làm kiểu lỗi là cách đơn giản để bắt đầu. [Mục 6](#6-kiểu-lỗi-của-riêng-bạn) sẽ nâng cấp.

### 4.4 if let

Khi chỉ quan tâm **một** nhánh, `if let` gọn hơn `match`:

```rust
fn main() {
    let answer = "42".parse::<u32>();
    if let Ok(n) = answer {
        println!("Đọc được số {n}");
    }

    if let Err(e) = "abc".parse::<u32>() {
        println!("Không đọc được: {e}");
    }
}
```

**Kết quả:**

```text
Đọc được số 42
Không đọc được: invalid digit found in string
```

### 4.5 Các phương thức tiện lợi

`Result` có nhiều phương thức giúp xử lý nhanh những tình huống thường gặp, đỡ phải viết `match`:

| Phương thức | Khi `Ok(v)` | Khi `Err(e)` |
|---|---|---|
| `unwrap()` | Trả `v` | **Panic** |
| `expect("lời nhắn")` | Trả `v` | **Panic** kèm lời nhắn của bạn |
| `unwrap_or(x)` | Trả `v` | Trả `x` |
| `unwrap_or_else(f)` | Trả `v` | Chạy `f` rồi trả kết quả của nó |
| `unwrap_or_default()` | Trả `v` | Trả giá trị mặc định của kiểu (`0`, chuỗi rỗng...) |
| `is_ok()`, `is_err()` | `true` / `false` | `false` / `true` |
| `ok()` | `Some(v)` | `None` (bỏ lý do lỗi) |
| `map(f)` | `Ok(f(v))` | Giữ nguyên `Err(e)` |
| `map_err(f)` | Giữ nguyên `Ok(v)` | `Err(f(e))` |

```rust
fn main() {
    let good: Result<u32, &str> = Ok(5);
    let bad: Result<u32, &str> = Err("hỏng");

    println!("{} {}", good.is_ok(), bad.is_err());

    println!("{}", good.unwrap());
    println!("{}", good.expect("phải có số"));

    println!("{}", bad.unwrap_or(0));
    println!("{}", bad.unwrap_or_else(|message| {
        println!("(xử lý lỗi: {message})");
        100
    }));
    println!("{}", bad.unwrap_or_default());

    println!("{:?}", good.ok());
    println!("{:?}", bad.ok());
    println!("{:?}", good.map(|n| n * 2));
    println!("{:?}", bad.map_err(|message| format!("Lỗi: {message}")));
}
```

**Kết quả:**

```text
true true
5
5
0
(xử lý lỗi: hỏng)
100
0
Some(5)
None
Ok(10)
Err("Lỗi: hỏng")
```

Một số lưu ý:

- `|message| { ... }` là *closure*, một hàm nhỏ không tên, sẽ học ở bài sau. Tạm hiểu là "khi gặp lỗi thì chạy đoạn này". Nó chỉ chạy khi thật sự là `Err`, khác với `unwrap_or(x)` luôn tính sẵn `x`.
- Ví dụ trên dùng lại `good` và `bad` nhiều lần được vì `u32` và `&str` đều là kiểu sao chép được (`Copy`). Với `Result<u32, String>`, mỗi lần `unwrap()` sẽ lấy đi quyền sở hữu của giá trị (xem bài ownership).
- `unwrap` và `expect` **là panic trá hình**. Dùng khi đang thử nhanh, trong bài tập, hoặc khi bạn **chắc chắn** không thể lỗi. Nếu dùng `expect`, hãy ghi điều bạn kỳ vọng (xem [mục 8.1](#81-chọn-panic-hay-result)).

---

## 5. Lan truyền lỗi với dấu hỏi chấm

### 5.1 Vấn đề: match lồng nhau

Khi hàm A gọi hàm B có thể lỗi, thường A không biết xử lý thế nào và muốn **chuyển lỗi lên cho nơi gọi A**. Viết bằng `match` rất dài:

```rust
use std::num::ParseIntError;

fn double_verbose(text: &str) -> Result<u32, ParseIntError> {
    let n = match text.trim().parse::<u32>() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    Ok(n * 2)
}

fn double(text: &str) -> Result<u32, ParseIntError> {
    let n = text.trim().parse::<u32>()?;
    Ok(n * 2)
}

fn main() {
    println!("{:?} {:?}", double_verbose(" 21 "), double(" 21 "));
    println!("{:?} {:?}", double_verbose("abc"), double("abc"));
}
```

**Kết quả:**

```text
Ok(42) Ok(42)
Err(ParseIntError { kind: InvalidDigit }) Err(ParseIntError { kind: InvalidDigit })
```

Hai hàm cho cùng kết quả. Hàm `double` dùng dấu `?` và ngắn hơn hẳn.

### 5.2 Toán tử dấu hỏi chấm

Đặt `?` ngay sau một biểu thức trả về `Result`:

- Nếu là `Ok(v)`: biểu thức nhận giá trị `v`, chương trình chạy tiếp.
- Nếu là `Err(e)`: hàm **trả về ngay** `Err(e)` cho nơi gọi.

Về bản chất, dòng `let n = text.trim().parse::<u32>()?;` gần như tương đương với:

```text
let n = match text.trim().parse::<u32>() {
    Ok(n) => n,
    Err(e) => return Err(From::from(e)),
};
```

Lời gọi `From::from(e)` là chỗ `?` **tự đổi kiểu lỗi** khi cần. Phần này sẽ rõ ở [mục 6.3](#63-from-và-dấu-hỏi-chấm).

Có thể nối nhiều `?` liên tiếp. Chương trình dừng ở lỗi đầu tiên gặp phải:

```text
let a: i32 = a.trim().parse()?;   // lỗi ở đây thì hàm trả về ngay
let b: i32 = b.trim().parse()?;   // chỉ chạy nếu dòng trên thành công
```

### 5.3 Chỉ dùng được trong hàm trả về Result hoặc Option

Vì `?` có thể `return Err(...)`, hàm chứa nó **phải** có kiểu trả về tương thích. Hàm `main` mặc định trả `()` nên không dùng `?` được:

```rust
fn main() {
    let n: u32 = "42".parse()?; // ❌ main trả về ()
    println!("{n}");
}
```

**Trình biên dịch báo:**

```text
error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
 --> src/main.rs:2:30
  |
1 | fn main() {
  | --------- this function should return `Result` or `Option` to accept `?`
2 |     let n: u32 = "42".parse()?; // ❌ main trả về ()
  |                              ^ cannot use the `?` operator in a function that returns `()`
  |
help: consider adding return type
  |
1 ~ fn main() -> Result<(), Box<dyn std::error::Error>> {
2 |     let n: u32 = "42".parse()?; // ❌ main trả về ()
3 |     println!("{n}");
4 +     Ok(())
  |
```

Thông báo đã gợi ý cách sửa: cho `main` trả về `Result`. Xem [mục 5.5](#55-main-trả-về-result).

### 5.4 Dấu hỏi chấm với Option

`?` cũng dùng được trên `Option` trong hàm trả về `Option`: gặp `None` thì hàm trả `None` ngay.

```rust
fn first_letter_upper(text: &str) -> Option<char> {
    let first = text.chars().next()?;
    Some(first.to_ascii_uppercase())
}

fn main() {
    println!("{:?}", first_letter_upper("rust"));
    println!("{:?}", first_letter_upper(""));
}
```

**Kết quả:**

```text
Some('R')
None
```

`text.chars().next()` lấy ký tự đầu tiên của chuỗi và trả `Option<char>`: chuỗi rỗng thì là `None`.

Không trộn lẫn: `?` trên `Option` chỉ dùng trong hàm trả `Option`, `?` trên `Result` chỉ dùng trong hàm trả `Result` (xem [mục 9.2](#92-lỗi-biên-dịch)).

### 5.5 main trả về Result

`main` được phép trả về `Result<(), E>`. Nhờ vậy bạn dùng `?` ngay trong `main`. Nhớ kết thúc bằng `Ok(())`:

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let n: u32 = "42".parse()?;
    println!("n = {n}");
    Ok(())
}
```

**Kết quả:**

```text
n = 42
```

Nếu có lỗi, `main` trả `Err` ra ngoài thì chương trình in lỗi (dạng `{:?}`) rồi thoát với **mã 1**:

```rs
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let n: u32 = "42".parse()?;
    println!("n = {n}");

    let m: u32 = "abc".parse()?; // 💥 trả Err ra khỏi main
    println!("m = {m}");

    Ok(())
}
```

**Khi chạy (mã thoát 1):**

```text
n = 42
Error: ParseIntError { kind: InvalidDigit }
```

Khác với panic, ở đây **không có** thông báo `thread 'main' panicked`, không có backtrace, và mã thoát là 1 chứ không phải 101.

---

## 6. Kiểu lỗi của riêng bạn

### 6.1 Vì sao cần

`Result<T, E>` chỉ có **một** kiểu lỗi `E`. Khi một hàm có nhiều nguồn lỗi khác nhau (đổi chuỗi thành số, chia cho 0, vượt giới hạn...), ta cần một kiểu lỗi chung. Thử dùng `String` làm kiểu lỗi mà vẫn dùng `?` trên `parse`:

```rust
fn parse_quantity(text: &str) -> Result<u32, String> {
    let n = text.trim().parse::<u32>()?; // ❌ ParseIntError không tự thành String
    Ok(n)
}

fn main() {
    println!("{:?}", parse_quantity("5"));
}
```

**Trình biên dịch báo:**

```text
error[E0277]: `?` couldn't convert the error to `String`
 --> src/main.rs:2:39
  |
1 | fn parse_quantity(text: &str) -> Result<u32, String> {
  |                                  ------------------- expected `String` because of this
2 |     let n = text.trim().parse::<u32>()?; // ❌ ParseIntError không tự thành String
  |                         --------------^ the trait `From<ParseIntError>` is not implemented for `String`
  |                         |
  |                         this can't be annotated with `?` because it has type `Result<_, ParseIntError>`
  |
  = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
```

Dòng `note` cho biết điều cốt lõi: `?` **tự đổi kiểu lỗi bằng `From`**. Compiler không biết cách đổi `ParseIntError` thành `String`. Có ba hướng giải quyết:

1. Đổi lỗi tại chỗ bằng `map_err` (dùng ở [Lab 1](#lab-1-kiểm-tra-số-lượng-nhập-vào)).
2. Tự định nghĩa một **enum lỗi** và dạy compiler cách đổi (mục 6.2 và 6.3).
3. Dùng `Box<dyn Error>` để chứa mọi loại lỗi (mục 6.4).

### 6.2 Enum lỗi và Display

Mỗi loại lỗi là một biến thể (variant) của enum. Một enum lỗi chuẩn cần ba thứ:

- `#[derive(Debug)]`: để in được bằng `{:?}`.
- `impl fmt::Display`: để in thân thiện bằng `{}`.
- `impl std::error::Error`: để nó được công nhận là một kiểu lỗi của Rust (cần cho `Box<dyn Error>`, mục 6.4).

```rust
use std::fmt;

#[derive(Debug)]
enum DivideError {
    DivideByZero,
    Overflow,
}

impl fmt::Display for DivideError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DivideError::DivideByZero => write!(f, "không thể chia cho 0"),
            DivideError::Overflow => write!(f, "kết quả vượt quá phạm vi"),
        }
    }
}

impl std::error::Error for DivideError {}

fn divide(a: i32, b: i32) -> Result<i32, DivideError> {
    if b == 0 {
        return Err(DivideError::DivideByZero);
    }
    a.checked_div(b).ok_or(DivideError::Overflow)
}

fn main() {
    println!("{:?}", divide(20, 4));
    println!("{:?}", divide(20, 0));
    println!("{:?}", divide(i32::MIN, -1));

    if let Err(e) = divide(20, 0) {
        println!("Lỗi: {e}");
    }
}
```

**Kết quả:**

```text
Ok(5)
Err(DivideByZero)
Err(Overflow)
Lỗi: không thể chia cho 0
```

Cách đọc các `impl` (bài sau sẽ giải thích đầy đủ, tạm chép theo mẫu):

- `impl fmt::Display for DivideError { ... }` nghĩa là "dạy kiểu `DivideError` cách tự in bằng `{}`". Bên trong, `write!(f, "...")` giống `println!` nhưng ghi vào `f`.
- `impl std::error::Error for DivideError {}` để trống vì chỉ cần "đóng dấu" công nhận.
- `i32::MIN / -1` tràn số, nên `checked_div` trả `None`, và `ok_or(...)` đổi `None` thành `Err(...)`.

### 6.3 From và dấu hỏi chấm

Để `?` đổi được lỗi của thư viện chuẩn thành lỗi của bạn, hãy chứa nó trong một biến thể rồi viết `impl From<...>`:

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum CalcError {
    Parse(ParseIntError),
    DivideByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::Parse(e) => write!(f, "không phải số nguyên hợp lệ ({e})"),
            CalcError::DivideByZero => write!(f, "không thể chia cho 0"),
        }
    }
}

impl std::error::Error for CalcError {}

impl From<ParseIntError> for CalcError {
    fn from(e: ParseIntError) -> Self {
        CalcError::Parse(e)
    }
}

fn divide(a: &str, b: &str) -> Result<i32, CalcError> {
    let a: i32 = a.trim().parse()?;
    let b: i32 = b.trim().parse()?;

    if b == 0 {
        return Err(CalcError::DivideByZero);
    }
    Ok(a / b)
}

fn main() {
    for (a, b) in [("20", "4"), ("20", "0"), ("hai", "4")] {
        match divide(a, b) {
            Ok(value) => println!("{a} / {b} = {value}"),
            Err(e) => println!("{a} / {b} -> lỗi: {e}"),
        }
    }

    println!("{:?}", divide("1", "0"));
}
```

**Kết quả:**

```text
20 / 4 = 5
20 / 0 -> lỗi: không thể chia cho 0
hai / 4 -> lỗi: không phải số nguyên hợp lệ (invalid digit found in string)
Err(DivideByZero)
```

Khi gặp `parse()?` với `parse` trả `Result<i32, ParseIntError>` còn hàm trả `Result<i32, CalcError>`, `?` làm như sau:

| Bước | Việc xảy ra |
|---|---|
| 1 | `parse` trả `Err(e)` với `e: ParseIntError` |
| 2 | `?` thấy kiểu lỗi của hàm là `CalcError`, nên tìm `impl From<ParseIntError> for CalcError` |
| 3 | Gọi `CalcError::from(e)`, tức `CalcError::Parse(e)` |
| 4 | Trả `Err(CalcError::Parse(e))` cho nơi gọi |

Mỗi nguồn lỗi mới thêm một biến thể và một `impl From`, `?` sẽ tự lo phần còn lại.

### 6.4 Box dyn Error

Nếu không cần phân biệt từng loại lỗi, `Box<dyn Error>` là lựa chọn ngắn gọn nhất: nó là "chiếc hộp" chứa được **bất kỳ** kiểu lỗi nào. (`Box` và `dyn` sẽ học kỹ ở bài sau, tạm hiểu như một cái hộp đa năng.)

```rust
use std::error::Error;

fn divide(a: &str, b: &str) -> Result<i32, Box<dyn Error>> {
    let a: i32 = a.trim().parse()?;
    let b: i32 = b.trim().parse()?;

    if b == 0 {
        return Err("không thể chia cho 0".into());
    }
    Ok(a / b)
}

fn main() {
    for (a, b) in [("20", "4"), ("20", "0"), ("hai", "4")] {
        match divide(a, b) {
            Ok(value) => println!("{a} / {b} = {value}"),
            Err(e) => println!("{a} / {b} -> lỗi: {e}"),
        }
    }
}
```

**Kết quả:**

```text
20 / 4 = 5
20 / 0 -> lỗi: không thể chia cho 0
hai / 4 -> lỗi: invalid digit found in string
```

- `?` tự đổi mọi kiểu lỗi chuẩn thành `Box<dyn Error>`, không cần viết `impl From`.
- `"...".into()` đổi một chuỗi thành `Box<dyn Error>` để trả lỗi tự viết nhanh.

Kiểu này cũng dùng được cho `main`:

```rs
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let a: i32 = "20".parse()?;
    let b: i32 = "0".parse()?;

    if b == 0 {
        return Err("không thể chia cho 0".into()); // 💥
    }
    println!("{}", a / b);
    Ok(())
}
```

**Khi chạy (mã thoát 1):**

```text
Error: "không thể chia cho 0"
```

Lỗi được in bằng `{:?}` nên chuỗi nằm trong dấu nháy kép.

### 6.5 Chọn cách nào

| Tình huống | Chọn |
|---|---|
| Chỉ một nguồn lỗi, đổi ngay tại chỗ | `map_err` |
| Thử nghiệm, bài tập, ứng dụng nhỏ, hàm `main` | `Box<dyn Error>` (hoặc `String`) |
| Mã mà người khác gọi và cần `match` theo từng loại lỗi (thư viện) | Enum lỗi riêng với `impl From` |

Trong dự án thật, hai thư viện `thiserror` (viết enum lỗi gọn hơn) và `anyhow` (kiểu lỗi tiện dụng cho ứng dụng) rất phổ biến. Bài này chưa dùng chúng.

---

## 7. Thực hành

Hai bài thực hành có hướng dẫn. Thực hiện trong `error_handling/src/main.rs` và chạy bằng `cargo run`. Để chạy được ngay mà không cần gõ từ bàn phím, các bài dùng danh sách chuỗi "giả lập" dữ liệu người dùng nhập.

### Lab 1. Kiểm tra số lượng nhập vào

**Mục tiêu:** đi từ `expect` (sập khi gặp dữ liệu sai) tới `Result` (xử lý lịch sự), rồi gom logic kiểm tra vào một hàm dùng `?`.

**Bước 1.** Chép và chạy chương trình dùng `expect`:

```rs
fn main() {
    let inputs = ["5", " 12 ", "abc", "-3"]; // giả lập 4 lần người dùng nhập

    for text in inputs {
        let quantity: u32 = text.trim().parse().expect("Số lượng phải là số nguyên không âm"); // 💥
        println!("Đặt {quantity} cái");
    }
}
```

**Khi chạy (mã thoát 101):**

```text
Đặt 5 cái
Đặt 12 cái

thread 'main' (12345) panicked at src/main.rs:5:49:
Số lượng phải là số nguyên không âm: ParseIntError { kind: InvalidDigit }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Chương trình sập ở `"abc"`, còn `"-3"` chưa kịp xử lý. Một dữ liệu sai làm hỏng cả chương trình là điều ta không muốn với **dữ liệu do người dùng nhập**.

**Bước 2.** Dùng `match` để xử lý từng giá trị mà không dừng:

```rust
fn main() {
    let inputs = ["5", " 12 ", "abc", "-3", "", "0"];

    for text in inputs {
        match text.trim().parse::<u32>() {
            Ok(quantity) => println!("{text:?} -> đặt {quantity} cái"),
            Err(e) => println!("{text:?} -> bỏ qua ({e})"),
        }
    }
}
```

**Kết quả:**

```text
"5" -> đặt 5 cái
" 12 " -> đặt 12 cái
"abc" -> bỏ qua (invalid digit found in string)
"-3" -> bỏ qua (invalid digit found in string)
"" -> bỏ qua (cannot parse integer from empty string)
"0" -> đặt 0 cái
```

Chương trình không sập nữa. Nhưng `"0"` vẫn được nhận: `0` là số hợp lệ, song **không hợp lệ** làm số lượng đặt hàng. Quy tắc nghiệp vụ như vậy cần thêm phần kiểm tra.

**Bước 3.** Gom việc kiểm tra vào hàm `parse_quantity` trả `Result<u32, String>`. Dùng `map_err` để đổi lỗi của `parse` thành thông điệp tiếng Việt, và `?` để chuyển lỗi lên:

```rust
const MAX_QUANTITY: u32 = 100;

fn parse_quantity(text: &str) -> Result<u32, String> {
    let quantity = text
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("không phải số nguyên không âm ({e})"))?;

    if quantity == 0 {
        return Err(String::from("số lượng phải lớn hơn 0"));
    }
    if quantity > MAX_QUANTITY {
        return Err(format!("tối đa {MAX_QUANTITY}, bạn nhập {quantity}"));
    }
    Ok(quantity)
}

fn main() {
    let inputs = ["5", " 12 ", "abc", "-3", "", "0", "500"];

    for text in inputs {
        match parse_quantity(text) {
            Ok(quantity) => println!("{text:?} -> đặt {quantity} cái"),
            Err(message) => println!("{text:?} -> lỗi: {message}"),
        }
    }
}
```

**Kết quả:**

```text
"5" -> đặt 5 cái
" 12 " -> đặt 12 cái
"abc" -> lỗi: không phải số nguyên không âm (invalid digit found in string)
"-3" -> lỗi: không phải số nguyên không âm (invalid digit found in string)
"" -> lỗi: không phải số nguyên không âm (cannot parse integer from empty string)
"0" -> lỗi: số lượng phải lớn hơn 0
"500" -> lỗi: tối đa 100, bạn nhập 500
```

**Bước 4.** Đọc dữ liệu thật từ bàn phím. Hàm `parse_quantity` giữ nguyên, chỉ `main` thay đổi:

```rs
use std::io;

const MAX_QUANTITY: u32 = 100;

fn parse_quantity(text: &str) -> Result<u32, String> {
    let quantity = text
        .trim()
        .parse::<u32>()
        .map_err(|e| format!("không phải số nguyên không âm ({e})"))?;

    if quantity == 0 {
        return Err(String::from("số lượng phải lớn hơn 0"));
    }
    if quantity > MAX_QUANTITY {
        return Err(format!("tối đa {MAX_QUANTITY}, bạn nhập {quantity}"));
    }
    Ok(quantity)
}

fn main() {
    println!("Nhập số lượng (1-{MAX_QUANTITY}):");

    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Không đọc được dòng nhập");

    match parse_quantity(&text) {
        Ok(quantity) => println!("Đặt {quantity} cái"),
        Err(message) => println!("Lỗi: {message}"),
    }
}
```

**Khi chạy, nhập `7`:**

```text
Nhập số lượng (1-100):
Đặt 7 cái
```

Với các lần nhập khác nhau:

| Bạn nhập | Chương trình in |
|---|---|
| `abc` | `Lỗi: không phải số nguyên không âm (invalid digit found in string)` |
| `500` | `Lỗi: tối đa 100, bạn nhập 500` |
| (chỉ nhấn Enter) | `Lỗi: không phải số nguyên không âm (cannot parse integer from empty string)` |

**Phân tích:**

- `read_line` trả `Result` (ở đây ta dùng `expect` vì việc đọc bàn phím hầu như không thất bại). Còn việc **người dùng gõ gì** thì ta xử lý bằng `parse_quantity`, hàm trả `Result`.
- `parse_quantity` là hàm thuần túy: nhận chuỗi, trả kết quả. Không `println!` bên trong, nên dễ dùng lại và dễ kiểm thử.
- `map_err(|e| ...)` đổi kiểu lỗi `ParseIntError` thành `String` ngay tại chỗ để `?` khớp với kiểu trả về (xem lại lỗi `E0277` ở [mục 6.1](#61-vì-sao-cần)).

**Thử thêm:** viết vòng `loop` hỏi lại cho đến khi nhập hợp lệ, giống trò chơi đoán số.

### Lab 2. Máy tính có kiểm tra lỗi

**Mục tiêu:** viết một máy tính nhỏ đọc biểu thức dạng `"20 / 4"`, dùng enum lỗi riêng, `From`, `?` và `checked_*`.

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum CalcError {
    WrongFormat,
    UnknownOperator(String),
    Parse(ParseIntError),
    DivideByZero,
    Overflow,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::WrongFormat => write!(f, "cần dạng: <số> <phép tính> <số>"),
            CalcError::UnknownOperator(op) => write!(f, "không hỗ trợ phép tính '{op}'"),
            CalcError::Parse(e) => write!(f, "không phải số nguyên hợp lệ ({e})"),
            CalcError::DivideByZero => write!(f, "không thể chia cho 0"),
            CalcError::Overflow => write!(f, "kết quả vượt quá phạm vi của i64"),
        }
    }
}

impl std::error::Error for CalcError {}

impl From<ParseIntError> for CalcError {
    fn from(e: ParseIntError) -> Self {
        CalcError::Parse(e)
    }
}

fn calculate(line: &str) -> Result<i64, CalcError> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(CalcError::WrongFormat);
    }

    let a: i64 = parts[0].parse()?;
    let b: i64 = parts[2].parse()?;

    match parts[1] {
        "+" => a.checked_add(b).ok_or(CalcError::Overflow),
        "-" => a.checked_sub(b).ok_or(CalcError::Overflow),
        "*" => a.checked_mul(b).ok_or(CalcError::Overflow),
        "/" => {
            if b == 0 {
                Err(CalcError::DivideByZero)
            } else {
                a.checked_div(b).ok_or(CalcError::Overflow)
            }
        }
        other => Err(CalcError::UnknownOperator(other.to_string())),
    }
}

fn main() {
    let lines = [
        "20 / 4",
        "7 * 6",
        "9 - 12",
        "5 / 0",
        "5 +",
        "8 % 3",
        "two + 2",
        "9223372036854775807 + 1",
    ];

    for line in lines {
        match calculate(line) {
            Ok(value) => println!("{line} = {value}"),
            Err(e) => println!("{line} -> lỗi: {e}"),
        }
    }
}
```

**Kết quả:**

```text
20 / 4 = 5
7 * 6 = 42
9 - 12 = -3
5 / 0 -> lỗi: không thể chia cho 0
5 + -> lỗi: cần dạng: <số> <phép tính> <số>
8 % 3 -> lỗi: không hỗ trợ phép tính '%'
two + 2 -> lỗi: không phải số nguyên hợp lệ (invalid digit found in string)
9223372036854775807 + 1 -> lỗi: kết quả vượt quá phạm vi của i64
```

**Phân tích:**

- Mỗi dòng trong `lines` có thể sai theo một cách riêng, và mỗi cách sai ứng với **một biến thể** của `CalcError`. Nơi gọi có thể `match` để xử lý từng loại.
- `line.split_whitespace().collect()` tách dòng thành các từ rồi gom vào `Vec<&str>`.
- Hai dòng `parts[0].parse()?` và `parts[2].parse()?` là chỗ `?` gọi `From<ParseIntError>` để đổi lỗi thành `CalcError::Parse(...)`.
- `checked_add`, `checked_sub`, `checked_mul`, `checked_div` trả `Option` (`None` khi tràn số), và `ok_or(CalcError::Overflow)` đổi `None` thành lỗi `Overflow`. Nhờ vậy phép tính không bao giờ panic vì tràn số.
- Dòng cuối chứng minh điều đó: `9223372036854775807 + 1` (giá trị `i64::MAX` cộng 1) trả lỗi thay vì panic.

**Thử thêm:** thêm phép lấy dư `%`. Gợi ý: dùng `checked_rem`, và nhớ chặn trường hợp `b == 0`.

---

## 8. Tìm hiểu chi tiết

### 8.1 Chọn panic hay Result

| Tình huống | Nên dùng | Lý do |
|---|---|---|
| Dữ liệu từ bên ngoài: người dùng nhập, tệp, mạng | `Result` | Sai là chuyện bình thường, nơi gọi cần được quyết định |
| Lỗi có cách khắc phục: thử lại, dùng giá trị mặc định, hỏi lại | `Result` | Panic làm mất cơ hội khắc phục |
| Điều lập trình viên đã đảm bảo lại bị phá vỡ (một bug) | `panic!` | Tiếp tục chạy với trạng thái sai còn nguy hiểm hơn là dừng |
| Người gọi vi phạm điều kiện ghi trong tài liệu của hàm | `panic!`, `assert!` | Đây là lỗi của phía gọi, không phải tình huống để "xử lý" |
| Ví dụ, bài tập, prototype, kiểm thử | `unwrap`, `expect` | Gọn, và lỗi lộ ra ngay |
| Bạn **chắc chắn** không thể lỗi nhưng compiler không biết | `expect("lý do")` | Ghi lại lý do vì sao bạn chắc chắn |

Ví dụ: chuỗi `"127.0.0.1"` được viết cứng trong code và chắc chắn là địa chỉ IP hợp lệ, nên `expect` là hợp lý. Lời nhắn nên nói **điều bạn kỳ vọng**:

```rust
use std::net::IpAddr;

fn main() {
    let home: IpAddr = "127.0.0.1".parse().expect("127.0.0.1 là địa chỉ IP hợp lệ");
    println!("{home}");
}
```

**Kết quả:**

```text
127.0.0.1
```

> **Quy tắc ngón tay cái:** nếu bạn định viết `.unwrap()` trên dữ liệu **do người dùng đưa vào**, hãy dừng lại và dùng `match` hoặc `?`.

### 8.2 Option hay Result

Rất nhiều thao tác trả `Option` để báo "không có". Ví dụ `get(i)` trên mảng, an toàn hơn `v[i]` (sẽ panic nếu ngoài phạm vi). Khi muốn đổi `Option` thành `Result` có lý do, dùng `ok_or`:

```rust
fn main() {
    let scores = [8, 7, 9];

    println!("{:?}", scores.get(2));
    println!("{:?}", scores.get(5));
    println!("{:?}", scores.get(2).ok_or("không có phần tử này"));
    println!("{:?}", scores.get(5).ok_or("không có phần tử này"));
}
```

**Kết quả:**

```text
Some(9)
None
Ok(9)
Err("không có phần tử này")
```

| Muốn... | Dùng |
|---|---|
| Truy cập phần tử mà chỉ số có thể ngoài phạm vi | `get(i)` thay vì `[i]` |
| Đổi `Option` thành `Result` | `ok_or(lỗi)` |
| Đổi `Result` thành `Option` (bỏ lý do lỗi) | `ok()` |

### 8.3 Display và Debug của lỗi

Mỗi giá trị lỗi thường in được theo hai cách:

```rust
fn main() {
    let e = "abc".parse::<u32>().unwrap_err();

    println!("{e}");
    println!("{e:?}");
}
```

**Kết quả:**

```text
invalid digit found in string
ParseIntError { kind: InvalidDigit }
```

| Cú pháp | Dành cho | Đặc điểm |
|---|---|---|
| `{}` (`Display`) | Người dùng cuối | Một câu dễ đọc |
| `{:?}` (`Debug`) | Lập trình viên | Cấu trúc chi tiết, dùng khi gỡ lỗi |

Hàm `main` trả `Err` in lỗi bằng dạng `{:?}`, nên ta thấy `Error: ParseIntError { kind: InvalidDigit }` ở [mục 5.5](#55-main-trả-về-result).

### 8.4 Các macro gây panic khác

Ngoài `panic!`, Rust có sẵn vài macro khác. Chúng đều dừng chương trình, khác nhau ở ý nghĩa và thông báo:

| Macro | Dùng để | Thông báo |
|---|---|---|
| `assert!(điều_kiện, "lời nhắn")` | Khẳng định điều kiện đúng | Lời nhắn của bạn |
| `assert_eq!(a, b)` | Khẳng định hai giá trị bằng nhau | ``assertion `left == right` failed`` kèm `left: ...` và `right: ...` |
| `assert_ne!(a, b)` | Khẳng định hai giá trị khác nhau | ``assertion `left != right` failed`` kèm `left: ...` và `right: ...` |
| `unreachable!("...")` | Đánh dấu chỗ không thể chạy tới | `internal error: entered unreachable code: ...` |
| `todo!()` | Chỗ còn phải viết sau | `not yet implemented` |
| `unimplemented!()` | Chức năng cố ý chưa làm | `not implemented` |

`todo!()` còn có ích khi phác thảo: nó hợp với **mọi kiểu trả về**, nên hàm chưa viết xong vẫn biên dịch được.

### 8.5 Mã thoát của chương trình

Khi kết thúc, chương trình trả cho hệ điều hành một **mã thoát** (*exit code*). Mã `0` nghĩa là thành công, mã khác `0` nghĩa là có lỗi:

| Cách kết thúc | Mã thoát |
|---|---|
| Chạy xong bình thường (`main` trả `()` hoặc `Ok(())`) | `0` |
| `main` trả `Err(...)` | `1` |
| Panic | `101` |

Xem mã thoát bằng cách gõ `echo $?` ngay sau khi chương trình chạy xong. Lệnh `cargo run` cũng trả về đúng mã này. Các công cụ tự động hoá (script, CI) dựa vào mã thoát để biết chương trình thành công hay thất bại.

### 8.6 Gom nhiều Result

Đây là phần xem trước, dùng iterator và closure sẽ học ở bài sau. Khi có một danh sách các `Result`, ta có thể gom chúng thành **một** `Result` của cả danh sách. Gặp lỗi đầu tiên thì dừng và trả lỗi đó:

```rust
fn main() {
    let good = ["1", "2", "3"];
    let bad = ["1", "x", "3"];

    let a: Result<Vec<u32>, _> = good.iter().map(|s| s.parse::<u32>()).collect();
    let b: Result<Vec<u32>, _> = bad.iter().map(|s| s.parse::<u32>()).collect();

    println!("{:?}", a);
    println!("{:?}", b);
}
```

**Kết quả:**

```text
Ok([1, 2, 3])
Err(ParseIntError { kind: InvalidDigit })
```

Khi cần "tất cả đều hợp lệ mới tiếp tục", đây là cách rất gọn.

---

## 9. Lỗi thường gặp

### 9.1 Hai thời điểm xuất hiện lỗi

| | Lỗi biên dịch | Lỗi lúc chạy |
|---|---|---|
| Xuất hiện khi | `cargo build`, `cargo check`, `cargo run` | Chương trình đang chạy |
| Dấu hiệu | `error[E0...]` | `thread 'main' panicked at ...` hoặc `Error: ...` |
| Chương trình có chạy không | Không (chưa tạo được) | Có, rồi dừng giữa chừng (mã 101 hoặc 1) |
| Cách xử lý | Đọc dòng `help:` | Đọc thông điệp và vị trí, bật `RUST_BACKTRACE=1` nếu cần ([mục 3.3](#33-đọc-thông-báo-panic-và-backtrace)) |

### 9.2 Lỗi biên dịch

| Mã lỗi | Thông báo | Nguyên nhân | Cách sửa |
|---|---|---|---|
| `E0308` | `mismatched types` | Gán `Result` vào biến kiểu giá trị; hàm trả `Result` mà thiếu `Ok(...)` ở cuối | Mở `Result` bằng `match`, `?`, `unwrap`, `expect`; thêm `Ok(())` |
| `E0277` | `the ? operator can only be used in a function that returns Result or Option` | Dùng `?` trong hàm trả `()` (kể cả `main`) | Cho hàm trả `Result` hoặc `Option` |
| `E0277` | `? couldn't convert the error to String` | Kiểu lỗi của hàm không khớp kiểu lỗi bên trong, chưa có `From` | `map_err`, thêm `impl From`, hoặc dùng `Box<dyn Error>` |
| `E0277` | `the ? operator can only be used on Results, not Options, in a function that returns Result` | Dùng `?` trên `Option` trong hàm trả `Result` | `ok_or(...)?` |
| `E0369` | `cannot add ... to Result<...>` | Làm phép toán trực tiếp trên `Result` | Lấy giá trị ra trước |
| `E0004` | `non-exhaustive patterns: Err(_) not covered` | `match` thiếu nhánh | Thêm nhánh còn thiếu |

**Dùng `?` trên `Option` trong hàm trả `Result`:**

```rust
fn first_char(text: &str) -> Result<char, String> {
    let c = text.chars().next()?; // ❌ ? trên Option trong hàm trả Result
    Ok(c)
}

fn main() {
    println!("{:?}", first_char("rust"));
}
```

**Trình biên dịch báo:**

```text
error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
 --> src/main.rs:2:32
  |
1 | fn first_char(text: &str) -> Result<char, String> {
  | ------------------------------------------------- this function returns a `Result`
2 |     let c = text.chars().next()?; // ❌ ? trên Option trong hàm trả Result
  |                                ^ use `.ok_or(...)?` to provide an error compatible with `Result<char, String>`
```

Cách sửa theo gợi ý: dùng `ok_or` để đổi `None` thành lỗi có lý do.

```rust
fn first_char(text: &str) -> Result<char, String> {
    let c = text.chars().next().ok_or(String::from("chuỗi rỗng"))?;
    Ok(c)
}

fn main() {
    println!("{:?}", first_char("rust"));
    println!("{:?}", first_char(""));
}
```

**Kết quả:**

```text
Ok('r')
Err("chuỗi rỗng")
```

**Làm phép toán trực tiếp trên `Result`:**

```rust
fn main() {
    let total = "5".parse::<u32>() + 1; // ❌ không cộng được với Result
    println!("{total:?}");
}
```

**Trình biên dịch báo:**

```text
error[E0369]: cannot add `{integer}` to `Result<u32, ParseIntError>`
 --> src/main.rs:2:36
  |
2 |     let total = "5".parse::<u32>() + 1; // ❌ không cộng được với Result
  |                 ------------------ ^ - {integer}
  |                 |
  |                 Result<u32, ParseIntError>
  |
...
```

Cách sửa: lấy giá trị ra trước, ví dụ `"5".parse::<u32>().unwrap() + 1` hoặc dùng `match`.

**`match` thiếu nhánh `Err`:**

```rust
fn main() {
    let result = "5".parse::<u32>();

    match result { // ❌ thiếu nhánh Err
        Ok(n) => println!("{n}"),
    }
}
```

**Trình biên dịch báo:**

```text
error[E0004]: non-exhaustive patterns: `Err(_)` not covered
 --> src/main.rs:4:11
  |
4 |     match result { // ❌ thiếu nhánh Err
  |           ^^^^^^ pattern `Err(_)` not covered
  |
...
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
  |
5 ~         Ok(n) => println!("{n}"),
6 ~         Err(_) => todo!(),
  |
```

Rust ép `match` phải bao phủ **mọi** trường hợp. Đây là điểm giúp bạn không bao giờ quên xử lý lỗi.

**Thiếu `Ok(())` ở cuối hàm trả `Result`:**

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let n: u32 = "42".parse()?;
    println!("n = {n}");
} // ❌ thiếu Ok(()) ở cuối
```

**Trình biên dịch báo:**

```text
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 | fn main() -> Result<(), ParseIntError> {
  |    ----      ^^^^^^^^^^^^^^^^^^^^^^^^^ expected `Result<(), ParseIntError>`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
```

Cách sửa: thêm dòng `Ok(())` ngay trước dấu `}` cuối cùng.

### 9.3 Cảnh báo bỏ qua Result

Gọi hàm trả `Result` mà không dùng kết quả thì compiler cảnh báo, vì lỗi có thể đang bị bỏ quên:

```rust
// ⚠️ Biên dịch được nhưng có cảnh báo: bỏ qua một Result
fn save() -> Result<(), String> {
    Err(String::from("ổ đĩa đầy"))
}

fn main() {
    save();
    println!("Xong");
}
```

**Cảnh báo của compiler:**

```text
warning: unused `Result` that must be used
 --> src/main.rs:7:5
  |
7 |     save();
  |     ^^^^^^
  |
  = note: this `Result` may be an `Err` variant, which should be handled
  = note: `#[warn(unused_must_use)]` (part of `#[warn(unused)]`) on by default
help: use `let _ = ...` to ignore the resulting value
  |
7 |     let _ = save();
  |     +++++++
```

**Kết quả:**

```text
Xong
```

Lỗi `ổ đĩa đầy` đã bị bỏ qua mà chương trình vẫn in `Xong`. Hãy xử lý bằng `match`, `?`, hoặc `if let Err(e) = save() { ... }`. Chỉ viết `let _ = save();` khi bạn **cố ý** bỏ qua.

### 9.4 Những thói quen nên tránh

| Thói quen | Hậu quả | Nên làm |
|---|---|---|
| `unwrap()` rải khắp nơi | Gặp dữ liệu sai là sập chương trình | `match`, `?`, hoặc `expect` có lý do |
| `unwrap_or(0)` cho xong chuyện | Lỗi bị giấu, số `0` sai chạy tiếp âm thầm | Chỉ dùng khi giá trị mặc định thật sự hợp lý |
| `panic!` cho dữ liệu người dùng nhập sai | Chương trình sập vì lỗi bình thường | Trả `Result` |
| `expect("lỗi")` mơ hồ | Lời nhắn không giúp tìm ra nguyên nhân | Mô tả điều kỳ vọng: `expect("PORT phải là số")` |
| `let _ = hàm_có_thể_lỗi();` | Mất thông tin lỗi | Ít nhất hãy in hoặc ghi lại lỗi |
| Trả `String` làm kiểu lỗi ở mọi nơi | Không phân biệt được loại lỗi | Enum lỗi khi nơi gọi cần xử lý khác nhau |

---

## 10. Bài tập

Hãy tự làm trước, chạy bằng `cargo run`, rồi mới mở phần đáp án. Mỗi bài có **kết quả mong đợi** để bạn tự kiểm tra.

### Cấp độ 1. Làm quen

#### Bài 1.1. Dự đoán kết quả

**Đề:** đọc đoạn mã sau và **dự đoán** bốn dòng in ra trước khi chạy.

```rust
fn main() {
    for text in ["10", "-2", "3.5", " 4"] {
        match text.parse::<i32>() {
            Ok(n) => println!("{text:?} -> {n}"),
            Err(_) => println!("{text:?} -> lỗi"),
        }
    }
}
```

<details>
<summary>Xem đáp án</summary>

**Kết quả:**

```text
"10" -> 10
"-2" -> -2
"3.5" -> lỗi
" 4" -> lỗi
```

Giải thích: `"-2"` hợp lệ vì `i32` có dấu. `"3.5"` có dấu chấm nên không phải số nguyên. `" 4"` có khoảng trắng đầu nên `parse` từ chối. Muốn nhận `" 4"`, hãy `trim()` trước.

</details>

#### Bài 1.2. Căn bậc hai an toàn

**Đề:** viết hàm `safe_sqrt(x: f64) -> Result<f64, String>`. Nếu `x >= 0` trả `Ok(căn bậc hai)` (dùng `x.sqrt()`). Ngược lại trả `Err` với thông điệp `"không tính được căn bậc hai của số âm"`. Hàm `main` cho sẵn:

```text
fn main() {
    for x in [16.0, -4.0, 2.0] {
        match safe_sqrt(x) {
            Ok(root) => println!("căn bậc hai của {x} là {root:.3}"),
            Err(message) => println!("{x}: {message}"),
        }
    }
}
```

**Kết quả mong đợi:**

```text
căn bậc hai của 16 là 4.000
-4: không tính được căn bậc hai của số âm
căn bậc hai của 2 là 1.414
```

**Gợi ý:** giống hàm `divide` ở [mục 4.3](#43-tự-viết-hàm-trả-về-result), dùng `if ... { Err(...) } else { Ok(...) }`.

<details>
<summary>Xem đáp án</summary>

```rust
fn safe_sqrt(x: f64) -> Result<f64, String> {
    if x < 0.0 {
        Err(String::from("không tính được căn bậc hai của số âm"))
    } else {
        Ok(x.sqrt())
    }
}

fn main() {
    for x in [16.0, -4.0, 2.0] {
        match safe_sqrt(x) {
            Ok(root) => println!("căn bậc hai của {x} là {root:.3}"),
            Err(message) => println!("{x}: {message}"),
        }
    }
}
```

</details>

#### Bài 1.3. Giá trị mặc định

**Đề:** cho `let inputs = ["3", "abc", "", "10"];`. Với mỗi chuỗi, đổi thành `u32`; nếu không đổi được thì dùng số lượng mặc định là `1`. **Không dùng `match`.** In ra từng dòng.

**Kết quả mong đợi:**

```text
Số lượng: 3
Số lượng: 1
Số lượng: 1
Số lượng: 10
```

**Gợi ý:** `unwrap_or(...)`.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    let inputs = ["3", "abc", "", "10"];

    for text in inputs {
        let quantity: u32 = text.trim().parse().unwrap_or(1);
        println!("Số lượng: {quantity}");
    }
}
```

Giải thích: `unwrap_or(1)` trả giá trị trong `Ok`, hoặc `1` nếu là `Err`. Cách này gọn, nhưng **giấu lỗi**: bạn không biết người dùng đã gõ sai. Chỉ dùng khi giá trị mặc định thật sự hợp lý.

</details>

### Cấp độ 2. Vận dụng

#### Bài 2.1. Dùng dấu hỏi chấm

**Đề:** viết hàm `sum_two(a: &str, b: &str) -> Result<i32, ParseIntError>` đổi hai chuỗi thành số (nhớ `trim()`), rồi trả tổng của chúng. Dùng `?`, không dùng `match`. Hàm `main` cho sẵn:

```text
fn main() {
    println!("{:?}", sum_two("20", "22"));
    println!("{:?}", sum_two("20", "x"));
}
```

**Kết quả mong đợi:**

```text
Ok(42)
Err(ParseIntError { kind: InvalidDigit })
```

**Gợi ý:** cần `use std::num::ParseIntError;` ở đầu tệp.

<details>
<summary>Xem đáp án</summary>

```rust
use std::num::ParseIntError;

fn sum_two(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let a: i32 = a.trim().parse()?;
    let b: i32 = b.trim().parse()?;
    Ok(a + b)
}

fn main() {
    println!("{:?}", sum_two("20", "22"));
    println!("{:?}", sum_two("20", "x"));
}
```

Giải thích: khi `a` hoặc `b` không hợp lệ, `?` trả `Err` ngay và dòng `Ok(a + b)` không chạy. Hai biến `a`, `b` kiểu `&str` được che (shadow) bởi hai biến `i32` cùng tên.

</details>

#### Bài 2.2. Dấu hỏi chấm với Option

**Đề:** viết hàm `last_digit(text: &str) -> Option<u32>` trả chữ số ở **cuối** chuỗi, hoặc `None` nếu chuỗi rỗng hay ký tự cuối không phải chữ số. Hàm `main` cho sẵn:

```text
fn main() {
    println!("{:?}", last_digit("room 42"));
    println!("{:?}", last_digit("room"));
    println!("{:?}", last_digit(""));
}
```

**Kết quả mong đợi:**

```text
Some(2)
None
None
```

**Gợi ý:** `text.chars().last()` trả `Option<char>`. Ký tự có phương thức `to_digit(10)` trả `Option<u32>`.

<details>
<summary>Xem đáp án</summary>

```rust
fn last_digit(text: &str) -> Option<u32> {
    text.chars().last()?.to_digit(10)
}

fn main() {
    println!("{:?}", last_digit("room 42"));
    println!("{:?}", last_digit("room"));
    println!("{:?}", last_digit(""));
}
```

Giải thích: `chars().last()?` cho ký tự cuối, hoặc trả `None` ngay nếu chuỗi rỗng. Sau đó `to_digit(10)` trả `None` nếu ký tự đó không phải chữ số (như `'m'`).

</details>

#### Bài 2.3. Sửa lỗi thiếu Ok

**Đề:** đoạn mã sau không biên dịch được. Tìm lỗi và sửa.

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let n: u32 = "42".parse()?;
    println!("n = {n}");
} // ❌
```

**Kết quả mong đợi:**

```text
n = 42
```

**Gợi ý:** `main` hứa trả `Result<(), ParseIntError>`, nhưng thân hàm đang trả gì?

<details>
<summary>Xem đáp án</summary>

```rust
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let n: u32 = "42".parse()?;
    println!("n = {n}");
    Ok(())
}
```

Giải thích: hàm trả `Result` phải kết thúc bằng một `Result`. Khi mọi việc thành công mà không có giá trị gì để trả, ta trả `Ok(())`, với `()` là "không có gì". Lỗi là `E0308 mismatched types`.

</details>

#### Bài 2.4. Kiểm tra tuổi

**Đề:** viết hàm `parse_age(text: &str) -> Result<u8, String>`:

- Đổi chuỗi (đã `trim`) thành `u8`. Nếu thất bại, trả `Err` dạng `"không phải số: <lỗi gốc>"` (dùng `map_err`).
- Nếu tuổi lớn hơn 150, trả `Err` dạng `"tuổi <tuổi> quá lớn (tối đa 150)"`.

Hàm `main` cho sẵn:

```text
fn main() {
    for text in ["25", "abc", "200", "-5", "300"] {
        println!("{text:?} -> {:?}", parse_age(text));
    }
}
```

**Kết quả mong đợi:**

```text
"25" -> Ok(25)
"abc" -> Err("không phải số: invalid digit found in string")
"200" -> Err("tuổi 200 quá lớn (tối đa 150)")
"-5" -> Err("không phải số: invalid digit found in string")
"300" -> Err("không phải số: number too large to fit in target type")
```

**Gợi ý:** `parse::<u8>()`. Kiểu `u8` chỉ chứa `0..=255` nên `"300"` thất bại ngay ở `parse`, còn `"200"` thì qua `parse` rồi mới bị chặn bởi phép kiểm tra `> 150`.

<details>
<summary>Xem đáp án</summary>

```rust
fn parse_age(text: &str) -> Result<u8, String> {
    let age = text
        .trim()
        .parse::<u8>()
        .map_err(|e| format!("không phải số: {e}"))?;

    if age > 150 {
        return Err(format!("tuổi {age} quá lớn (tối đa 150)"));
    }
    Ok(age)
}

fn main() {
    for text in ["25", "abc", "200", "-5", "300"] {
        println!("{text:?} -> {:?}", parse_age(text));
    }
}
```

</details>

### Cấp độ 3. Thử thách

#### Bài 3.1. Enum lỗi cho tuổi

**Đề:** làm lại bài 2.4, nhưng dùng **enum lỗi** thay cho `String`:

```text
enum AgeError {
    NotANumber(ParseIntError),
    TooOld(u8),
}
```

Yêu cầu: `#[derive(Debug)]`; `impl fmt::Display for AgeError`; `impl From<ParseIntError> for AgeError`; hàm `parse_age(text: &str) -> Result<u8, AgeError>` dùng `?` trực tiếp trên `parse` (không dùng `map_err`). Hàm `main` cho sẵn:

```text
fn main() {
    for text in ["25", "abc", "200", "300"] {
        match parse_age(text) {
            Ok(age) => println!("{text:?} -> {age} tuổi"),
            Err(e) => println!("{text:?} -> lỗi: {e}"),
        }
    }
}
```

**Kết quả mong đợi:**

```text
"25" -> 25 tuổi
"abc" -> lỗi: không phải số (invalid digit found in string)
"200" -> lỗi: 200 tuổi là quá lớn
"300" -> lỗi: không phải số (number too large to fit in target type)
```

**Gợi ý:** theo mẫu của `CalcError` ở [mục 6.3](#63-from-và-dấu-hỏi-chấm). Dòng `let age: u8 = text.trim().parse()?;` chỉ biên dịch được khi đã có `impl From`.

<details>
<summary>Xem đáp án</summary>

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AgeError {
    NotANumber(ParseIntError),
    TooOld(u8),
}

impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AgeError::NotANumber(e) => write!(f, "không phải số ({e})"),
            AgeError::TooOld(age) => write!(f, "{age} tuổi là quá lớn"),
        }
    }
}

impl From<ParseIntError> for AgeError {
    fn from(e: ParseIntError) -> Self {
        AgeError::NotANumber(e)
    }
}

fn parse_age(text: &str) -> Result<u8, AgeError> {
    let age: u8 = text.trim().parse()?;
    if age > 150 {
        return Err(AgeError::TooOld(age));
    }
    Ok(age)
}

fn main() {
    for text in ["25", "abc", "200", "300"] {
        match parse_age(text) {
            Ok(age) => println!("{text:?} -> {age} tuổi"),
            Err(e) => println!("{text:?} -> lỗi: {e}"),
        }
    }
}
```

Giải thích: so với bài 2.4, nơi gọi giờ có thể `match` theo `AgeError::NotANumber` hay `AgeError::TooOld` để phản ứng khác nhau, thay vì chỉ nhận một chuỗi thông báo.

</details>

#### Bài 3.2. Bốn đoạn mã, bốn lỗi

**Đề:** mỗi đoạn mã dưới đây đều không biên dịch được. Với từng đoạn, hãy **dự đoán lỗi** trước khi chạy `cargo check`, rồi sửa.

**Đoạn (a):**

```rust
fn main() {
    let n: u32 = "7".parse()?; // ❌
    println!("{n}");
}
```

**Đoạn (b):**

```rust
fn first_char(text: &str) -> Result<char, String> {
    let c = text.chars().next()?; // ❌
    Ok(c)
}

fn main() {
    println!("{:?}", first_char("rust"));
}
```

**Đoạn (c):**

```rust
fn main() {
    let result = "5".parse::<u32>();

    match result {
        Ok(n) => println!("{n}"), // ❌ thiếu một nhánh
    }
}
```

**Đoạn (d):**

```rust
fn main() {
    let total = "5".parse::<u32>() + 1; // ❌
    println!("{total:?}");
}
```

<details>
<summary>Xem đáp án</summary>

| Đoạn | Lỗi | Nguyên nhân | Cách sửa |
|---|---|---|---|
| (a) | `E0277` the `?` operator can only be used in a function that returns `Result` or `Option` | `main` trả `()` nên không dùng được `?` | Đổi thành `fn main() -> Result<(), std::num::ParseIntError>` và thêm `Ok(())` ở cuối; hoặc dùng `expect` |
| (b) | `E0277` the `?` operator can only be used on `Result`s, not `Option`s | Dùng `?` trên `Option` trong hàm trả `Result` | `text.chars().next().ok_or(String::from("chuỗi rỗng"))?` |
| (c) | `E0004` non-exhaustive patterns: `Err(_)` not covered | `match` thiếu nhánh `Err` | Thêm `Err(e) => println!("lỗi: {e}"),` |
| (d) | `E0369` cannot add `{integer}` to `Result<u32, ParseIntError>` | Cộng trực tiếp `Result` với số | Lấy giá trị ra trước: `"5".parse::<u32>().unwrap() + 1`, hoặc dùng `match` |

</details>

#### Bài 3.3. Panic hay Result

**Đề:** với mỗi tình huống, chọn **`Result`** hay **`panic!`** (hoặc `unwrap`/`expect`) và giải thích ngắn gọn.

| # | Tình huống |
|---|---|
| 1 | Người dùng nhập tuổi là `"abc"` |
| 2 | Chương trình đọc tệp cấu hình nhưng tệp không tồn tại |
| 3 | Mảng `week` luôn có 7 phần tử, còn chỉ số `day` do chính chương trình của bạn tính ra, và nó luôn nằm trong `0..=6` |
| 4 | Hàm `average(list)` được ghi rõ trong tài liệu: "danh sách không được rỗng", nhưng một lập trình viên khác truyền vào danh sách rỗng |
| 5 | Gọi một dịch vụ qua mạng và bị quá thời gian chờ |
| 6 | Viết nhanh một đoạn thử nghiệm để thử một ý tưởng, chưa cần chạy thật |

<details>
<summary>Xem đáp án</summary>

| # | Chọn | Lý do |
|---|---|---|
| 1 | `Result` | Dữ liệu từ bên ngoài, sai là bình thường. Cần báo lỗi và cho nhập lại |
| 2 | `Result` | Có thể khắc phục: tạo tệp mặc định, báo lỗi rõ ràng, hoặc thoát có kiểm soát |
| 3 | `panic!` (qua `[]` hoặc `expect`) | Nếu chỉ số ra ngoài `0..=6` thì là bug trong chương trình, không phải tình huống để xử lý |
| 4 | `panic!` hoặc `assert!` | Người gọi vi phạm điều kiện đã ghi trong tài liệu. Đây là lỗi lập trình ở phía gọi |
| 5 | `Result` | Lỗi mạng xảy ra thường xuyên. Nơi gọi cần quyết định thử lại hay báo lỗi |
| 6 | `unwrap` / `expect` | Prototype cần gọn. Khi làm thật, hãy thay bằng xử lý lỗi đúng cách |

</details>

---

## 11. Tóm tắt

| Chủ đề | Ghi nhớ |
|---|---|
| Hai loại lỗi | Khôi phục được → `Result`; không khôi phục được (bug) → `panic!` |
| Rust không có exception | Khả năng lỗi nằm trong **kiểu trả về**, compiler buộc bạn xử lý |
| `Result<T, E>` | `Ok(T)` hoặc `Err(E)`; `Option<T>`: `Some(T)` hoặc `None` |
| `panic!` | In thông báo rồi dừng, mã thoát `101`; bật `RUST_BACKTRACE=1` để xem backtrace |
| Panic tự sinh | Chỉ số ngoài phạm vi, chia cho 0, tràn số (debug), `unwrap` trên `None`/`Err` |
| Mở `Result` | `match`, `if let`, hoặc `unwrap`/`expect`/`unwrap_or`/`unwrap_or_else` |
| `unwrap`, `expect` | Là panic trá hình; dùng cho thử nghiệm hoặc khi chắc chắn; ưu tiên `expect` với lời nhắn rõ |
| `?` | `Ok` → lấy giá trị; `Err` → trả lỗi ngay cho nơi gọi, tự đổi kiểu qua `From` |
| Phạm vi của `?` | Chỉ trong hàm trả `Result` (cho `Result`) hoặc `Option` (cho `Option`), kể cả `main` |
| `main` trả `Result` | `fn main() -> Result<(), E>`, nhớ `Ok(())`; trả `Err` thì thoát với mã `1` |
| Enum lỗi riêng | `#[derive(Debug)]` + `impl Display` + `impl Error` + `impl From<...>` |
| `Box<dyn Error>` | Chứa mọi loại lỗi, tiện cho `main` và ứng dụng nhỏ; mất khả năng `match` theo loại |
| Chọn `Result` hay `panic!` | Dữ liệu ngoài và lỗi có thể khắc phục → `Result`; bug và vi phạm điều kiện → `panic!` |
| Đừng giấu lỗi | Cẩn thận với `unwrap()` bừa, `unwrap_or` che lỗi, `let _ =` bỏ qua `Result` |

---

## 12. Tài liệu tham khảo

- [The Rust Programming Language, 9. Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [The Rust Programming Language, 9.1 Unrecoverable Errors with panic!](https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html)
- [The Rust Programming Language, 9.2 Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [The Rust Programming Language, 9.3 To panic! or Not to panic!](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)
- [Rust By Example, Error handling](https://doc.rust-lang.org/rust-by-example/error.html)
- [Tài liệu thư viện chuẩn, `std::result`](https://doc.rust-lang.org/std/result/index.html)
- [Tài liệu thư viện chuẩn, `std::option`](https://doc.rust-lang.org/std/option/index.html)
- [Tài liệu thư viện chuẩn, trait `std::error::Error`](https://doc.rust-lang.org/std/error/trait.Error.html)
- [Tài liệu thư viện chuẩn, trait `From`](https://doc.rust-lang.org/std/convert/trait.From.html)
- [Tài liệu thư viện chuẩn, macro `panic!`](https://doc.rust-lang.org/std/macro.panic.html)
- [The Rust Reference, Operator expressions (gồm toán tử `?`)](https://doc.rust-lang.org/reference/expressions/operator-expr.html)
- [Tra cứu mã lỗi, E0277](https://doc.rust-lang.org/error_codes/E0277.html)
- [Tra cứu mã lỗi, E0308](https://doc.rust-lang.org/error_codes/E0308.html)
- [Tra cứu mã lỗi, E0004](https://doc.rust-lang.org/error_codes/E0004.html)
