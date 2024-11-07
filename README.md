### 一：基础概念

#### 所有权规则

- 每个值都有一个变量，这个变量是该值的所有者
- 每个值同时只能有一个所有者
- 当所有者超出作用域时，该值将被删除

#### rust 内存释放

在变量离开作用域时，自动调用drop函数来释放资源并进行清理工作

#### String类型

- 字符串字面值: 在编译时就知道它的内容，其文本内容直接被硬编码到最终的可执行文件里
    - 值不可变，因此速度快、高效
- String类型，为了支持可变性，需要在heap上分配内存来保存编译时未知的文本内容
    - 一个String包含3个部分：1、指向存放字符串内容的内存的指针 2、长度 3、容量
    - String类型数据的描述信息放在 stack 上
    - 存放字符串内容的部分在 heap 上
    - 操作系统必须在运行时请求内存，通过调用 String::from 来实现

#### 移动（Move）

```
let str1 = String::from("hello");
let str2 = str1;
```

- 为了保证内存安全，避免二次释放(double free):
    - 当把str1赋给str2时，仅会复制stack上的数据描述信息，不会复制指针所指向的heap上的内存。
    - 当str1离开作用域的时候，会让str1失效，也不需要释放任何资源。
    - 在str2离开作用域的时候，才会进行资源释放。

![Memory Layout](images/move_img.png)

> [!Tip]
> 也可以将以上内容理解为浅拷贝(shallow copy)，但rust同时还会让str1失效。如果需要实现深拷贝(deep copy)，可使用clone方法

![Memory Layout](images/clone_img.png)

#### Stack上的数据: 复制

- Copy trait, 可以用于像整数一样的简单值，完全存放在stack上面的类型
- 如果一个类型实现了Copy这个trait，那么旧的变量在赋值后不需要 clone 也仍然可用
- 如果一个类型或者该类型的一部分实现了Drop trait，那么Rust不允许让它再实现Copy trait

#### 函数的参数与返回值

- 在语义上，将值传递给函数和把值赋给变量是类型的，将值传递给函数将会发生移动或复制。
- 函数在返回值的过程中同样也会发生所有权的转移

> [!IMPORTANT]
>
> 一个变量的所有权总是遵循同样的模式：
> - 把一个值赋给其它变量时就会发生移动
> - 当一个包含heap数据的变量离开作用域时，它的值就会被drop函数清理，除非数据的所有权移动到另一个变量上了

#### 引用与借用

- 参数的类型是 &String 而不是String
- & 符号就表示引用，允许你引用某些值而不取得其所有权
- 我们把引用作为函数参数这个行为叫做借用
- 和变量一样，引用默认也是不可变的

![Memory Layout](images/quote_img.png)

#### 可变引用

- 引用使用 &mut 可以成为可变引用
- 在特定作用域内，对某一块数据，只能有一个可变的引用
- 可以通过创建新的作用域，来允许非同时的创建多个可变引用

#### 引用的规则

- 在任何给定的时刻，只能满足下列条件之一：
    - 一个可变的引用
    - 任意数量的不可变引用
- 引用必须一直有效，否则会出现悬空指针，编译会报错

#### 切片(slice)

- slice 是一种不持有所有权的数据类型
- 字符串切片是指向字符串中一部分内容的引用
- 字符串字面值被直接存储在二进制程序中，let s = "Hello,World".变量s的数据类型就是字符串切片(&str)，它是一个指向二进制程序特定位置的切片。
- 切片的索引遵循 左闭右开 原则
- 定义函数时使用字符串切片来代替字符串引用会使我们的API更加通用，且不会损失任何功能。

![Memory Layout](images/slice_img.png)

> [!Tip]
> 在rust中，字符串的底层表示是 UTF-8 编码的字节序列，字符（char）是一个 4 字节的 Unicode 标量值，
> 每个 Unicode 字符可以占用 1 到 4 个字节，如：英文字符占 1 字节，中文字符占 3 字节
> 而 Rust 的字符串切片（&str）要求始终切割在字符的有效边界上。
> 如果尝试从一个 多字节字符 的中间进行切片，程序会触发Panic。

