//! # 🔮 Write readable regular expressions
//!
//! The crate provides a clean and readable way of writing your regex in the Rust programming language:
//!
//! <table>
//! <tr>
//! <td>
//!
//! Without `pretty_regex`
//!  
//! </td>
//! <td>
//!
//! With `pretty_regex`
//!
//! </td>
//! </tr>
//!
//! <tr>
//! <td>
//!
//! ```reg
//! \d{5}(-\d{4})?
//! ```
//!
//! </td>
//! <td>
//!
//! ```ignore
//! digit() * 5 + (just("-") + digit() * 4).optional()
//! ```
//!
//! </td>
//! </tr>
//! <tr>
//! <td>
//!
//! ```reg
//! ^(?:\d){4}(?:(?:\-)(?:\d){2}){2}$
//! ```
//!
//! </td>
//! <td>
//!
//! ```ignore
//! beginning() + digit() * 4
//!             + (just("-") + digit() * 2) * 2
//!             + ending()
//! ```
//!
//! </td>
//! </tr>
//!
//! <tr>
//! <td>
//!
//! ```reg
//! rege(x(es)?|xps?)
//! ```
//!
//! </td>
//! <td>
//!
//! ```ignore
//! just("rege") + (just("x") + just("es").optional())
//!              | (just("xp") + just("s").optional())
//! ```
//!
//! </td>
//! </tr>
//! </table>
//!
//! # How to use the crate?
//!
//! To convert a `PrettyRegex` struct which is constructed using all these `then`, `one_of`, `beginning`, `digit`, etc. functions into
//! a real regex (from `regex` crate), you can call `to_regex` or `to_regex_or_panic`:
//!
//! ```
//! use pretty_regex::digit;
//! let regex = digit().to_regex_or_panic();
//!
//! assert!(regex.is_match("3"));
//! ```

use regex::{escape, Regex};
use unicode::Category;

use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, BitOr, Mul, Range, RangeInclusive},
};

pub mod logic;
pub mod prelude;
pub mod unicode;

/// Represents the state when regular expression is for a single-character ASCII class
/// (the kind surrounded by colons and two layers of square brackets).
pub struct Ascii;

/// Represents the state when regular expression is for a custom single-character class
/// (the kind surrounded by one layer of square brackets).
pub struct Custom;

/// Represents the state when regular expression corresponds to a single-character character.
pub struct CharClass<T>(PhantomData<T>);

/// Represents the state when regular expression is a standard single-character class
/// (the kind in most cases starts with a backslash followed by a letter)
///
/// E.g. `\d`, `\p{Arabic}`.
pub struct Standard;

/// Represents the state when regular expression is a literal string of characters.
pub struct Text;

/// Represents the state when it is any arbitrary regular expression.
pub struct Chain;

/// Represents the state when regular expression is a quantifier (e.g., an expression
/// that matches a given number of a target).
///
/// These expressions are greedy by default and can be converted to a lazy match.
pub struct Quantifier;

pub struct PrettyRegex<T = Chain>(String, PhantomData<T>);

impl<T> PrettyRegex<T> {
    /// Creates a new empty [`PrettyRegex`].
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self(String::new(), PhantomData)
    }

    /// Converts the [`PrettyRegex`] into a real [`Regex`].
    #[inline]
    #[must_use]
    pub fn to_regex(&self) -> Result<Regex, regex::Error> {
        Regex::new(&self.0)
    }

    /// Converts the [`PrettyRegex`] into a real [`Regex`].
    ///
    /// # Panics
    ///
    /// If the regular expression is not valid.
    #[inline]
    #[must_use]
    pub fn to_regex_or_panic(&self) -> Regex {
        self.to_regex().unwrap()
    }

    /// Allows to chain [`PrettyRegex`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::just;
    /// let regex = just("a").then(just("b")).to_regex_or_panic();
    ///
    /// assert!(regex.is_match("ab"));
    /// assert!(!regex.is_match("ac"));
    /// ```
    #[inline]
    #[must_use]
    pub fn then<U>(self, then: PrettyRegex<U>) -> PrettyRegex<Chain> {
        PrettyRegex::from(self.0 + &then.0)
    }
}

