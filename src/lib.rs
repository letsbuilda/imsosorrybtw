//! I'm so sorry...

const WORD_REPLACE: [(&'static str, &'static str); 9] = [
    ("small", "smol"),
    ("cute", "kawaii~"),
    ("fluff", "floof"),
    ("love", "luv"),
    ("stupid", "baka"),
    ("idiot", "baka"),
    ("what", "nani"),
    ("meow", "nya~"),
    ("roar", "rawrr~"),
];

/// Replace certain words with uwuified equivalents, such as:
///
/// - `small` -> `smol`
/// - `cute` -> `kawaii~`
/// - `fluff` -> `floof`
/// - ...
///
/// # Example:
///
/// ```
/// use imsosorrybtw::replace_words;
/// let text = "small dogs";
/// let uwuified = replace_words(text);
///
/// assert_eq!(uwuified, "smol dogs");
/// ```
pub fn replace_words(text: &str) -> String {
    WORD_REPLACE
        .iter()
        .fold(text.to_string(), |acc, (word, replacement)| {
            acc.replace(word, replacement)
        })
}

/// Replace certain characters with their UwU equivalents:
///
/// - `r` -> `w`
/// - `l` -> `w`
/// - `n` -> `ny`
/// - `ove` -> `uv`
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

/// UwUify a string. This basically turns the text lowercase and applies every other function in
/// this crate to a string in order:
///
/// - `replace_words`
/// - `char_replace`
#[allow(clippy::doc_markdown)]
#[must_use]
pub fn uwuify(text: &str) -> String {
    let mut text = text.to_lowercase();
    text = replace_words(&text);
    text = char_replace(&text);

    text
}
