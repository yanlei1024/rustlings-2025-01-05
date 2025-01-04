fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);

    vec
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics2() {
        let vec0 = vec![22, 44, 66];

        // 对 `vec0` 进行克隆(clone)，
        // 这样移动到 `fill_vec` 中的是克隆后的副本，而不是`vec0` 本身。
        let vec1 = fill_vec(vec0.clone());

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
