use rabbitsay::{Config, say_with};

#[test]
fn test_full_message_flow() {
    let config = Config {
        max_width: 10,
        padding: 2,
    };
    let message = "Hello, this is a test message";
    let output = say_with(message, config);
    assert_eq!(
        output,
        r"  ┌───────────┐
  │  Hello,   │
  │ this is a │
  │   test    │
  │  message  │
  └───────────┘
(\__/) ||
(•ㅅ•) ||
/ 　 づ "
    )
}
