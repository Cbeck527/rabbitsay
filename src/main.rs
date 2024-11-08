use clap::Parser;
use rabbitsay::cli::Args;
use rabbitsay::format_message;
use rabbitsay::text::wrap_text;

fn main() {
    let args = Args::parse();
    let lines = wrap_text(&args.message, &args.max_width);
    let output = format_message(&lines, args.max_width, args.spacing);
    println!("{}", output);
}
