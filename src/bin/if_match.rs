fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4")
    } else if number % 3 == 0 {
        println!("number is divisible by 3")
    } else if number % 2 == 0 {
        println!("number is divisible by 2")
    } else {
        println!("not found case")
    }
    match_demo(number);
    if_let(Some(0u8))
}

fn match_demo(number: i32) {
    match number {
        n if n % 4 == 0 => println!("number is divisible by 4"),
        n if n % 3 == 0 => println!("number is divisible by 3"),
        n if n % 2 == 0 => println!("number is divisible by 2"),
        _ => println!("not found case"),
    }
}

fn if_let(v: Option<u8>) {
    match v {
        Some(3) => println!("hello"),
        _ => (),
    }

    if let Some(3) = v {
        println!("hello");
    } else {
        println!("others")
    }

    if Some(3) == v {
        println!("hello");
    } else {
        println!("others");
    }
}