impl<T, R> From<T> for PrettyRegex<R>
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self(value.into(), PhantomData)
    }
}

impl PrettyRegex<Quantifier> {
    /// Adds a lazy modifier to [`Quantifier`].
    ///
    /// ```
    /// # use pretty_regex::just;
    /// let regex = just("a").repeats_at_least(3).lazy();
    /// ```
    ///
    /// Not everything can be lazy. For instance, this spinnet of code doesn't
    /// compile:
    ///
    /// ```compile_fail
    /// # use pretty_regex::just;
    /// let regex = just("a").lazy();
    /// ```
    #[inline]
    #[must_use]
    pub fn lazy(&self) -> PrettyRegex<Chain> {
        PrettyRegex::from(format!("{}?", self.0))
    }
}

impl<T> From<PrettyRegex<T>> for Regex {
    fn from(value: PrettyRegex<T>) -> Self {
        value.to_regex().unwrap()
    }
}

impl<L, R> Add<PrettyRegex<R>> for PrettyRegex<L> {
    type Output = PrettyRegex<Chain>;

    /// Allows to chain [`PrettyRegex`].
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::just;
    /// let regex = (just("a") + just("b")).to_regex_or_panic();
    ///
    /// assert!(regex.is_match("ab"));
    /// assert!(!regex.is_match("ac"));
    /// ```
    fn add(self, rhs: PrettyRegex<R>) -> Self::Output {
        self.then(rhs)
    }
}

impl<T> Display for PrettyRegex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Adds a matching text into a [`PrettyRegex`].
///
/// # Example
///
/// ```
/// # use pretty_regex::just;
/// assert!(just("a").to_regex_or_panic().is_match("a"));
/// assert!(!just("a").to_regex_or_panic().is_match("b"));
/// ```
#[inline]
#[must_use]
pub fn just(text: impl Into<String>) -> PrettyRegex<Text> {
    PrettyRegex::from(format!("(?:{})", escape(&*text.into())))
}

/// Makes regex from unescaped text. It allows to add a regex string directly into a
/// [`PrettyRegex`] object.
///
/// # Example
///
/// ```
/// # use pretty_regex::nonescaped;
/// let regex = nonescaped(r"^\d$").to_regex_or_panic();
/// assert!(!regex.is_match("a"));
/// assert!(regex.is_match("2"));
/// ```
#[inline]
#[must_use]
pub fn nonescaped(text: impl Into<String>) -> PrettyRegex<Chain> {
    PrettyRegex::from(format!("(?:{})", &*text.into()))
}

/// Matches any character, except for newline (`\n`).
///
/// # Example
///
/// ```
/// # use pretty_regex::any;
/// assert!(!any().to_regex_or_panic().is_match("\n"));
/// assert!(any().to_regex_or_panic().is_match("a"));
/// ```
#[inline]
#[must_use]
pub fn any() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(r".")
}

/// Matches digit character class (`\d`).
///
/// # Example
///
/// ```
/// # use pretty_regex::digit;
/// assert!(digit().to_regex_or_panic().is_match("1"));
/// assert!(digit().to_regex_or_panic().is_match("7"));
/// assert!(!digit().to_regex_or_panic().is_match("a"));
/// ```
#[inline]
#[must_use]
pub fn digit() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(r"\d")
}

/// Matches word character class (`\w`) - any alphanumeric character or underscore (`_`).
///
/// # Example
///
/// ```
/// # use pretty_regex::word;
/// assert!(word().to_regex_or_panic().is_match("a"));
/// assert!(word().to_regex_or_panic().is_match("2"));
/// assert!(word().to_regex_or_panic().is_match("_"));
/// assert!(!word().to_regex_or_panic().is_match("?"));
/// ```
#[inline]
#[must_use]
pub fn word() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(r"\w")
}

/// Matches a word boundary (`\b`).
#[inline]
#[must_use]
pub fn word_boundary() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(r"\b")
}

