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
    fn another_function(x: i32, y: i32) {
        println!("x 的值为 : {}", x);
        println!("y 的值为 : {}", y);
    }

    // let a = (let b = 2);
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("x 的值为 : {}", x);
    println!("y 的值为 : {}", y);

    fn five() -> i32 {
        return 5;
    }
    println!("five() 的值为: {}", five());
}

// 条件、循环语句
fn test005() {
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
    println!("{}", x);

    // 移动Move(堆) -- 其他数据类型
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}, world!", s1); // 错误！s1 已经失效

    // 克隆Clone
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

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

    // 租借被移动后需重新租借
    let s1 = String::from("hello");
    let mut s2 = &s1;
    let s3 = s2;
    s2 = &s3; // 重新从 s3 租借所有权
    println!("{}", s2);

    // 禁止修改租借的值
    let s1 = String::from("run");
    let s2 = &s1;
    println!("{}", s2);
    // s2.push_str("oob"); // 错误

    // 可以修改可变引用的值
    let mut s1 = String::from("run");
    // s1 是可变的
    let s2 = &mut s1;
    // s2 是可变的引用
    s2.push_str("oob");
    println!("{}", s2);

    // 可变引用不允许多重引用
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

// 涉及函数的所有权机制
fn takes_ownership(some_string: String) {
    // 一个 String 参数 some_string 传入，有效
    println!("{}", some_string);
} // 函数结束, 参数 some_string 在这里释放

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

fn test007() {
    let s = String::from("hello");
    // s 被声明有效

    takes_ownership(s);
    // s 的值被当作参数传入函数
    // 所以可以当作 s 已经被移动，从这里开始已经无效

    let x = 5;
    // x 被声明有效

    makes_copy(x);
    // x 的值被当作参数传入函数
    // 但 x 是基本类型，依然有效
    // 在这里依然可以使用 x 却不能使用 s
}

// 函数返回值的所有权机制
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 被声明有效

    return some_string;
    // some_string 被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string 被声明有效
    a_string // a_string 被当作返回值移出函数
}

fn test008() {
    let s1 = gives_ownership();
    // gives_ownership 移动它的返回值到 s1

    let s2 = String::from("hello");
    // s2 被声明有效

    let s3 = takes_and_gives_back(s2);
    // s2 被当作参数移动, s3 获得返回值所有权
    // println!("{}", s2) // s2无效
}

fn main() {
    test004();
}
