use clap::Parser;
use rabbitsay::cli::Args;
use rabbitsay::text::wrap_text;
use rabbitsay::format_message;

fn main() {
    let args = Args::parse();
    let lines = wrap_text(&args.message, &args.max_width);
    let output = format_message(&lines, args.max_width, args.spacing);
    println!("{}", output);
}
