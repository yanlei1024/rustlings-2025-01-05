// TODO: 通过移动这个宏的整个定义语句来修复编译器错误。
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
