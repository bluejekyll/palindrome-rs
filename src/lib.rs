// Copyright (C) 2016 Benjamin Fry <benjaminfry@me.com>

use std::ascii::AsciiExt;

/// returns true if and only if the phrase is a palindrome
///
/// # Arguments
///
/// `phrase` - the phrase to inspect
///
/// This is just to be fun with iterators...
pub fn is_palindrome(phrase: &str) -> bool {
    println!("phrase: {}", phrase);
    // remove all none alphabetic characters
    let chars: Vec<char> = phrase.chars().filter(|c| c.is_alphabetic()).collect();
    let half = (chars.len() as f32 / 2f32).ceil() as usize;

    // take the first half
    chars.iter().take(half)
                // zip with the second half...
                .zip(chars.iter()
                          // which needs to be reversed...
                          .rev()
                          // take the last half
                          .take(half))
                // check that all the chars from the begining and end match
                .all(|(first_char, last_char)| {
                    first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
                })
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome(""));
    assert!(is_palindrome("A man, a plan, a canal, Panama!"));
    assert!(is_palindrome("Amor, Roma"));
    assert!(is_palindrome("race car"));
    assert!(is_palindrome("stack cats"));
    assert!(is_palindrome("step on no pets"));
    assert!(is_palindrome("taco cat"));
    assert!(is_palindrome("put it up"));
    assert!(is_palindrome("Was it a car or a cat I saw?"));
    assert!(is_palindrome("No ‘x’ in Nixon"));

    // negative for good messure
    assert!(!is_palindrome("Not a palindrome"))
}
