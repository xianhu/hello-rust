/**
 * Rust基础练习
 */

// Cargo基础
// cargo build: 构建rust项目
// cargo run: 运行rust项目
// cargo clippy: 类似eslint，lint工具检查代码可以优化的地方
// cargo fmt: 类似go fmt，代码格式化
// cargo tree: 查看第三方库的版本和依赖关系
// cargo bench: 运行benchmark(基准测试,性能测试)
// cargo build/run --release: 使用 release 编译会比默认的 debug 编译性能提升 10 倍以上

// 输出练习
fn test001() {
    // 输出到命令行
    let a = 12;
    println!("a is {}", a);
    println!("a is {}, a again is {}", a, a);
    println!("a is {0}, a again is {0}", a);
    println!("{{}}");
}

// 基础语法
fn test002() {
    // 变量
    let a = 123;
    // a = "abc";
    // a = 4.56;
    // a = 456;

    // 可变变量
    let mut a1 = 123;
    a1 = 456;

    // 可变变量不可修改类型
    // a1 = "abc";

    // 重新赋值
    let a1 = 123;
    let a1 = "abc";

    // 常量，不可重新赋值
    const A2: i32 = 123;
    // let A2 = 456;

    // 重影
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}

// 数据类型
fn test003() {
    // 数字类型
    // i8 i16 i32 i64 i128
    // u8 u16 u32 u64 u128
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // 数字操作
    let sum = 5 + 10; // 加
    let difference = 95.5 - 4.3; // 减
    let product = 4 * 30; // 乘
    let quotient = 56.7 / 32.2; // 除
    let remainder = 43 % 5; // 求余

    // 布尔类型
    let b1 = true;
    let b2 = false;

    // 复合类型 -- 元组（不同类型）
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    // 复合类型 -- 数组（同类型）
    let a = [1, 2, 3, 4, 5];
    let mut a = [1, 2, 3, 4];
    let b = ["January", "February", "March"];
    let c: [i32; 5] = [1, 2, 3, 4, 5];
    let d = [3; 5];
}

// 函数和语句、表达式
fn test004() {
    // 基本函数
    fn another_function(x: i32, y: i32) {
        println!("x 的值为 : {}", x);
        println!("y 的值为 : {}", y);
    }

    // 代码块
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);

    // 函数返回值和类型
    fn five() -> i32 {
        return 5;
    }
    println!("five() 的值为: {}", five());
}

// 条件、循环语句
fn test005() {
    // if else
    let a = 12;
    let b;
    if a > 0 {
        b = 1;
    } else if a < 0 {
        b = -1;
    } else {
        b = 0;
    }
    println!("b is {}", b);

    // if else 表达式
    let a = 3;
    let number = if a > 0 { 1 } else { -1 };
    println!("number 为 {}", number);

    // while循环
    let mut number = 1;
    while number != 4 {
        println!("{}", number);
        number += 1;
    }

    // loop循环
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    loop {
        let ch = s[i];
        if ch == 'O' {
            break;
        }
        println!("\'{}\'", ch);
        i += 1;
    }

    // loop通过break返回值
    let s = ['R', 'U', 'N', 'O', 'O', 'B'];
    let mut i = 0;
    let location = loop {
        let ch = s[i];
        if ch == 'O' {
            break i;
        }
        i += 1;
    };
    println!(" \'O\' 的索引为 {}", location);

    // for循环
    let a = [10, 20, 30, 40, 50];
    for i in a.iter() {
        println!("值为 : {}", i);
    }

    // for循环
    let a = [10, 20, 30, 40, 50];
    for i in 0..5 {
        println!("a[{}] = {}", i, a[i]);
    }
}