#### 结构体(struct)

- 使用struct关键字，并为整个struct命名
- 在花括号内，为所有字段(Field)定义名称和类型
- 每个字段定义均需以逗号结束
- 实例化struct，需要为每个字段指定具体值，无需按声明的顺序进行指定
- 一旦struct的实例是 mut 可变的，那么实例中所有的字段都是可变的
- 当字段名与字段值对应变量名相同时，可以使用字段初始化简写
- 当你想基于某个struct实例A来创建一个新实例B的时候，可以在B中使用struct更新语法（..A）
- Tuple struct 整体存在命名，但里面的元素没有名
- 可以定义没有任何字段的struct，叫做 Unit-Like struct.

```
// struct
struct User {
  name: String,
  age: u64,
  active: bool,
}

let user1 = User {
  name: String::from("hello"),
  age: 100,
  active: true,
}

let user2 = User {
  name: String::from("world"),
  ..user1
}

// Tuple struct
struct Color(i32,i32,i32);
let black = Color(0,0,0);
```

#### struct数据的所有权

```rust
struct User {
    name: String,
    age: u64,
    active: bool,
}
```

- 以上结构体的字段使用了String而不是&str:
    - 该struct实例拥有其所有的数据
    - 只要该struct实例是有效的，那么里面的字段数据也是有效的
- struct里也可以存放引用，但需要使用生命周期：
    - 生命周期保证只要struct实例是有效的，那么里面的引用也是有效的
    - 如果struct里面存储引用，而不使用生命周期，就会报错

#### struct的方法

- 方法和函数类型：fn关键字、名称、参数、返回值
- 方法与函数不同之处：
    - 方法是在struct(或者enum、trait对象)的上下文中定义
    - 第一个参数是self，表示方法被调用的struct实例
- 方法在 impl 块里定义
- 方法的第一个参数可以是&self，也可以获得其 所有权 或 可变借用
- 在调用方法时，rust会自动引用或解引用

#### 关联函数

- 在 impl 块里面不把self作为第一个参数定义的这类函数，就叫关联函数
- 关联函数通常用于构造器
- 关联函数使用 :: 符号调用

#### 枚举

- 使用 enum + 名称 + {} 定义
- 可将数据附加到枚举的变体中，每个变体可以拥有不同的类型以及关联的数据量

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V5(String),
}
```
```
// 枚举理解
msg = Message::Write("hello")
      |         |      |
      |         |      └── 关联数据
      |         └── 变体标识符
      └── 枚举类型
