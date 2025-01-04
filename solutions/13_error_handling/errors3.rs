// 这是一个程序，它尝试使用上一个练习中完成的 `total_cost` 函数。
// 然而并不起作用! 为什么呢? 我们应该如何修复它?

use std::num::ParseIntError;

// 不要修改此函数。
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}

fn main() -> Result<(), ParseIntError> {
    //    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ 添加此处内容
    let mut tokens = 100;
    let pretend_user_input = "8";

    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {tokens} tokens.");
    }

    // 添加了这一行代码，用于返回期望的 `Result` 类型中的 `Ok` 变体。
    Ok(())
}
