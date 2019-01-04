// Copyright (C) 2016 Benjamin Fry <benjaminfry@me.com>

use std::ascii::AsciiExt;

/// returns true if and only if the phrase is a palindrome
///
/// # Arguments
///
/// * `phrase` - the phrase to inspect
///
/// This is just to have fun with iterators...
/// This should be O(n/2), and would be fewer lines without all the comments.
pub fn is_palindrome(phrase: &str) -> bool {
    // get the chars iterator and associated index
    phrase.char_indices().filter(|&(_,c)| c.is_alphabetic())
                // zip with the second half...
                .zip(phrase.char_indices()
                           // which needs to be reversed...
                           .rev()
                           // and filter out bad cars
                           .filter(|&(_,c)| c.is_alphabetic()))
                // accept all input until the indexes have crossed
                .take_while(|&((first_count, _), (last_count, _))| {first_count < last_count})
                // check that all the chars from the begining and end match
                .all(|((_, first_char), (_, last_char))| {
                    first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
                })
}

/// this is a classic array implementation (for comparison)
///
/// # Arguments
///
/// * `phrase` - the phrase to inspect
///
/// This is definitely not the most efficient, but is meant show the difference to above. And be
///  close to the semantics used in C/C++, with the exception of the chars().collect() this should
///  be O(n/2).
pub fn is_palindrome_classic(phrase: &str) -> bool {
    if phrase.len() == 0 { return true }

    // need to do this because Rust is rightly trying to force you away from treating strs as
    //  arrays
    let phrase: Vec<char> = phrase.chars().collect();

    // start from the beginning
    let mut first_idx = 0;

    // and the end, btw, don't forget the off-by-one b/c of len() is actually past the last index...
    //  this is a classic error avoided implicitly above.
    let mut last_idx = phrase.len() - 1;
    // loop and guard that we don't go too far
    while first_idx < last_idx {
        // filter out non-alphabetics, the += and -= would be something you could accidentally screw up,
        //   avoided in the iterator based impl
        if !phrase[first_idx].is_alphabetic() { first_idx += 1; continue }
        if !phrase[last_idx].is_alphabetic() { last_idx -= 1; continue }

        // compare the values, did we compare the correct indexes? again avoided in the iterator impl
        if phrase[first_idx].to_ascii_lowercase() != phrase[last_idx].to_ascii_lowercase() {
            return false;
        }

        // same += and -= potential bug avoided in the iterator impl
        first_idx += 1;
        last_idx -= 1;
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
    assert!(is_palindrome("a"));
    assert!(is_palindrome(".,,/.,/.,././.,=-=--"));
    assert!(is_palindrome("a,,,,,,,,,,,,,,,a"));
    assert!(is_palindrome("a,,,,,bb,,,,,,,,a"));
    assert!(is_palindrome("race car"));
    assert!(is_palindrome("racecar"));
    assert!(is_palindrome("raccar"));
    assert!(is_palindrome("A man, a plan, a canal, Panama!"));
    assert!(is_palindrome("Amor, Roma"));
    assert!(is_palindrome("stack cats"));
    assert!(is_palindrome("step on no pets"));
    assert!(is_palindrome("taco cat"));
    assert!(is_palindrome("put it up"));
    assert!(is_palindrome("Was it a car or a cat I saw?"));
    assert!(is_palindrome("No ‘x’ in Nixon"));

    // negative for good messure
    assert!(!is_palindrome("Not a palindrome"))
}
