use std::ops::{BitAnd, Not, Sub};

use crate::{Ascii, Chain, CharClass, Custom, PrettyRegex, Standart, Text};

impl<T> PrettyRegex<CharClass<T>> {
    /// ```
    /// use pretty_regex::{ascii_alphabetic, ascii_alphanumeric};
    ///
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
}

impl<L, R> BitAnd<PrettyRegex<CharClass<R>>> for PrettyRegex<L> {
    type Output = PrettyRegex<CharClass<Custom>>;

    /// ```
    /// use pretty_regex::{ascii_alphabetic, ascii_alphanumeric};
    ///
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
    /// # use std::ops::Sub;
    /// # use pretty_regex::{ascii_alphabetic, ascii_alphanumeric};
    /// let regex = ascii_alphanumeric().sub(ascii_alphabetic()).to_regex_or_panic();
    ///
    /// assert!(regex.is_match("3"));
    /// assert!(!regex.is_match("a"));
    /// ```
    fn sub(self, rhs: PrettyRegex<CharClass<R>>) -> Self::Output {
        PrettyRegex::from(format!("[{}--{}]", self, rhs))
    }
}

pub fn not<T, M>(regex: PrettyRegex<T>) -> PrettyRegex<M>
where
    PrettyRegex<T>: Not<Output = PrettyRegex<M>>,
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

impl Not for PrettyRegex<CharClass<Ascii>> {
    type Output = Self;

    fn not(self) -> Self::Output {
        todo!()
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
