use nom::branch::alt;
use nom::combinator::{eof, value};
use nom::error::ParseError;
use nom::number::complete::{le_u16, le_u32};
use nom::{Input, Parser};

pub fn le_u16_or_default_eof<I, E>(default: u16) -> impl Parser<I, Output = u16, Error = E>
where
    I: Input<Item = u8>,
    E: ParseError<I>,
{
    alt((le_u16, value(default, eof)))
}

pub fn le_u32_or_default_eof<I, E>(default: u32) -> impl Parser<I, Output = u32, Error = E>
where
    I: Input<Item = u8>,
    E: ParseError<I>,
{
    alt((le_u32, value(default, eof)))
}