// Rust所有权
fn test006() {
    // 所有权规则
    // Rust 中的每个值都有一个变量，称为其所有者。
    // 一次只能有一个所有者。
    // 当所有者不在程序运行范围时，该值将被删除。

    // 变量与数据交互的方式
    // 移动(栈) -- 基本数据类型
    let x = 5;
    let y = x;
    println!("{}, {}", x, y);

    // 移动Move(堆) -- 其他数据类型
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, {}", s1, s2); // 错误！s1 已经失效

    // 克隆Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    // 涉及函数的所有权机制 -- 基本数据类型
    fn makes_copy(some_integer: i32) {
        // 一个 i32 参数 some_integer 传入，有效
        println!("{}", some_integer);
    } // 函数结束, 参数 some_integer 是基本类型, 无需释放
    let x = 5;
    // x 被声明有效
    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效

    // 涉及函数的所有权机制 -- 其他数据类型
    fn takes_ownership(some_string: String) {
        // 一个 String 参数 some_string 传入，有效
        println!("{}", some_string);
    } // 函数结束, 参数 some_string 在这里释放
    let s = String::from("hello");
    // s 被声明有效
    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    // 函数返回值的所有权机制
    fn gives_ownership() -> String {
        let some_string = String::from("hello");
        // some_string 被声明有效
        return some_string;
        // some_string 被当作返回值移动出函数
    }
    let s1 = gives_ownership();
    // gives_ownership 移动它的返回值到 s1

    fn takes_and_gives_back(a_string: String) -> String {
        // a_string 被声明有效
        return a_string; // a_string 被当作返回值移出函数
    }
    let s2 = String::from("hello");
    // s2 被声明有效
    let s3 = takes_and_gives_back(s2);
    // s2 被当作参数移动, s3 获得返回值所有权
    // println!("{}", s2) // s2无效

    // 引用和租借Borrow
    let s1 = String::from("hello");
    let s2 = &s1;
    println!("s1 is {}, s2 is {}", s1, s2);

    // 涉及函数的引用和租借
    fn calculate_length(s: &String) -> usize {
        s.len()
    }
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
    // 这里的s1传入函数时是租借的形式，此时s1依然有效

    // 租借被移动后需重新租借
    let s1 = String::from("hello");
    let mut s2 = &s1;
    let s3 = s1;
    s2 = &s3; // 重新从 s3 租借所有权
    println!("{}", s2);

    // 禁止修改租借的值
    let s1 = String::from("run");
    let s2 = &s1;
    println!("{}", s2);
    // s2.push_str("oob"); // 错误

    // 可以修改可变引用的值（可变租借）
    let mut s1 = String::from("run");
    // s1 是可变的
    let s2 = &mut s1;
    // s2 是可变的引用
    s2.push_str("oob");
    println!("{}", s2);

    // 可变租借不允许多重引用租借
    let mut s = String::from("hello");
    let r1 = &mut s;
    // let r2 = &mut s;

    // 垂悬引用（Dangling References）
    // fn dangle() {
    //     let s = String::from("hello");
    //     &s
    // }
    // let reference_to_nothing = dangle();
}

// Slice类型
fn test007() {
    // 字符串切片
    let s = String::from("broadcast");
    let part1 = &s[0..5];
    let part2 = &s[5..9];
    println!("{}={}+{}", s, part1, part2);

    let mut s = String::from("runoob");
    let slice = &s[0..3];
    // s.push_str("yes!"); // 错误
    println!("slice = {}", slice);

    // 非字符串切片
    let arr = [1, 3, 5, 7, 9];
    let part = &arr[0..3];
    for i in part.iter() {
        println!("{}", i);
    }
}

// 结构体
fn test008() {
    // 结构体定义
    struct Site {
        domain: String,
        name: String,
        nation: String,
        found: u32,
    }

    // 结构体实例
    let runoob = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013,
    };

    // 结构体实例
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain, // 等同于 domain : domain,
        name,   // 等同于 name : name,
        nation: String::from("China"),
        found: 2013,
    };

    // 结构体实例
    let site = Site {
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        ..runoob
    };

    // 元组结构体
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);

    // 输出结构体
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);

    // 结构体方法
    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
        fn wider(&self, rect: &Rectangle) -> bool {
            self.width > rect.width
        }
    }
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1's area is {}", rect1.area());
    let rect2 = Rectangle {
        width: 40,
        height: 20,
    };
    println!("{}", rect1.wider(&rect2));

    // 结构体关联函数
    impl Rectangle {
        fn create(width: u32, height: u32) -> Rectangle {
            Rectangle { width, height }
        }
    }
    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);
    println!("{}", rect.area());

    // 单元结构体
    struct UnitStruct;
}

