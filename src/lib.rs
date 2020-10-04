#![forbid(unsafe_code)]

use std::iter::FromIterator;

//TODO: Support ::before and the like.
pub fn scope_css(input: &str, selector: &str) -> String {
    let mut processed = 0usize;
    let mut whitespace_start = 0usize;
    let mut at_found = false;
    let mut result = String::with_capacity(2 * input.len());

    let mut i = 0;
    for c in input.chars() {
        let c_len = String::from_iter(&[c]).len();
        match c {
            '@' => at_found = true,
            w if w.is_whitespace() => (),
            '{' | ',' => {
                if !at_found {
                    //TODO: This still messes up if there's multibyte characters anywhere.
                    result = result + &input[processed..whitespace_start] + selector;
                    processed = whitespace_start;
                }
                whitespace_start = i + c_len;
                at_found = false;
            }
            '}' => {
                whitespace_start = i + c_len;
                at_found = false;
            }
            _ => whitespace_start = i + c_len,
        }
        i += c_len;
    }

    result += &input[processed..];
    result = result.replace(":scope", &format!(":not({0})>{0}", selector));
    result.shrink_to_fit();
    result
}
