//! Terminal output formatting.
//!
//! Provides color-coded helpers, a shared dialoguer theme, and an ASCII
//! art banner — all styled with a **Catppuccin Mocha**-inspired palette.
//!
//! # Crate foundations
//!
//! * [`console`] — styling (`style()`), terminal I/O (`Term`)
//! * [`dialoguer`] — interactive prompts (`Input`, `Confirm`, `FuzzySelect`)
//! * [`indicatif`] — progress bars and spinners
//!
//! The `console` crate is the shared foundation that `dialoguer` and
//! `indicatif` build on, so colors and styles are consistent everywhere.

use console::{Style, style};
use dialoguer::theme::ColorfulTheme;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

// ── Catppuccin Mocha palette (RGB) ──────────────────────────────────────
//
// We use `style(text).color256(n)` when broad 256-color support is enough,
// but for our curated palette the `console` crate's `Style::from_dotted_str`
// method doesn't cover RGB — so we reach for `style().fg(Color::Rgb { .. })`.
// `console` re-exports `Color` from the `owo-colors` crate.

/// Convenience wrapper: apply the **Lavender** foreground to `text`.
fn lavender(text: &str) -> console::StyledObject<&str> {
    style(text).color256(183) // closest 256-color to Catppuccin Lavender
}

/// **Green** accent (Catppuccin "Green").
fn green(text: &str) -> console::StyledObject<&str> {
    style(text).color256(114)
}

/// **Peach** accent (Catppuccin "Peach").
fn peach(text: &str) -> console::StyledObject<&str> {
    style(text).color256(216)
}

/// **Red** accent (Catppuccin "Red").
fn red(text: &str) -> console::StyledObject<&str> {
    style(text).color256(211)
}

/// **Mauve** accent (Catppuccin "Mauve") — used for headers and emphasis.
fn mauve(text: &str) -> console::StyledObject<&str> {
    style(text).color256(183)
}

/// **Teal** accent — used for key names.
#[allow(dead_code)]
fn teal(text: &str) -> console::StyledObject<&str> {
    style(text).color256(116)
}

/// **Overlay0** — dimmed/subtle text.
fn dim(text: &str) -> console::StyledObject<&str> {
    style(text).color256(102)
}

// ── Public output helpers ───────────────────────────────────────────────

/// Informational message: lavender `➤` prefix.
pub fn info(message: &str) {
    println!("{} {}", lavender("➤"), message);
}

/// Success message: green `✓` prefix.
pub fn success(message: &str) {
    println!("{} {}", green("✓"), message);
}

/// Warning message (written to **stderr**): peach `⚠` prefix.
pub fn warning(message: &str) {
    eprintln!("{} {}", peach("⚠"), message);
}

/// Error message (written to **stderr**): red `✗` prefix.
pub fn error(message: &str) {
    eprintln!("{} {}", red("✗"), message);
}

/// Section header: mauve, bold.
pub fn header(message: &str) {
    println!("{}", style(message).color256(183).bold());
}

/// Detail line with a Unicode tree connector (`├─`).
///
/// Use for all items in a list *except* the last one — pair with
/// [`detail_last()`] for the final item.
pub fn detail(message: &str) {
    println!("  {} {}", dim("├─"), dim(message));
}

/// Last detail line in a tree (`└─`).
pub fn detail_last(message: &str) {
    println!("  {} {}", dim("└─"), dim(message));
}

/// Completed step indicator: green `✓` with indent.
pub fn step_ok(label: &str) {
    println!("  {} {}", green("✓"), label);
}

/// Failed step indicator: red `✗` with indent.
pub fn step_fail(label: &str) {
    println!("  {} {}", red("✗"), label);
}

/// Key–value pair: teal key, dimmed separator, then value.
#[allow(dead_code)]
pub fn key_value(key: &str, value: &str) {
    println!("  {} {} {}", teal(key), dim(":"), value);
}

/// Print an empty line — useful as a visual separator between sections.
pub fn newline() {
    println!();
}

// ── ASCII art banner ────────────────────────────────────────────────────

/// Print the block-letter **enx** banner (Catppuccin Mauve).
///
/// Used by `enx setup` as the first-impression splash screen.
pub fn banner() {
    let art = [
        r"  ███████╗███╗   ██╗██╗  ██╗",
        r"  ██╔════╝████╗  ██║╚██╗██╔╝",
        r"  █████╗  ██╔██╗ ██║ ╚███╔╝ ",
        r"  ██╔══╝  ██║╚██╗██║ ██╔██╗ ",
        r"  ███████╗██║ ╚████║██╔╝ ██╗",
        r"  ╚══════╝╚═╝  ╚═══╝╚═╝  ╚═╝",
    ];

    newline();
    for line in &art {
        println!("{}", mauve(line).bold());
    }
    println!("       {}", dim("developer workflow tool"));
    newline();
}

// ── Shared dialoguer theme ──────────────────────────────────────────────

/// Return a Catppuccin-styled [`ColorfulTheme`] for dialoguer prompts.
///
/// Use via `Input::with_theme(&output::theme())` (and the same for
/// `Confirm`, `Select`, `FuzzySelect`, etc.) so every interactive prompt
/// has a consistent look.
pub fn theme() -> ColorfulTheme {
    ColorfulTheme {
        prompt_prefix: style("?".to_string()).color256(183).bold(),
        success_prefix: style("✓".to_string()).color256(114).bold(),
        error_prefix: style("✗".to_string()).color256(211).bold(),
        hint_style: Style::new().color256(102),
        values_style: Style::new().color256(116),
        active_item_style: Style::new().color256(183).bold(),
        inactive_item_style: Style::new().color256(102),
        active_item_prefix: style("❯".to_string()).color256(183),
        picked_item_prefix: style("✓".to_string()).color256(114),
        unpicked_item_prefix: style(" ".to_string()),
        ..ColorfulTheme::default()
    }
}

// ── Spinner helpers ─────────────────────────────────────────────────────

/// Create a Catppuccin-styled spinner with `message` as the initial label.
///
/// The caller drives the spinner by calling methods on the returned
/// [`ProgressBar`]. Typical lifecycle:
///
/// ```ignore
/// let sp = output::spinner("Cloning repository...");
/// // … do work …
/// sp.finish_with_message("✓ Cloned repository");
/// ```
pub fn spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ")
            .template("  {spinner:.magenta} {msg}")
            .expect("valid spinner template"),
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}
