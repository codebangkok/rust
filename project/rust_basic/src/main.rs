#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![allow(while_true)]
#![allow(unreachable_code)]
#![allow(unused_mut)]
#![allow(unused_imports)]

// use rust_basic::person::Person;
// use rust_basic::customer::Customer;
use rust_basic::{customer::Customer, person::Person, speaking::Speaking};
use std::collections::HashMap;

fn main() {
    //Variables
    let mut x: i32;
    x = 10;
    x = 20;

    let x = 10;
    let (x, y) = (10, 20);

    const PI: f64 = 3.14;

    //Tuple
    let x = (1, 3.14, 1000);
    let x: (u8, f64, i32) = (1, 3.14, 1000);
    let a = x.0;
    let b = x.1;
    let c = x.2;

    //Array
    let x: [i32; 5];
    let x = [1, 2, 3, 4, 5];
    let x = [0; 5];

    //If
    let score = 50;

    let mut grade = "";
    if score >= 80 {
        grade = "A";
    } else if score >= 70 {
        grade = "B";
    } else if score >= 60 {
        grade = "C";
    } else if score >= 50 {
        grade = "D";
    } else {
        grade = "F";
    }

    let grade = if score >= 80 {
        "A"
    } else if score >= 70 {
        "B"
    } else if score >= 60 {
        "C"
    } else if score >= 50 {
        "D"
    } else {
        "F"
    };

    // let result = score >= 50 ? "Pass" : "Fail";
    let result = if score >= 50 { "Pass" } else { "Fail" };

    //Loop
    while true {
        break;
    }

    'label1: loop {
        'label2: loop {
            break 'label1;
            continue 'label2;
        }
    }

    for i in 0..3 {
        println!("{}", i);
    }

    for i in 0..=3 {
        println!("{}", i);
    }

    let numbers = [10, 20, 30];
    for n in numbers.iter() {
        println!("{}", n);
    }
    for n in [10, 20, 30].iter() {
        println!("{}", n);
    }

    let numbers = [(1, 2), (3, 4)];
    for (i, j) in numbers.iter() {
        println!("{} {}", i, j);
    }

    //String
    let x = "Hello";
    let x = String::from("Hello");
    let x = "Hello".to_string();

    //Collection
    let mut x: Vec<i32> = Vec::new();
    x.push(10);
    x.push(20);
    x.push(30);
    let y = x.pop();
    let mut x = vec![1, 2, 3];

    //HashMap
    let mut x: HashMap<&str, &str> = HashMap::new();
    x.insert("th", "Thailand");
    x.insert("us", "United State");
    let y = x.get("th");
    println!("{}", y.unwrap());

    //Struct
    let p = Person::new("Bond".to_string(), 18);
    p.hello();

    //Traits
    p.speak();

    //Enum
    let x = Colors::Red;

    let mut color = "";
    match x {
        Colors::Red => color = "red",
        Colors::Green => color = "green",
        _ => color = "blue",
    }

    let color = match x {
        Colors::Red => "red",
        Colors::Green => "green",
        Colors::Blue => "blue",
    };

    let x = check_grade(-1);
    match x {
        GradeResult::Error(e) => println!("{}", e),
        GradeResult::Value(g) => println!("{}", g),
    }

    let x = check_grade2(-1);
    match x {
        None => println!("error"),
        Some(v) => println!("{}", v),
    }

    let x = check_grade3(-1);
    match x {
        Err(e) => println!("{}", e),
        Ok(v) => println!("{}", v),
    }

    let x = check_grade3(100);
    if x.is_err() {
        return;
    }
    let y = x.unwrap();

    let x = check_grade3(100);
    if let Ok(v) = x {
        println!("{}", v);
    }

    let x = check_grade3(100);
    let y = match x {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };

    //Closures
    let x = add(10, 20);
    let x = |a, b| a + b;
    let y = x(10, 20);
    let y = cal(10, 20, x);
    let y = cal(10, 20, |a, b| a - b);
    let y = cal(10, 20, add);
}

fn cal<F: Fn(i32, i32) -> i32>(a: i32, b: i32, f: F) -> i32 {
    f(a, b)
}

fn cal2<F>(a: i32, b: i32, f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(a, b)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn check_grade(score: i32) -> GradeResult {
    if score < 0 || score > 100 {
        return GradeResult::Error("score is not correct".to_string());
    }

    GradeResult::Value("A".to_string())
}

fn check_grade2(score: i32) -> Option<String> {
    if score < 0 || score > 100 {
        return None;
    }

    Some("A".to_string())
}

fn check_grade3(score: i32) -> Result<String, String> {
    if score < 0 || score > 100 {
        return Err("score is not correct".to_string());
    }

    Ok("A".to_string())
}

enum GradeResult {
    Value(String),
    Error(String),
}

enum Colors {
    Red,
    Green,
    Blue,
}

fn get_number() -> i32 {
    let a = 10;
    let b = 20;
    a + b
}
