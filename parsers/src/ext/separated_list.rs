use std::marker::PhantomData;

use crate::gen::{separated_list0_inner, separated_list1_inner};
use next_gen::gen_iter;
use nom::{error::ParseError, Err, InputLength, Parser};

pub struct SeperatedList1<F, G, O, O2, C> {
    pub(crate) f: F,
    pub(crate) g: G,
    pub(crate) _output: PhantomData<(O, O2, C)>,
}

impl<I, F, G, O, O2, C, E> Parser<I, C, E> for SeperatedList1<F, G, O, O2, C>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
    C: Default + Extend<O>,
{
    fn parse(&mut self, input: I) -> nom::IResult<I, C, E> {
        let mut res = C::default();
        let input = gen_iter! {
            for v in separated_list1_inner(input, &mut self.f, &mut self.g) {
                res.extend(Some(v));
            }
        }?;
        Ok((input, res))
    }
}

pub struct SeperatedList0<F, G, O, O2, C> {
    pub(crate) f: F,
    pub(crate) g: G,
    pub(crate) _output: PhantomData<(O, O2, C)>,
}

impl<I, F, G, O, O2, C, E> Parser<I, C, E> for SeperatedList0<F, G, O, O2, C>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    G: Parser<I, O2, E>,
    E: ParseError<I>,
    C: Default + Extend<O>,
{
    fn parse(&mut self, input: I) -> nom::IResult<I, C, E> {
        let mut res = C::default();
        let input = gen_iter! {
            for v in separated_list0_inner(input, &mut self.f, &mut self.g) {
                res.extend(Some(v));
            }
        }?;
        Ok((input, res))
    }
}

pub struct Many1<F, O, C> {
    pub(crate) f: F,
    pub(crate) _output: PhantomData<(O, C)>,
}

impl<I, F, O, E, C> Parser<I, C, E> for Many1<F, O, C>
where
    I: Clone + InputLength,
    F: Parser<I, O, E>,
    E: ParseError<I>,
    C: Default + Extend<O>,
{
    fn parse(&mut self, mut input: I) -> nom::IResult<I, C, E> {
        let mut res = C::default();

        // Parse the first element
        let (i1, n) = self.f.parse(input)?;
        res.extend(Some(n));
        input = i1;

        loop {
            match self.f.parse(input.clone()) {
                Err(Err::Error(_)) => return Ok((input, res)),
                Err(e) => return Err(e),
                Ok((i1, o)) => {
                    res.extend(Some(o));
                    input = i1;
                }
            }
        }
    }
}