```

> [!IMPORTANT]
> 
> 1. 模式匹配是获取枚举值的主要方式
>    - 使用 match 表达式
>    - 使用 if let 语法（适用于只关心一个变体的情况）
> 2. 每个变体可以包含不同类型和数量的数据
>    - 单元变体：无数据
>    - 结构体变体：命名字段
>    - 元组变体：匿名字段
> 3. 枚举值的所有权
>    - 当枚举包含数据（如 String）时，需要考虑所有权规则
>    - 在 match 中使用引用可以避免移动所有权

#### match
```
// 匹配守卫语法
n if condition => expression
|  |           |
|  |           └── 匹配成功时执行的代码
|  └── 条件表达式
└── 绑定变量
```

> [!IMPORTANT] 
> 注意事项
> 1. 匹配是按顺序的，第一个匹配的分支会被执行
> 2. 守卫条件可以访问匹配模式中绑定的变量
> 3. 在分支的代码块中可以继续使用这个绑定的变量
> 4. 最后的 _ 通常用作默认分支，处理所有其他情况
> 5. 匹配必须是穷尽的（处理所有可能的情况）

##### if let
- 处理只关心一种匹配而忽略其他匹配的情况

```rust
fn main(){
  let v = Some(0u8);
  match v {
    Some(3) => println!("hello"),
    _ => (),
  }
  
  if let Some(3) = v {
    println!("hello");
  }else { 
    println!("others")
  }
}
```

#### Package, Crate, Module
- Crate 的类型
  - binary
  - library
- Crate Root
  - 是源代码文件
  - Rust编译器从这里开始，组成你的Crate的根Module
- 一个Package
  - 包含1个Cargo.toml,它描述了如何构建这些Crates
  - 只能包含0-1个library crate
  - 可以包含任意数量的binary crate
    - 文件放在src/bin下，每个文件是单独的binary crate
  - 但必须至少包含一个crate(library或binary)
- Cargo的惯例
  - src/main.rs
    - binary crate 的crate root
    - crate名称 与 package名称 相同 
  - src/lib.rs
    - package包含一个library crate
    - library crate的crate root
    - crate名称 与package名称 相同
  - Cargo 把crate root文件交给rustc来构建library或者binary
- Module
  - 在一个crate内，将代码进行分组
  - 增加可读性，易于复用
  - 控制项目的私有性，public、private
  - 使用 mod 关键字建立module
  - mod可嵌套
  - mod可包含其它项(struct、enum、常量、trait、函数等)的定义
  - src/main.rs 和 src/lib.rs ，这两个文件(任意一个)的内容形成了名为 crate 的模块，位于整个模块树的根部
  - 模块定义时，如果模块名后边是 “;”，而不是代码块：
    - Rust会从与模块同名的文件中加载内容
    - 模块树的结构不会变化

#### Vector
- Vec<T>,叫做Vector
  - 由标准库提供
  - 可存储多个值
  - 只能存储相同类型的数据
  - 值在内存中连续存放
  - 当Vector离开作用域后，它就会被清理掉，它所有的元素也会被清理掉
- 删除Vec元素
  - remove() 会移动删除点后的所有元素，复杂度为 O(n)
  - swap_remove() 不保持顺序但效率更高，复杂度为 O(1)
  - pop() 只能删除最后一个元素
  - 删除所有元素：使用 clear()
  - retain() 和 drain() 适合批量操作
  - 删除元素时要注意索引范围，越界会导致 panic

#### HashMap
- 数据存储在heap上
- 同构的，一个HashMap中，所有的K必须是同一类型，所有的V也必须是同一种类型
- 在元素类型为 Tuple 的 Vector 上使用 collect方法，可以组建一个HashMap
  - 要求Tuple有两个值：一个作为K，一个作为V
  - collect 方法可以把数据整合成很多种集合类型，包括HashMap
- insert 方法插入键值对到HashMap
- entry方法：检查指定的K是否对应一个V
  - 参数为K
  - 返回 enum Entry：代表值是否存在
- Entry的 or_insert 方法：
  - 如果K存在，返回对应的 V 的一个可变引用
  - 如果K不存在，将方法参数作为 K 的新值插入，返回这个值的可变引用
- 默认情况下，HashMap使用加密功能强大的Hash函数，可以抵抗拒绝服务（DoS）攻击，不是可用的最快Hash算法，但具有更好的安全性
- Hash函数决定 HashMap 如何在内存中存放K和V
- 可以指定不同的Hasher来切换到另一个函数，Hasher是实现BuildHasher trait的类型

#### Option枚举
```rust
enum Option<T> {
  Some(T),
  None,
}
```
- 定义在标准库中
- Option<T> 和 T 是不同的类型，若想使用Option<T>中的T，必须将它转换为T
- 它包含在Prelude（预导入模块）中，可直接使用
  - Option<T>
  - Some(T)
  - None

#### Result 枚举
- 它包含在Prelude（预导入模块）中，可直接使用
```rust
enum Result<T,E> {
  Ok(T),
  Err(E),
}
```
- T: 操作成功情况下，Ok变体里返回的数据的类型
- E: 操作失败情况下，Err变体里返回的错误的类型

#### Rust错误处理
> [!IMPORTANT]
> panic!
> - 当panic发生
>   - 默认情况：
>     - 程序会打印一个错误信息
>     - 程序展开调用栈(工作量大)，Rust沿着调用栈往回走，清理每个遇到的函数中的数据
>     - 退出程序
>   - 立即中止调用栈
>     - 不进行清理，直接停止程序
>     - 内存需要 OS 进行清理
> - 想让二进制文件更小，把设置从 “展开” 改为 “中止”
>   - 在Cargo.toml中适当的profile部分设置
>     - panic = ‘abort’
> - 通过设置环境变量RUST_BACKTRACE可得到回溯信息
> - 为了获取带有调试信息的回溯，必须启用调试符号(不带 --release)
> - ```set RUST_BACKTRACE=1 && cargo run```

> [!IMPORTANT] 
> unwrap
> - match表达式的一个快捷方法
>   - 如果 Result 结果是Ok，返回Ok里面的值
>   - 如果 Result 结果是Err，调用panic!宏
  
> [!IMPORTANT]
> expect
> - 与 unwrap 类似，但可指定错误信息

> [!IMPORTANT]
> ？
> - ？运算符可作为传播错误的一种快捷方式
> - 如果Result是Ok：Ok中的值就是 <b>表达式</b> 的结果，然后继续执行程序
> - 如果Result是Err：Err就是 <b>整个函数</b> 的返回值，就像使用了return
> - Trait std::convert::From 上的from函数，主要用于错误之间的转换
> - 被 ？所应用的错误，会隐式的被from函数处理
> - 当 ？调用from函数时，它所接收的错误类型会被转化为当前函数返回类型所定义的错误类型
> - ？运算符只能用于返回结果类型是 Result or Option or implements `Try`的函数
> - 适用于针对不同错误原因，返回同一种错误类型
>   - 只要每个错误类型实现了转换为所返回的错误类型的from函数

#### 泛型
关键点：
1. 泛型提供了代码重用的能力
2. 使用 trait 约束来限制类型必须实现的功能
3. where 子句可以使复杂约束更清晰
4. 泛型在编译时会被单态化，没有运行时开销
5. 可以为特定类型实现特定的行为
- 基本语法：
```rust
// T 是类型参数
fn function_name<T>(parameter: T) -> T {
    // 函数体
}

