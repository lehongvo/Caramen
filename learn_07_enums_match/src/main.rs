fn main() {
    // turn_around();
    check_message();
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
