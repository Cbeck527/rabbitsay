//! A fun library for displaying messages with a cute ASCII rabbit.
//!
//! This library provides both a simple high-level API and low-level
//! building blocks for creating rabbitsay messages. Perfect for CLI tools,
//! TUI applications, or anywhere you need a cute rabbit to say something!
//!
//! # Quick Start
//!
//! The simplest way to use rabbitsay:
//!
//! ```
//! use rabbitsay::say;
//!
//! let output = say("Hello, world!");
//! println!("{}", output);
//! ```
//!
//! # Custom Configuration
//!
//! For more control over formatting:
//!
//! ```
//! use rabbitsay::{say_with, Config};
//!
//! let config = Config {
//!     max_width: 20,
//!     padding: 2,
//! };
//!
//! let output = say_with("A longer message that needs wrapping", config);
//! println!("{}", output);
//! ```
//!
//! # Low-Level API
//!
//! For complete control, use the individual functions:
//!
//! ```
//! use rabbitsay::{wrap_text, build_sign, indent_rabbit, RABBIT_HANDLE_POSITION};
//!
//! let message = "Hello, world!";
//! let lines = wrap_text(message, 20);
//! let mut sign = build_sign(&lines, 4);
//!
//! let sign_width = sign.first()
//!     .map(|line| line.chars().count())
//!     .unwrap_or(0);
//!
//! // For narrow signs, indent the sign to align with the rabbit's handle
//! // For wide signs, indent the rabbit to center its handle under the sign
//! let (sign_indent, rabbit_indent) = if sign_width < RABBIT_HANDLE_POSITION * 2 {
//!     (RABBIT_HANDLE_POSITION.saturating_sub(sign_width / 2), 0)
//! } else {
//!     (0, (sign_width / 2).saturating_sub(RABBIT_HANDLE_POSITION))
//! };
//!
//! if sign_indent > 0 {
//!     let indent_str = " ".repeat(sign_indent);
//!     sign = sign.iter()
//!         .map(|line| format!("{}{}", indent_str, line))
//!         .collect();
//! }
//!
//! println!("{}", sign.join("\n"));
//! println!("{}", indent_rabbit(rabbit_indent));
//! ```

/// Configuration for generating rabbitsay messages.
///
/// # Examples
///
/// ```
/// use rabbitsay::Config;
///
/// // Use default configuration
/// let config = Config::default();
///
/// // Custom configuration
/// let config = Config {
///     max_width: 20,
///     padding: 2,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum width for text wrapping
    pub max_width: usize,
    /// Padding around the message in the box
    pub padding: usize,
}

impl Default for Config {
    /// Creates a default configuration with reasonable values.
    ///
    /// - `max_width`: 40 characters
    /// - `padding`: 4 characters
    fn default() -> Self {
        Self {
            max_width: 40,
            padding: 4,
        }
    }
}

/// Position of the rabbit's handle (||) center in the ASCII art.
/// The handle is at columns 7-8, so we center at position 8.
pub const RABBIT_HANDLE_POSITION: usize = 8;

/// Wraps text to fit within a maximum width, breaking at word boundaries.
///
/// Words longer than `max_width` will be placed on their own line.
/// Leading and trailing whitespace is trimmed from each line.
///
/// # Arguments
///
/// * `text` - The input text to wrap
/// * `max_width` - Maximum number of characters per line
///
/// # Returns
///
/// A vector of strings, each representing one line of wrapped text.
///
/// # Examples
///
/// ```
/// use rabbitsay::wrap_text;
///
/// let lines = wrap_text("Hello world", 5);
/// assert_eq!(lines, vec!["Hello", "world"]);
///
/// let lines = wrap_text("Hello beautiful world", 10);
/// assert_eq!(lines, vec!["Hello", "beautiful", "world"]);
/// ```
pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();
    for word in text.split_whitespace() {
        let space_needed = if current_line.is_empty() { 0 } else { 1 };
        let total_length = current_line.len() + space_needed + word.len();

        if total_length > max_width {
            if !current_line.is_empty() {
                lines.push(current_line.trim().to_string())
            }
            current_line = word.to_string();
        } else {
            if !current_line.is_empty() {
                current_line.push(' ')
            }
            current_line.push_str(word);
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line.trim().to_string());
    }

    lines
}