// 实际使用
fn main() {
    let result = function_name(42);       // T 是 i32
    let result = function_name("hello");  // T 是 &str
}
```
- 多个类型参数：
```rust
// 多个泛型参数用逗号分隔
fn print_pair<T, U>(t: T, u: U) {
    println!("Pair: {:?}, {:?}", t, u);
}

fn main() {
    print_pair(1, "hello");    // T 是 i32, U 是 &str
    print_pair(true, 42.0);    // T 是 bool, U 是 f64
}
```
- 带约束的泛型：
```rust
// T 必须实现 Display trait
fn print<T: std::fmt::Display>(value: T) {
    println!("Value is: {}", value);
}

// 多个约束使用 + 
fn print_debug<T: std::fmt::Display + std::fmt::Debug>(value: T) {
    println!("Value is: {} or {:?}", value, value);
}

// where 语法（更清晰的约束写法）
fn complex_function<T, U>(t: T, u: U) -> i32 
where 
    T: std::fmt::Display,
    U: Clone + std::fmt::Debug,
{
    println!("T: {}", t);
    println!("U: {:?}", u);
    42
}
```
- 泛型结构体和方法：
```rust
// 泛型结构体
struct Point<T> {
    x: T,
    y: T,
}

// 为泛型结构体实现方法
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
}

// 为特定类型实现方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```
- 泛型枚举：
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}

