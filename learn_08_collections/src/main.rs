use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, hash_map};
fn main() {
    // Check data();
    // data();
    // check_push_data();
    // check_mut();
    // vec_function();
    // check_hash_map();
    // check_hash_map_delete();
    // check_hash_map_entry();
    // check_hash_map_update();
    // check_hash_map_update_ownership();
    // check_bTreeMap();
    // ex1();
    // ex2();
    vec_and_string();
}

fn data() {
    let mut empty: Vec<i32> = Vec::new();

    let numbers = vec![1, 2, 3];
    let zeros = vec![0; 5];
    let names = vec!["An", "Bình"];
    println!("{:?} {:?} {:?} {:?}", empty, numbers, zeros, names);
}

fn check_push_data() {
    let mut v = Vec::new();
    v.push(10); // từ đây compiler biết v là Vec<i32>
    println!("{:?}", v);
}

fn check_mut() {
    let mut v = vec![1, 2, 3];
    v.push(4); // thêm vào cuối
    v.insert(0, 0); // chèn vào vị trí 0
    println!("{:?}", v);

    let last = v.pop(); // bỏ phần tử cuối, trả về Option
    println!(
        "{:?} {:?}",
        match last {
            Some(value) => value,
            _ => 0,
        },
        v
    );

    let removed = v.remove(1); // bỏ phần tử ở vị trí 1, trả về chính phần tử đó
    println!("{removed} {:?}", v);

    let removed = v.remove(1); // bỏ phần tử ở vị trí 1, trả về chính phần tử đó
    println!("{removed} {:?}", v);
    println!("len = {}, rỗng? {}", v.len(), v.is_empty());

    let mut empty: Vec<i32> = Vec::new();
    println!("{:?}", empty.pop());

    println!("Value : {:?}", v);

    let v = vec![10, 20, 30];
    let second = v[1];
    println!("v[1] = {second}");
    println!(
        "{:?}",
        match v.get(1) {
            Some(x) => x,
            _ => &0,
        }
    );
    println!("{:?}", v.get(10));
}

fn vec_function() {
    let mut scores = vec![8, 7, 9];

    for s in &scores {
        println!("điểm: {s}");
    }

    for s in &mut scores {
        *s += 1;
    }

    println!("{:?}", scores);

    for value in scores.iter().enumerate() {
        println!("{:?}", value);
    }

    println!("{:?}", scores);

    let mut v = vec![1, 2, 3];
    let first = &v[0];
    println!("Phần tử đầu: {first}");
    v.push(4);

    let array = [1, 2, 3];
    let vector = vec![4, 5, 6];

    fn sum(numbers: &Vec<i32>) -> i32 {
        let mut total = 0;
        for n in numbers {
            total += n;
        }
        total
    }

    println!("{}", sum(&array.to_vec()));
    println!("{}", sum(&vector));

    let mut v = vec![30, 10, 20];

    v.sort(); // sắp xếp tăng dần
    println!("{:?}", v);
    println!("{}", v.contains(&20)); // có chứa 20 không

    v.reverse(); // đảo ngược
    println!("{:?}", v);

    v.extend([1, 2]); // nối thêm nhiều phần tử
    println!("{:?}", v);

    println!("{:?} {:?}", v.first(), v.last());

    enum Cell {
        Int(i32),
        Text(String),
    }

    let row = vec![Cell::Int(3), Cell::Text(String::from("hello"))];

    for cell in &row {
        match cell {
            Cell::Int(n) => println!("số: {n}"),
            Cell::Text(s) => println!("chuỗi: {s}"),
        }
    }

    let n = "42";
    let text = n.to_string();
    let back: i32 = text.parse().expect("không phải số");

    println!("{text} {}", back + 1);
}

pub fn check_hash_map() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("An"), 8);
    scores.insert(String::from("Bình"), 9);

    println!("Số cặp: {}", scores.len());
    println!("{:?}", scores.get("An"));
    println!("{:?}", scores.get("Chi"));
    println!("{}", scores.contains_key("Bình"));

    let mut scores = HashMap::new();
    scores.insert("An", 8);
    let an = scores.get("An").copied().unwrap_or(0);
    let chi = scores.get("Ana").copied().unwrap_or(0);
    println!("An: {an}, Chi: {chi}");

    match scores.get("AAA") {
        Some(value) => println!("Value is {:?}", value),
        None => println!("Not ok"),
    }
}