/// Builds a bordered message box around the provided lines of text.
///
/// Creates a box using Unicode box-drawing characters (┌─┐│└─┘) with
/// the text centered inside. The box width is determined by the longest
/// line plus padding.
///
/// # Arguments
///
/// * `lines` - The lines of text to display in the box
/// * `padding` - Extra space to add around the text (applied to width)
///
/// # Returns
///
/// A vector of strings representing each line of the bordered box.
///
/// # Examples
///
/// ```
/// use rabbitsay::build_sign;
///
/// let lines = vec!["Hello".to_string(), "World".to_string()];
/// let sign = build_sign(&lines, 2);
///
/// assert_eq!(sign[0], "┌───────┐");
/// assert_eq!(sign[1], "│ Hello │");
/// assert_eq!(sign[2], "│ World │");
/// assert_eq!(sign[3], "└───────┘");
/// ```
pub fn build_sign(lines: &[String], padding: usize) -> Vec<String> {
    let longest_line = lines.iter().map(|line| line.len()).max().unwrap_or(0);
    let total_width = longest_line + padding;

    let mut sign_parts = vec![format!("┌{}┐", "─".repeat(total_width))];
    for line in lines {
        sign_parts.push(format!("│{line:^total_width$}│"))
    }
    sign_parts.push(format!("└{}┘", "─".repeat(total_width)));

    sign_parts
}

/// Creates an ASCII art rabbit with the specified indentation.
///
/// The rabbit is 10 characters wide with its handle (||) at position 7-8.
/// Each line of the rabbit will be prefixed with the specified number of spaces.
///
/// # Arguments
///
/// * `indent` - Number of spaces to add before each line of the rabbit
///
/// # Returns
///
/// A string containing the indented rabbit ASCII art.
///
/// # Examples
///
/// ```
/// use rabbitsay::indent_rabbit;
///
/// let rabbit = indent_rabbit(4);
/// assert!(rabbit.contains("(\\__/)"));  // The rabbit ears
/// assert!(rabbit.contains("(•ㅅ•)"));   // The rabbit face
/// assert!(rabbit.starts_with("\n"));   // Starts with newline
/// ```
pub fn indent_rabbit(indent: usize) -> String {
    let i = " ".repeat(indent);
    format!(
        r"
{i}(\__/) ||
{i}(•ㅅ•) ||
{i}/ 　 づ "
    )
}

/// Generate a complete rabbitsay message with default configuration.
///
/// This is the simplest way to use rabbitsay as a library. It uses
/// reasonable defaults (40 character width, 4 character padding).
///
/// # Arguments
///
/// * `message` - The message to display
///
/// # Returns
///
/// A string containing the complete rabbitsay output, ready to print.
///
/// # Examples
///
/// ```
/// use rabbitsay::say;
///
/// let output = say("Hello, world!");
/// println!("{}", output);
/// // Displays:
/// // ┌────────────────┐
/// // │  Hello, world! │
/// // └────────────────┘
/// //    (\__/) ||
/// //    (•ㅅ•) ||
/// //    / 　 づ
/// ```
pub fn say(message: &str) -> String {
    say_with(message, Config::default())
}

/// Generate a rabbitsay message with custom configuration.
///
/// This function allows full control over the text wrapping width
/// and padding around the message.
///
/// # Arguments
///
/// * `message` - The message to display
/// * `config` - Configuration for text wrapping and padding
///
/// # Returns
///
/// A string containing the complete rabbitsay output, ready to print.
///
/// # Examples
///
/// ```
/// use rabbitsay::{say_with, Config};
///
/// let config = Config {
///     max_width: 20,
///     padding: 2,
/// };
///
/// let output = say_with("Hello, this is a longer message!", config);
/// println!("{}", output);
/// ```
pub fn say_with(message: &str, config: Config) -> String {
    let lines = wrap_text(message, config.max_width);
    let mut sign = build_sign(&lines, config.padding);

    let sign_width = sign.first().map(|line| line.chars().count()).unwrap_or(0);

    // If the sign is narrower than twice the handle position,
    // indent the sign so its center aligns with the handle
    let sign_indent = if sign_width < RABBIT_HANDLE_POSITION * 2 {
        RABBIT_HANDLE_POSITION.saturating_sub(sign_width / 2)
    } else {
        0
    };

    // Apply indentation to sign if needed
    if sign_indent > 0 {
        let indent_str = " ".repeat(sign_indent);
        sign = sign
            .iter()
            .map(|line| format!("{}{}", indent_str, line))
            .collect();
    }

    // After indenting the sign, recalculate the rabbit indent
    // If we indented the sign, rabbit stays at position 0
    // Otherwise, center the handle under the sign
    let rabbit_indent = if sign_indent > 0 {
        0 // Sign was indented to align with handle, rabbit stays put
    } else {
        (sign_width / 2).saturating_sub(RABBIT_HANDLE_POSITION)
    };

    format!("{}{}", sign.join("\n"), indent_rabbit(rabbit_indent))
}
