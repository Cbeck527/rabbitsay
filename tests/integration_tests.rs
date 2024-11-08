use rabbitsay::{cli::Args, format_message, text::wrap_text};

#[test]
fn test_full_message_flow() {
    // Simulate CLI args
    let args = Args {
        message: "Hello, this is a test message".to_string(),
        max_width: 10,
        spacing: 2,
    };

    // Process the message
    let lines = wrap_text(&args.message, &args.max_width);
    let output = format_message(&lines, args.max_width, args.spacing);

    assert_eq!(
        output,
        r"    ┌────────────┐
    |   Hello,   |
    | this is a  |
    |    test    |
    |  message   |
    └────────────┘
(\__/) ||
(•ㅅ•) ||
/ 　 づ
"
    )
}
