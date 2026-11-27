fn main() {
    // check_loop_code();
    // check_loop_code_t();
    // check_generic_struct();
    // check_type_name();
    // check_life_time();
    // check_work();
    // check_struct_ref();
    // check_lifetime_impl();
    // check_lifetime_static();
    // ex2();
    ex3();
}

fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn check_loop_code() {
    println!("{}", largest_i32(&[34, 50, 25, 100, 65]));
    println!("{}", largest_char(&['y', 'm', 'a', 'q']));
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {}
    }
    largest
}

fn check_loop_code_t() {
    let numbers = [34, 50, 25, 100, 65];
    let letters = ['y', 'm', 'a', 'q'];
    let words = ["cam", "bưởi11", "aa"];

    println!("Số lớn nhất: {}", largest(&numbers));
    println!("Ký tự lớn nhất: {}", largest(&letters));
    println!("Chuỗi lớn nhất: {}", largest(&words));
}

#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn check_generic_struct() {
    let mixed = Point { x: 5, y: 4.0 };
    let text = Point { x: "một", y: 'z' };

    println!("x = {}, y = {:?}", mixed.x, mixed.y);
    println!("{:?}", text);
}

fn check_type_name() {
    fn describe<T>(_value: T) -> &'static str {
        std::any::type_name::<T>()
    }

    println!("{}", describe(5));
    println!("{}", describe(2.5));
    println!("{}", describe("chữ"));
}

trait Summary {
    fn summarize(&self) -> String;
}

struct Article {
    title: String,
    author: String,
}
struct Tweet {
    username: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, bởi {}", self.title, self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

pub fn check_trait() {
    let article = Article {
        title: String::from("Học Rust"),
        author: String::from("An"),
    };
    let tweet = Tweet {
        username: String::from("rustlang"),
        content: String::from("Rust 2024 ra mắt!"),
    };
    println!("{}", article.summarize());
    println!("{}", tweet.summarize());
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

pub fn check_life_time_demo() {
    let string1 = String::from("một chuỗi dài");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str()); // ❌ string2 sống ngắn hơn result
        println!("Dài nhất: {result}");
    }
}

fn first<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

fn greeting<'a>() -> &'a str {
    "Helloo"
}

fn check_life_time() {
    let a = String::from("một");
    let result;
    {
        let b = String::from("hai");
        result = first(a.as_str(), b.as_str());
    }
    println!("{result}");

    println!("{}", greeting());
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(value) => &s[..value],
        None => s,
    }
}

fn check_work() {
    let sentence = String::from("hello wide world");
    println!("{}", first_word(&sentence));
}

fn check_struct_ref() {
    struct Excerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("Gọi tôi là Ishmael. Nhiều năm trước...");
    let first_sentence = novel.split(".").next().unwrap();
    let excerpt = Excerpt {
        part: first_sentence,
    };
    println!("{}", excerpt.part);
}

fn check_lifetime_impl() {
    struct Excerpt<'a> {
        part: &'a str,
    }

    impl<'a> Excerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        fn announce_and_return(&self, announcement: &str) -> &str {
            println!("Chú ý! {announcement}");
            self.part
        }
    }

    let text = String::from("Một. Hai. Ba.");
    let excerpt = Excerpt {
        part: text.as_str(),
    };

    println!("level = {}", excerpt.level());
    println!("{}", excerpt.announce_and_return("sắp in"));
}

pub fn check_lifetime_static() {
    fn language() -> &'static str {
        "Rust"
    }

    let s: &'static str = "Chuỗi này sống suốt chương trình";
    println!("{s}");
    println!("{}", language());
}

pub fn ex1() {
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    #[derive(Debug, PartialEq, PartialOrd)]
    struct Version {
        major: u32,
        minor: u32,
    }

    let numbers = [34, 50, 25, 100, 65];
    let temperatures = [36.6, 39.2, 37.0];
    let letters = ['y', 'm', 'a', 'q'];
    let versions = [
        Version { major: 1, minor: 9 },
        Version { major: 2, minor: 0 },
        Version {
            major: 1,
            minor: 12,
        },
    ];

    println!("Số lớn nhất: {}", largest(&numbers));
    println!("Nhiệt độ cao nhất: {}", largest(&temperatures));
    println!("Ký tự lớn nhất: {}", largest(&letters));

    let newest = largest(&versions);
    println!("Phiên bản mới nhất: {}.{}", newest.major, newest.minor);
}

fn ex2() {
    use std::f64::consts::PI;

    trait Shape {
        fn name(&self) -> String;
        fn area(&self) -> f64;

        // Phương thức có sẵn: dùng lại name() và area() của từng hình
        fn describe(&self) -> String {
            format!("{} có diện tích {:.2}", self.name(), self.area())
        }
    }

    struct Circle {
        radius: f64,
    }
    struct Rectangle {
        width: f64,
        height: f64,
    }
    struct Triangle {
        base: f64,
        height: f64,
    }
    impl Shape for Circle {
        fn name(&self) -> String {
            String::from("Hình tròn")
        }
        fn area(&self) -> f64 {
            PI * self.radius * self.radius
        }
    }

    impl Shape for Rectangle {
        fn name(&self) -> String {
            String::from("Hình chữ nhật")
        }
        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    impl Shape for Triangle {
        fn name(&self) -> String {
            String::from("Tam giác")
        }
        fn area(&self) -> f64 {
            0.5 * self.base * self.height
        }
    }

    fn print_info<T: Shape>(shape: &T) {
        println!("{}", shape.describe());
    }

    // Bước 1
    print_info(&Circle { radius: 2.0 });
    print_info(&Rectangle {
        width: 3.0,
        height: 4.5,
    });

    // Bước 2: một danh sách chứa nhiều kiểu hình khác nhau (gọi động)
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle {
            width: 3.0,
            height: 4.5,
        }),
        Box::new(Triangle {
            base: 6.0,
            height: 2.5,
        }),
    ];

    let mut total = 0.0;
    for shape in &shapes {
        total += shape.area();
    }
    println!("Tổng diện tích {} hình: {:.2}", shapes.len(), total);
}

pub fn ex3() {
    struct TextStats<'a> {
        text: &'a str,
    }

    impl<'a> TextStats<'a> {
        fn new(text: &'a str) -> Self {
            TextStats { text }
        }

        fn word_count(&self) -> usize {
            self.text.split_whitespace().count()
        }

        fn longest_word(&self) -> &'a str {
            let mut longest = self.text.split(' ').next().unwrap();
            for word in self.text.split_whitespace() {
                if word.len() <= longest.len() {
                    longest = word;
                }
            }
            longest
        }
    }

    let text = String::from("ownership makes Rust safe and fast");
    let stats = TextStats::new(&text);

    println!("Số từ: {}", stats.word_count());
    println!("Từ dài nhất: {}", stats.longest_word());
}
