use regex::{escape, Regex};

use std::{
    fmt::Display,
    marker::PhantomData,
    ops::{Add, Range, RangeInclusive},
};

pub mod unicode;

/// Represents the state when regular expression is for a single-character ASCII class
/// (the kind surrounded by colons and two layers of square brackets).
pub struct Ascii;

/// Represents the state when regular expression is for a custom single-character class
/// (the kind surrounded by one layer of square brackets).
pub struct Custom;

/// Represents the state when regular expression corresponds to a single-character character.
pub struct SingleCharacter<T>(PhantomData<T>);

/// Represents the state when regular expression is a standard single-character class
/// (the kind that starts with a backslash followed by a letter)
///
/// E.g. `\d`, `\p{Arabic}`.
pub struct Standart;

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

    /// Creates a new [`PrettyRegex`] from a string (unescaped).
    #[inline]
    #[must_use]
    pub fn from(regex: impl Into<String>) -> Self {
        Self(regex.into(), PhantomData)
    }

    /// Creates a new [`PrettyRegex`] from a unicode category.
    #[inline]
    #[must_use]
    pub fn from_unicode_category(
        category: unicode::Category,
    ) -> PrettyRegex<SingleCharacter<Standart>> {
        category.to_regex()
    }

    /// Creates a new [`PrettyRegex`] from a unicode script.
    #[inline]
    #[must_use]
    pub fn from_unicode_script(script: unicode::Script) -> PrettyRegex<SingleCharacter<Standart>> {
        script.to_regex()
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

    /// Allows to create [`PrettyRegex`]-s that made up from 2 or more.
    ///
    /// ```
    /// use pretty_regex::just;
    ///
    /// let regex = just("a").and_then(just("b")).to_regex_or_panic();
    ///
    /// assert!(regex.is_match("ab"));
    /// assert!(!regex.is_match("ac"));
    /// ```
    #[inline]
    #[must_use]
    pub fn and_then(self, then: PrettyRegex<T>) -> Self {
        Self(self.0 + &then.0, PhantomData)
    }
}

impl PrettyRegex<Quantifier> {
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

impl<T, U> Add<PrettyRegex<U>> for PrettyRegex<T> {
    type Output = PrettyRegex<Chain>;

