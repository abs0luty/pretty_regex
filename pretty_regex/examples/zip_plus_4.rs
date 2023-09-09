use pretty_regex::prelude::*;

fn main() {
    let pretty_regex = digit() * 5 + (just("-") + digit() * 4).optional();
    let regex = pretty_regex.to_regex_or_panic();

    assert!(regex.is_match("12345-6789"));
    assert!(regex.is_match("12345"));
}