// 枚举类
fn test009() {
    // 基本枚举
    #[derive(Debug)]
    enum Book {
        Papery,
        Electronic,
    }
    let book = Book::Papery;
    println!("{:?}", book);

    // 带有数据类型
    #[derive(Debug)]
    enum Book1 {
        Papery(u32),
        Electronic(String),
    }
    let book = Book1::Papery(1001);
    let ebook = Book1::Electronic(String::from("url://..."));

    // 为属性命名
    enum Book2 {
        Papery { index: u32 },
        Electronic { url: String },
    }
    let book = Book2::Papery { index: 1001 };

    // match 语法，类似于switch
    enum Bookm {
        Papery { index: u32 },
        Electronic { url: String },
    }
    let book = Bookm::Papery { index: 1001 };
    match book {
        Bookm::Papery { index } => {
            println!("Papery book {}", index);
        }
        Bookm::Electronic { url } => {
            println!("E-book {}", url);
        }
    }

    // match 语法，类似于switch
    enum Bookmm {
        Papery(u32),
        Electronic { url: String },
    }
    let book = Bookmm::Papery(1001);
    match book {
        Bookmm::Papery(i) => {
            println!("{}", i);
        }
        Bookmm::Electronic { url } => {
            println!("{}", url);
        }
    }

    // Option 枚举类
    enum Option<T> {
        Some(T),
        None,
    }
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }

    // 为空时需要指定不为空时的类型
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        }
        Option::None => {
            println!("opt is nothing");
        }
    }

    let t = Some(64);
    match t {
        Some(64) => println!("Yes"),
        _ => println!("No"),
    }

    // if let语法
    // if let 匹配值 = 源变量 {
    //     语句块
    // }
    let i = 0;
    if let 0 = i {
        println!("zero");
    }

    // 枚举类型的if let语法
    enum Book00 {
        Papery(u32),
        Electronic(String),
    }
    let book = Book00::Electronic(String::from("url"));
    if let Book00::Papery(index) = book {
        println!("Papery {}", index);
    } else {
        println!("Not papery book");
    }
}

// Rust 组织管理
// Rust 中有三个重要的组织概念：箱、包、模块。

// 箱（Crate）："箱"是二进制程序文件或者库文件，存在于"包"中

// 包（Package）
// 当我们使用 Cargo 执行 new 命令创建 Rust 工程时，工程目录下会建立一个 Cargo.toml 文件。
// 工程的实质就是一个包，包必须由一个 Cargo.toml 文件来管理，该文件描述了包的基本信息以及依赖项。
// 一个包最多包含一个库"箱"，可以包含任意数量的二进制"箱"，但是至少包含一个"箱"（不管是库还是二进制"箱"）。
// 当使用 cargo new 命令创建完包之后，src 目录下会生成一个 main.rs 源文件，Cargo 默认这个文件为二进制箱的根，编译之后的二进制箱将与包名相同。

// 模块（Module）
// 组织模块的主要结构往往是树。Java 组织功能模块的主要单位是类，而 JavaScript 组织模块的主要方式是 function。
// Rust 中的组织单位是模块（Module）。
mod nation {
    mod government {
        fn govern() {}
    }
    mod congress {
        fn legislate() {}
    }
    mod court {
        fn judicial() {}
    }
}
// crate::nation::government::govern();

// 访问权限
mod nation1 {
    pub mod government {
        pub fn govern() {}
    }
    mod congress {
        pub fn legislate() {}
    }
    mod court {
        fn judicial() {
            super::government::govern();
            super::congress::legislate();
        }
    }
}

// 难以发现的模块，second_module是一个不带后缀的文件名
// mod second_module;

// use 关键字
// use 关键字能够将模块标识符引入当前作用域：
// use crate::nation::government::govern;
// use crate::nation::govern as nation_govern;

// 引用标准库
use std::f64::consts::PI;

use std::fs::File;
use std::io;
use std::io::Read;

fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 错误处理
fn test010() {
    // 不可恢复错误
    panic!("error occured");
    println!("Hello, Rust");

    // 可恢复的错误
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        }
        Err(err) => {
            println!("Failed to open the file.");
        }
    }

    // 简化match语法
    let f = File::open("hello.txt");
    if let Ok(file) = f {
        println!("File opened successfully.");
    } else {
        println!("Failed to open the file.");
    }

    // 可恢复错误按不可恢复错误处理
    let f1 = File::open("hello.txt").unwrap();
    let f2 = File::open("hello.txt").expect("Failed to open.");

    // kind 方法
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("No such file");
            }
            _ => {
                println!("Cannot read the file");
            }
        },
    }
}

