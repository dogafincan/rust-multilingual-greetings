pub fn random_greeting() -> &'static str {
    let greetings = ["안녕하세요", "こんにちは", "你好", "สวัสดีครับ", "Xin chào"];
    debug_assert!(greetings.len() > 0, "expected at least one greeting");

    for greeting in greetings {
        debug_assert!(greeting.len() > 0, "expected a non-empty string")
    }

    let random_index = fastrand::usize(0..greetings.len());
    greetings[random_index]
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