    fn add(self, rhs: PrettyRegex<U>) -> Self::Output {
        PrettyRegex::from(format!("{}{}", self, rhs))
    }
}

impl<T> From<String> for PrettyRegex<T> {
    fn from(value: String) -> Self {
        Self::from(value)
    }
}

impl<T> Display for PrettyRegex<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

pub fn just(text: impl Into<String>) -> PrettyRegex<Text> {
    PrettyRegex::from(format!("(?:{})", escape(&*text.into())))
}

pub fn nonescaped(text: impl Into<String>) -> PrettyRegex<Ascii> {
    PrettyRegex::from(format!("(?:{})", &*text.into()))
}

/// Matches any character, except for newline (`\n`).
///
/// ```
/// use pretty_regex::any;
///
/// assert!(!any().to_regex_or_panic().is_match("\n"));
/// assert!(any().to_regex_or_panic().is_match("a"));
/// ```
#[inline]
#[must_use]
pub fn any() -> PrettyRegex<SingleCharacter<Standart>> {
    PrettyRegex::from(r".")
}

/// Matches digit character class (`\d`).
///
/// ```
/// use pretty_regex::digit;
///
/// assert!(digit().to_regex_or_panic().is_match("1"));
/// assert!(digit().to_regex_or_panic().is_match("2"));
/// assert!(!digit().to_regex_or_panic().is_match("a"));
/// ```
#[inline]
#[must_use]
pub fn digit() -> PrettyRegex<SingleCharacter<Standart>> {
    PrettyRegex::from(r"\d")
}

/// Matches non-digit character class (`\D`).
///
/// ```
/// use pretty_regex::not_digit;
///
/// assert!(!not_digit().to_regex_or_panic().is_match("1"));
/// assert!(!not_digit().to_regex_or_panic().is_match("2"));
/// assert!(not_digit().to_regex_or_panic().is_match("a"));
#[inline]
#[must_use]
pub fn not_digit() -> PrettyRegex<SingleCharacter<Standart>> {
    PrettyRegex::from(r"\D")
}

pub fn word() -> PrettyRegex<SingleCharacter<Standart>> {
    PrettyRegex::from(r"\w")
}

pub fn not_word() -> PrettyRegex<SingleCharacter<Standart>> {
    PrettyRegex::from(r"\W")
}

pub fn whitespace() -> PrettyRegex<SingleCharacter<Standart>> {
    PrettyRegex::from(r"\s")
}

pub fn not_whitespace() -> PrettyRegex<SingleCharacter<Standart>> {
    PrettyRegex::from(r"\S")
}

pub fn alphabetic() -> PrettyRegex<Ascii> {
    PrettyRegex::from(r"[[:alpha:]]")
}

pub fn ascii_alphanumeric() -> PrettyRegex<Ascii> {
    PrettyRegex::from(r"[[:alnum:]]")
}

/// Matches lowercase characters (in `Ll` Unicode category).
///
/// ```
/// use pretty_regex::lowercase;
///
/// assert!(lowercase().to_regex_or_panic().is_match("a"));
/// assert!(lowercase().to_regex_or_panic().is_match("ю"));
/// assert!(!lowercase().to_regex_or_panic().is_match("A"));
/// assert!(!lowercase().to_regex_or_panic().is_match("!"));
/// assert!(!lowercase().to_regex_or_panic().is_match(" "));
/// ```
#[inline]
#[must_use]
pub fn lowercase() -> PrettyRegex<Ascii> {
    PrettyRegex::from(r"\p{Ll}")
}

/// Matches ascii lowercase characters (`a-z`).
///
/// ```
/// use pretty_regex::ascii_lowercase;
///
/// assert!(ascii_lowercase().to_regex_or_panic().is_match("a"));
/// assert!(ascii_lowercase().to_regex_or_panic().is_match("b"));
/// assert!(!ascii_lowercase().to_regex_or_panic().is_match("ю"));
/// assert!(!ascii_lowercase().to_regex_or_panic().is_match("A"));
/// assert!(!ascii_lowercase().to_regex_or_panic().is_match("!"));
/// assert!(!ascii_lowercase().to_regex_or_panic().is_match(" "));
/// ```
#[inline]
#[must_use]
pub fn ascii_lowercase() -> PrettyRegex<Ascii> {
    PrettyRegex::from(r"[[:lower:]]")
}

/// Matches anything within a specified set of characters.
///
/// ```
/// use pretty_regex::within;
///
/// assert!(within(&['a', 'b']).to_regex_or_panic().is_match("a"));
/// assert!(within(&['a', 'b']).to_regex_or_panic().is_match("b"));
/// assert!(!within(&['a', 'b']).to_regex_or_panic().is_match("c"));
#[inline]
#[must_use]
pub fn within<T>(set: &[T]) -> PrettyRegex<SingleCharacter<Custom>>
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
/// ```
/// use pretty_regex::without;
///
/// assert!(!without(&['a', 'b']).to_regex_or_panic().is_match("a"));
/// assert!(!without(&['a', 'b']).to_regex_or_panic().is_match("b"));
/// assert!(without(&['a', 'b']).to_regex_or_panic().is_match("c"));
/// ```
#[inline]
#[must_use]
pub fn without<T>(set: &[T]) -> PrettyRegex<SingleCharacter<Custom>>
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
/// ```
/// use pretty_regex::within_char_range;
///
/// assert!(within_char_range('a'..='z').to_regex_or_panic().is_match("a"));
/// assert!(!within_char_range('a'..='z').to_regex_or_panic().is_match("Z"));
/// ```
#[inline]
#[must_use]
pub fn within_char_range(range: RangeInclusive<char>) -> PrettyRegex<SingleCharacter<Custom>> {
    PrettyRegex::from(format!("[{}-{}]", range.start(), range.end()))
}

/// Matches characters outside of a given range.
///
/// ```
/// use pretty_regex::without_char_range;
///
/// assert!(!without_char_range('a'..='z').to_regex_or_panic().is_match("a"));
/// assert!(without_char_range('a'..='z').to_regex_or_panic().is_match("Z"));
/// ```
#[inline]
#[must_use]
pub fn without_char_range(range: RangeInclusive<char>) -> PrettyRegex<SingleCharacter<Custom>> {
    PrettyRegex::from(format!("[^{}-{}]", range.start(), range.end()))
}

impl<T> PrettyRegex<T> {
    /// ```
    /// use pretty_regex::just;
    ///
    /// let regex = just("foo")
    ///     .repeats_exactly_n_times(3)
    ///     .to_regex_or_panic();
    ///
    /// assert!(regex.is_match("foofoofoo"));
    /// assert!(!regex.is_match("foo"));
    /// assert!(!regex.is_match("bar"));
    /// ```
    #[inline]
    #[must_use]
    pub fn repeats_exactly_n_times(self, times: usize) -> PrettyRegex<Quantifier> {
        PrettyRegex::from(format!("(?:{}){{{}}}", self, times))
    }

    /// ```
    /// use pretty_regex::just;
    ///
    /// let regex = just("foo")
    ///     .repeats_at_least_n_times(2)
    ///     .to_regex_or_panic();
    ///
    /// assert!(!regex.is_match("foo"));
    /// assert!(regex.is_match("foofoo"));
    /// assert!(!regex.is_match("bar"));
    /// ```
    #[inline]
    #[must_use]
    pub fn repeats_at_least_n_times(self, times: usize) -> PrettyRegex<Quantifier> {
        PrettyRegex::from(format!("(?:{}){{{},}}", self, times))
    }

    /// ```
    /// use pretty_regex::just;
    ///
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

    /// ```
    /// use pretty_regex::just;
    ///
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

    /// ```
    /// use pretty_regex::just;
    ///
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

    /// ```
    /// use pretty_regex::just;
    ///
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

    pub fn capture(self) -> PrettyRegex<Chain> {
        PrettyRegex::from(format!("({})", self))
    }

    /// ```
    /// use pretty_regex::just;
    /// ```
    pub fn named_capture(self, name: &str) -> PrettyRegex<Chain> {
        PrettyRegex::from(format!("(?P<{}>{})", name, self))
    }
}

/// ```
/// use pretty_regex::{one_of, just};
///
/// let regex = one_of(&[just("hi"), just("bar")]).to_regex_or_panic();
///
/// assert!(regex.is_match("hi"));
/// assert!(regex.is_match("bar"));
/// assert!(!regex.is_match("baz"));
/// ```
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
