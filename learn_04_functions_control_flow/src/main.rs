fn main() {
    println!("Bắt đầu");
    say_hello();
    say_hello();
    print_total(10, 10);

    println!("Value {:?}", add(1, 2));
    println!("Value {:?} and {}", square(23), plus_one(2));

    check_one();

    println!("Value {:?}", sign(1));
    println!("Value {:?}", sign(-11));
    println!("Value {:?}", sign(0));

    println!("area {:?}", area(12, 2));

    div(11);
    div(10);

    point(10 as f64);
    point(8 as f64);
    point(7 as f64);

    result(2);
    result(6);

    check_loop(1, 1);

    let mut counter = 0;
    let first = loop {
        counter += 1;
        if counter % 5 == 0 && counter % 7 == 0 {
            break counter; // trả về counter làm giá trị của cả vòng loop
        }
    };
    println!("Value is {first}");

    check_while(1, 10);

    println!("Check for {:?}", check_for(1, 10));

    println!("================");

    check_iter_enumerate();
}

fn say_hello() {
    println!("Xin chào từ một hàm khác!");
}

fn print_total(quantity: u32, unit_price: u32) {
    let total = quantity * unit_price;
    println!("{quantity} x {unit_price} = {total} đồng");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn square(x: i32) -> i32 {
    x * x
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn check_one() {
    let y = {
        let mut x: u64 = 3;
        x = x + 1;
        x + 2
    };

    println!("y = {:?}", y);
}

fn sign(n: i32) -> i32 {
    if n < 0 {
        return -1;
    }
    if n == 0 {
        return 0;
    }
    1 // biểu thức cuối: giá trị trả về khi không gặp return nào
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn div(number: u64) {
    if number % 2 == 0 {
        println!("{number} là số chẵn");
    } else {
        println!("{number} là số lẻ");
    }
}

fn point(score: f64) {
    if score >= 9.0 {
        println!("Xuất sắc");
    } else if score >= 8.0 {
        println!("Giỏi");
    } else if score >= 6.5 {
        println!("Khá");
    } else {
        println!("Trung bình trở xuống");
    }
}

fn result(score: u64) {
    let result = if score >= 5 { "đậu" } else { "rớt" };
    println!("Kết quả thi: {result}");

    let a = 12;
    let b = 30;
    let larger = if a > b { a } else { b };
    println!("Số lớn hơn: {larger}");
}

fn check_loop(mut count: u64, max: u64) {
    loop {
        println!("number is {count}");
        count += 1;
        if count >= max {
            break;
        }
    }
}

fn check_while(mut count: u64, max: u64) {
    while count < max {
        println!("Check ok with {count}");
        count += 1;
    }
}

fn check_for(value: u64, lenth: u32) -> Vec<u64> {
    let scores = vec![8, 7, 9, 10, 6];
    for score in scores {
        println!("Score {score}");
    }

    for i in 1..4 {
        println!("{i} ");
    }

    for i in 1..=4 {
        println!("{i} ");
    }

    for i in (1..=4).rev() {
        println!("{i} ");
    }

    for i in (0..10).step_by(3) {
        print!("{i} ");
    }

    for _ in 0..3 {
        print!("*");
    }

    let mut list = Vec::new();
    for i in 1..lenth {
        list.push(value * i as u64);
    }
    list
}

fn check_iter_enumerate() {
    let names = vec!["An", "Bình", "Chi"];

    for name in names.iter() {
        println!("Xin chào {name}");
    }

    for name in names.iter().enumerate() {
        println!("Xin chào {:?}", name);
    }

    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // bỏ qua số chẵn, sang lượt kế
        }
        if i > 7 {
            break; // dừng hẳn khi i > 7
        }
        println!("{i}");
    }

    'acc: for a in 1..10 {
        for b in 1..10 {
            if a + b == 10 && a * b == 21 {
                println!("Tìm thấy: {a} + {b} = 10 và {a} x {b} = 21");
                break 'acc; // thoát cả hai vòng for
            }
        }
    }

    multiplication_table(3);

    let (quotient, remainder) = div_rem(17, 5);
    println!("17 = 5 x {quotient} + {remainder}");
}

fn multiplication_table(n: u32) {
    for i in 1..=10 {
        println!("{n} x {i} = {}", n * i);
    }
}

fn div_rem(a: u32, b: u32) -> (u32, u32) {
    describe(3);
    (a / b, a % b)
}

fn describe(n: i32) {
    factorial(10);

    let half = if n % 2 == 0 {
        n / 2
    } else {
        println!("{n} là số lẻ, bỏ qua");
        return; // kiểu !, nên nhánh else vẫn hợp lệ dù không cho ra i32
    };

    println!("Một nửa của {n} là {half}");
}

fn factorial(n: u64) -> u64 {
    if n <= 1 {
        println!("ok");
        1
    } else {
        println!("not ok");
        n * factorial(n - 1)
    }
}