/// Matches whitespace character class (`\s`).
///
/// # Example
///
/// ```
/// # use pretty_regex::whitespace;
/// assert!(whitespace().to_regex_or_panic().is_match("\n"));
/// assert!(whitespace().to_regex_or_panic().is_match(" "));
/// assert!(!whitespace().to_regex_or_panic().is_match("a"));
/// ```
#[inline]
#[must_use]
pub fn whitespace() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(r"\s")
}

/// Matches ascii alphabetic characters (`a-zA-Z`).
///
/// # Example
///
/// ```
/// # use pretty_regex::ascii_alphabetic;
/// assert!(ascii_alphabetic().to_regex_or_panic().is_match("a"));
/// assert!(ascii_alphabetic().to_regex_or_panic().is_match("B"));
/// assert!(!ascii_alphabetic().to_regex_or_panic().is_match("1"));
/// assert!(!ascii_alphabetic().to_regex_or_panic().is_match(" "));
/// ```
#[inline]
#[must_use]
pub fn ascii_alphabetic() -> PrettyRegex<CharClass<Ascii>> {
    PrettyRegex::from(r"[[:alpha:]]")
}

/// Matches ascii alphanumeric characters (`a-zA-Z0-9`).
///
/// # Example
///
/// ```
/// # use pretty_regex::ascii_alphanumeric;
/// assert!(ascii_alphanumeric().to_regex_or_panic().is_match("a"));
/// assert!(ascii_alphanumeric().to_regex_or_panic().is_match("Z"));
/// assert!(ascii_alphanumeric().to_regex_or_panic().is_match("7"));
/// assert!(!ascii_alphanumeric().to_regex_or_panic().is_match(" "));
/// ```
#[inline]
#[must_use]
pub fn ascii_alphanumeric() -> PrettyRegex<CharClass<Ascii>> {
    PrettyRegex::from(r"[[:alnum:]]")
}

/// Matches alphabetic characters (in `Letter`  Unicode category).
///
/// # Example
///
/// ```
/// # use pretty_regex::alphabetic;
/// assert!(alphabetic().to_regex_or_panic().is_match("a"));
/// assert!(alphabetic().to_regex_or_panic().is_match("ю"));
/// assert!(alphabetic().to_regex_or_panic().is_match("A"));
/// assert!(!alphabetic().to_regex_or_panic().is_match("5"));
/// assert!(!alphabetic().to_regex_or_panic().is_match("!"));
/// ```
#[inline]
#[must_use]
pub fn alphabetic() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(Category::Letter)
}

/// Matches alphanumeric characters (in `Letter` and `Number` Unicode categories).
///
/// # Example
///
/// ```
/// # use pretty_regex::alphanumeric;
/// assert!(alphanumeric().to_regex_or_panic().is_match("a"));
/// assert!(alphanumeric().to_regex_or_panic().is_match("ю"));
/// assert!(alphanumeric().to_regex_or_panic().is_match("A"));
/// assert!(alphanumeric().to_regex_or_panic().is_match("5"));
/// assert!(!alphanumeric().to_regex_or_panic().is_match("!"));
/// ```
#[inline]
#[must_use]
pub fn alphanumeric() -> PrettyRegex<Chain> {
    one_of(&[
        PrettyRegex::from(Category::Letter),
        PrettyRegex::from(Category::Number),
    ])
}

/// Matches lowercase characters (in `Lowercase_Letter` Unicode category).
///
/// # Example
///
/// ```
/// # use pretty_regex::lowercase;
/// assert!(lowercase().to_regex_or_panic().is_match("a"));
/// assert!(lowercase().to_regex_or_panic().is_match("ю"));
/// assert!(!lowercase().to_regex_or_panic().is_match("A"));
/// assert!(!lowercase().to_regex_or_panic().is_match("!"));
/// assert!(!lowercase().to_regex_or_panic().is_match(" "));
/// ```
#[inline]
#[must_use]
pub fn lowercase() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(Category::LowercaseLetter)
}

