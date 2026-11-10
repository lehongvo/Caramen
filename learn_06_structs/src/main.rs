fn main() {
    // println!("Hello, world!");
    // print_data();

    // Show unit_struct;
    // unit_struct();

    // Show use_struct
    // use_struct();

    // Show say_user
    // say_user();

    // Show check_mul()
    // check_mul();

    // Show associated_fn()
    // associated_fn();

    // Show ex1
    // ex1();

    // Show ex2
    // ex2();

    // show
    // move_and_copy();

    // Show struct_in_struct
    // struct_in_struct();

    // check_ref
    check_ref();
}

struct Student {
    name: String,
    age: u8,
    is_active: bool,
}

fn print_data() {
    let student = Student {
        name: String::from("An"),
        age: 20,
        is_active: true,
    };
    println!(
        "{} - {} tuổi - đang học: {}",
        student.name, student.age, student.is_active
    );

    tuple_struct();
}

struct Color1(u8, u8, u8);

#[derive(Debug)]
struct Meters(f64);

fn tuple_struct() {
    let red = Color(255, 0, 0);
    let height = Meters(1.75);

    println!("Đỏ: {} {} {}", red.0, red.1, red.2);
    println!("Chiều cao: {:?} m", height);

    let Color(r, g, b) = red;
    println!("{r} {g} {b}");
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn describe(color: &Color) {
    println!("RGB: {} {} {}", color.0, color.1, color.2);
}

struct Marker;

fn unit_struct() {
    let _m = Marker;
    println!("Marker chiếm {} byte", size_of::<Marker>());
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn use_struct() {
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

#[derive(Debug)]
struct User1 {
    username: String,
    email: String,
    active: bool,
}

fn build_user(username: String, email: String, active: bool) -> User1 {
    User1 {
        username, // viết tắt của username: username
        email,    // viết tắt của email: email
        active,
    }
}

fn say_user() {
    let mut user: User1 = build_user(String::from("LEVO"), String::from("an@example.com"), false);
    println!("{:?}", user);
}
#[derive(Debug, Clone)]
struct User2 {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn update_user(user: &mut User2, username: String, email: String) -> User2 {
    user.username = username;
    user.email = email;
    user.clone()
}

fn say_user2() {
    let mut user1 = User2 {
        username: String::from("Vo Le"),
        email: String::from("index@gmail.com"),
        sign_in_count: 123,
        active: false,
    };

    let user2 = User2 {
        email: String::from("binh@example.com"),
        ..user1.clone()
    };
    let new_user_name = String::from("Vo");
    let new_email = String::from("index@gmail.com");

    println!("{:?}", update_user(&mut user1, new_user_name, new_email));
    println!("{:?}", user2);
}

fn say_user_01() {
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

    println!(
        "{} {} {} {}",
        user1.username, user1.email, user1.sign_in_count, user1.active
    );
    println!(
        "{} {} {} {}",
        user3.username, user3.email, user3.sign_in_count, user3.active
    );
}

fn check_mul() {
    struct Student {
        name: String,
        age: u8,
    }

    let mut student = Student {
        name: String::from("An"),
        age: 20,
    };

    student.age += 1;
    println!("{} có tuổi {}", student.name, student.age);

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 3, y: 4 };
    println!("{:?}", p);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // &self: chỉ đọc
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

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
        Rectangle {
            width: side,
            height: side,
        }
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

fn associated_fn() {
    let new_rectangle: Rectangle = Rectangle::new(12, 23);
    let new_rectangle1: Rectangle = Rectangle::new(12, 22);
    let new_rectangle2: Rectangle = Rectangle::new(12, 24);
    println!(
        "Can hold {:?}",
        Rectangle::can_hold(&new_rectangle, &new_rectangle1)
    );
    println!(
        "Can hold {:?}",
        Rectangle::can_hold(&new_rectangle, &new_rectangle2)
    );
}

fn ex1() {
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn new(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }

        fn area(&self) -> u32 {
            self.width * self.height
        }

        fn perimeter(&self) -> u32 {
            2 * (self.width * self.height)
        }

        fn can_hold(&self, other: &Rectangle) -> bool {
            self.height >= other.height && self.width >= other.width
        }
    }

    let rect = Rectangle::new(12, 20);
    let rect1 = Rectangle::new(12, 21);

    println!("area {:#?}", rect.area());
    println!("perimeter {:#?}", rect.perimeter());
    println!("can_hold {:#?}", rect.can_hold(&rect1));
}

fn ex2() {
    #[derive(Debug, Clone)]
    struct BankAccount {
        owner: String,
        balance: u64,
    }

    const DEFAULT_BALANCE: u64 = 1_000_000;
    const MAX_BALANCE: u64 = 1_000_000_000;
    const ZERO_BALANCE: u64 = 0;

    impl BankAccount {
        fn new(owner: &str) -> Self {
            Self {
                owner: String::from(owner),
                balance: DEFAULT_BALANCE,
            }
        }

        fn deposit(&mut self, amount: u64) {
            self.balance = (self.balance + amount).min(DEFAULT_BALANCE)
        }

        fn withdraw(&mut self, amount: u64) -> bool {
            if self.balance >= amount {
                self.balance -= amount;
                true;
            }
            false
        }

        fn summary(&self) -> String {
            format!("{}: {} đồng", self.owner, self.balance)
        }

        fn close(&mut self) -> bool {
            self.balance = 0;
            true
        }
    }

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

fn move_and_copy() {
    #[derive(Debug, Clone)]
    struct Student {
        name: String,
        age: u8,
    }

    let a = Student {
        name: String::from("An"),
        age: 20,
    };
    let b = a.clone();
    println!("{:?} {:?}", a, b);
}

fn struct_in_struct() {
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

    let point = Point { x: 3.0, y: 10.0 };
    let c = Circle {
        center: point,
        radius: 10.0,
    };
    println!("Tâm: ({:.1}, {:.1})", c.center.x, c.center.y);
    println!("Diện tích: {:.2}", c.area());
    println!("{:?}", c);
}

mod bank_tower;
use bank_tower::Account;

// fn check_mod() {
//     let owner = "VoLH";
//     let balance = 10 as u64;
//     let account = Account::new(owner, balance);
//     println!("Balance {}", account.balance())
// }

struct UserA<'all> {
    name: &'all str,
}

fn check_ref() {
    let user = UserA { name: "An" };
    println!("{}", user.name);
}
