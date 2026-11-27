fn main() {
    // check_loop_code();
    // check_loop_code_t();
    // check_generic_struct();
    // check_type_name();
    // check_life_time();
    // check_work();
    // check_struct_ref();
    // check_lifetime_impl();
    check_lifetime_static();
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

pub fn ex1() {}
