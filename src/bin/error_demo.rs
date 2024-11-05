use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    // demo 1
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // demo2 使用 unwrap 替换 demo1 中的match
    let f = File::open("hello.txt").unwrap();
    // demo3 使用 expect 替换 demo1 中的match, 相比 unwrap 而言，expect 可以添加自定义错误信息
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

// 错误的传递
fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(str) => Ok(s),
        Err(error) => Err(error),
    }
}

// 改进写法1
fn read_username_from_file2() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 改进写法2
fn read_username_from_file3() -> Result<String, std::io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
