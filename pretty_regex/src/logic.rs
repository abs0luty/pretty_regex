use std::ops::{BitAnd, BitXor, Not, Sub};

use crate::{Ascii, Chain, CharClass, Custom, PrettyRegex, Standart, Text};

impl<T> PrettyRegex<CharClass<T>> {
    /// Returns intersection between two character classes.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::{ascii_alphabetic, ascii_alphanumeric};
    /// let regex = ascii_alphabetic().and(ascii_alphanumeric()).to_regex_or_panic();
    ///
    /// assert!(regex.is_match("a"));
    /// assert!(!regex.is_match("3"));
    /// ```
    #[inline]
    #[must_use]
    pub fn and<R>(self, rhs: PrettyRegex<CharClass<R>>) -> PrettyRegex<CharClass<Custom>> {
        self & rhs
    }

    /// Returns symmetric difference between two character classes.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::within_char_range;
    /// let regex = within_char_range('a'..='f')
    ///     .symmetric_difference_with(within_char_range('c'..='z'))
    ///     .to_regex_or_panic();
    ///
    /// assert!(regex.is_match("a"));
    /// assert!(regex.is_match("z"));
    /// assert!(!regex.is_match("d"));
    /// ```
    #[inline]
    #[must_use]
    pub fn symmetric_difference_with<R>(
        self,
        rhs: PrettyRegex<CharClass<R>>,
    ) -> PrettyRegex<CharClass<Custom>> {
        self ^ rhs
    }
}

/// Returns symmetric difference between two character classes.
///
/// # Example
///
/// ```
/// # use pretty_regex::{within_char_range, symmetric_difference_between};
/// let regex = symmetric_difference_between(
///         within_char_range('a'..='f'),
///         within_char_range('c'..='z')
///     )
///     .to_regex_or_panic();
///
/// assert!(regex.is_match("a"));
/// assert!(regex.is_match("z"));
/// assert!(!regex.is_match("d"));
/// ```
pub fn symmetric_difference_between<L, R>(
    left: PrettyRegex<CharClass<L>>,
    right: PrettyRegex<CharClass<R>>,
) -> PrettyRegex<CharClass<Custom>> {
    left ^ right
}

impl<L, R> BitAnd<PrettyRegex<CharClass<R>>> for PrettyRegex<L> {
    type Output = PrettyRegex<CharClass<Custom>>;

    /// Returns intersection between two character classes.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::{ascii_alphabetic, ascii_alphanumeric};
    /// let regex = (ascii_alphabetic() & ascii_alphanumeric()).to_regex_or_panic();
    ///
    /// assert!(regex.is_match("a"));
    /// assert!(!regex.is_match("3"));
    /// ```
    #[inline]
    #[must_use]
    fn bitand(self, rhs: PrettyRegex<CharClass<R>>) -> Self::Output {
        PrettyRegex::from(format!("[{}&&{}]", self, rhs))
    }
}

impl<L, R> Sub<PrettyRegex<CharClass<R>>> for PrettyRegex<L> {
    type Output = PrettyRegex<CharClass<Custom>>;

    /// Removes character from second character class, that also appear in the first character
    /// class.
    ///
    /// ```
    /// # use pretty_regex::{ascii_alphabetic, ascii_alphanumeric};
    /// let regex = (ascii_alphanumeric() - ascii_alphabetic()).to_regex_or_panic();
    ///
    /// assert!(regex.is_match("3"));
    /// assert!(!regex.is_match("a"));
    /// ```
    fn sub(self, rhs: PrettyRegex<CharClass<R>>) -> Self::Output {
        PrettyRegex::from(format!("[{}--{}]", self, rhs))
    }
}

/// Matches everything, except for what can be matched by an original regex.
///
/// ```
/// # use pretty_regex::{not, digit};
/// let not_digit = not(digit()).to_regex_or_panic();
///
/// assert!(!not_digit.is_match("1"));
/// assert!(not_digit.is_match("a"));
/// ```
pub fn not<T, O>(regex: PrettyRegex<T>) -> PrettyRegex<O>
where
    PrettyRegex<T>: Not<Output = PrettyRegex<O>>,
{
    regex.not()
}

impl Not for PrettyRegex<CharClass<Standart>> {
    type Output = Self;

    /// ```
    /// # use pretty_regex::digit;
    /// let regex = (!digit()).to_regex_or_panic();
    ///
    /// assert!(!regex.is_match("1"));
    /// assert!(regex.is_match("a"));
    /// ```
    fn not(self) -> Self::Output {
        if self.0.len() < 2 {
            return self;
        }

        if self.0.chars().nth(1).unwrap().is_lowercase() {
            PrettyRegex::from(
                self.0
                    .replace(r"\d", r"\D")
                    .replace(r"\p", r"\P")
                    .replace(r"\w", r"\W")
                    .replace(r"\s", r"\S")
                    .replace(r"\b", r"\B"),
            )
        } else {
            PrettyRegex::from(
                self.0
                    .replace(r"\D", r"\d")
                    .replace(r"\P", r"\p")
                    .replace(r"\W", r"\w")
                    .replace(r"\S", r"\s")
                    .replace(r"\B", r"\b"),
            )
        }
    }
}

impl Not for PrettyRegex<CharClass<Custom>> {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self
            .0
            .chars()
            .nth(1)
            .expect("There must be 2 characters in custom regex")
            == '^'
        {
            PrettyRegex::from(self.0.replace("[^", "["))
        } else {
            PrettyRegex::from(self.0.replace("[", "[^"))
        }
    }
}

impl Not for PrettyRegex<CharClass<Ascii>> {
    type Output = Self;

    fn not(self) -> Self::Output {
        if self
            .0
            .chars()
            .nth(3)
            .expect("There must be 4 characters in ascii regex")
            == '^'
        {
            PrettyRegex::from(self.0.replace("[[:^", "[[:"))
        } else {
            PrettyRegex::from(self.0.replace("[[:", "[[:^"))
        }
    }
}

impl Not for PrettyRegex<Text> {
    type Output = PrettyRegex<Chain>;

    fn not(self) -> Self::Output {
        PrettyRegex::from(
            self.0
                .chars()
                .into_iter()
                .map(|c| format!("[^{}]", c))
                .collect::<String>(),
        )
    }
}

impl<T, M> BitXor<PrettyRegex<CharClass<M>>> for PrettyRegex<CharClass<T>> {
    type Output = PrettyRegex<CharClass<Custom>>;

    /// Returns symmetric difference between two character classes.
    ///
    /// # Example
    ///
    /// ```
    /// # use pretty_regex::within_char_range;
    /// let regex = (within_char_range('a'..='f') ^ within_char_range('c'..='z'))
    ///     .to_regex_or_panic();
    ///
    /// assert!(regex.is_match("a"));
    /// assert!(regex.is_match("z"));
    /// assert!(!regex.is_match("d"));
    /// ```
    fn bitxor(self, rhs: PrettyRegex<CharClass<M>>) -> Self::Output {
        PrettyRegex::from(format!("[{}~~{}]", self, rhs))
    }
}
