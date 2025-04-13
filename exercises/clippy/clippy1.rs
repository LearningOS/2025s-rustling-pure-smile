// clippy1.rs
//
// The Clippy tool is a collection of lints to analyze your code so you can
// catch common mistakes and improve your Rust code.
//
// For these exercises the code will fail to compile when there are clippy
// warnings check clippy's suggestions from the output to solve the exercise.
//
// Execute `rustlings hint clippy1` or use the `hint` watch subcommand for a
// hint.


//调用
use std::f32::consts::PI;

fn main() {
    // ‌硬编码π值不精确‌：
    //let pi = 3.14f32; // 近似值，精度不足// Clippy会警告应使用标准库提供的精确π常量。
    let radius = 5.00f32;
    // let area = pi * f32::powi(radius, 2);这种写法不符合Rust的面向对象调用风格。
    let area = PI * radius.powi(2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius,
        area
    );
}
