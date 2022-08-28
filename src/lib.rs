pub fn random_greeting() -> &'static str {
    let greetings = ["안녕하세요", "こんにちは", "你好", "สวัสดีครับ", "Xin chào"];
    let test = fastrand::usize(0..5);
    greetings[test]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_random_greeting() {
        fastrand::seed(1);
        let result = random_greeting();
        assert_eq!(result, "Xin chào");
    }
}
