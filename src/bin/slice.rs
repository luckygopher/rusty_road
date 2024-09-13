
fn main() {
    let s = String::from("Hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{},{}", hello, world);

    let my_string = String::from("Hello world");
    let word = first_world(&my_string[..]);
    println!("{}", word);

    let my_string_literal = "hello world";
    let word = first_world(my_string_literal);
    println!("{}", word);
}


fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i,&item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
