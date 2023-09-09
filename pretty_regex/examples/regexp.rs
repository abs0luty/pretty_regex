use pretty_regex::prelude::*;

fn main() {
    let pretty_regex =
        just("rege") + (just("x") + just("es").optional()) | (just("xp") + just("e").optional());

    let regex = pretty_regex.to_regex_or_panic();

    assert!(regex.is_match("regexes"));
    assert!(regex.is_match("regex"));
    assert!(regex.is_match("regexp"));
    assert!(regex.is_match("regexps"))
}
