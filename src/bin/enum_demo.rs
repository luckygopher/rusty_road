// 定义枚举
enum IpAddrKind {
    V4,
    V6,
}

// 将数据附加到枚举的变体中
enum IpAddrKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,                       // 单元变体，不包含任何数据
    Move { x: i32, y: i32 },    // 结构体变体
    Write(String),              // 元组变体，包含一个 String
    ChangeColor(i32, i32, i32), // 元组变体，包含三个 i32
}

// 为枚举实现方法
impl Message {
    fn process_message(&self) {
        match self {
            Message::Quit => {
                // 这个变体没有关联数据
                println!("收到退出指令");
            }
            Message::Move { x, y } => {
                // 结构体变体，可以通过字段名访问数据
                println!("移动到: x={}, y={}", x, y);
            }
            Message::Write(text) => {
                // 元组变体，可以给关联数据命名
                println!("写入文本: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                // 多个关联数据的元组变体
                println!("改变颜色为: RGB({},{},{})", r, g, b);
            }
        }
    }
}

// 结构体表示IP地址类型和值
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // demo 1
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let company = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // demo 2
    let home1 = IpAddrKind2::V4(127, 0, 0, 1);
    let company1 = IpAddrKind2::V6(String::from("::1"));

    // demo3
    let quit = Message::Quit;
    let move1 = Message::Move { x: 1, y: 2 };
    let write = Message::Write(String::from("hello"));
    let change_color = Message::ChangeColor(0, 255, 255);

    // 1. 使用 match 匹配
    match &write {
        Message::Write(text) => println!("内容是: {}", text), // 输出: 内容是: hello
        _ => println!("其他变体"),
    }

    // 2. 不能直接访问数据
    // 这样是错误的：
    // println!("{}", msg.text);  // ❌ 编译错误

    // 3. 如果需要频繁访问数据，可以在 match 中解构
    let text = match write {
        Message::Write(t) => t,
        _ => String::new(),
    };
    // 现在 text = "hello"

    quit.process_message()
}
