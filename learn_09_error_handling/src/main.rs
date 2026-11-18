use std::fmt;
use std::num::ParseIntError;

fn main() {
    // check_result();
    // check_if_let();
    // check_unwrap();
    // first_letter_upper();
    // print_enum_display();
    // check_display_from();
    // box_dyne_error();
    // box_dyne_error();
    // ex1();
    // ex2();
    // mutiple_result();
    ex();
}

fn print_check() {
    let arrays = ["42", "abc", "-7", "", " 5"];
    for text in arrays {
        let value: u32 = match text.trim().parse() {
            Ok(value) => value,
            Err(e) => {
                println!("Error is {:?}", e);
                continue;
            }
        };
        println!("Value is {:?}", value);
    }
}

fn check_result() {
    let a = 10.4;
    let b = 0.0;
    match divide(a, b) {
        Ok(value) => println!("Value is {:?}", value),
        Err(error) => println!("Error with {:?}", error),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err(String::from("Zezo Value"));
    }
    Ok(a / b)
}

fn check_if_let() -> u32 {
    let val = "42".parse::<u32>();
    if let Ok(value) = val {
        println!("Value is {value}");
    }

    let val1 = "42s".parse::<u32>();

    if let Err(error) = val1 {
        println!("Error is {error}");
    }

    100
}

fn check_unwrap() {
    let good: Result<u32, &str> = Ok(5);
    let bad: Result<u32, &str> = Err("Not ok");

    println!("{}, {}", good.is_ok(), bad.is_err());
    println!("{}", good.unwrap());
    println!("{}", good.expect("Panic! for now!"));

    println!("{}", bad.unwrap_or(0));
    println!(
        "{}",
        bad.unwrap_or_else(|message| {
            println!("Handle errpr with {}", message);
            check_if_let()
        })
    );

    println!("unwrap_or_default is {}", bad.unwrap_or_default());
    println!("Println value x2 {:?}", good.ok());
    println!("Println value x2 {:?}", good.map(|message| message * 2));
}

fn first_letter_upper_internal(text: &str) -> Option<char> {
    let first = text.chars().next()?;
    Some(first.to_ascii_uppercase())
}

fn first_letter_upper() {
    println!("{:?}", first_letter_upper_internal("rust"));
    println!("{:?}", first_letter_upper_internal(""))
}

fn main_result() -> Result<(), ParseIntError> {
    let n: u32 = "43".parse()?;
    Ok(())
}

fn parse_quantity(text: &str) -> Result<u32, String> {
    let n = text
        .trim()
        .parse::<u32>()
        .map_err(|value| value.to_string())?;
    Ok(n)
}

fn print_enum_display() {
    #[derive(Debug, Clone)]
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

    println!("{:?}", divide(20, 4));
    println!("{:?}", divide(20, 0));
    println!("{:?}", divide(i32::MIN, -1));

    if let Err(e) = divide(20, 0) {
        println!("Lỗi: {e}");
    }
}

pub fn check_display_from() {
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

    for (a, b) in [("20", "4"), ("20", "0"), ("hai", "4")] {
        match divide(a, b) {
            Ok(value) => println!("{a} / {b} = {value}"),
            Err(e) => println!("{a} / {b} -> lỗi: {e}"),
        }
    }

    println!("{:?}", divide("1", "0"));
}

pub fn box_dyne_error() {
    use std::error::Error;

    fn divide(a: &str, b: &str) -> Result<i32, Box<dyn Error>> {
        let a: i32 = a.trim().parse()?;
        let b: i32 = b.trim().parse()?;

        if b == 0 {
            return Err("không thể chia cho 0".into());
        }
        Ok(a / b)
    }

    for (a, b) in [("20", "4"), ("20", "0"), ("hai", "4")] {
        match divide(a, b) {
            Ok(value) => println!("{a} / {b} = {value}"),
            Err(e) => println!("{a} / {b} -> lỗi: {e}"),
        }
    }
}

pub fn ex1() {
    let inputs = ["5", " 12 ", "abc", "-3"];

    for text in inputs {
        match text.trim().parse::<u32>() {
            Ok(value) => println!("Ok with value {:?}", value),
            Err(message) => println!("Print with value {:?}", message),
        }
    }
}

pub fn ex2() {
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

    for line in [
        "20 / 4",
        "7 * 6",
        "9 - 12",
        "5 / 0",
        "5 +",
        "8 % 3",
        "two + 2",
        "9223372036854775807 + 1",
    ] {
        match calculate(line) {
            Ok(value) => println!("{line} = {value}"),
            Err(e) => println!("{line} -> lỗi: {e}"),
        }
    }
}

pub fn mutiple_result() {
    let good = ["1", "2", "3"];
    let bad = ["1", "x", "3"];

    let a: Result<Vec<u32>, _> = good.iter().map(|s| s.parse::<u32>()).collect();
    let b: Result<Vec<u32>, _> = bad.iter().map(|s| s.parse::<u32>()).collect();

    println!("{:?}", a);
    println!("{:?}", b);
}

fn ex() {
    for text in ["10", "-2", "3.5", " 4"] {
        match text.parse::<i32>() {
            Ok(n) => println!("{text:?} -> {n}"),
            Err(_) => println!("{text:?} -> lỗi"),
        }
    }

    fn safe_sqrt(x: f64) -> Result<f64, String> {
        if x < 0.0 {
            Err(String::from("không tính được căn bậc hai của số âm"))
        } else {
            Ok(x.sqrt())
        }
    }

    let inputs = ["3", "abc", "", "10"];

    for text in inputs {
        let quantity: u32 = text.trim().parse().unwrap_or(1212121);
        println!("Số lượng: {quantity}");
    }
}
