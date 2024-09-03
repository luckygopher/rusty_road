fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4")
    }else if number % 3 == 0 {
        println!("number is divisible by 3")
    }else if number % 2 == 0 {
        println!("number is divisible by 2")
    }else {
        println!("not found case")
    }
    match_demo(number)
}

fn match_demo(number:i32) {
    match number {
        n if n % 4 == 0 => println!("number is divisible by 4"),
        n if n % 3 == 0 => println!("number is divisible by 3"),
        n if n % 2 == 0 => println!("number is divisible by 2"),
        _ =>  println!("not found case"),
    }
}