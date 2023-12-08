use std::{
    convert::Infallible,
    ops::{Range, RangeFrom, RangeTo},
};

use nom::{
    character::complete::line_ending,
    error::{ErrorKind, ParseError},
    Compare, IResult, InputIter, InputLength, Parser, Slice,
};
use nom_supreme::ParserExt;

pub struct Noop;

impl<I, E> Parser<I, (), E> for Noop {
    fn parse(&mut self, input: I) -> IResult<I, (), E> {
        Ok((input, ()))
    }
}

impl<I, O, E, P: Parser<I, O, E>> ParserExt2<I, O, E> for P {}
pub trait ParserExt2<I, O, E>: ParserExt<I, O, E> {
    fn separated_list0<G, O2, C>(self, g: G) -> impl Parser<I, C, E>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
        I: Clone + InputLength,
        E: ParseError<I>,
        C: Default + Extend<O>,
    {
        self.separated_list1(g).opt().map(Option::unwrap_or_default)
    }

    fn separated_list1<G, O2, C>(self, g: G) -> impl Parser<I, C, E>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
        I: Clone + InputLength,
        E: ParseError<I>,
        C: Default + Extend<O>,
    {
        parse_separated_impl(
            self,
            g,
            C::default,
            |mut collection, value| {
                collection.extend_one(value);
                Ok(collection)
            },
            |_input, err: Infallible| match err {},
        )
    }

    fn lines<C>(self) -> impl Parser<I, C, E>
    where
        Self: Sized,
        I: Slice<Range<usize>> + Slice<RangeFrom<usize>> + Slice<RangeTo<usize>>,
        I: Clone + InputIter + InputLength,
        I: Compare<&'static str>,
        C: Default + Extend<O>,
        E: ParseError<I>,
    {
        self.terminate_list1(line_ending)
    }

    fn terminate_list1<G, O2, C>(self, g: G) -> impl Parser<I, C, E>
    where
        G: Parser<I, O2, E>,
        Self: Sized,
        I: Clone + InputLength,
        E: ParseError<I>,
        C: Default + Extend<O>,
    {
        parse_terminated_impl(
            self,
            g,
            C::default,
            |mut collection, value| {
                collection.extend_one(value);
                Ok(collection)
            },
            |_input, err: Infallible| match err {},
        )
    }

    fn many1<C>(mut self) -> impl Parser<I, C, E>
    where
        Self: Sized,
        I: Clone + InputLength,
        E: ParseError<I>,
        C: Default + Extend<O>,
    {
        move |mut input: I| {
            let mut res = C::default();
    
            // Parse the first element
            let (i1, n) = self.parse(input)?;
            res.extend_one(n);
            input = i1;
    
            loop {
                match self.parse(input.clone()) {
                    Err(nom::Err::Error(_)) => return Ok((input, res)),
                    Err(e) => return Err(e),
                    Ok((i1, o)) => {
                        res.extend_one(o);
                        input = i1;
                    }
                }
            }
        }
    }
}

pub struct MutRef<'a, P>(pub &'a mut P);

impl<'a, P: Parser<I, O, E>, I, O, E> Parser<I, O, E> for MutRef<'a, P> {
    fn parse(&mut self, input: I) -> IResult<I, O, E> {
        P::parse(self.0, input)
    }
}

#[inline]
fn parse_separated_impl<Input, ParseOutput, SepOutput, ParseErr, Accum, FoldErr>(
    mut parser: impl Parser<Input, ParseOutput, ParseErr>,
    mut separator: impl Parser<Input, SepOutput, ParseErr>,

    mut init: impl FnMut() -> Accum,
    mut fold: impl FnMut(Accum, ParseOutput) -> Result<Accum, FoldErr>,

    mut build_error: impl FnMut(Input, FoldErr) -> ParseErr,
) -> impl Parser<Input, Accum, ParseErr>
where
    Input: Clone + InputLength,
    ParseErr: ParseError<Input>,
{
    move |input: Input| {
        let mut accum = init();

        // Parse the first element
        let (mut input, value) = parser.parse(input)?;
        match fold(accum, value) {
            Ok(a) => accum = a,
            Err(err) => return Err(nom::Err::Error(build_error(input, err))),
        };

        loop {
            match separator.parse(input.clone()) {
                Err(nom::Err::Error(_)) => break Ok((input, accum)),
                Err(e) => break Err(e),
                Ok((i, _)) => {
                    // infinite loop check: the parser must always consume
                    if i.input_len() == input.input_len() {
                        break Err(nom::Err::Error(ParseErr::from_error_kind(
                            i,
                            ErrorKind::SeparatedList,
                        )));
                    }
                    match parser.parse(i) {
                        Err(nom::Err::Error(_)) => break Ok((input, accum)),
                        Err(e) => break Err(e),
                        Ok((i, value)) => {
                            accum = fold(accum, value)
                                .map_err(|err| nom::Err::Error(build_error(input, err)))?;
                            input = i;
                        }
                    }
                }
            }
        }
    }
}

#[inline]
fn parse_terminated_impl<Input, ParseOutput, TermOutput, ParseErr, Accum, FoldErr>(
    mut parser: impl Parser<Input, ParseOutput, ParseErr>,
    mut terminator: impl Parser<Input, TermOutput, ParseErr>,

    mut init: impl FnMut() -> Accum,
    mut fold: impl FnMut(Accum, ParseOutput) -> Result<Accum, FoldErr>,

    mut build_error: impl FnMut(Input, FoldErr) -> ParseErr,
) -> impl Parser<Input, Accum, ParseErr>
where
    Input: Clone + InputLength,
    ParseErr: ParseError<Input>,
{
    move |mut input: Input| {
        let mut accum = init();

        // Parse the first element
        let value: ParseOutput;
        (input, value) = parser.parse(input)?;
        (input, _) = terminator.parse(input)?;
        match fold(accum, value) {
            Ok(a) => accum = a,
            Err(err) => return Err(nom::Err::Error(build_error(input, err))),
        };

        loop {
            let value;
            (input, value) = match parser.parse(input.clone()) {
                Ok((i, n)) => (i, n),
                Err(nom::Err::Error(_)) => break,
                Err(e) => return Err(e),
            };

            let len = input.input_len();

            input = match terminator.parse(input.clone()) {
                Ok((i, _)) => i,
                Err(nom::Err::Error(_)) => break,
                Err(e) => return Err(e),
            };

            // infinite loop check: the parser must always consume
            if input.input_len() == len {
                return Err(nom::Err::Error(ParseErr::from_error_kind(
                    input,
                    ErrorKind::SeparatedList,
                )));
            }

            match fold(accum, value) {
                Ok(a) => accum = a,
                Err(err) => return Err(nom::Err::Error(build_error(input, err))),
            };
        }

        Ok((input, accum))
    }
}
