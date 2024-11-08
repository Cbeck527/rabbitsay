pub mod cli;
pub mod text;

pub fn format_message(lines: &[String], width: usize, spacing: usize) -> String {
    let total_width = width + spacing;
    let edge = "─".repeat(total_width);

    let innards: String = lines
        .iter()
        .map(|line| format!("    |{:^width$}|", line, width = total_width))
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        "    ┌{}┐\n{}\n    └{}┘{}",
        edge,
        innards,
        edge,
        text::RABBIT
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_message_single_line() {
        let lines = vec!["Hello".to_string()];
        let width = 10;
        let spacing = 2;
        let result = format_message(&lines, width, spacing);
        let expected = format!(
            "    ┌{}┐\n    |{}|\n    └{}┘{}",
            "─".repeat(12),
            "   Hello    ",
            "─".repeat(12),
            text::RABBIT
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_format_message_multiple_lines() {
        let lines = vec!["Hello".to_string(), "World".to_string()];
        let width = 10;
        let spacing = 2;
        let result = format_message(&lines, width, spacing);
        let expected = format!(
            "    ┌{}┐\n    |{}|\n    |{}|\n    └{}┘{}",
            "─".repeat(12),
            "   Hello    ",
            "   World    ",
            "─".repeat(12),
            text::RABBIT
        );
        assert_eq!(result, expected);
    }
}
