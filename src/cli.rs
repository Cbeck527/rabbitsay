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
pub struct Args {
    /// message on the rabbit's sign
    pub message: String,

    /// maximum width of the rabbit's sign
    #[arg(short = 'w', long, default_value_t = 16)]
    pub max_width: usize,

    /// spacing around the message
    #[arg(short, long, default_value_t = 4)]
    pub spacing: usize,
}
