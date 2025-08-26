//! Basic example showing how to use the rabbitsay library.
//!
//! Run with: `cargo run --example basic`

use rabbitsay::{Config, say, say_with};

fn main() {
    println!("=== Simple API (recommended) ===\n");

    // The simplest way - just call say()!
    let output = say("Hello from rabbitsay! This is the easiest way to use the library.");
    println!("{}", output);

    println!("\n=== Custom Configuration ===\n");

    // With custom configuration for narrower output
    let config = Config {
        max_width: 20,
        padding: 2,
    };

    let output = say_with(
        "This is a longer message that will be wrapped at 20 characters!",
        config,
    );
    println!("{}", output);

    println!("\n=== Different Padding ===\n");

    // Minimal padding for a compact look
    let config = Config {
        max_width: 15,
        padding: 1,
    };

    let output = say_with("Compact rabbit says hi!", config);
    println!("{}", output);

    println!("\n=== Library Usage Example ===\n");

    // In a real library/TUI app, you might store the output
    let messages = vec![
        say("Error: File not found!"),
        say_with(
            "Warning: Low disk space",
            Config {
                max_width: 25,
                padding: 3,
            },
        ),
        say("Success!"),
    ];

    // Then display them as needed
    for (i, msg) in messages.iter().enumerate() {
        if i == 1 {
            // Only show the warning for this example
            println!("{}", msg);
        }
    }
}
