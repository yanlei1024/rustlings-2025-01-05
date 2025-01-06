// 结构体(Structs)包含数据(data)，但也可以包含逻辑(logic)。
// 在这个练习中，我们已经定义了 `Package` 结构体，并且我们想要测试一些与之相关的逻辑。

#[derive(Debug)]
struct Package {
    // 发件人所处的国家
    sender_country: String,
    // 收件人所处的国家
    recipient_country: String,
    // 包裹的重量
    weight_in_grams: u32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: u32) -> Self {
        if weight_in_grams < 10 {
            // 在Rust中不应该这样处理错误，不过我们稍后会学习错误处理相关的内容。
            panic!("无法运输重量低于10克的包裹");
        }

        Self {
            sender_country,
            recipient_country,
            weight_in_grams,
        }
    }

    // TODO: 给函数签名添加正确的返回类型。
    fn is_international(&self) -> bool {
        // TODO: 阅读使用此方法的测试，以了解在什么情况下一个包裹会被视为国际包裹。
        self.sender_country != self.recipient_country
    }

    // TODO: 给函数签名添加正确的返回类型。
    fn get_fees(&self, cents_per_gram: u32) -> u32 {
        // TODO: 计算包裹的费用。
        cents_per_gram * self.weight_in_grams
    }
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");

        Package::new(sender_country, recipient_country, 5);
    }

    #[test]
    fn create_international_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Russia");

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let sender_country = String::from("Canada");
        let recipient_country = sender_country.clone();

        let package = Package::new(sender_country, recipient_country, 1200);

        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Spain");

        let cents_per_gram = 3;

        let package = Package::new(sender_country, recipient_country, 1500);

        assert_eq!(package.get_fees(cents_per_gram), 4500);
        assert_eq!(package.get_fees(cents_per_gram * 2), 9000);
    }
}
