use std::fmt::format;

fn main() {
    // turn_around();
    // check_message();
    // check_impl();
    // check_option();
    // check_option_correct();
    // check_match_pattern();
    // check_plus_one();
    // check_roll();
    // check_age_group();
    // check_if_let();
    // check_while_let();
    // check_result();
    // ex1();
    // ex2();
    // ex3();
    // ex4();
    check_action();
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}
fn turn_around_check(value: Direction) -> Direction {
    match value {
        Direction::North => Direction::North,
        Direction::South => Direction::South,
        Direction::East => Direction::East,
        Direction::West => Direction::West,
    }
}

fn turn_around() {
    let heading = Direction::North;
    println!("Đang đi về hướng {:?}", heading);

    let back = turn_around_check(heading);
    println!("Quay đầu thì đi về hướng {:?}", back);
}

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

fn check_message() {
    let messsages = [
        Message::Quit,
        Message::Move { x: 1, y: 10 },
        Message::Write(String::from("Index")),
        Message::Color(2, 3, 4),
    ];
    for value in messsages {
        println!("Ok {:?}", describe(&value));
    }
}

fn check_impl() {
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

    let coins = [Coin::Penny, Coin::Nickel, Coin::Dime, Coin::Quarter];
    let mut total = 0;

    for coin in &coins {
        println!("{:?} có giá trị {} xu", coin, coin.cents());
        total += coin.cents();
    }
    println!("Tổng cộng: {total} xu");
}

fn check_option() {
    let some = Some(4);
    let none: Option<i32> = None;

    match some {
        Some(n) => println!("Value is {:}", n),
        None => println!("Not a value"),
    }

    println!("unwrap_or is {:?}", some.unwrap_or(0));
    println!("unwrap_or is {:?}", none.unwrap_or(0));

    println!("{} {}", some.is_some(), none.is_none());

    println!("Mapping data: {:?}", some.map(|n| n * 2));
    println!("Mapping data: {:?}", some.map(|n| n * 2));
}

fn check_option_correct() {
    let scores = [12, 2, 3, 4];
    println!("{:?}", scores.get(1));
    println!("{:?}", scores.get(10));
    println!("{:?}", scores.first());
    println!("{:?}", "rust".chars().next());
    println!("{:?}", 250u8.checked_add(1));
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: &Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("May mắn!");
            1
        }
        Coin::Nickel => {
            println!("ok 5");
            5
        }
        Coin::Dime => {
            println!("ok 10");
            10
        }
        Coin::Quarter => {
            println!("ok 25");
            25
        }
    }
}

fn check_match_pattern() {
    let coins = [Coin::Penny, Coin::Dime, Coin::Quarter, Coin::Nickel];
    let mut total = 10;
    for coin in coins {
        total += value_in_cents(&coin);
    }

    println!("Total is {:?}", total);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => x,
    }
}

fn check_plus_one() {
    let day = 10;
    let kind = match day {
        6 | 10 => "cuối tuần",
        _ => "ngày thường",
    };
    println!("Value {:?}", kind);

    println!("Value is {:?}", plus_one(Some(2)));
    println!("Value is {:?}", plus_one(None));
}

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

fn check_roll() {
    for roll in [2, 3, 4, 10] {
        match roll {
            2 => println!("Value is {}", 2),
            3 => println!("Value is {}", 3),
            4 => println!("Value is {}", 4),
            _ => println!("Not ok"),
        }
    }

    for n in [-5, 0, 2, 50, 100] {
        println!("{n}: {}", classify(n));
    }

    println!("{} {} {}", kind_of('R'), kind_of('7'), kind_of('#'));
    println!("=========");
    println!("{:?}", parity(Some(0)));
    println!("{:?}", parity(Some(1)));
    println!("{:?}", parity(Some(2)));
}

fn parity(x: Option<i32>) -> String {
    match x {
        Some(n) if n % 2 == 0 => format!("{n} là số chẵn"),
        Some(n) => format!("{n} là số lẻ"),
        None => String::from("không có số"),
    }
}

fn age_group(age: u32) -> String {
    match age {
        n @ 0..=12 => format!("{:?} age new", n),
        n @ 13..=19 => format!("{:?} age medium", n),
        n => format!("{:?} age old", n),
    }
}

fn describe_point(p: (i32, i32)) -> String {
    match p {
        (0, 0) => String::from("gốc tọa độ"),
        (x, 0) => format!("trên trục hoành, x = {x}"),
        (0, y) => format!("trên trục tung, y = {y}"),
        (x, y) if x == y => format!("trên đường chéo, x = y = {x}"),
        (x, y) => format!("điểm ({x}, {y})"),
    }
}

