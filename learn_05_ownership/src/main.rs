fn main() {
    // Check check_drop
    //     check_drop();

    // Check check_string
    //     check_string();

    // Show say_ok
    // say_ok();

    // Show check_move_by_can_use
    check_move_by_can_use();
}

fn check_move_by_can_use() {
    let mut s = String::from("old");
    let t = s; // s bị move sang t
    s = String::from("new"); // gán giá trị mới: s hợp lệ trở lại

    println!("{s} {t}");
}

struct Noisy(u32);

impl Drop for Noisy {
    fn drop(&mut self) {
        println!("drop {}", self.0);
    }
}

fn say_ok() {
    let _a = Noisy(1);
    let _b = Noisy(2);

    {
        let _c = Noisy(3);
        println!("trong khối");
    }

    consume(Noisy(4));

    let e = Noisy(5);
    drop(e); // drop sớm, không cần đợi hết phạm vi
    println!("e đã bị drop sớm");

    println!("cuối main");
} // _b rồi _a bị drop ở đây

fn consume(n: Noisy) {
    println!("consume {}", n.0);
}

fn check_drop() {
    {
        let s = String::from("hello"); // s bắt đầu có hiệu lực
        println!("{s}");
    } // s ra khỏi phạm vi: Rust tự giải phóng bộ nhớ của s

    println!("đã ra khỏi khối");
}

fn check_string() {
    // let a = 5;
    // let s = String::from("hello");

    // println!("a: {} byte (trên stack)", size_of::<i32>());
    // println!(
    //     "s: {} byte (trên stack: con trỏ + len + capacity)",
    //     size_of::<String>()
    // );
    // println!("chữ \"hello\": {} byte (trên heap)", s.len());

    // println!("địa chỉ của a    : {:p}", &a);
    // println!("địa chỉ của s    : {:p}", &s);
    // println!("địa chỉ chữ hello: {:p}", s.as_ptr());

    // let s2 = s;
    // println!("sau khi move, chữ hello vẫn ở: {:p}", s2.as_ptr());

    check_move_copy_clone();
}

fn check_move_copy_clone() {
    // let s1 = String::from("hello");
    // let s2 = s1; // quyền sở hữu chuyển từ s1 sang s2 (move)

    // println!("{s2}");

    // let s3: String = String::from("hello");
    // let s4 = s3.clone(); // chép cả dữ liệu trên heap

    // println!("s1 = {s3}, s2 = {s4}");

    // Show check_ownership_funciton
    check_ownership_funciton();

    // Show change
}

fn check_ownership_funciton() {
    let s = String::from("hello");

    takes_ownership(&s); // s bị move vào hàm, từ đây s không dùng được nữa
    println!("Value {s}");

    let x = 5;
    makes_copy(x); // i32 là Copy: x vẫn dùng được
    println!("x = {x}");

    calculate_length(&s);
    println!("S = {s}");

    let mut s = String::from("hello");
    change(&mut s);
    println!("Value is {:?}", s);
}

fn takes_ownership(some_string: &String) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

fn calculate_length(s: &String) -> (&String, usize) {
    let value_length = s.len();
    (s, value_length)
}

fn change(some_string: &mut String) -> &mut String {
    check_borrow();
    some_string.push_str(", world");
    some_string
}

fn check_borrow() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" a");
    } // r1 ra khỏi phạm vi, kết thúc lần mượn thứ nhất

    let r2 = &mut s; // OK
    r2.push_str(" b");

    println!("{s}");

    check_slice();
}

fn make_word(text: &str) -> String {
    let upper = text.to_uppercase();
    upper
}

fn check_slice() {
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    let value = s.clone().split_off(1);
    println!("{hello}|{world}|{value}");
    check_string_str(&s);
    s.clear();
}

fn check_string_str(s: &str) -> &str {
    check_slide_array();
    let bytes = s.as_bytes(); // chuỗi dưới dạng dãy byte

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; // gặp dấu cách đầu tiên: trả về đoạn phía trước
        }
    }

    &s[..] // không có dấu cách: cả chuỗi là một từ
}

fn check_slide_array() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..=3]; // kiểu &[i32]
    println!("===> {}", slice.len());
    println!("{slice:?}");
    println!("{}", sum(slice));
}

fn sum(nums: &[i32]) -> i32 {
    let mut note = String::from("ACX");
    print_note(&note);
    add_signature(&mut note, "VoLH");
    print_note(&note);
    nums.iter().sum()
}

fn print_note(text: &String) {
    // println!("===>01");
    // println!("Ghi chú: {text}");
    // println!("===>01");
}

fn add_signature(text: &mut String, author: &str) {
    text.push_str(" - ");
    text.push_str(author);

    let scores = [8, 7, 9, 10, 5, 6];

    let first_three = &scores[..3];
    let last_three = &scores[3..];
    println!("==============================>");
    println!("3 bài đầu: {first_three:?}, tổng {}", sum_new(first_three));
    println!("3 bài sau: {last_three:?}, tổng {}", sum_new(last_three));
    println!(
        "Cả kỳ: tổng {}, cao nhất {}",
        sum_new(&scores),
        max(&scores)
    );
}

fn sum_new(nums: &[i32]) -> i32 {
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