// 泛型与特性
fn max(array: &[i32]) -> i32 {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

fn max1<T>(array: &[T]) -> T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

fn test011() {
    let a = [2, 4, 6, 3, 1];
    println!("max = {}", max(&a));
    println!("max = {}", max1(&a));

    // 结构体与枚举类中的泛型
    struct Point<T> {
        x: T,
        y: T,
    }
    enum Option<T> {
        Some(T),
        None,
    }
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

// 特性
// 特性（trait）概念接近于 Java 中的接口（Interface）
trait Descriptive {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u8,
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

// 默认特性
trait Descriptive1 {
    fn describe(&self) -> String {
        String::from("[Object]")
    }
}

struct Person1 {
    name: String,
    age: u8,
}

impl Descriptive1 for Person1 {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

fn test012() {
    let cali = Person {
        name: String::from("Cali"),
        age: 24,
    };
    println!("{}", cali.describe());
}

// 特性做参数
fn output(object: impl Descriptive) {
    println!("{}", object.describe());
}

// 特性做返回值
fn person() -> impl Descriptive {
    Person {
        name: String::from("Cali"),
        age: 24,
    }
}

// 生命周期
fn test013() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);

    // 生命周期注释
    // &i32        // 常规引用
    // &'a i32     // 含有生命周期注释的引用
    // &'a mut i32 // 可变型含有生命周期注释的引用
    fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s2.len() > s1.len() {
            s2
        } else {
            s1
        }
    }

    // 静态生命周期
}

use std::fs;
use std::io::prelude::*;
use std::io::stdin;

// 文件和IO
fn test014() {
    // 接收命令行参数
    let args = std::env::args();
    for arg in args {
        println!("{}", arg);
    }

    // 命令行输入
    let mut str_buf = String::new();
    stdin()
        .read_line(&mut str_buf)
        .expect("Failed to read line.");
    println!("Your input line is \n{}", str_buf);

    // 文件读取
    let text = fs::read_to_string("D:\\text.txt").unwrap();
    println!("{}", text);

    // 二进制文件读取
    let content = fs::read("D:\\text.txt").unwrap();
    println!("{:?}", content);

    // 大文件读取
    let mut buffer = [0u8; 5];
    let mut file = fs::File::open("D:\\text.txt").unwrap();
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);
    file.read(&mut buffer).unwrap();
    println!("{:?}", buffer);

    // 文件写入
    fs::write("D:\\text.txt", "FROM RUST PROGRAM").unwrap();

    // 文件写入
    let mut file = File::create("D:\\text.txt").unwrap();
    file.write(b"FROM RUST PROGRAM").unwrap();
}

use std::collections::HashMap;

// 集合与字符串
fn test015() {
    // 向量
    let vector: Vec<i32> = Vec::new(); // 创建类型为 i32 的空向量
    let vector = vec![1, 2, 4, 8]; // 通过数组创建向量

    let mut vector = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    vector.push(64);
    println!("{:?}", vector);

    let mut v1: Vec<i32> = vec![1, 2, 4, 8];
    let mut v2: Vec<i32> = vec![16, 32, 64];
    v1.append(&mut v2);
    println!("{:?}", v1);

    let mut v = vec![1, 2, 4, 8];
    println!(
        "{}",
        match v.get(0) {
            Some(value) => value.to_string(),
            None => "None".to_string(),
        }
    );

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // 字符串
    let string = String::new();
    let one = 1.to_string(); // 整数到字符串
    let float = 1.3.to_string(); // 浮点数到字符串
    let slice = "slice".to_string(); // 字符串切片到字符串

    let mut s = String::from("run");
    s.push_str("oob"); // 追加字符串切片
    s.push('!'); // 追加字符

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);

    let s = "你好";
    let len = s.len();
    let len = s.chars().count();

    let s = String::from("hello中文");
    for c in s.chars() {
        println!("{}", c);
    }

    let s = String::from("EN中文");
    let a = s.chars().nth(2);
    println!("{:?}", a);

    let s = String::from("EN中文");
    let sub = &s[0..2];
    println!("{}", sub);

    // 映射表Map
    let mut map = HashMap::new();
    map.insert("color", "red");
    map.insert("size", "10 m^2");
    println!("{}", map.get("color").unwrap());

    let mut map = HashMap::new();
    map.insert("color", "red");
    map.insert("size", "10 m^2");
    for p in map.iter() {
        println!("{:?}", p);
    }

    map.entry("color").or_insert("red");
}

// 面向对象
pub struct ClassName {
    field: i32,
}

impl ClassName {
    pub fn new(value: i32) -> ClassName {
        ClassName { field: value }
    }

    pub fn public_method(&self) {
        println!("from public method");
        self.private_method();
    }

    fn private_method(&self) {
        println!("from private method");
    }
}

// 并发编程
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn spawn_function() {
    for i in 0..5 {
        println!("spawned thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn test016() {
    thread::spawn(spawn_function);
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 闭包（closures）传递参数
    thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 闭包其他形式
    let inc = |num: i32| -> i32 { num + 1 };
    println!("inc(5) = {}", inc(5));
    let inc = |num| num + 1;
    println!("inc(5) = {}", inc(5));

    // join 方法
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("spawned thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();

    // move 强制所有权迁移
    let s = "hello";
    let handle = thread::spawn(move || {
        println!("{}", s);
    });
    handle.join().unwrap();

    // 消息传递
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

fn main() {
    test011();
}