enum Option<T> {
    Some(T),
    None,
}
```

#### trait
关键点：
1. trait 定义了类型可以实现的行为
2. 可以为任何类型实现任何 trait（遵循孤儿规则）
3. trait 可以有默认实现
4. trait 默认实现的方法可以调用trait中其它的方法，即使这些方法没有默认实现
5. trait 可以作为参数和返回值 
6. trait 对象提供了运行时多态 
7. 可以组合多个 trait 
8. trait 可以继承

- 基本定义和实现
```rust
// 定义 trait
trait Animal {
    // 必须实现的方法
    fn make_sound(&self) -> String;
    
    // 带默认实现的方法
    fn description(&self) -> String {
        String::from("这是一个动物")
    }
}

// 为结构体实现 trait
struct Dog {
    name: String
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("汪汪!")
    }
    
    // 可以覆盖默认实现
    fn description(&self) -> String {
        format!("这是一只叫{}的狗", self.name)
    }
}
```
- trait 作为参数
```rust
// 接受任何实现了 Animal trait 的类型
fn pet_animal(animal: &impl Animal) {
    println!("声音: {}", animal.make_sound());
}

// 使用 trait bound 语法（等价写法）
fn pet_animal_bound<T: Animal>(animal: &T) {
    println!("声音: {}", animal.make_sound());
}

// 多个 trait 约束
fn examine(animal: &(impl Animal + std::fmt::Debug)) {
    println!("检查动物: {:?}", animal);
}

fn examine1<T: Animal + std::fmt::Debug>(animal: T) {
  println!("检查动物: {:?}", animal);
}

fn examine1<T>(animal: T)
where
    T: Animal + std::fmt::Debug,
{
  println!("检查动物: {:?}", animal);
}

```
- trait 继承
```rust
trait Animal {
    fn make_sound(&self) -> String;
}

// Pet trait 继承 Animal trait
trait Pet: Animal {
    fn name(&self) -> String;
}

struct Dog {
    dog_name: String
}

// 实现 Pet trait 必须同时实现 Animal trait
impl Animal for Dog {
    fn make_sound(&self) -> String {
        String::from("汪汪!")
    }
}

impl Pet for Dog {
    fn name(&self) -> String {
        self.dog_name.clone()
    }
}
```
- 泛型trait
```rust
trait Converter<T> {
  fn convert(&self) -> T;
}

struct Celsius(f64);
struct Fahrenheit(f64);

impl Converter<Fahrenheit> for Celsius {
  fn convert(&self) -> Fahrenheit {
    Fahrenheit(self.0 * 9.0/5.0 + 32.0)
  }
}
```
- 条件实现
```rust
struct Wrapper<T>(Vec<T>);

// 只为实现了 Display 的类型实现 Display
impl<T: std::fmt::Display> std::fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```
- 自动实现的trait
```rust
// 自动派生常用 trait
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 现在 Point 自动实现了 Debug、Clone 和 PartialEq trait
```

- 关联类型trait 与 泛型trait
```rust
// 使用泛型的 trait
trait GenericContainer<T> {
    fn add(&mut self, item: T);
    fn get(&self) -> Option<&T>;
}

// 使用关联类型的 trait
trait AssocContainer {
    type Item;
    fn add(&mut self, item: Self::Item);
    fn get(&self) -> Option<&Self::Item>;
}

// 使用泛型时，可以为同一类型多次实现 trait
struct MyBox<T>(T);

impl<T> GenericContainer<T> for MyBox<T> {
    fn add(&mut self, item: T) { self.0 = item; }
    fn get(&self) -> Option<&T> { Some(&self.0) }
}
impl GenericContainer<String> for MyBox<i32> {
    fn add(&mut self, _item: String) { }
    fn get(&self) -> Option<&String> { None }
}

// 使用关联类型时，只能实现一次
impl<T> AssocContainer for MyBox<T> {
    type Item = T;
    fn add(&mut self, item: T) { self.0 = item; }
    fn get(&self) -> Option<&T> { Some(&self.0) }
}
```