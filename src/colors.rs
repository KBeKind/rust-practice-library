
/// Returns a string wrappind in ANSI red color codes.
/// # Examples:
/// ```
/// use rust-proj-5::colors::red;
/// let red_string = red("Hello this string is red");
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}