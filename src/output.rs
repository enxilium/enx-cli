// Terminal output formatting. Defines functions to print colorized/styled messages to STDOUT.

use colored::Colorize;

pub fn info(message: &str) {
    println!("{} {}", "➤".cyan(), message);
}

pub fn success(message: &str) {
    println!("{} {}", "✓".green(), message);
}

pub fn warning(message: &str) {
    println!("{} {}", "⚠".yellow(), message);
}

pub fn error(message: &str) {
    println!("{} {}", "✗".red(), message);
}

pub fn header(message: &str) {
    println!("{}", message.bold());
}

pub fn detail(message: &str) {
    println!("    {} {}", "∟".dimmed(), message.dimmed());
}