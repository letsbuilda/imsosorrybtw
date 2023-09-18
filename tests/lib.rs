//! Tests for the `imsosorrybtw` crate.
//! If you're apologizing, might as well do it properly!

use imsosorrybtw::{char_replace, uwuify};

/// Test the `char_replace` function.
#[test]
fn test_char_replace() {
    // This test is the same as the example in the docs, but it's good to have
    // it here as well.

    let text = "I'm so sorry...";
    let uwuified = char_replace(text);

    assert_eq!(uwuified, "I'm so sowwy...");
}

/// Test if `uwuify` turns the text lowercase.
#[test]
fn test_uwuify_lowercase() {
    let text = "I'm so sorry...";
    let uwuified = uwuify(text);

    assert_eq!(uwuified, "i'm so sowwy...");
}
