use std::io; // 使用了io这个库，io库是在标准库中的
            // prelude

fn  main() {  // fn声明一个函数
    println!("Hello, world!");  // println! 这是一个宏，把文字输出到屏幕上  
    println!("Hello, world!");
    let num = 1;
    let bar = num;
    // 在rust中，所以的遍历默认是不可变的
    // 若要可变，前面要加入mut关键字
    let mut guess = String::new();
    // new()是关联函数，是对这个类进行实现，不是对这个实例
    // new()是创建类型实例的惯用函数名
    /*
        let 声明变量 
        mut 变量可变
        String::new() 创建一个空的字符串实例
           
    */
    io::stdin().read_line(&mut guess)
        .expect("error");
    /*
        调用io这个库 上的stdin()函数
        其中有一个read_line这个方法，这是读取行
        &mut: guess可变，且是使用guess的引用
        注意，引用在rust中默认是不可变的
        .expect() 报错时的提示信息
    */
    
    println!("Guess: {}", guess);
}