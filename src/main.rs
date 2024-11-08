use clap::Parser;

/// Say (possibly mean) things with a cute rabbit.
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = r"
     ┌───────────┐
     | rabbitsay |
     └───────────┘
  (\__/) ||
  (•ㅅ•) ||
  / 　 づ

  RabbitSay - say (possibly mean) things with a cute rabbit."
)]
struct Args {
    /// message on the rabbit's sign
    message: String,

    /// maximum width of the rabbit's sign
    #[arg(short = 'w', long, default_value_t = 16)]
    max_width: usize,

    /// spacing around the message
    #[arg(short, long, default_value_t = 4)]
    spacing: usize,
}

const RABBIT: &str = r"
(\__/) ||
(•ㅅ•) ||
/ 　 づ
";

fn wrap_text(text: &str, max_width: &usize) -> Vec<String> {
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

fn main() {
    let args = Args::parse();

    let max_content_width = &args.max_width;
    let lines = wrap_text(&args.message, &args.max_width);

    let spacing = &args.spacing;
    let width = max_content_width + spacing;
    let edge = "─".repeat(width);

    // Create the message box
    let innards: String = lines
        .iter()
        .map(|line| format!("    |{:^width$}|", line, width = width))
        .collect::<Vec<String>>()
        .join("\n");

    // Format the final output
    println!("    ┌{}┐\n{}\n    └{}┘{}", edge, innards, edge, RABBIT);
}
