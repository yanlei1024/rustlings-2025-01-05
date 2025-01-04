fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        output.push(2 * element);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // 下面是一个先对元素进行映射(map)，然后收集(collect)成动态数组的例子。
    // 我们先把`input`切片里的每个元素都变成(映射)其原本的值加1 。
    //         1 -> 1 + 1 => 2
    //         2 -> 2 + 1 => 3
    //         3 -> 3 + 1 => 4
    //         n -> n + 1 => n + 1
    // 例如，如果输入的是 `[1, 2, 3]` ，那得到的输出就会是 `[2, 3, 4]` 。
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // 我们之后会更深入地探讨迭代器(iterators)，不过就目前而言，这些就足够了!
    // 进阶提示：这个方法效率更高，因为它会自动预先分配足够的容量(capacity)。
    // 在 `vec_loop` 方法中，可以通过使用 `Vec::with_capacity(input.len())` 来替代 `Vec::new()` 手动完成这一操作。 
    input.iter().map(|element| 2 * element).collect()
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
