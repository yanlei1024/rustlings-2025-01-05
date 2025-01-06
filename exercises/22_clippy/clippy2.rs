fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: 修复此行中的Clippy lint(检查提示)。
    if let Some(x) = option {
        res += x;
    }

    println!("{res}");
}
