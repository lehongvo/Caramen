use std::any::type_name_of_val;
fn main() {
    // let age = 20;
    // println!("age: {}", age);
    // let height = 1.75122;
    // println!("height: {:.1}", height);
    // let name = "John";
    // println!("name: {}", name);
    // let is_student = true;
    // println!("is_student: {}", is_student);
    // let is_student = false;
    // println!("is_student: {}", is_student);
    // let is_student = true;
    // println!("is_student: {:?}", is_student);

    // Show action number
    // action_number();

    // Show shadowing variable
    // shadowing();

    // Show const_value
    // const_value();

    // Show bool_value
    // bool_value();

    // Show tuple_value
    // tuple_value();

    // Show as
    // as_value();

    // Show lab_1
    // lab_1();

    // // lab_2
    // lab_2();

    // let discount: u32;
    // let is_member = true;

    // if is_member {
    //     discount = 10;
    // } else {
    //     discount = 0;
    // }
    // println!("Log {}", discount);

    // let (x, y) = (3, 4);
    // let [first, _, third] = [10, 20, 30];
    // let (name, _) = ("An", 20);

    // println!("{x} {y} {first} {third} {name}");

    // let x: u8 = 255;
    // println!("===========================");
    // println!("{}", x.wrapping_add(1));
    // println!("{:?}", x.checked_add(1));
    // println!("{}", x.saturating_add(1));
    // println!("{:?}", x.overflowing_add(1));

    // Show ex1
    ex1();
}

pub const SECONDS_IN_MINUTE: u32 = 60;
pub static PI: f64 = 3.14159265358979323846;

fn action_number() {
    let mut x = 5;
    println!("x: {}", x);
    x = 6;
    println!("x: {}", x);
    x += 1;
    println!("x: {}", x);

    println!("SECONDS_IN_MINUTE: {}", SECONDS_IN_MINUTE);
    println!("PI: {}", PI + 1.0);
}

fn shadowing() {
    let x = 5;
    println!("x: {}", x);

    {
        let x = 10;
        println!("x: {}", x);
    }

    println!("x: {}", x);

    let spaces = "  ";
    println!("spaces: {}", spaces);
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    let mul_value01: (u64, i32, f32) = (3, 4, 5.0);
    println!("mul_value01 is {:?}", mul_value01);

    let mul_value02: [i32; 4] = [1, 3, 4, 4];
    println!("mul_vmul_value02alue01 is {:?}", mul_value02);
    println!("Type {}", type_name_of_val(&mul_value02));
}

pub fn const_value() {
    println!("i8: {}....{}", i8::MIN, i8::MAX);
    println!("u8:  {} .. {}", u8::MIN, u8::MAX);
    println!("i32: {} .. {}", i32::MIN, i32::MAX);
    println!("u32: {} .. {}", u32::MIN, u32::MAX);
    println!("i64: {} .. {}", i64::MIN, i64::MAX);
    println!("u64: {} .. {}", u64::MIN, u64::MAX);

    println!("Demo: {}", 1_22);

    let a = 2;
    let b = 222.343434343333333;
    println!("a/b = {:?}", b / a as f64);
    println!("a/b = {:?}", 9.0 / 5.0);
    println!("a/b = {:?}", 9 / 5);
}

pub fn bool_value() {
    let is_learning = false;
    let is_finished: bool = true;

    let letter = 'z';
    let match_letter: char = 'Z';
    let emoji = '😻';
    println!("{letter} : {match_letter} : {emoji}");
    println!(
        "bool: {} byte, char: {} byte",
        size_of::<bool>(),
        size_of::<char>()
    );
}

pub fn tuple_value() {
    let person = ("An", 10, 1.2343343);
    println!("Log: {:?}", person);
    let (name, age, value) = person;
    println!("Name is {:?}, age is {:?}, value is {:?}", name, age, value);
}

pub fn array_value() {
    let scopes = ["admin", "cs", "user"];
    println!("Log : {:?}", scopes.len());
    let value = [8, 7, 9, 10, 6];
    println!("Log : {:?}", value.len());
    let mut zeros = vec![0; 3];
    println!("Log : {:?}", zeros);
    for x in value {
        zeros.push(x);
    }
    println!("Log : {:?}", zeros);
}

pub fn as_value() {
    let total: i32 = 7_i32;
    let count: i32 = 2_i32;
    let average = total as f64 / count as f64;
    println!("log : {:?}", average);

    println!("{}", 3.99_f64 as i32);
    println!("{}", 300_i32 as u8);
    println!("{}", -1_i32 as u32);
    println!("{}", 'A' as u8);
    println!("{}", 97_u8 as char);
}

pub fn lab_1() {
    let item = "Bút bi";
    let unit_price = 5_000;
    let mut quantity = 3;
    quantity += 2;

    let subtotal = unit_price * quantity;
    println!("{item}: {quantity} x {unit_price} = {subtotal} đồng");
}

pub const VAT_PERCENT: u32 = 8;
pub fn lab_2() {
    let item = "Bút bi";
    let unit_price: u32 = 5_000_u32;

    // Giả lập chuỗi người dùng nhập, giống kết quả của read_line
    let quantity = "  5\n";
    let quantity = quantity.trim(); // vẫn là &str, đã bỏ khoảng trắng và '\n'
    let quantity: u32 = quantity.parse().ok().unwrap_or(10);

    let subtotal = unit_price * quantity;
    let vat = subtotal * VAT_PERCENT / 100;
    let total = subtotal + vat;

    println!("{item}: {quantity} x {unit_price} = {subtotal} đồng");
    println!("VAT {VAT_PERCENT}%: {vat} đồng");
    println!("Tổng cộng: {total} đồng");
}

const SECONDS_PER_DAY: u32 = 24 * 60 * 60;
const SECONDS_PER_WEEK: u32 = SECONDS_PER_DAY * 7;

pub fn ex1() {
    //     let mut score = 70;
    //     println!("Điểm ban đầu: {score}");
    //     score = 85; // ❌ gán lại biến bất biến
    //     println!("Điểm sau khi thi lại: {score}");

    //     println!("Một ngày có {SECONDS_PER_DAY} giây");
    //     println!("Một tuần có {SECONDS_PER_WEEK} giây");
    let number = "  42\n";
}