fn check_hash_map_delete() {
    let mut scores = HashMap::new();
    scores.insert("An", 8);
    let old = scores.insert("An", 10);
    println!(
        "{:?}",
        match old {
            Some(ok) => ok,
            _ => 0,
        }
    );

    let removed = scores.remove("An");
    println!("{:?}", removed);
    println!("{:?}", scores);
    println!("{:?}", scores.remove("An")); // lần này không còn
}

fn check_hash_map_entry() {
    let mut stock: HashMap<&str, u32> = HashMap::new();
    stock.insert("bút", 10);

    stock.entry("bút").or_insert(20);
    stock.entry("vở").or_insert(222);

    let mut items: Vec<_> = stock.iter().collect();
    items.sort();
    for (index, value) in items {
        println!("{index}: {value}");
    }
}

fn check_hash_map_update() {
    let mut scores = HashMap::from([("An", 8), ("Bình", 9)]);
    for value in scores.values_mut() {
        *value += 1;
    }

    let mut names: Vec<_> = scores.keys().collect();
    for name in names {
        println!("{name}: {}", scores[name]);
    }

    let total: i32 = scores.values().sum();
    println!("Tổng: {total}");
}

fn check_hash_map_update_ownership() {
    let name = String::from("An");
    let mut scores = HashMap::new();
    scores.insert(name, 9);
    println!("scores is {:?}", scores);
}

fn check_bTreeMap() {
    let mut set = HashSet::new();
    set.insert("An");
    set.insert("Bình");
    set.insert("An");

    let mut set = BTreeSet::new();
    set.insert("An");
    set.insert("Bình");
    set.insert("Bình3");
    set.insert("Bình2");
    println!("{:?}", set);

    let mut phone = HashMap::new();
    phone.insert("An", "0901");
    phone.insert("An3", "0901");
    phone.insert("An1", "0901");
    phone.insert("Bình", "0902");
    println!("phone==> {:?}", phone);

    let mut phone1 = BTreeMap::new();
    phone1.insert("An", "0901");
    phone1.insert("An3", "0901");
    phone1.insert("An1", "0901");
    phone1.insert("Bình", "0902");
    for (key, value) in &phone1 {
        println!("Key: {:?}, Value: {:?}", key, value);
    }
}

fn ex1() {
    let text = "Rust rất hay. Học Rust mỗi ngày, rất vui. Rust!";
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.trim_matches(['.', ',', '!', '?']).to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    let mut result: Vec<_> = counts.into_iter().collect();
    result.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    for (key, value) in result {
        println!("{:?}: {:?}", key, value);
    }
}

fn ex2() {
    let records = [
        ("An", 8),
        ("Bình", 6),
        ("An", 10),
        ("Chi", 9),
        ("Bình", 7),
        ("An", 9),
    ];

    let mut grades: HashMap<String, Vec<u32>> = HashMap::new();
    for (name, scope) in records {
        grades.entry(name.to_string()).or_default().push(scope);
    }

    let mut names: Vec<&String> = grades.keys().collect();
    for name in names {
        println!("{name}: {:?}", grades[name]);
    }
}

fn vec_and_string() {
    let colors = vec!["đỏ", "xanh", "vàng"];

    let index = match colors.get(1) {
        Some(val) => val,
        None => "",
    };
    println!("{:?}", index);

    let mut scores = vec![8, 7, 9];
    for s in &mut scores {
        *s += 1;
    }
    println!("{:?}", scores);

    let fruits = ["táo", "cam", "táo", "lê", "cam", "táo"];

    let mut counts: HashMap<&str, u32> = HashMap::new();
    for fruit in fruits {
        *counts.entry(fruit).or_insert(0) += 1;
    }

    let mut result: Vec<_> = counts.into_iter().collect();
    result.sort();
    for (fruit, count) in result {
        println!("{fruit}: {count}");
    }

    let v = vec![1, 2, 3];
    let x: Option<_> = v.get(0);
    println!("{:?}", x);
}
