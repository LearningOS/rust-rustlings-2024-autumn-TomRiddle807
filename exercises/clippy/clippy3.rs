// clippy3.rs
// 
// Here's a couple more easy Clippy fixes, so you can see its utility.
//
// Execute `rustlings hint clippy3` or use the `hint` watch subcommand for a hint.

fn main() {
    // 可能缺少逗号
    let _ = vec![-1, -2, -3];

    // 使用 unwrap() 将会导致 panic
    let my_option: Option<()> = None;
    if my_option.is_none() {
        // my_option.unwrap(); // 这里会导致 panic
    }

    // 看起来您试图交换 `value_a` 和 `value_b`
    let mut value_a = 1;
    let mut value_b = 2;
    std::mem::swap(&mut value_a, &mut value_b);

    // 在 `None` 值上使用了 `unwrap()`
    // let my_option: Option<()> = None;
    // my_option.unwrap(); // 这里会导致 panic

    // 此 let 绑定具有 unit 值
    // let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    vec![1, 2, 3, 4, 5].resize(0, 5);
    println!("This Vec is empty, see? {:?}", ());
}