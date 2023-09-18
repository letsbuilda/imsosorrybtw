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
#[allow(clippy::doc_markdown)]
#[must_use]
pub fn char_replace(text: &str) -> String {
    // TODO: We should probably account for capitalization here via a regex or something.

    text.replace(['r', 'l'], "w")
        .replace('n', "ny")
        .replace("ove", "uv")
}

/// UwUify a string. This basically applies every other function in this crate to a string in order:
///
/// - `to_lowercase` (standard string method, not in crate)
/// - `char_replace`
#[allow(clippy::doc_markdown)]
#[must_use]
pub fn uwuify(text: &str) -> String {
    let text = text.to_lowercase();
    char_replace(&text)
}
