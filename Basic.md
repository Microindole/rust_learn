### 2
```rust
use std::io; // 使用了io这个库，io库是在标准库中的
            // prelude
use rand::Rng; // trait
use std::cmp::Ordering;
//         Ordering 是一个枚举，分别有 Greater, Equal, Less三种类型


// fn  main() {  // fn声明一个函数
//     println!("Hello, world!");  // println! 这是一个宏，把文字输出到屏幕上
//     println!("Hello, world!");
//     let num = 1;
//     let bar = num;
//     // 在rust中，所以的遍历默认是不可变的
//     // 若要可变，前面要加入mut关键字
//     let mut guess = guessing::new();
//     // new()是关联函数，是对这个类进行实现，不是对这个实例
//     // new()是创建类型实例的惯用函数名
//     /*
//         let 声明变量
//         mut 变量可变
//         guessing::new() 创建一个空的字符串实例
// 
//     */
//     io::stdin().read_line(&mut guess)
//         .expect("error");
//     /*
//         调用io这个库 上的stdin()函数
//         其中有一个read_line这个方法，这是读取行
//         &mut: guess可变，且是使用guess的引用
//         注意，引用在rust中默认是不可变的
//         .expect() 报错时的提示信息
//         
//     */
//     
//     /* 在Cargo.toml中加入依赖，类似与maven，就可以直接导入包
//         版本会自动升级到第2个小数点前的版本的最新版
//             比如0.3.14 ==> 0.3.23
//         rust server会（修改toml时）自动更新版本
//     
//     
//     
//     
//      */
//     // 导入包后，第一次会编译依赖项
//     // Cargo.lock类似与package.json
//     // cargo update可以更新Cargo.lock中的包版本
//     
// 
//     println!("Guess: {}", guess);
// }

fn main() {
    println!("Hello, world!");
    
    // let secret_number = rand::thread_rng();
    let rng = rand::rng().random_range(1..101);
    // 0..100 表示[0,100)
    println!("reg = {}",rng);

    /*
    loop {
        // 这里是无限循环
    }
    */
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("err");

        /* rust中允许使用同名的新变量来隐藏
            原来同名的旧变量
            通常用在需要类型转换的场景中
         */
        // let guess:u32 = guess.trim().parse().expect("Please type a number!");
        // 变量:类型 显示声明变量类型 其中u32表示无符号整数类型

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 使用match代替expect处理错误
        println!("{}",guess);



        match guess.cmp(&rng){
            // guess.cmp表比较 , match是根据cmp的返回值来决定下一步做什么
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {println!("You win!");break;},
            Ordering::Greater => println!("Too big!"),
        }
    }
}
```

### 3
##### 变量与可变性
- 变量
> 声明变量使用let关键字
> 默认情况变量不变
> 声明变量时，在前面加入 mut ，就可以使变量可变

- 常量与变量
> 常量不可用用mut关键字
> 声明常量使用 const, 不能用 let, 且类型必须被标注
> 命名用大写: 多个单词用下划线隔开

- 隐藏
```rust
let x = 5;
let x = x + 1;
```
> 使用隐藏 let 声明的新变量类型可以和原变量不一样

##### 数据类型：标量类型
```rust
    println!("Hello, world!");
    let mut x = 5;
    println!{"x = {}",x};

    x = 6;


    println!{"x = {}",x};
    let x = "    ";
    let x = x.len();
    println!("x = {}",x);
    println!("MAX_POINTS = {}", MAX_POINTS);
```

> 如果parse()可能的类型比较多(例如吧String转为整数的parse方法), 就必须添加类型标注，否则编译会报错
```rust
let guess:u32 = "42".parse().expect("Not a number!");
``` 
 - [ x ]
```rust
let guess = "42".parse().expect("not a num ")
```
- 整数类型: 
- - u32, i32等(32表示占据32位的空间)
- - usize, isize: 根据计算机类型判断
---
 - - 整数字面值
 - - > 98_222, 0xff, 0o77, 0b11_00, b'A'(Byte类型)
 - - > 允许使用类型作为后缀
 - 整数溢出
 - 浮点类型
 - - f32, f64(默认)

 - 布尔类型
 - > bool  true false
 - 字符类型
 - > 'a'
   > 占4个字节大小，是Unicode的标量值


##### 数据类型：复合类型