fn check_age_group() {
    let ages: [u32; 3] = [1, 18, 29];
    for age in ages {
        println!("!!!!! is {:?}", age_group(age));
    }

    for p in [(0, 0), (3, 0), (0, -2), (4, 4), (1, 2)] {
        println!("{:?}: {}", p, describe_point(p));
    }

    #[derive(Debug, Clone)]
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

    println!("{}", where_is(&Point { x: 0, y: 0, z: 0 }));
    println!("{}", where_is(&Point { x: 5, y: 0, z: 0 }));
    println!("{}", where_is(&Point { x: 2, y: 8, z: 9 }));

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

    println!("{}", describe(Some(Coin::Quarter)));
    println!("{}", describe(Some(Coin::Penny)));
    println!("{}", describe(None));
}

fn check_if_let() {
    let max: Option<u8> = Some(3);
    let none: Option<u8> = None;
    match max {
        Some(max) => println!("Max is {:?}", max),
        _ => println!("No value"),
    }

    if let Some(x) = max {
        println!("Max is {:?}", x)
    }
    if let Some(x) = none {
        println!("Max111 is {:?}", x)
    } else {
        print!("Is ok");
    }
    println!("describe_age is {:?}", describe_age(Some(32)));
    println!("describe_age is {:?}", describe_age(None));
}

fn describe_age(input: Option<u32>) -> String {
    let Some(age) = input else {
        return String::from("không rõ tuổi");
    };

    // từ đây trở đi, age là u32 bình thường
    format!("{age} tuổi")
}

fn check_while_let() {
    let mut numbers = [10, 20, 30].into_iter();
    while let Some(number) = numbers.next() {
        println!("Value is : {:?}", number);
    }
    println!("Out");

    let c = 'e';
    let is_vowel = matches!(c, 'a' | 'e' | 'i' | 'o' | 'u');
    let x = Some(7);
    let is_big = matches!(x, Some(value) if value > 10);
    println!("{is_vowel} {is_big}");
}

fn check_result() {
    for text in ["42", "12", "abcd"] {
        match text.parse::<i32>() {
            Ok(n) => println!("OK with value {:?}", n),
            Err(e) => println!("Not Ok with error {:?}", e),
        }
    }
    println!("========================");
    let input = ["12", "bac", "7"];
    let mut total = 0;
    for text in input {
        let Ok(n) = text.parse::<u32>() else {
            println!("Bỏ qua {text:?}: không phải số");
            continue;
        };
        total += n;
    }
    println!("Total is {:?}", total);
}

fn ex1() {
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
            !matches!(self, OrderStatus::Cancelled(_))
        }
    }

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

fn ex2() {
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

fn ex3() {
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
            (State::HasMoney(money), Event::Select { price }) if money >= price => {
                State::Dispensing
            }
            (State::HasMoney(_), Event::Cancel) => State::Idle,
            (State::Dispensing, Event::Take) => State::Idle,
            (current, _) => current,
        }
    }

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

use std::mem::size_of;

fn ex4() {
    println!("bool: {} byte", size_of::<bool>());
    println!("Option<bool>: {} byte", size_of::<Option<bool>>());
    println!("u32: {} byte", size_of::<u32>());
    println!("Option<u32>: {} byte", size_of::<Option<u32>>());
    println!("&u8: {} byte", size_of::<&u8>());
    println!("Option<&u8>: {} byte", size_of::<Option<&u8>>());

    #[derive(Debug, Clone, Copy)]
    enum Level {
        Low,
        Mid,
        High = 10,
    }

    let levels = [Level::Low, Level::Mid, Level::High];
    for item in levels {
        println!("===> {:?}:{:?}", item, item as i32);
    }

    let n: u8 = 3;

    let name = match n {
        1 => "một",
        2 => "hai",
        _ => "No",
    };

    println!("{name}");
}

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

use std::f64::consts::PI;
fn check_action() {
    enum Status {
        Active,
        Inactive,
        Banned,
    }

    fn label(status: Status) -> &'static str {
        match status {
            Status::Active => "hoạt động",
            Status::Inactive => "tạm ngưng",
            _ => "non",
        }
    }

    fn main() {
        println!("{}", label(Status::Active));
        println!("{}", label(Status::Inactive));
        println!("{}", label(Status::Banned));
    }
}