/// Matches ascii lowercase characters (`a-z`).
///
/// # Example
///
/// ```
/// # use pretty_regex::ascii_lowercase;
/// assert!(ascii_lowercase().to_regex_or_panic().is_match("a"));
/// assert!(ascii_lowercase().to_regex_or_panic().is_match("b"));
/// assert!(!ascii_lowercase().to_regex_or_panic().is_match("ю"));
/// assert!(!ascii_lowercase().to_regex_or_panic().is_match("A"));
/// assert!(!ascii_lowercase().to_regex_or_panic().is_match("!"));
/// assert!(!ascii_lowercase().to_regex_or_panic().is_match(" "));
/// ```
#[inline]
#[must_use]
pub fn ascii_lowercase() -> PrettyRegex<CharClass<Ascii>> {
    PrettyRegex::from(r"[[:lower:]]")
}

/// Matches anything within a specified set of characters.
///
/// # Example
///
/// ```
/// # use pretty_regex::within;
/// assert!(within(&['a', 'b']).to_regex_or_panic().is_match("a"));
/// assert!(within(&['a', 'b']).to_regex_or_panic().is_match("b"));
/// assert!(!within(&['a', 'b']).to_regex_or_panic().is_match("c"));
#[inline]
#[must_use]
pub fn within<T>(set: &[T]) -> PrettyRegex<CharClass<Custom>>
where
    T: Display,
{
    PrettyRegex::from(format!(
        "[{}]",
        set.into_iter().map(|c| c.to_string()).collect::<String>()
    ))
}

/// Matches anything outside of a specified set of characters.
///
/// # Example
///
/// ```
/// # use pretty_regex::without;
/// assert!(!without(&['a', 'b']).to_regex_or_panic().is_match("a"));
/// assert!(!without(&['a', 'b']).to_regex_or_panic().is_match("b"));
/// assert!(without(&['a', 'b']).to_regex_or_panic().is_match("c"));
/// ```
#[inline]
#[must_use]
pub fn without<T>(set: &[T]) -> PrettyRegex<CharClass<Custom>>
where
    T: Display,
{
    PrettyRegex::from(format!(
        "[^{}]",
        set.into_iter().map(|c| c.to_string()).collect::<String>()
    ))
}

/// Matches characters within a given range.
///
/// # Example
///
/// ```
/// # use pretty_regex::within_char_range;
/// assert!(within_char_range('a'..='z').to_regex_or_panic().is_match("a"));
/// assert!(!within_char_range('a'..='z').to_regex_or_panic().is_match("Z"));
/// ```
#[inline]
#[must_use]
pub fn within_char_range(range: RangeInclusive<char>) -> PrettyRegex<CharClass<Custom>> {
    PrettyRegex::from(format!("[{}-{}]", range.start(), range.end()))
}

/// Matches characters outside of a given range.
///
/// # Example
///
/// ```
/// # use pretty_regex::without_char_range;
/// assert!(!without_char_range('a'..='z').to_regex_or_panic().is_match("a"));
/// assert!(without_char_range('a'..='z').to_regex_or_panic().is_match("Z"));
/// ```
#[inline]
#[must_use]
pub fn without_char_range(range: RangeInclusive<char>) -> PrettyRegex<CharClass<Custom>> {
    PrettyRegex::from(format!("[^{}-{}]", range.start(), range.end()))
}

/// Matches the beginning of the text or SOF with multi-line mode off (`^`).
///
/// # Example
///
/// ```
/// # use pretty_regex::{just, beginning};
/// let regex = beginning().then(just("foo")).to_regex_or_panic();
///
/// assert!(regex.is_match("foo"));
/// assert!(!regex.is_match("ffoo"));
/// ```
#[inline]
#[must_use]
pub fn beginning() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(r"^")
}

/// Matches the end of the text or EOF with multi-line mode on (`$`).
///
/// # Example
///
/// ```
/// # use pretty_regex::{just, ending};
/// let regex = just("foo").then(ending()).to_regex_or_panic();
///
/// assert!(regex.is_match("foo"));
/// assert!(!regex.is_match("foof"));
/// ```
#[inline]
#[must_use]
pub fn ending() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(r"$")
}

