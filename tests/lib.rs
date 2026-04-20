//! Tests for the `imsosorrybtw` crate.
//! If you're apologizing, might as well do it properly!

use imsosorrybtw::uwuify;

/// Test if `uwuify` turns the text lowercase.
#[test]
fn test_uwuify_lowercase() {
    let text = "i'M sO SoRrY...";
    let uwuified = uwuify(text);

    assert_eq!(uwuified, "i'm so sowwy...");
}
