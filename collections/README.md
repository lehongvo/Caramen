# Tập hợp dữ liệu (Collections) trong Rust

> Tài liệu học tập về các collection của thư viện chuẩn Rust: `Vec`, `String` và `HashMap`. Gồm khái niệm, cách dùng, thực hành và bài tập có đáp án.

| Thông tin | Chi tiết |
|---|---|
| Chủ đề | `Vec<T>`, `String` (UTF-8), `HashMap<K, V>`, nhắc ngắn `HashSet` và `BTreeMap` |
| Môi trường | Rust 1.99.1 (stable), Cargo 1.99.1, edition 2024 |
| Dự án | `collections` (thư mục này) |
| Kiến thức cần có | `Option` và `match` (bài [`enums_match`](../enums_match/README.md)); mượn `&` và `&mut` (bài [`ownership`](../ownership/README.md)) |
| Nguồn tham khảo chính | *The Rust Programming Language*, chương 8 |

## Mục lục

1. [Mục tiêu bài học](#1-mục-tiêu-bài-học)
2. [Khái niệm](#2-khái-niệm)
3. [Vec](#3-vec)
4. [String](#4-string)
5. [HashMap](#5-hashmap)
6. [Thực hành](#6-thực-hành)
7. [Tìm hiểu chi tiết](#7-tìm-hiểu-chi-tiết)
8. [Lỗi thường gặp](#8-lỗi-thường-gặp)
9. [Bài tập](#9-bài-tập)
10. [Tóm tắt](#10-tóm-tắt)
11. [Tài liệu tham khảo](#11-tài-liệu-tham-khảo)

---

## 1. Mục tiêu bài học

Sau bài học này bạn có thể:

- [ ] Giải thích collection là gì và khác array, tuple ở điểm nào.
- [ ] Dùng `Vec<T>`: tạo, thêm, xoá, truy cập, duyệt và sửa phần tử.
- [ ] Giải thích vì sao `v[i]` có thể làm chương trình panic còn `v.get(i)` trả về `Option`.
- [ ] Giải thích vì sao không thể thêm phần tử vào `Vec` khi còn đang giữ tham chiếu tới một phần tử của nó.
- [ ] Phân biệt `String`, `&str`, byte và `char`; hiểu vì sao `s[0]` bị cấm và xử lý đúng chuỗi tiếng Việt.
- [ ] Dùng `HashMap<K, V>`: `insert`, `get`, `entry().or_insert`, duyệt, và hiểu quy tắc ownership với khoá và giá trị.
- [ ] Chọn đúng giữa `Vec`, `HashMap`, `HashSet` và `BTreeMap`.
- [ ] Đọc hiểu và sửa các lỗi `E0502`, `E0277`, `E0382`, `E0507`.

**Cách dùng tài liệu.** Chép từng ví dụ vào `src/main.rs` rồi chạy:

```bash
cd collections
cargo run
```

Các ví dụ được đánh dấu như sau:

- Ví dụ có chú thích `// ❌` **không biên dịch được** (cố ý, để minh hoạ lỗi).
- Ví dụ có chú thích `// ⚠️` biên dịch được nhưng sinh cảnh báo.
- Các ví dụ còn lại biên dịch sạch và cho đúng kết quả ghi bên dưới.
- Lỗi xảy ra **lúc chạy** (panic) được trích dẫn thông báo thật trong phần văn bản, không đặt trong khối mã chạy được.

---

## 2. Khái niệm

### 2.1 Collection là gì

Collection là cấu trúc dữ liệu chứa **nhiều giá trị** trong một. Khác với array và tuple (độ dài cố định, biết ngay lúc biên dịch), dữ liệu của collection nằm trên **heap**, nên **số phần tử có thể tăng giảm lúc chương trình chạy**.

| | Tuple | Array `[T; N]` | `Vec<T>` | `String` | `HashMap<K, V>` |
|---|---|---|---|---|---|
| Chứa | Nhiều kiểu khác nhau | Một kiểu | Một kiểu | Văn bản UTF-8 | Cặp khoá → giá trị |
| Độ dài | Cố định | Cố định | Đổi được lúc chạy | Đổi được lúc chạy | Đổi được lúc chạy |
| Dữ liệu nằm ở | Ngay tại biến (thường là stack) | Ngay tại biến (thường là stack) | Heap | Heap | Heap |
| Có thứ tự | Có | Có | Có | Có | Không |

Với collection, biến chỉ giữ một "tay cầm" nhỏ (con trỏ, độ dài...), còn dữ liệu thật nằm trên heap (xem [mục 7.1](#71-vec-và-string-trong-bộ-nhớ)).

### 2.2 Ba collection thường dùng

| Collection | Ý nghĩa | Dùng khi | Cần `use` |
|---|---|---|---|
| `Vec<T>` | Danh sách các phần tử **cùng kiểu**, có thứ tự | Cần danh sách thay đổi được độ dài | Không (có sẵn) |
| `String` | Văn bản UTF-8 có thể sửa | Cần chuỗi sở hữu, nối, sửa | Không (có sẵn) |
| `HashMap<K, V>` | Tra cứu giá trị theo khoá | Đếm, gom nhóm, tra cứu nhanh theo tên hoặc mã | `use std::collections::HashMap;` |

### 2.3 Ôn nhanh hai ý cần dùng

- **`Option<T>`** là enum có hai dạng: `Some(giá_trị)` hoặc `None`. Nhiều phương thức của collection (`get`, `pop`, `first`...) trả về `Option` vì phần tử có thể không tồn tại. Cách xử lý thường dùng: `match` hoặc `unwrap_or` (xem bài [`enums_match`](../enums_match/README.md)).
- **Mượn (borrow):** `&x` để đọc, `&mut x` để sửa. Tại một thời điểm, hoặc có **nhiều** `&`, hoặc có **một** `&mut`, không có cả hai cùng lúc (xem bài [`ownership`](../ownership/README.md)). Quy tắc này giải thích phần lớn lỗi bạn sẽ gặp với collection.

---

## 3. Vec

`Vec<T>` (đọc là "vector") là danh sách co giãn các phần tử **cùng kiểu `T`**, nằm liền nhau trong bộ nhớ.

### 3.1 Tạo Vec

```rust
fn main() {
    let empty: Vec<i32> = Vec::new();
    let numbers = vec![1, 2, 3];
    let zeros = vec![0; 4];
    let names: Vec<&str> = vec!["An", "Bình"];

    println!("{:?} {:?} {:?} {:?}", empty, numbers, zeros, names);
}
```

**Kết quả:**

```text
[] [1, 2, 3] [0, 0, 0, 0] ["An", "Bình"]
```

| Cách tạo | Ý nghĩa |
|---|---|
| `Vec::new()` | Vec rỗng. Compiler cần biết kiểu phần tử: ghi `Vec<i32>`, hoặc để nó suy ra từ lần `push` đầu tiên |
| `vec![1, 2, 3]` | Macro tạo Vec với các giá trị cho sẵn |
| `vec![0; 4]` | Vec gồm 4 phần tử giống nhau (cùng cú pháp với array `[0; 4]`) |
| `Vec::with_capacity(n)` | Vec rỗng nhưng dự trữ sẵn chỗ cho ít nhất `n` phần tử (xem [mục 3.6](#36-dung-lượng-capacity)) |

Kiểu phần tử có thể được suy ra từ các lần dùng về sau:

```rust
fn main() {
    let mut v = Vec::new();
    v.push(10); // từ đây compiler biết v là Vec<i32>
    println!("{:?}", v);
}
```

**Kết quả:**

```text
[10]
```

Nếu không có thông tin nào về kiểu, compiler báo lỗi `E0282` (xem [mục 8.2](#82-lỗi-biên-dịch)).

### 3.2 Thêm và xoá phần tử

Vec phải khai báo `mut` mới thêm hoặc xoá được.

```rust
fn main() {
    let mut v = vec![1, 2, 3];

    v.push(4); // thêm vào cuối
    v.insert(0, 0); // chèn vào vị trí 0
    println!("{:?}", v);

    let last = v.pop(); // bỏ phần tử cuối, trả về Option
    println!("{:?} {:?}", last, v);

    let removed = v.remove(1); // bỏ phần tử ở vị trí 1, trả về chính phần tử đó
    println!("{removed} {:?}", v);

    println!("len = {}, rỗng? {}", v.len(), v.is_empty());

    let mut empty: Vec<i32> = Vec::new();
    println!("{:?}", empty.pop());
}
```

**Kết quả:**

```text
[0, 1, 2, 3, 4]
Some(4) [0, 1, 2, 3]
1 [0, 2, 3]
len = 3, rỗng? false
None
```

| Phương thức | Tác dụng | Trả về |
|---|---|---|
| `push(x)` | Thêm `x` vào cuối | Không |
| `pop()` | Bỏ phần tử cuối | `Option<T>` (`None` nếu Vec rỗng) |
| `insert(i, x)` | Chèn `x` vào vị trí `i`, dồn các phần tử sau sang phải | Không |
| `remove(i)` | Bỏ phần tử ở vị trí `i`, dồn các phần tử sau sang trái | `T` |
| `clear()` | Xoá hết phần tử | Không |
| `len()` | Số phần tử | `usize` |
| `is_empty()` | Vec có rỗng không | `bool` |

> `insert(i, x)` và `remove(i)` **panic** nếu `i` vượt phạm vi cho phép. Cả hai phải dịch chuyển các phần tử phía sau nên chậm khi Vec lớn (xem [mục 7.5](#75-độ-phức-tạp-của-các-thao-tác)).

### 3.3 Truy cập phần tử

Có hai cách, khác nhau ở chỗ chuyện gì xảy ra khi chỉ số ngoài phạm vi:

```rust
fn main() {
    let v = vec![10, 20, 30];

    let second = v[1];
    println!("v[1] = {second}");

    println!("{:?}", v.get(1));
    println!("{:?}", v.get(10));

    match v.get(10) {
        Some(x) => println!("Có phần tử: {x}"),
        None => println!("Không có phần tử thứ 10"),
    }
}
```

**Kết quả:**

```text
v[1] = 20
Some(20)
None
Không có phần tử thứ 10
```

| | `v[i]` | `v.get(i)` |
|---|---|---|
| Kiểu trả về | Chính phần tử (`T`) | `Option<&T>` |
| Chỉ số ngoài phạm vi | **Panic** (chương trình dừng) | Trả về `None` |
| Dùng khi | Chắc chắn chỉ số hợp lệ; nếu sai thì đó là lỗi lập trình | Chỉ số có thể sai (dữ liệu nhập vào, kết quả tính toán) |

Nếu `v` có 3 phần tử mà gọi `v[100]`, chương trình dừng với thông báo:

```text
index out of bounds: the len is 3 but the index is 100
```

Vài điểm cần nhớ:

- **Chỉ số phải có kiểu `usize`.** Viết `v[i]` với `i: i32` bị lỗi `E0277` (`the type [{integer}] cannot be indexed by i32`). Dùng `v[i as usize]` khi cần.
- `v.first()` và `v.last()` trả về `Option<&T>` cho phần tử đầu và cuối, an toàn cả khi Vec rỗng.
- `v.get(i)` mượn phần tử (`&T`). Muốn lấy bản sao của số, dùng `.copied()` (xem [mục 5.3](#53-đọc-giá-trị)).

### 3.4 Duyệt Vec

```rust
fn main() {
    let mut scores = vec![8, 7, 9];

    for s in &scores {
        println!("điểm: {s}");
    }

    for s in &mut scores {
        *s += 1; // s là &mut i32, phải giải tham chiếu bằng *
    }
    println!("{:?}", scores);

    for (i, s) in scores.iter().enumerate() {
        println!("{i}: {s}");
    }
}
```

**Kết quả:**

```text
điểm: 8
điểm: 7
điểm: 9
[9, 8, 10]
0: 9
1: 8
2: 10
```

| Cách viết | Phần tử nhận được | Sau vòng lặp |
|---|---|---|
| `for x in &v` | `&T` (chỉ đọc) | `v` vẫn dùng được |
| `for x in &mut v` | `&mut T` (sửa được qua `*x`) | `v` vẫn dùng được |
| `for x in v` | `T` (lấy sở hữu) | `v` **không** dùng được nữa (lỗi `E0382`, xem [mục 8.2](#82-lỗi-biên-dịch)) |
| `v.iter().enumerate()` | Cặp `(chỉ số, &T)` | `v` vẫn dùng được |

Quy tắc chọn: mặc định viết `for x in &v`; chỉ bỏ dấu `&` khi muốn "tiêu thụ" Vec.

### 3.5 Mượn khi đang giữ tham chiếu

Đây là lỗi kinh điển với `Vec`:

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    v.push(4); // ❌ đang mượn v ở dòng trên
    println!("Phần tử đầu: {first}");
}
```

**Trình biên dịch báo:**

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:4:5
  |
3 |     let first = &v[0];
  |                  - immutable borrow occurs here
4 |     v.push(4); // ❌ đang mượn v ở dòng trên
  |     ^^^^^^^^^ mutable borrow occurs here
5 |     println!("Phần tử đầu: {first}");
  |                             ----- immutable borrow later used here
```

**Vì sao Rust cấm?** Khi `Vec` hết chỗ, `push` xin một vùng nhớ lớn hơn, chép toàn bộ phần tử sang đó rồi giải phóng vùng cũ. Nếu `first` vẫn trỏ vào vùng cũ thì nó trỏ vào bộ nhớ đã bị giải phóng. Rust chặn tình huống này ngay lúc biên dịch.

Hai cách sửa:

**Cách 1:** dùng xong tham chiếu rồi mới thay đổi Vec.

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("Phần tử đầu: {first}"); // dùng xong, first hết "sống"
    v.push(4);
    println!("{:?}", v);
}
```

**Kết quả:**

```text
Phần tử đầu: 1
[1, 2, 3, 4]
```

**Cách 2:** sao chép giá trị thay vì mượn (được vì `i32` là kiểu `Copy`).

```rust
fn main() {
    let mut v = vec![1, 2, 3];
    let first = v[0]; // sao chép số 1, không còn mượn v
    v.push(4);
    println!("Phần tử đầu: {first}, v = {:?}", v);
}
```

**Kết quả:**

```text
Phần tử đầu: 1, v = [1, 2, 3, 4]
```

### 3.6 Dung lượng (capacity)

`Vec` có hai con số: `len` (số phần tử đang chứa) và `capacity` (số phần tử chứa được trước khi phải xin thêm bộ nhớ). Luôn có `len <= capacity`.

```rust
fn main() {
    let mut v: Vec<i32> = Vec::with_capacity(10);
    println!("len = {}, đủ chỗ cho 10 phần tử: {}", v.len(), v.capacity() >= 10);

    for i in 0..10 {
        v.push(i);
    }
    println!("len = {}, vẫn đủ chỗ cho 10: {}", v.len(), v.capacity() >= 10);

    v.push(10); // vượt dung lượng đã dự trữ, Vec tự xin thêm chỗ
    println!("len = {}, capacity > 10: {}", v.len(), v.capacity() > 10);
}
```

**Kết quả:**

```text
len = 0, đủ chỗ cho 10 phần tử: true
len = 10, vẫn đủ chỗ cho 10: true
len = 11, capacity > 10: true
```

`with_capacity` chỉ là một cách **tối ưu** khi biết trước số phần tử: tránh xin bộ nhớ nhiều lần. Bỏ nó đi chương trình vẫn đúng. Tài liệu chỉ cam kết "ít nhất" `n` phần tử, nên đừng viết code phụ thuộc vào con số `capacity` chính xác.

### 3.7 Vec, array và slice

| | Array `[T; N]` | `Vec<T>` |
|---|---|---|
| Độ dài | Cố định, là một phần của kiểu | Thay đổi lúc chạy |
| `push` / `pop` | Không có | Có |
| Khi dùng | Số phần tử cố định, biết trước (tháng trong năm, toạ độ x y z) | Số phần tử chưa biết trước |

**Slice** `&[T]` là một "lát nhìn" vào dãy phần tử liên tiếp, lấy được từ cả array lẫn Vec. Giống nguyên tắc `&str` cho tham số chuỗi, **hàm chỉ đọc dãy phần tử nên nhận `&[T]`**, như vậy truyền được cả hai:

```rust
fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}

fn main() {
    let array = [1, 2, 3];
    let vector = vec![4, 5, 6];

    println!("{}", sum(&array));
    println!("{}", sum(&vector));
}
```

**Kết quả:**

```text
6
15
```

Hàm viết `fn sum(numbers: &Vec<i32>)` sẽ không nhận được array, và đòi người gọi phải có đúng một `Vec`. Nên ưu tiên `&[T]`.

### 3.8 Một số phương thức hay dùng

```rust
fn main() {
    let mut v = vec![30, 10, 20];

    v.sort(); // sắp xếp tăng dần
    println!("{:?}", v);
    println!("{}", v.contains(&20)); // có chứa 20 không

    v.reverse(); // đảo ngược
    println!("{:?}", v);

    v.extend([1, 2]); // nối thêm nhiều phần tử
    println!("{:?}", v);

    println!("{:?} {:?}", v.first(), v.last());
}
```

**Kết quả:**

```text
[10, 20, 30]
true
[30, 20, 10]
[30, 20, 10, 1, 2]
Some(30) Some(2)
```

`contains` nhận **tham chiếu** (`&20`), vì nó chỉ so sánh chứ không lấy giá trị đi.

### 3.9 Vec chứa nhiều kiểu: dùng enum

`Vec` chỉ chứa **một kiểu**. Cần lưu nhiều loại giá trị trong cùng một danh sách thì gói chúng vào các biến thể của một `enum` (xem bài [`enums_match`](../enums_match/README.md)):

```rust
enum Cell {
    Int(i32),
    Text(String),
}

fn main() {
    let row = vec![Cell::Int(3), Cell::Text(String::from("xin chào"))];

    for cell in &row {
        match cell {
            Cell::Int(n) => println!("số: {n}"),
            Cell::Text(s) => println!("chuỗi: {s}"),
        }
    }
}
```

**Kết quả:**

```text
số: 3
chuỗi: xin chào
```

---

## 4. String

### 4.1 Ôn nhanh: String và &str

| | `&str` | `String` |
|---|---|---|
| Sở hữu dữ liệu | Không (chỉ mượn) | Có |
| Sửa được | Không | Có (khi khai báo `mut`) |
| Dữ liệu nằm ở | Bất kỳ đâu (chuỗi viết sẵn trong code nằm trong chương trình) | Heap |
| Ví dụ | `"Rust"` | `String::from("Rust")` |

Thực chất `String` là một `Vec<u8>` được bọc lại, đảm bảo các byte bên trong luôn là **UTF-8 hợp lệ**. Vì vậy `String` có nhiều phương thức giống `Vec` (`push`, `pop`, `len`, `capacity`, `clear`...).

### 4.2 Tạo String

```rust
fn main() {
    let a = String::new(); // rỗng
    let b = "Rust".to_string();
    let c = String::from("Rust");
    let d = "Rust".to_owned();
    let e = format!("{}-{}", "Rust", 2024);

    println!("[{a}] {b} {c} {d} {e}");
}
```

**Kết quả:**

```text
[] Rust Rust Rust Rust-2024
```

`to_string()`, `String::from()` và `to_owned()` cùng làm một việc với chuỗi viết sẵn: tạo `String` sở hữu một bản sao của nó. Chọn cách nào cũng được, miễn nhất quán trong dự án.

### 4.3 Thêm và nối chuỗi

```rust
fn main() {
    let mut s = String::from("Xin");
    s.push(' '); // thêm MỘT char
    s.push_str("chào"); // thêm một &str
    s += "!"; // giống push_str
    println!("{s}");

    let a = String::from("Rust");
    let b = String::from("rất");
    let c = String::from("hay");
    let sentence = a + " " + &b + " " + &c; // a bị chuyển đi; b, c chỉ bị mượn
    println!("{sentence}");

    let x = String::from("tic");
    let y = String::from("tac");
    let z = String::from("toe");
    let game = format!("{x}-{y}-{z}"); // không lấy đi x, y, z
    println!("{game}");
    println!("{y}"); // y vẫn dùng được
}
```

**Kết quả:**

```text
Xin chào!
Rust rất hay
tic-tac-toe
tac
```

| Cách | Khi dùng | Ảnh hưởng đến biến gốc |
|---|---|---|
| `push(c)` | Thêm một `char` | Sửa `s` tại chỗ |
| `push_str(&str)` hoặc `+=` | Thêm một đoạn chuỗi | Sửa `s` tại chỗ |
| `a + &b` | Nối hai chuỗi thành chuỗi mới | `a` **bị chuyển đi** (không dùng lại được); `b` vẫn còn |
| `format!(...)` | Ghép nhiều mảnh | Không lấy đi biến nào |

Vì sao `a + &b` cần dấu `&` ở vế phải? Phép `+` của `String` có dạng `fn add(self, s: &str) -> String`: nó **lấy sở hữu** vế trái và chỉ **mượn** vế phải dưới dạng `&str`. Giá trị `&b` có kiểu `&String`, được Rust tự đổi thành `&str` (*deref coercion*). Viết `a + b` (thiếu `&`) hoặc dùng lại `a` sau phép `+` đều gây lỗi (xem [mục 8.2](#82-lỗi-biên-dịch)).

Khi cần ghép từ ba mảnh trở lên, `format!` dễ đọc hơn chuỗi dài các dấu `+`.

### 4.4 Vì sao không thể index s[0]

`String` lưu văn bản dạng **UTF-8**: mỗi ký tự chiếm từ **1 đến 4 byte**.

```rust
fn main() {
    for c in ['a', 'à', 'ệ', '😻'] {
        println!("{c}: {} byte", c.len_utf8());
    }
}
```

**Kết quả:**

```text
a: 1 byte
à: 2 byte
ệ: 3 byte
😻: 4 byte
```

Vì vậy `len()` đếm **byte**, không đếm ký tự:

```rust
fn main() {
    let hello = "Xin chào";
    let city = "Việt Nam";

    println!("{hello}: {} byte, {} ký tự", hello.len(), hello.chars().count());
    println!("{city}: {} byte, {} ký tự", city.len(), city.chars().count());
}
```

**Kết quả:**

```text
Xin chào: 9 byte, 8 ký tự
Việt Nam: 10 byte, 8 ký tự
```

Giờ thì thấy vì sao `s[0]` mơ hồ: nó là **byte** đầu, **ký tự** đầu, hay **chữ cái** đầu? Với tiếng Việt, ba thứ đó khác nhau. Rust không đoán thay bạn mà cấm hẳn:

```rust
fn main() {
    let s = String::from("xin chào");
    let c = s[0]; // ❌ không index được String
    println!("{c}");
}
```

**Trình biên dịch báo:**

```text
error[E0277]: the type `str` cannot be indexed by `{integer}`
 --> src/main.rs:3:15
  |
3 |     let c = s[0]; // ❌ không index được String
  |               ^ string indices are ranges of `usize`
  |
  = help: the trait `SliceIndex<str>` is not implemented for `{integer}`
  = note: you can use `.chars().nth()` or `.bytes().nth()`
```

Cách làm đúng: chọn rõ bạn cần gì (xem [mục 4.5](#45-cắt-chuỗi-theo-byte) và [mục 4.6](#46-duyệt-string)).

**Một chữ nhìn thấy có thể gồm nhiều `char`.** Chữ có dấu có thể được lưu dạng *dựng sẵn* (một `char`) hoặc dạng *tổ hợp* (chữ cái cộng dấu tách rời). Hai dạng nhìn giống hệt nhau nhưng khác nhau với máy:

```rust
fn main() {
    let precomposed = "\u{e9}"; // chữ é dựng sẵn: 1 char
    let combined = "e\u{301}"; // chữ e cộng dấu sắc tách rời: 2 char

    println!("{} {}", precomposed.chars().count(), combined.chars().count());
    println!("{}", precomposed == combined);
}
```

**Kết quả:**

```text
1 2
false
```

Đa số văn bản tiếng Việt bạn gõ ra là dạng dựng sẵn, nên `chars()` đủ dùng trong bài này. Muốn xử lý "chữ nhìn thấy" một cách chính xác trong mọi trường hợp, cần thư viện ngoài như `unicode-segmentation`; thư viện chuẩn không có.

### 4.5 Cắt chuỗi theo byte

Cú pháp `&s[a..b]` cắt theo **vị trí byte** và cho ra `&str`:

```rust
fn main() {
    let s = "Xin chào";

    println!("{}", &s[0..3]); // 3 byte đầu là "Xin"
    println!("{:?}", s.get(0..3)); // phiên bản an toàn, trả về Option
    println!("{:?}", s.get(0..7)); // byte 7 nằm giữa chữ à

    let first_six: String = s.chars().take(6).collect(); // 6 KÝ TỰ đầu
    println!("{first_six}");
}
```

**Kết quả:**

```text
Xin
Some("Xin")
None
Xin ch
```

Chữ `à` chiếm byte 6 và 7 (tính từ 0). Nếu viết `&s[0..7]`, ranh giới cắt rơi vào **giữa** chữ `à`, và chương trình panic:

```text
end byte index 7 is not a char boundary; it is inside 'à' (bytes 6..8 of string)
```

Quy tắc: cắt theo byte chỉ an toàn khi biết chắc ranh giới (chuỗi toàn ASCII, hoặc vị trí lấy từ `char_indices`). Dùng `s.get(a..b)` khi không chắc, hoặc dùng `chars().take(n)` khi cần "n ký tự đầu".

### 4.6 Duyệt String

```rust
fn main() {
    let s = "chào";

    let chars: Vec<char> = s.chars().collect();
    let bytes: Vec<u8> = s.bytes().collect();
    println!("{:?}", chars);
    println!("{:?}", bytes);

    for (i, c) in s.char_indices() {
        println!("{i}: {c}");
    }
}
```

**Kết quả:**

```text
['c', 'h', 'à', 'o']
[99, 104, 195, 160, 111]
0: c
1: h
2: à
4: o
```

| Phương thức | Cho ra | Dùng khi |
|---|---|---|
| `chars()` | Từng `char` | Xử lý theo ký tự (đảo ngược, đếm, lọc) |
| `bytes()` | Từng `u8` | Làm việc ở mức byte (mã hoá, giao thức) |
| `char_indices()` | Cặp `(vị trí byte, char)` | Cần biết ký tự nằm ở byte nào, để cắt chuỗi an toàn |

Chú ý chữ `à` có vị trí byte là 2 và chiếm 2 byte, nên chữ `o` sau nó nằm ở vị trí 4, không phải 3. Lệnh `.collect()` gom kết quả vào một collection, nên bạn phải ghi rõ kiểu đích (`Vec<char>`, `Vec<u8>`, `String`...).

### 4.7 Các phương thức thường dùng

```rust
fn main() {
    let s = "  Rust rất thú vị  ";
    let t = s.trim();
    println!("[{t}]");

    println!("{}", t.to_uppercase());
    println!("{}", t.contains("thú"));
    println!("{}", t.replace("rất", "khá"));

    let words: Vec<&str> = t.split_whitespace().collect();
    println!("{:?} ({} từ)", words, words.len());
}
```

**Kết quả:**

```text
[Rust rất thú vị]
RUST RẤT THÚ VỊ
true
Rust khá thú vị
["Rust", "rất", "thú", "vị"] (4 từ)
```

| Phương thức | Tác dụng |
|---|---|
| `trim()` | Bỏ khoảng trắng hai đầu (trả về `&str`) |
| `to_uppercase()`, `to_lowercase()` | Đổi hoa, thường (hiểu Unicode, kể cả tiếng Việt), trả về `String` mới |
| `contains(p)`, `starts_with(p)`, `ends_with(p)` | Kiểm tra chứa, bắt đầu, kết thúc bằng `p` |
| `replace(a, b)` | Thay mọi `a` bằng `b`, trả về `String` mới |
| `split_whitespace()` | Tách theo khoảng trắng |
| `lines()` | Tách theo từng dòng |
| `is_empty()` | Chuỗi có rỗng không |

> Các phương thức này **không sửa** chuỗi gốc mà trả về giá trị mới. Gọi `s.trim();` rồi bỏ kết quả là vô ích, và compiler cảnh báo (xem [mục 8.3](#83-cảnh-báo-thường-gặp)).

### 4.8 Chuyển đổi qua lại

| Từ | Sang | Cách viết |
|---|---|---|
| `&str` | `String` | `.to_string()`, `String::from(s)`, `.to_owned()`, `.into()` |
| `String` | `&str` | `&s` (tự đổi khi truyền vào hàm), hoặc `s.as_str()` |
| Số | `String` | `n.to_string()`, hoặc `format!("{n}")` |
| `&str` / `String` | Số | `s.parse::<T>()`, trả về `Result` (xem `parse` trong dự án trò chơi đoán số `gessing_game`) |

```rust
fn main() {
    let n = 42;
    let text = n.to_string();
    let back: i32 = text.parse().expect("không phải số");

    println!("{text} {}", back + 1);
}
```

**Kết quả:**

```text
42 43
```

---

## 5. HashMap

### 5.1 Khái niệm

`HashMap<K, V>` lưu các **cặp khoá → giá trị** và tra cứu giá trị theo khoá rất nhanh. Giống một cuốn danh bạ: tìm theo tên (khoá) để ra số điện thoại (giá trị).

- Mỗi khoá xuất hiện **tối đa một lần**; thêm cùng khoá lần nữa sẽ **ghi đè** giá trị.
- Mọi khoá cùng một kiểu `K`; mọi giá trị cùng một kiểu `V`.
- **Không có thứ tự.** Thứ tự duyệt không xác định và có thể khác nhau giữa các lần chạy chương trình (xem [mục 7.6](#76-vì-sao-thứ-tự-của-hashmap-thay-đổi)).
- Kiểu khoá phải so sánh bằng được và băm được (`Eq` và `Hash`). `String`, `&str`, số nguyên, `bool`, `char` và tuple của chúng đều dùng được. `f64` thì **không** (số thực không có `Eq`).
- **Không nằm trong prelude**, phải khai báo: `use std::collections::HashMap;`.

### 5.2 Tạo và thêm

```rust
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("An"), 8);
    scores.insert(String::from("Bình"), 9);

    println!("Số cặp: {}", scores.len());
    println!("{:?}", scores.get("An"));
    println!("{:?}", scores.get("Chi"));
    println!("{}", scores.contains_key("Bình"));

    let ids = HashMap::from([(1, "An"), (2, "Bình")]);
    println!("{:?}", ids.get(&2));
}
```

**Kết quả:**

```text
Số cặp: 2
Some(8)
None
true
Some("Bình")
```

- `HashMap::new()` tạo map rỗng; `HashMap::from([...])` tạo từ một array các cặp.
- `get` nhận **tham chiếu tới khoá** và trả về `Option<&V>`. Với khoá `String`, truyền `"An"` (kiểu `&str`) được luôn, không cần tạo `String` mới. Với khoá số, truyền `&2`.
- Không in cả map bằng `{:?}` khi cần kết quả ổn định, vì thứ tự không xác định.

### 5.3 Đọc giá trị

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("An", 8);

    let an = scores.get("An").copied().unwrap_or(0);
    let chi = scores.get("Chi").copied().unwrap_or(0);
    println!("An: {an}, Chi: {chi}");

    match scores.get("An") {
        Some(score) => println!("An có {score} điểm"),
        None => println!("Không có An"),
    }
}
```

**Kết quả:**

```text
An: 8, Chi: 0
An có 8 điểm
```

- `.copied()` biến `Option<&u32>` thành `Option<u32>` bằng cách sao chép (chỉ dùng với kiểu `Copy` như số).
- `.unwrap_or(0)` lấy giá trị bên trong, hoặc `0` nếu là `None`.
- Cũng có thể viết `scores["An"]`, nhưng nếu khoá không tồn tại thì **panic** với thông báo `no entry found for key`. Dùng `get` khi khoá có thể không có.

### 5.4 Ghi đè và xoá

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("An", 8);

    let old = scores.insert("An", 10); // cùng khoá: ghi đè, trả về giá trị cũ
    println!("{:?}", old);

    let removed = scores.remove("An");
    println!("{:?}", removed);

    println!("{:?}", scores.remove("An")); // lần này không còn
}
```

**Kết quả:**

```text
Some(8)
Some(10)
None
```

`insert` trả về `Option<V>`: `None` nếu khoá mới, `Some(giá trị cũ)` nếu vừa ghi đè. `remove` trả về giá trị vừa xoá (nếu có).

### 5.5 entry: chỉ thêm khi chưa có

Nhu cầu rất hay gặp: "nếu khoá chưa có thì thêm, nếu có rồi thì giữ nguyên". Dùng `entry`:

```rust
use std::collections::HashMap;

fn main() {
    let mut stock: HashMap<&str, u32> = HashMap::new();
    stock.insert("bút", 10);

    stock.entry("bút").or_insert(99); // đã có: giữ nguyên 10
    stock.entry("vở").or_insert(5); // chưa có: thêm mới

    let mut items: Vec<_> = stock.iter().collect();
    items.sort(); // sắp xếp để thứ tự in ổn định
    for (name, qty) in items {
        println!("{name}: {qty}");
    }
}
```

**Kết quả:**

```text
bút: 10
vở: 5
```

`entry(khoá)` trả về một "ô" tương ứng với khoá đó (có thể trống hoặc đã có giá trị). `or_insert(v)` điền `v` vào nếu ô trống, rồi trả về `&mut V` trỏ tới giá trị trong map.

### 5.6 Cập nhật dựa trên giá trị cũ

Vì `or_insert` trả về `&mut V`, ta có thể sửa giá trị ngay. Mẫu **đếm số lần xuất hiện** kinh điển:

```rust
use std::collections::HashMap;

fn main() {
    let text = "xin chào xin chào rust";
    let mut counts: HashMap<&str, u32> = HashMap::new();

    for word in text.split_whitespace() {
        let count = counts.entry(word).or_insert(0);
        *count += 1; // count là &mut u32 nên cần *
    }

    let mut result: Vec<_> = counts.into_iter().collect();
    result.sort();
    println!("{:?}", result);
}
```

**Kết quả:**

```text
[("chào", 2), ("rust", 1), ("xin", 2)]
```

Mỗi từ: nếu chưa có thì đặt `0`, rồi cộng `1`. Có thể viết gọn thành `*counts.entry(word).or_insert(0) += 1;`. Với kiểu có giá trị mặc định (số là `0`, `Vec` là rỗng, `String` là rỗng), dùng `or_default()` thay cho `or_insert(...)` (xem [Lab 2](#lab-2-quản-lý-điểm-học-sinh)).

### 5.7 Duyệt HashMap

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::from([("An", 8), ("Bình", 9)]);

    for score in scores.values_mut() {
        *score += 1;
    }

    let mut names: Vec<_> = scores.keys().collect();
    names.sort();
    for name in names {
        println!("{name}: {}", scores[name]);
    }

    let total: i32 = scores.values().sum();
    println!("Tổng: {total}");
}
```

**Kết quả:**

```text
An: 9
Bình: 10
Tổng: 19
```

| Cách duyệt | Cho ra |
|---|---|
| `for (k, v) in &map` | Từng cặp `(&K, &V)`, thứ tự không xác định |
| `map.keys()` | Chỉ các khoá |
| `map.values()` | Chỉ các giá trị |
| `map.values_mut()` | Các giá trị dạng `&mut V`, sửa được |
| `for (k, v) in map` | Lấy sở hữu từng cặp (map không dùng được nữa) |

Muốn in theo thứ tự ổn định: gom khoá vào `Vec` rồi `sort()` như trên, hoặc dùng `BTreeMap` (xem [mục 5.9](#59-hashset-và-btreemap)).

### 5.8 Ownership với HashMap

Khi `insert`, khoá và giá trị được **đưa vào** map:

| Kiểu của giá trị đưa vào | Chuyện gì xảy ra |
|---|---|
| Kiểu `Copy` (`i32`, `bool`, `char`...) | **Sao chép** vào map; biến gốc vẫn dùng được |
| Kiểu sở hữu (`String`, `Vec`...) | **Chuyển** vào map; biến gốc không dùng được nữa |
| Tham chiếu (`&str`...) | Chỉ sao chép tham chiếu; dữ liệu gốc phải sống lâu hơn map (xem bài [`generics_traits_lifetimes`](../generics_traits_lifetimes/README.md)) |

Muốn tiếp tục dùng `String` sau khi `insert`, đưa vào một **bản sao**:

```rust
use std::collections::HashMap;

fn main() {
    let name = String::from("An");
    let mut scores = HashMap::new();

    scores.insert(name.clone(), 8); // đưa một bản sao vào map
    println!("{name}: {:?}", scores.get(&name));
}
```

**Kết quả:**

```text
An: Some(8)
```

Quên `.clone()` thì gặp lỗi `E0382` (xem [mục 8.2](#82-lỗi-biên-dịch)).

### 5.9 HashSet và BTreeMap

Hai collection khác cũng nằm trong `std::collections`, đáng biết:

- **`HashSet<T>`**: tập hợp các giá trị **không trùng nhau**. Giống `HashMap` nhưng chỉ có khoá. `insert` trả về `bool` cho biết giá trị có mới hay không.
- **`BTreeMap<K, V>`**: giống `HashMap` nhưng luôn **sắp xếp theo khoá**, nên duyệt ra đúng thứ tự tăng dần.

```rust
use std::collections::{BTreeMap, HashSet};

fn main() {
    let mut tags = HashSet::new();
    println!("{}", tags.insert("rust")); // giá trị mới
    println!("{}", tags.insert("rust")); // đã có rồi
    println!("{}", tags.len());

    let mut ranking = BTreeMap::new();
    ranking.insert(3, "Chi");
    ranking.insert(1, "An");
    ranking.insert(2, "Bình");
    println!("{:?}", ranking);
}
```

**Kết quả:**

```text
true
false
1
{1: "An", 2: "Bình", 3: "Chi"}
```

| | `HashMap` | `BTreeMap` |
|---|---|---|
| Thứ tự duyệt | Không xác định | Theo thứ tự khoá |
| Tốc độ tra cứu | Nhanh hơn (trung bình hằng số) | Chậm hơn một chút (tỉ lệ `log n`) |
| Khi dùng | Mặc định | Cần thứ tự, hoặc cần lấy theo khoảng khoá |

---

## 6. Thực hành

Hai bài thực hành có hướng dẫn, đều dùng `HashMap`. Thực hiện trong `collections/src/main.rs` và chạy bằng `cargo run`.

### Lab 1. Đếm từ trong câu

**Mục tiêu:** dùng `HashMap`, `entry` và các phương thức của `String` để thống kê tần suất từ.

**Bước 1.** Đếm theo từng từ tách bằng khoảng trắng, in theo thứ tự chữ cái để kết quả ổn định:

```rust
use std::collections::HashMap;

fn main() {
    let text = "Rust rất hay. Học Rust mỗi ngày, rất vui. Rust!";

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for word in text.split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }

    let mut result: Vec<_> = counts.into_iter().collect();
    result.sort();
    for (word, count) in result {
        println!("{word}: {count}");
    }
}
```

**Kết quả:**

```text
Học: 1
Rust: 2
Rust!: 1
hay.: 1
mỗi: 1
ngày,: 1
rất: 2
vui.: 1
```

Có hai vấn đề: `"Rust"` và `"Rust!"` bị coi là hai từ khác nhau (dấu câu dính vào từ), và `"Học"` khác `"học"` nếu viết hoa thường lẫn lộn.

**Bước 2.** Chuẩn hoá từ trước khi đếm (bỏ dấu câu ở hai đầu, đổi thành chữ thường) rồi sắp xếp theo số lần giảm dần:

```rust
use std::collections::HashMap;

fn main() {
    let text = "Rust rất hay. Học Rust mỗi ngày, rất vui. Rust!";

    let mut counts: HashMap<String, u32> = HashMap::new();
    for raw in text.split_whitespace() {
        let word = raw.trim_matches(['.', ',', '!', '?']).to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }

    let mut result: Vec<(String, u32)> = counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    for (word, count) in &result {
        println!("{word}: {count}");
    }
}
```

**Kết quả:**

```text
rust: 3
rất: 2
hay: 1
học: 1
mỗi: 1
ngày: 1
vui: 1
```

**Phân tích:**

- `trim_matches([...])` bỏ các ký tự liệt kê ở **hai đầu** của từ. `to_lowercase()` tạo ra `String` **mới**, nên khoá của map phải là `String` (map sở hữu khoá), không còn là `&str` như bước 1.
- `entry(word)` **chuyển** `word` vào map. Không thể dùng lại `word` sau dòng đó.
- `|a, b| ...` là *closure*: một hàm nhỏ viết ngay tại chỗ, sẽ học kỹ sau. Ở đây tạm đọc là "so sánh hai phần tử `a` và `b` như sau": trước hết theo **số lần giảm dần** (`b.1.cmp(&a.1)`), nếu bằng nhau thì theo **chữ cái** (`.then(a.0.cmp(&b.0))`).
- Cần sắp xếp vì thứ tự duyệt `HashMap` không xác định.

**Thử thêm:**

1. Đổi `text` thành một câu của bạn và dự đoán kết quả trước khi chạy.
2. Chỉ in 3 từ đầu bảng: thay `for (word, count) in &result` bằng `for (word, count) in result.iter().take(3)`.

### Lab 2. Quản lý điểm học sinh

**Mục tiêu:** dùng `HashMap<String, Vec<u32>>` (mỗi học sinh có một danh sách điểm), `entry().or_default()` và `get` với `match`.

**Bước 1.** Gom điểm theo tên:

```rust
use std::collections::HashMap;

fn main() {
    let records = [("An", 8), ("Bình", 6), ("An", 10), ("Chi", 9), ("Bình", 7), ("An", 9)];

    let mut grades: HashMap<String, Vec<u32>> = HashMap::new();
    for (name, score) in records {
        grades.entry(name.to_string()).or_default().push(score);
    }

    let mut names: Vec<&String> = grades.keys().collect();
    names.sort();
    for name in names {
        println!("{name}: {:?}", grades[name]);
    }
}
```

**Kết quả:**

```text
An: [8, 10, 9]
Bình: [6, 7]
Chi: [9]
```

`or_default()` điền giá trị mặc định của kiểu nếu ô còn trống (với `Vec<u32>` là Vec rỗng), rồi trả về `&mut Vec<u32>`. Nhờ đó `.push(score)` thêm điểm vào danh sách của đúng học sinh, dù là lần đầu hay lần sau.

**Bước 2.** Tính điểm trung bình và tra cứu một học sinh bằng `get`:

```rust
use std::collections::HashMap;

fn main() {
    let records = [("An", 8), ("Bình", 6), ("An", 10), ("Chi", 9), ("Bình", 7), ("An", 9)];

    let mut grades: HashMap<String, Vec<u32>> = HashMap::new();
    for (name, score) in records {
        grades.entry(name.to_string()).or_default().push(score);
    }

    let mut names: Vec<&String> = grades.keys().collect();
    names.sort();
    for name in names {
        let scores = &grades[name];
        let sum: u32 = scores.iter().sum();
        let average = sum as f64 / scores.len() as f64;
        println!("{name}: {:?} -> trung bình {average:.1}", scores);
    }

    for name in ["An", "Dũng"] {
        match grades.get(name) {
            Some(scores) => println!("{name} có {} điểm", scores.len()),
            None => println!("{name} chưa có trong danh sách"),
        }
    }
}
```

**Kết quả:**

```text
An: [8, 10, 9] -> trung bình 9.0
Bình: [6, 7] -> trung bình 6.5
Chi: [9] -> trung bình 9.0
An có 3 điểm
Dũng chưa có trong danh sách
```

**Phân tích:**

- `scores.iter().sum()` cộng mọi phần tử; phải ghi kiểu `let sum: u32` để compiler biết cộng ra kiểu nào. Đây là cách dùng *iterator*, sẽ học kỹ ở phần sau.
- `sum as f64 / scores.len() as f64`: ép cả hai về số thực trước khi chia, vì Rust không tự trộn số nguyên với số thực (xem bài `variables`). `len()` trả về `usize`.
- `grades.get(name)` trả về `Option<&Vec<u32>>`: `match` buộc ta xử lý cả trường hợp học sinh không tồn tại.

**Thử thêm:**

1. Thêm học sinh `("Dũng", 5)` vào `records` và quan sát kết quả của hai vòng lặp.
2. Đổi `HashMap` thành `BTreeMap` và bỏ đoạn gom và sắp xếp `names`. Danh sách tự có thứ tự theo tên.

---

## 7. Tìm hiểu chi tiết

### 7.1 Vec và String trong bộ nhớ

`Vec<T>` và `String` đều gồm hai phần: một **"tay cầm"** nhỏ nằm ở biến (con trỏ, độ dài, dung lượng) và **vùng dữ liệu** trên heap.

```text
Biến v (stack)              Heap
┌──────────────┐          ┌───┬───┬───┬───┬───┐
│ ptr ─────────┼────────► │ 1 │ 2 │ 3 │ _ │ _ │
│ len = 3      │          └───┴───┴───┴───┴───┘
│ capacity = 5 │            dùng 3 ô   còn trống 2 ô
└──────────────┘
```

Vì "tay cầm" luôn cùng kích thước dù có bao nhiêu phần tử:

```rust
fn main() {
    println!("Vec<i32>: {} byte", size_of::<Vec<i32>>());
    println!("String:   {} byte", size_of::<String>());
    println!("&str:     {} byte", size_of::<&str>());
    println!("[i32; 3]: {} byte", size_of::<[i32; 3]>());
}
```

**Kết quả:**

```text
Vec<i32>: 24 byte
String:   24 byte
&str:     16 byte
[i32; 3]: 12 byte
```

Trên máy 64 bit: `Vec` và `String` là 24 byte (3 giá trị, mỗi giá trị 8 byte) dù chứa 3 hay 3 triệu phần tử; `&str` là 16 byte (con trỏ và độ dài); còn array `[i32; 3]` là 12 byte vì **chính dữ liệu nằm trong biến**.

### 7.2 Slice của Vec

Cú pháp khoảng `a..b` (lấy từ `a` đến trước `b`) tạo slice `&[T]` mà không sao chép dữ liệu:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    println!("{:?}", &v[1..4]);
    println!("{:?}", &v[..2]);
    println!("{:?}", &v[3..]);
}
```

**Kết quả:**

```text
[2, 3, 4]
[1, 2]
[4, 5]
```

Khoảng ngoài phạm vi (ví dụ `&v[2..10]`) làm chương trình panic, giống `v[i]`.

### 7.3 Lấy phần tử ra khỏi collection

Với kiểu `Copy` (số, `bool`, `char`), `let x = v[0];` sao chép giá trị ra, không vấn đề gì. Với kiểu sở hữu như `String`, **không thể "lấy ra" bằng cách đánh chỉ số**, vì như vậy phần tử trong Vec thành giá trị đã bị chuyển đi (lỗi `E0507`, xem [mục 8.2](#82-lỗi-biên-dịch)). Có ba cách:

```rust
fn main() {
    let mut names = vec![String::from("An"), String::from("Bình")];

    let borrowed = &names[0]; // 1. mượn: không tốn thêm bộ nhớ
    println!("{borrowed}");

    let cloned = names[0].clone(); // 2. sao chép: có bản riêng
    println!("{cloned}");

    let owned = names.remove(0); // 3. lấy hẳn ra: Vec mất phần tử đó
    println!("{owned} / còn lại: {:?}", names);
}
```

**Kết quả:**

```text
An
An
An / còn lại: ["Bình"]
```

### 7.4 Chọn collection nào

| Nhu cầu | Chọn |
|---|---|
| Số phần tử cố định, biết trước lúc biên dịch | Array `[T; N]` |
| Danh sách có thứ tự, thêm bớt, duyệt | `Vec<T>` |
| Văn bản | `String` (sở hữu) hoặc `&str` (mượn) |
| Tra cứu theo khoá, đếm, gom nhóm | `HashMap<K, V>` |
| Tập giá trị không trùng, hỏi "đã có chưa" | `HashSet<T>` |
| Cần duyệt theo thứ tự của khoá | `BTreeMap<K, V>` |

Khi phân vân, bắt đầu với `Vec` hoặc `HashMap`; hầu hết bài toán thông thường dùng hai loại này.

### 7.5 Độ phức tạp của các thao tác

Bảng dưới là mức độ **tăng thời gian khi số phần tử `n` lớn lên** (ký hiệu Big-O), để biết thao tác nào "rẻ", thao tác nào "đắt":

| Thao tác | `Vec` | `HashMap` | `BTreeMap` |
|---|---|---|---|
| Thêm | `push` ở cuối: hằng số (trung bình) | Hằng số (trung bình) | Tỉ lệ `log n` |
| Tìm theo chỉ số / khoá | `v[i]`: hằng số | `get`: hằng số (trung bình) | Tỉ lệ `log n` |
| Tìm giá trị bất kỳ (`contains`) | Tỉ lệ `n` (phải xét từng phần tử) | Không áp dụng (tìm theo khoá) | Không áp dụng |
| Chèn hoặc xoá ở đầu / giữa | `insert`, `remove`: tỉ lệ `n` (phải dồn phần tử) | Hằng số (trung bình) | Tỉ lệ `log n` |

Với dữ liệu nhỏ (vài trăm phần tử), khác biệt không đáng kể. Bảng này quan trọng khi dữ liệu lớn.

### 7.6 Vì sao thứ tự của HashMap thay đổi

`HashMap` đặt mỗi cặp vào vị trí tính từ "mã băm" của khoá. Thuật toán băm mặc định (hiện là SipHash 1-3) được chọn ưu tiên **an toàn** (chống kiểu tấn công làm hàng loạt khoá đụng nhau để làm chậm chương trình) và dùng **khoá băm ngẫu nhiên** cho mỗi map. Kết quả là cùng một tập cặp có thể được duyệt theo thứ tự khác nhau giữa các lần chạy. Chạy cùng một chương trình in `HashMap` nhiều lần sẽ thấy thứ tự thay đổi.

Hệ quả cho code của bạn:

- **Đừng dựa vào thứ tự duyệt** của `HashMap`.
- Cần kết quả ổn định (in ra, so sánh, kiểm thử): sắp xếp khoá trước khi in, hoặc dùng `BTreeMap`.

### 7.7 Chọn công cụ nào?

| Tình huống | Chọn |
|---|---|
| Hàm chỉ đọc dãy phần tử | Tham số `&[T]` |
| Hàm chỉ đọc chuỗi | Tham số `&str` |
| Cần sửa hoặc nối chuỗi, hoặc giữ chuỗi lâu dài | `String` |
| Kiểm tra chỉ số có thể sai | `v.get(i)` thay cho `v[i]` |
| Cần phần tử `String` ra khỏi Vec mà vẫn giữ Vec | `.clone()`, hoặc `&v[i]` nếu chỉ cần đọc |
| Muốn in `HashMap` ổn định | Sắp xếp khoá, hoặc `BTreeMap` |

---

## 8. Lỗi thường gặp

### 8.1 Lỗi lúc chạy (panic)

Các lỗi sau **biên dịch được** nhưng làm chương trình dừng khi chạy:

| Tình huống | Thông báo (Rust 1.99.1) | Cách tránh |
|---|---|---|
| `v[100]` khi Vec có 3 phần tử | `index out of bounds: the len is 3 but the index is 100` | `v.get(100)` |
| `&s[0..7]` cắt giữa chữ `à` của `"Xin chào"` | `end byte index 7 is not a char boundary; it is inside 'à' (bytes 6..8 of string)` | `s.get(0..7)`, hoặc `chars()` |
| `map["Dũng"]` khi khoá không tồn tại | `no entry found for key` | `map.get("Dũng")` |

Điểm chung: có một lựa chọn "an toàn" trả về `Option` (`get`) cho mỗi cú pháp "tiện" có thể panic.

### 8.2 Lỗi biên dịch

| Mã lỗi | Thông báo | Nguyên nhân | Cách sửa |
|---|---|---|---|
| `E0502` | `cannot borrow v as mutable because it is also borrowed as immutable` | Sửa Vec khi còn tham chiếu tới phần tử | Dùng xong tham chiếu trước, hoặc sao chép giá trị ([mục 3.5](#35-mượn-khi-đang-giữ-tham-chiếu)) |
| `E0277` | `the type str cannot be indexed by {integer}` | Viết `s[0]` với chuỗi | `chars()`, `get(a..b)` ([mục 4.4](#44-vì-sao-không-thể-index-s0)) |
| `E0382` | `borrow of moved value` | Dùng lại biến sau khi đã chuyển đi (`for x in v`, `a + &b`, `insert`) | Mượn bằng `&`, hoặc `.clone()`, hoặc `format!` |
| `E0507` | `cannot move out of index of Vec<String>` | `let x = v[0];` với kiểu không `Copy` | `&v[0]`, `.clone()`, hoặc `remove(0)` |
| `E0308` | `mismatched types` | `a + b` với `b` là `String`; gán `Option` vào biến kiểu thường | `a + &b`; xử lý `Option` bằng `match` |
| `E0282` | `type annotations needed for Vec<_>` | `Vec::new()` mà không có thông tin về kiểu phần tử | `let v: Vec<i32> = Vec::new();` |
| `E0599` | `no method named push found for array` | Gọi `push` trên array | Dùng `Vec` |
| `E0369` | `cannot add {integer} to Option<&{integer}>` | Cộng thẳng kết quả của `get` | `match`, hoặc `.copied().unwrap_or(0)` |
| `E0433` | `cannot find type HashMap in this scope` | Quên `use` | `use std::collections::HashMap;` |

Một số lỗi đáng xem kỹ.

**E0382 khi dùng lại Vec sau vòng `for`.** Vòng `for x in v` lấy sở hữu `v`:

```rust
fn main() {
    let names = vec![String::from("An"), String::from("Bình")];
    for n in names {
        println!("{n}");
    }
    println!("{}", names.len()); // ❌ names đã bị chuyển đi
}
```

**Trình biên dịch báo:**

```text
error[E0382]: borrow of moved value: `names`
 --> src/main.rs:6:20
  |
2 |     let names = vec![String::from("An"), String::from("Bình")];
  |         ----- move occurs because `names` has type `Vec<String>`, which does not implement the `Copy` trait
3 |     for n in names {
  |              ----- `names` moved due to this implicit call to `.into_iter()`
...
6 |     println!("{}", names.len()); // ❌ names đã bị chuyển đi
  |                    ^^^^^ value borrowed here after move
  |
help: consider iterating over a slice of the `Vec<String>`'s content to avoid moving into the `for` loop
  |
3 |     for n in &names {
  |              +
```

Cách sửa là điều `help:` gợi ý: viết `for n in &names`.

**E0507 khi lấy `String` ra khỏi Vec bằng chỉ số.**

```rust
fn main() {
    let names = vec![String::from("An"), String::from("Bình")];
    let first = names[0]; // ❌ không "lấy ra" khỏi Vec được
    println!("{first}");
}
```

**Trình biên dịch báo:**

```text
error[E0507]: cannot move out of index of `Vec<String>`
 --> src/main.rs:3:17
  |
3 |     let first = names[0]; // ❌ không "lấy ra" khỏi Vec được
  |                 ^^^^^^^^ move occurs because value has type `String`, which does not implement the `Copy` trait
  |
help: consider borrowing here
  |
3 |     let first = &names[0]; // ❌ không "lấy ra" khỏi Vec được
  |                 +
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let first = names[0].clone(); // ❌ không "lấy ra" khỏi Vec được
  |                         ++++++++
```

Compiler gợi ý hai cách: `&names[0]` (mượn) hoặc `names[0].clone()` (sao chép). Xem thêm [mục 7.3](#73-lấy-phần-tử-ra-khỏi-collection).

**E0308 khi nối hai `String` bằng `+`.**

```rust
fn main() {
    let a = String::from("Xin ");
    let b = String::from("chào");
    let c = a + b; // ❌ vế phải phải là &str
    println!("{c}");
}
```

**Trình biên dịch báo:**

```text
error[E0308]: mismatched types
 --> src/main.rs:4:17
  |
4 |     let c = a + b; // ❌ vế phải phải là &str
  |                 ^ expected `&str`, found `String`
  |
help: consider borrowing here
  |
4 |     let c = a + &b; // ❌ vế phải phải là &str
  |                 +
```

**E0599 khi gọi `push` trên array.** Lỗi này rất hay gặp lúc mới học, vì array và Vec trông giống nhau:

```rust
fn main() {
    let mut zeros = [0; 3];
    zeros.push(1); // ❌ mảng không có push
}
```

**Trình biên dịch báo:**

```text
error[E0599]: no method named `push` found for array `[{integer}; 3]` in the current scope
 --> src/main.rs:3:11
  |
3 |     zeros.push(1); // ❌ mảng không có push
  |           ^^^^ method not found in `[{integer}; 3]`
```

Cách sửa: dùng `vec![0; 3]` (xem [mục 3.7](#37-vec-array-và-slice)).

**E0369 khi cộng kết quả của `get`.**

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("An", 8);
    let next = scores.get("An") + 1; // ❌ get trả về Option, không phải số
    println!("{next}");
}
```

**Trình biên dịch báo:**

```text
error[E0369]: cannot add `{integer}` to `Option<&{integer}>`
 --> src/main.rs:6:33
  |
6 |     let next = scores.get("An") + 1; // ❌ get trả về Option, không phải số
  |                ---------------- ^ - {integer}
  |                |
  |                Option<&{integer}>
```

Cách sửa: lấy giá trị ra khỏi `Option` trước, ví dụ `scores.get("An").copied().unwrap_or(0) + 1`.

### 8.3 Cảnh báo thường gặp

```rust
// ⚠️ Biên dịch được nhưng có 3 cảnh báo
fn main() {
    let s = String::from("  xin chào  ");
    s.trim();
    s.to_uppercase();
    let mut v = vec![1, 2, 3];
    println!("{:?} {s}", v);
}
```

| Cảnh báo | Ý nghĩa | Cách xử lý |
|---|---|---|
| `unused return value of ... trim that must be used` | `trim()` trả về chuỗi mới, không sửa `s`; kết quả bị bỏ phí | Gán kết quả: `let t = s.trim();` |
| `unused return value of ... to_uppercase that must be used` | Tương tự với `to_uppercase()` | `let upper = s.to_uppercase();` |
| `variable does not need to be mutable` | `v` có `mut` nhưng không bao giờ bị sửa | Bỏ `mut` |

Các cảnh báo `must be used` thường là dấu hiệu bạn tưởng phương thức sửa chuỗi tại chỗ trong khi nó trả về một giá trị mới.

---

## 9. Bài tập

Hãy tự làm trước, chạy bằng `cargo run`, rồi mới mở phần đáp án. Mỗi bài có **kết quả mong đợi** để bạn tự kiểm tra. Các bài dùng `HashMap` đều in theo thứ tự sắp xếp để kết quả ổn định.

### Cấp độ 1. Làm quen

#### Bài 1.1. Thao tác cơ bản với Vec

**Đề:** cho `let mut v = vec![5, 3, 8];`. Lần lượt: thêm `1` vào cuối, chèn `0` vào đầu, bỏ phần tử cuối (`pop`), rồi sắp xếp tăng dần. In `v` sau **mỗi** bước.

**Kết quả mong đợi:**

```text
[5, 3, 8, 1]
[0, 5, 3, 8, 1]
[0, 5, 3, 8]
[0, 3, 5, 8]
```

**Gợi ý:** `push`, `insert`, `pop`, `sort`.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    let mut v = vec![5, 3, 8];

    v.push(1);
    println!("{:?}", v);

    v.insert(0, 0);
    println!("{:?}", v);

    v.pop();
    println!("{:?}", v);

    v.sort();
    println!("{:?}", v);
}
```

`pop()` trả về `Option` nhưng ở đây ta không cần giá trị bị bỏ, nên có thể bỏ qua kết quả.

</details>

#### Bài 1.2. Truy cập an toàn

**Đề:** cho `let colors = vec!["đỏ", "xanh", "vàng"];`. Dùng `get` để lấy phần tử ở vị trí `1` và vị trí `5`. Nếu có thì in `Màu thứ <i>: <màu>`, nếu không thì in `Không có màu thứ <i>`.

**Kết quả mong đợi:**

```text
Màu thứ 1: xanh
Không có màu thứ 5
```

**Gợi ý:** `get` trả về `Option<&&str>`; dùng `match`.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    let colors = vec!["đỏ", "xanh", "vàng"];

    match colors.get(1) {
        Some(color) => println!("Màu thứ 1: {color}"),
        None => println!("Không có màu thứ 1"),
    }

    match colors.get(5) {
        Some(color) => println!("Màu thứ 5: {color}"),
        None => println!("Không có màu thứ 5"),
    }
}
```

</details>

#### Bài 1.3. Byte và ký tự

**Đề:** với chuỗi `"Hà Nội"`, in số byte và số ký tự theo mẫu bên dưới.

**Kết quả mong đợi:**

```text
Hà Nội: 9 byte, 6 ký tự
```

**Gợi ý:** `len()` đếm byte; `chars().count()` đếm ký tự.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    let city = "Hà Nội";
    println!("{city}: {} byte, {} ký tự", city.len(), city.chars().count());
}
```

Giải thích: `à` chiếm 2 byte và `ộ` chiếm 3 byte, các ký tự còn lại 1 byte: `1 + 2 + 1 + 1 + 3 + 1 = 9` byte cho 6 ký tự.

</details>

### Cấp độ 2. Vận dụng

#### Bài 2.1. Sửa lỗi: tăng điểm cho mọi học sinh

**Đề:** đoạn mã sau không biên dịch được. Sửa để cộng 1 điểm cho mỗi phần tử.

```rust
fn main() {
    let mut scores = vec![8, 7, 9];
    for s in &scores {
        *s += 1; // ❌
    }
    println!("{:?}", scores);
}
```

**Kết quả mong đợi:**

```text
[9, 8, 10]
```

**Gợi ý:** đọc dòng `help:` trong thông báo lỗi.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    let mut scores = vec![8, 7, 9];
    for s in &mut scores {
        *s += 1;
    }
    println!("{:?}", scores);
}
```

Giải thích: `&scores` cho ra các tham chiếu **chỉ đọc**, không gán được qua `*s`. `&mut scores` cho ra tham chiếu sửa được.

</details>

#### Bài 2.2. Ghép họ và tên

**Đề:** cho `first = String::from("Nguyễn")` và `last = String::from("An")`. Ghép thành họ tên đầy đủ **mà không làm mất** `first` và `last`: sau đó vẫn in được cả hai.

**Kết quả mong đợi:**

```text
Nguyễn An
Họ: Nguyễn, Tên: An
```

**Gợi ý:** `a + &b` làm mất `a`. Có cách ghép mà không lấy đi biến nào.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    let first = String::from("Nguyễn");
    let last = String::from("An");

    let full = format!("{first} {last}");
    println!("{full}");
    println!("Họ: {first}, Tên: {last}");
}
```

`format!` chỉ mượn các biến, nên `first` và `last` vẫn dùng được.

</details>

#### Bài 2.3. Đảo ngược chuỗi

**Đề:** đảo ngược chuỗi `"Việt Nam"` theo **ký tự** (không phải byte) rồi in.

**Kết quả mong đợi:**

```text
maN tệiV
```

**Gợi ý:** `chars()`, `.rev()`, rồi `.collect()` vào `String`.

<details>
<summary>Xem đáp án</summary>

```rust
fn main() {
    let s = "Việt Nam";
    let reversed: String = s.chars().rev().collect();
    println!("{reversed}");
}
```

Giải thích: đảo theo byte sẽ làm hỏng các chữ nhiều byte như `ệ`. `chars()` tách đúng từng ký tự nên đảo an toàn.

</details>

#### Bài 2.4. Đếm số lần xuất hiện

**Đề:** cho `let fruits = ["táo", "cam", "táo", "lê", "cam", "táo"];`. Đếm số lần xuất hiện của mỗi loại quả bằng `HashMap`, in theo thứ tự chữ cái.

**Kết quả mong đợi:**

```text
cam: 2
lê: 1
táo: 3
```

**Gợi ý:** `entry(...).or_insert(0)`, rồi gom vào `Vec` và `sort()`.

<details>
<summary>Xem đáp án</summary>

```rust
use std::collections::HashMap;

fn main() {
    let fruits = ["táo", "cam", "táo", "lê", "cam", "táo"];

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for fruit in fruits {
        *counts.entry(fruit).or_insert(0) += 1;
    }

    let mut result: Vec<_> = counts.into_iter().collect();
    result.sort();
    for (fruit, count) in result {
        println!("{fruit}: {count}");
    }
}
```

</details>

#### Bài 2.5. Gom từ theo độ dài

**Đề:** cho `let words = ["rust", "là", "ngôn", "ngữ", "hay"];`. Gom các từ theo **số ký tự**, rồi in cả map. Dùng `BTreeMap` để các độ dài được sắp xếp tăng dần.

**Kết quả mong đợi:**

```text
{2: ["là"], 3: ["ngữ", "hay"], 4: ["rust", "ngôn"]}
```

**Gợi ý:** khoá là `usize` (kết quả của `chars().count()`), giá trị là `Vec<&str>`; dùng `entry(...).or_default().push(...)`.

<details>
<summary>Xem đáp án</summary>

```rust
use std::collections::BTreeMap;

fn main() {
    let words = ["rust", "là", "ngôn", "ngữ", "hay"];

    let mut groups: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
    for word in words {
        groups.entry(word.chars().count()).or_default().push(word);
    }

    println!("{:?}", groups);
}
```

Giải thích: dùng `chars().count()` chứ không phải `len()`, vì `len()` đếm byte (`"là"` có 3 byte nhưng chỉ 2 ký tự). Trong mỗi nhóm, các từ giữ nguyên thứ tự xuất hiện.

</details>

### Cấp độ 3. Thử thách

#### Bài 3.1. Phát hiện giá trị trùng

**Đề:** cho `let numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];`. Dùng `HashSet` để in ra (1) số giá trị khác nhau và (2) số lần gặp lại một giá trị đã có.

**Kết quả mong đợi:**

```text
Số giá trị khác nhau: 7
Số lần gặp lại giá trị đã có: 3
```

**Gợi ý:** `insert` trả về `bool`: `true` nếu là giá trị mới, `false` nếu đã có.

<details>
<summary>Xem đáp án</summary>

```rust
use std::collections::HashSet;

fn main() {
    let numbers = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3];

    let mut seen = HashSet::new();
    let mut repeats = 0;
    for n in numbers {
        if !seen.insert(n) {
            repeats += 1;
        }
    }

    println!("Số giá trị khác nhau: {}", seen.len());
    println!("Số lần gặp lại giá trị đã có: {repeats}");
}
```

Giải thích: các giá trị khác nhau là `1, 2, 3, 4, 5, 6, 9` (7 giá trị). Các lần gặp lại là số `1` lần thứ hai, `5` lần thứ hai và `3` lần thứ hai (3 lần).

</details>

#### Bài 3.2. Bốn đoạn mã, bốn lỗi

**Đề:** mỗi đoạn mã dưới đây đều không biên dịch được. Với từng đoạn, hãy **dự đoán lỗi** trước khi chạy `cargo check`, rồi sửa.

**Đoạn (a):**

```rust
fn main() {
    let a = String::from("Xin ");
    let b = String::from("chào");
    let c = a + b; // ❌
    println!("{c}");
}
```

**Đoạn (b):**

```rust
fn main() {
    let v = vec![1, 2, 3];
    let x: i32 = v.get(0); // ❌
    println!("{x}");
}
```

**Đoạn (c):**

```rust
fn main() {
    let mut scores = HashMap::new(); // ❌
    scores.insert("An", 8);
    println!("{:?}", scores.get("An"));
}
```

**Đoạn (d):**

```rust
fn main() {
    let names = vec![String::from("An")];
    for n in names {
        println!("{n}");
    }
    println!("{}", names.len()); // ❌
}
```

<details>
<summary>Xem đáp án</summary>

| Đoạn | Lỗi | Nguyên nhân | Cách sửa |
|---|---|---|---|
| (a) | `E0308` mismatched types | Vế phải của `+` phải là `&str`, đang truyền `String` | `a + &b` |
| (b) | `E0308` mismatched types | `get` trả về `Option<&i32>`, không phải `i32` | `let x = v[0];` hoặc `let x = *v.get(0).unwrap();`, hoặc xử lý `Option` bằng `match` |
| (c) | `E0433` cannot find type `HashMap` | Chưa `use` | Thêm `use std::collections::HashMap;` ở đầu tệp |
| (d) | `E0382` borrow of moved value | `for n in names` đã lấy sở hữu `names` | `for n in &names` |

</details>

#### Bài 3.3. Danh bạ điện thoại

**Đề:** dùng `HashMap<String, String>` (tên → số điện thoại). Thêm `An: 0901`, `Bình: 0902`, `Chi: 0903`. Cập nhật số của An thành `0911`. Xoá Chi và in giá trị vừa xoá (dạng `Option`). Cuối cùng in danh bạ theo thứ tự tên.

**Kết quả mong đợi:**

```text
Đã xoá Chi: Some("0903")
An: 0911
Bình: 0902
```

**Gợi ý:** `insert` cùng khoá sẽ ghi đè; `remove` trả về `Option`; sắp xếp khoá trước khi in.

<details>
<summary>Xem đáp án</summary>

```rust
use std::collections::HashMap;

fn main() {
    let mut phones: HashMap<String, String> = HashMap::new();
    phones.insert("An".to_string(), "0901".to_string());
    phones.insert("Bình".to_string(), "0902".to_string());
    phones.insert("Chi".to_string(), "0903".to_string());

    phones.insert("An".to_string(), "0911".to_string()); // cập nhật

    let removed = phones.remove("Chi");
    println!("Đã xoá Chi: {:?}", removed);

    let mut names: Vec<&String> = phones.keys().collect();
    names.sort();
    for name in names {
        println!("{name}: {}", phones[name]);
    }
}
```

Giải thích: `insert` lần hai với khoá `"An"` ghi đè số cũ. `remove("Chi")` nhận `&str` và trả về `Some("0903")`. Danh bạ còn hai người, in theo thứ tự chữ cái của tên.

</details>

---

## 10. Tóm tắt

| Chủ đề | Ghi nhớ |
|---|---|
| Collection | Chứa nhiều giá trị, dữ liệu trên **heap**, co giãn lúc chạy; array và tuple thì cố định |
| `Vec<T>` | Danh sách cùng kiểu: `push`, `pop`, `insert`, `remove`, `len`; cần `mut` để sửa |
| Truy cập `Vec` | `v[i]` **panic** khi sai chỉ số; `v.get(i)` trả về `Option<&T>`; chỉ số kiểu `usize` |
| Duyệt `Vec` | `&v` đọc, `&mut v` sửa (qua `*x`), `v` lấy sở hữu |
| Mượn và sửa | Không `push` được khi còn tham chiếu tới phần tử (`E0502`) |
| Tham số hàm | Nhận `&[T]` thay cho `&Vec<T>`, nhận `&str` thay cho `&String` |
| `String` | UTF-8, `len()` đếm **byte**; `chars().count()` đếm ký tự |
| Không index chuỗi | `s[0]` bị cấm (`E0277`); dùng `chars()`, `char_indices()`, `get(a..b)` |
| Nối chuỗi | `push`, `push_str`, `+=`; `a + &b` lấy mất `a`; `format!` không lấy gì |
| `HashMap<K, V>` | Khoá → giá trị; `use std::collections::HashMap;`; `insert`, `get`, `remove`, `entry` |
| `entry` | `*map.entry(k).or_insert(0) += 1` là mẫu đếm; `or_default()` cho kiểu có mặc định |
| Thứ tự `HashMap` | **Không xác định**, đổi giữa các lần chạy; cần ổn định thì sắp xếp hoặc dùng `BTreeMap` |
| Ownership | `insert` chuyển `String` vào map; muốn giữ dùng `.clone()` |
| Tập hợp khác | `HashSet` (không trùng), `BTreeMap` (có thứ tự theo khoá) |

---

## 11. Tài liệu tham khảo

- [The Rust Programming Language, 8.1 Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [The Rust Programming Language, 8.2 Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [The Rust Programming Language, 8.3 Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [Rust By Example, Vectors](https://doc.rust-lang.org/rust-by-example/std/vec.html)
- [Rust By Example, Strings](https://doc.rust-lang.org/rust-by-example/std/str.html)
- [Rust By Example, HashMap](https://doc.rust-lang.org/rust-by-example/std/hash.html)
- [Tài liệu thư viện chuẩn, `Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Tài liệu thư viện chuẩn, `String`](https://doc.rust-lang.org/std/string/struct.String.html)
- [Tài liệu thư viện chuẩn, `HashMap`](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [Tài liệu thư viện chuẩn, `std::collections` (so sánh và chọn collection)](https://doc.rust-lang.org/std/collections/index.html)
- [Tra cứu mã lỗi, ví dụ E0502](https://doc.rust-lang.org/error_codes/E0502.html)
