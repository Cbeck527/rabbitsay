use clap::Parser;
use rabbitsay::{Config, say_with};

/// Say (possibly mean) things with a cute rabbit.
#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = r"
   ┌───────────┐
   │ rabbitsay │
   └───────────┘
(\__/) ||
(•ㅅ•) ||
/ 　 づ

RabbitSay - say (possibly mean) things with a cute rabbit."
)]
struct Cli {
    /// padding around the message
    #[arg(short = 'p', long, default_value_t = 4)]
    pub padding: usize,

    /// maximum width of the sign
    #[arg(short = 'w', long, default_value_t = 16)]
    pub max_width: usize,

    /// message on the rabbit's sign
    pub message: String,
}

fn main() {
    let cli = Cli::parse();

    let config = Config {
        max_width: cli.max_width,
        padding: cli.padding,
    };

    println!("{}", say_with(&cli.message, config));
}
