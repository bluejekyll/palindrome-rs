// Copyright (C) 2016 Benjamin Fry <benjaminfry@me.com>

use std::ascii::AsciiExt;

/// returns true if and only if the phrase is a palindrome
///
/// # Arguments
///
/// * `phrase` - the phrase to inspect
///
/// This is just to be fun with iterators...
/// This should be O(2n), and would be fewer lines without all the comments.
pub fn is_palindrome(phrase: &str) -> bool {
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

/// this is a class array implementation (for comparison)
///
/// # Arguments
///
/// * `phrase` - the phrase to inspect
///
/// This is definitely not the most efficient, but is meant show the difference to above. And be
///  close to the semantics used in C/C++, with the exception of the chars().collect() this should
///  be O(2n) like above.
pub fn is_palindrome_classic(phrase: &str) -> bool {
    // remove all none alphabetic characters
    let mut chars: Vec<char> = Vec::with_capacity(phrase.len());

    // specifically not using some of Rust's nice features for this copy
    //  but need to use the iterator b/c the std library is removing direct access to the chars
    //  i.e. there are better ways to do this ;)
    let phrase: Vec<char> = phrase.chars().collect();
    for i in 0..phrase.len() {
        // annoyingly traking a separate index
        let c = phrase[i];
        // ok, using push, but still...
        if c.is_alphabetic() { chars.push(c) }
    }

    let half = (chars.len() as f32 / 2f32).ceil() as usize;

    // now do the looping comparison
    for i in 0..half {
        // ug, indexes
        if chars[i].to_ascii_lowercase() !=
           // here's a classic off by one error you don't get in the iterator model
           chars[chars.len() - i - 1].to_ascii_lowercase() { return false }
    }

    // is this actually simpler or more readable? I don't think so...

    true
}

#[test]
fn test_is_palindrome() {
    test_is_palindrome_base(is_palindrome);
}

#[test]
fn test_palindrome_classic() {
    test_is_palindrome_base(is_palindrome_classic);
}

#[cfg(test)]
fn test_is_palindrome_base<F>(is_palindrome: F) where F: Fn(&str) -> bool {
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