/// Matches the beginning of the text even with multi-line mode on (`\A`).
///
/// # Example
///
/// ```
/// # use pretty_regex::{just, text_beginning};
/// let regex = text_beginning().then(just("foo")).to_regex_or_panic();
///
/// assert!(regex.is_match("foo"));
/// assert!(!regex.is_match("ffoo"));
/// ```
#[inline]
#[must_use]
pub fn text_beginning() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(r"\A")
}

/// Matches the end of the text even with multi-line mode on (`\z`).
///
/// # Example
///
/// ```
/// # use pretty_regex::{just, text_ending};
/// let regex = just("foo").then(text_ending()).to_regex_or_panic();
///
/// assert!(regex.is_match("foo"));
/// assert!(!regex.is_match("foof"));
/// ```
#[inline]
#[must_use]
pub fn text_ending() -> PrettyRegex<CharClass<Standard>> {
    PrettyRegex::from(r"\z")
}

impl<T> Mul<usize> for PrettyRegex<T> {
    type Output = PrettyRegex<Quantifier>;

    /// Matches the pattern a given amount of times.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::just;
    /// let regex = (just("foo") * 3)
    ///     .to_regex_or_panic();
    ///
    /// assert!(regex.is_match("foofoofoo"));
    /// assert!(!regex.is_match("foo"));
    /// assert!(!regex.is_match("bar"));
    /// ```
    fn mul(self, rhs: usize) -> Self::Output {
        self.repeats(rhs)
    }
}

impl<T> PrettyRegex<T> {
    /// Matches the pattern a given amount of times.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::just;
    /// let regex = just("foo")
    ///     .repeats(3)
    ///     .to_regex_or_panic();
    ///
    /// assert!(regex.is_match("foofoofoo"));
    /// assert!(!regex.is_match("foo"));
    /// assert!(!regex.is_match("bar"));
    /// ```
    #[inline]
    #[must_use]
    pub fn repeats(self, times: usize) -> PrettyRegex<Quantifier> {
        PrettyRegex::from(format!("(?:{}){{{}}}", self, times))
    }

    /// Matches the pattern at least a given amount of times.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::just;
    /// let regex = just("foo")
    ///     .repeats_at_least(2)
    ///     .to_regex_or_panic();
    ///
    /// assert!(!regex.is_match("foo"));
    /// assert!(regex.is_match("foofoo"));
    /// assert!(!regex.is_match("bar"));
    /// ```
    #[inline]
    #[must_use]
    pub fn repeats_at_least(self, times: usize) -> PrettyRegex<Quantifier> {
        PrettyRegex::from(format!("(?:{}){{{},}}", self, times))
    }

    /// Matches the pattern one or more times.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::just;
    /// let regex = just("foo")
    ///     .repeats_one_or_more_times()
    ///     .to_regex_or_panic();
    ///
    /// assert!(regex.is_match("foo"));
    /// assert!(regex.is_match("foofoo"));
    /// assert!(!regex.is_match("bar"));
    /// ```
    #[inline]
    #[must_use]
    pub fn repeats_one_or_more_times(self) -> PrettyRegex<Quantifier> {
        PrettyRegex::from(format!("(?:{})+", self))
    }

    /// Matches the pattern optionally (zero or one time).
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::just;
    /// let regex = just("foo")
    ///     .optional()
    ///     .to_regex_or_panic();
    ///
    /// assert!(regex.is_match(""));
    /// assert!(regex.is_match("foo"));
    /// ```
    #[inline]
    #[must_use]
    pub fn optional(self) -> PrettyRegex<Quantifier> {
        PrettyRegex::from(format!("(?:{})?", self))
    }

    /// Matches the pattern zero or more times.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::just;
    /// let regex = just("foo")
    ///     .repeats_zero_or_more_times()
    ///     .to_regex_or_panic();
    ///
    /// assert!(regex.is_match(""));
    /// assert!(regex.is_match("foo"));
    /// assert!(regex.is_match("foofoo"));
    /// ```
    #[inline]
    #[must_use]
    pub fn repeats_zero_or_more_times(self) -> PrettyRegex<Quantifier> {
        PrettyRegex::from(format!("(?:{})*", self))
    }

