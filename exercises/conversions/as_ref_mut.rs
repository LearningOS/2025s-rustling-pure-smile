// as_ref_mut.rs
//
// AsRef and AsMut allow for cheap reference-to-reference conversions. Read more
// about them at https://doc.rust-lang.org/std/convert/trait.AsRef.html and
// https://doc.rust-lang.org/std/convert/trait.AsMut.html, respectively.
//
// Execute `rustlings hint as_ref_mut` or use the `hint` watch subcommand for a
// hint.



// Obtain the number of bytes (not characters) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.适当地添加AsRef特性作为特性绑定。

// 添加了 T: AsRef<str> 的 trait 约束，确保 arg 可以转换为 &str。
// 这样就可以调用 as_ref() 方法将 arg 转换为 &str，然后调用 as_bytes() 方法获取字节数。
use std::convert::AsRef;
use std::ops::DerefMut;
fn byte_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

// Obtain the number of characters (not bytes) in the given argument.
// TODO: Add the AsRef trait appropriately as a trait bound.

// 同样添加了 T: AsRef<str> 的 trait 约束。
// 调用 as_ref() 方法将 arg 转换为 &str，然后调用 chars() 方法获取字符数。
fn char_counter<T:AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// Squares a number using as_mut().
// TODO: Add the appropriate trait bound.

// std::fmt::Debug 用于调试目的，Copy 确保 T 类型可以被复制。
fn num_sq<T: AsMut<u32>>(arg: &mut T){
    // TODO: Implement the function body.
    // ???
    // 函数主体使用 *arg = *arg * *arg; 将 arg 的值平方。
    let val = arg.as_mut();
    *val = (*val) * (*val);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
