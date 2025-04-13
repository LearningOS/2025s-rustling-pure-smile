// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let mut res = 42;
    let option = Some(12);
    //for循环替换为if let语句。这样代码会更清晰，也更符合Rust的惯用法。
    // for x in option {
    //     res += x;
    // }
    if let Some(x) = option{
        res += x;
    } 

    println!("{}", res);
}
