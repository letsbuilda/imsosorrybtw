//! I'm so sorry...

/// Replace certain characters with their UwU equivalents:
///
/// - `r` -> `w`
/// - `l` -> `w`
/// - `n` -> `ny`
/// - `ove` -> `uv`
///
/// This will **NOT** account for capitalization.
///
/// # Example
///
/// ```
/// use imsosorrybtw::char_replace;
///
/// let text = "I'm so sorry...";
/// let uwuified = char_replace(text);
///
/// assert_eq!(uwuified, "I'm so sowwy...");
/// ```
pub fn char_replace(text: &str) -> String {
    // TODO: We should probably account for capitalization here via a regex or something.

    text.replace("r", "w")
        .replace("l", "w")
        .replace("n", "ny")
        .replace("ove", "uv")
}

/// UwUify a string. This basically applies every other function in this crate to a string in order:
///
/// - `char_replace`
pub fn uwuify(text: &str) -> String {
    char_replace(text)
}