    /// Matches the pattern `n` times where `n` is within a given range.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::just;
    /// let regex = just("f")
    ///     .repeats_n_times_within(3..5)
    ///     .to_regex_or_panic();
    ///
    /// assert!(!regex.is_match("f"));
    /// assert!(!regex.is_match("ff"));
    /// assert!(regex.is_match("ffff"));
    /// ```
    #[inline]
    #[must_use]
    pub fn repeats_n_times_within(self, range: Range<usize>) -> PrettyRegex<Quantifier> {
        PrettyRegex::from(format!("(?:{}){{{},{}}}", self, range.start, range.end))
    }

    /// Adds a capturnig group around a specific regular expression.
    ///
    /// # Example
    ///
    /// Let's say that we want to process simple date consisting of
    /// month and day number. The problem is that we need to save the data
    /// about these numbers to use it later. That's why we use captures!
    ///
    /// It's important that "unnamed" captures can only be matched using numbers, which
    /// are sequenced from left to right. The number depends on the order of the regular
    /// expression in the chain.
    ///
    /// ```
    /// # use pretty_regex::{digit, just};
    /// let regex = digit().repeats(2).unnamed_capture()
    ///     .then(just("-"))
    ///     .then(digit().repeats(2).unnamed_capture())
    ///     .to_regex_or_panic();
    ///
    /// let captures = regex.captures("08-05").unwrap();
    ///
    /// assert_eq!(captures.get(1).unwrap().as_str(), "08");
    /// assert_eq!(captures.get(2).unwrap().as_str(), "05");
    /// ```
    #[inline]
    #[must_use]
    pub fn unnamed_capture(self) -> PrettyRegex<Chain> {
        PrettyRegex::from(format!("({})", self))
    }

    /// Adds a named capturing groupd around a specific regular expression.
    ///
    /// # Example
    ///
    /// Let's say that we want to process simple date consisting of
    /// month and day number. The problem is that we need to save the data
    /// about these numbers to use it later. That's why we use captures!
    /// See [`PrettyRegex::unnamed_capture`] for more details.
    ///
    /// Here we can give captures a specified names, to then match on them:
    ///
    ///
    /// ```
    /// # use pretty_regex::{digit, just};
    /// let regex = digit().repeats(2).named_capture("month")
    ///     .then(just("-"))
    ///     .then(digit().repeats(2).named_capture("day"))
    ///     .to_regex_or_panic();
    ///
    /// let captures = regex.captures("08-05").unwrap();
    ///
    /// assert_eq!(&captures["month"], "08");
    /// assert_eq!(&captures["day"], "05");
    /// ```
    #[inline]
    #[must_use]
    pub fn named_capture(self, name: impl AsRef<str>) -> PrettyRegex<Chain> {
        PrettyRegex::from(format!("(?P<{}>{})", name.as_ref(), self))
    }
}

/// Establishes an OR relationship between regular expressions.
///
/// # Example
///
/// ```
/// # use pretty_regex::{one_of, just};
/// let regex = one_of(&[just("hi"), just("bar")]).to_regex_or_panic();
///
/// assert!(regex.is_match("hi"));
/// assert!(regex.is_match("bar"));
/// assert!(!regex.is_match("baz"));
/// ```
#[must_use]
pub fn one_of<S>(options: &[S]) -> PrettyRegex<Chain>
where
    S: Display,
{
    let mut regex_string = format!("{}", options[0]);

    for idx in 1..options.len() {
        regex_string = format!("{}|{}", regex_string, options[idx])
    }

    PrettyRegex::from(regex_string)
}

impl<T, M> BitOr<PrettyRegex<M>> for PrettyRegex<T> {
    type Output = PrettyRegex<Chain>;

    /// Establishes an OR relationship between regular expressions.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::{one_of, just};
    /// let regex = (just("hi") | just("bar")).to_regex_or_panic();
    ///
    /// assert!(regex.is_match("hi"));
    /// assert!(regex.is_match("bar"));
    /// assert!(!regex.is_match("baz"));
    /// ```
    fn bitor(self, rhs: PrettyRegex<M>) -> Self::Output {
        one_of(&[self.to_string(), rhs.to_string()])
    }
}
