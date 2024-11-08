pub const RABBIT: &str = r"
(\__/) ||
(•ㅅ•) ||
/ 　 づ
";

pub fn wrap_text(text: &str, max_width: &usize) -> Vec<String> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in words {
        if current_line.len() + word.len() + 1 <= *max_width {
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        } else {
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_line = word.to_string();
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_text_short_message() {
        let text = "Hello world";
        let max_width = 16;
        let result = wrap_text(text, &max_width);
        assert_eq!(result, vec!["Hello world"]);
    }

    #[test]
    fn test_wrap_text_long_message() {
        let text = "This is a very long message that should be wrapped";
        let max_width = 10;
        let result = wrap_text(text, &max_width);
        assert_eq!(
            result,
            vec![
                "This is a",
                "very long",
                "message",
                "that",
                "should be",
                "wrapped"
            ]
        );
    }
}
