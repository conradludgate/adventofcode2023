#![feature(extend_one)]

use std::str::FromStr;

use nom::{
    bytes::complete::is_a,
    character::complete::{digit1, line_ending},
    error::{ErrorKind, ParseError},
    Err, IResult, InputIter, InputLength, InputTake, Parser,
};

mod ext;
pub use ext::*;
use nom_supreme::ParserExt;

/// ```
/// let line = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15";
/// let segments = ["Sensor at x=",", y=",": closest beacon is at x=",", y="];
/// assert_eq!(parsers::split_many(line, segments), Some(["2","18","-2","15"]));
/// ```
pub fn split_many<'a, const N: usize>(
    mut s: &'a str,
    mut delimiters: [&'a str; N],
) -> Option<[&'a str; N]> {
    s = s.strip_prefix(delimiters[0])?;
    for i in 1..N {
        (delimiters[i - 1], s) = s.split_once(delimiters[i])?;
    }
    delimiters[N - 1] = s;
    Some(delimiters)
}

pub fn number<O>(input: &str) -> IResult<&str, O>
where
    O: FromStr,
{
    digit1.parse_from_str().parse(input)
}

pub fn binary(input: &str) -> IResult<&str, usize> {
    is_a("01")
        .map_res(|s| usize::from_str_radix(s, 2))
        .parse(input)
}

pub fn lines<'a, O, E, F>(f: F) -> impl Parser<&'a str, Vec<O>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    f.separated_list1(line_ending)
}

pub fn grid<'a, O, E, F>(f: F) -> impl Parser<&'a str, Vec<Vec<O>>, E>
where
    F: Parser<&'a str, O, E>,
    E: ParseError<&'a str>,
{
    f.many1().separated_list1(line_ending)
}

pub fn separated_array<I, O, O2, E, F, G, const N: usize>(sep: G, f: F) -> impl Parser<I, [O; N], E>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
{
    f.separated_array(sep)
}

pub fn skip<I, E>(count: usize) -> impl Fn(I) -> IResult<I, (), E>
where
    E: ParseError<I>,
    I: InputIter + InputTake,
{
    move |i: I| match i.slice_index(count) {
        Err(_needed) => Err(Err::Error(E::from_error_kind(i, ErrorKind::Eof))),
        Ok(index) => Ok((i.take_split(index).0, ())),
    }
}

#[macro_export]
/// Builds a pattern that can match tuple lists
///
/// ```
/// use parsers::cons;
/// let cons![false, 1, Some(2)] = ((false, 1), Some(2)) else {
///     panic!("didn't match");
/// };
/// ```
macro_rules! cons {
    ($($elem:pat),+ $(,)?) => {
        $crate::__cons_impl!($($elem),* =>)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __cons_impl {
    // reverse list
    ($head:pat $(,$tail:pat)* => $($head2:pat),*) => {
        $crate::__cons_impl!($($tail),* => $head $(,$head2)*)
    };
    // pick off front of reversed
    (=> $tail:pat, $($head:pat),+) => {
        ($crate::__cons_impl![=> $($head),*], $tail)
    };
    // base case
    (=> $tail:pat) => {
        $tail
    };
}
