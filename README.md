##### rust 内存释放
在变量离开作用域时，自动调用drop函数来释放资源并进行清理工作

##### String类型
- 字符串字面值: 在编译时就知道它的内容，其文本内容直接被硬编码到最终的可执行文件里
  - 值不可变，因此速度块、高效
- String类型，为了支持可变性，需要在heap上分配内存来保存编译时未知的文本内容
  - 一个String包含3个部分：1、指向存放字符串内容的内存的指针 2、长度 3、容量
  - String类型数据的描述信息放在 stack 上
  - 存放字符串内容的部分在 heap 上
  - 操作系统必须在运行时请求内存，通过调用 String::from 来实现

##### 移动（Move）
```
let str1 = String::from("hello");
let str2 = str1;
```
- 为了保证内存安全，避免二次释放(double free):
  - 当把str1赋给str2时，仅会复制stack上的数据描述信息，不会复制指针所指向的heap上的内存。
  - 当str1离开作用域的时候，会让str1失效，也不需要释放任何资源。
  - 在str2离开作用域的时候，才会进行资源释放。

==也可以将以上内容理解为浅拷贝(shallow copy)，但rust同时还会让str1失效。如果需要实现深拷贝(deep copy)，可使用clone方法==

##### Stack上的数据: 复制
- Copy trait, 可以用于像整数一样的简单值，完全存放在stack上面的类型
- 如果一个类型实现了Copy这个trait，那么旧的变量在赋值后不需要 clone 也仍然可用
- 如果一个类型或者该类型的一部分实现了Drop trait，那么Rust不允许让它再实现Copy trait