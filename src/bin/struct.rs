#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) ->bool{
        self.width>other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle{
        Rectangle{
            width: size,
            length: size,
        }
    }
}

// 结构体的使用demo
fn main() {
    let rect1 = Rectangle {
        width: 30,
        length:50,
    };
    let rect2 = Rectangle {
        width: 10,
        length:20,
    };
    let rect3 = Rectangle {
        width: 40,
        length:60,
    };
    // 利用Display trait 打印
    println!("{}", rect1.area());
    // 利用Debug trait 打印
    println!("{:#?}", rect1);

    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));

    let s = Rectangle::square(10);
    println!("{}", s.area())
}