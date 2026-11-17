fn main() {
    // check_result();
    // check_if_let();
    check_unwrap();
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
