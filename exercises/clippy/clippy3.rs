// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.



/*
Option错误处理‌
‌数组语法错误‌
‌Vec错误使用‌
‌变量交换错误‌ 
*/

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    match my_option {
        Some(_) => println!("Got Some value!"),
        None => println!("Expected Some value but got None"),
    }

    let my_arr = &[
        -1, -2, -3,// 添加缺失的逗号
        -4, -5, -6
    ];
    println!("My array! Here it is: {:?}", my_arr);

    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let mut my_empty_vec = vec![1,2,3,4,5];
    my_empty_vec.clear();  // 正确使用方法：先存储Vec再调用resize
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    // 正确交换变量
    std::mem::swap(&mut value_a, &mut value_b);  // 使用内存交换函数
    println!("value a: {}; value b: {}", value_a, value_b);
}
