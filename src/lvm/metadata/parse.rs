use crate::lvm::metadata::MetadataValue;
use std::collections::HashMap;
use winnow::ascii::{digit1, escaped, multispace0};
use winnow::combinator::{alt, delimited, preceded, repeat, separated, terminated};
use winnow::error::ParserError;
use winnow::stream::{Accumulate, AsChar, Compare, ParseSlice, Stream, StreamIsPartial};
use winnow::token::{any, none_of, one_of};
use winnow::Parser;

pub fn metadata<Input, Error>(input: &mut Input) -> Result<HashMap<String, MetadataValue>, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    preceded(multispace0, object_body).parse_next(input)
}

fn integer<Input, Error>(input: &mut Input) -> Result<u64, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: ParseSlice<u64>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    lexeme(digit1).parse_to().parse_next(input)
}

fn quoted_string<Input, Error, Output>(input: &mut Input) -> Result<Output, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Token: AsChar + Clone,
    Output: Accumulate<char>,
    Error: ParserError<Input>,
{
    lexeme(delimited(
        '"',
        escaped(
            none_of(('\\', '"')).map(AsChar::as_char),
            '\\',
            any.map(AsChar::as_char),
        ),
        '"',
    ))
    .parse_next(input)
}

fn array<Input, Error>(input: &mut Input) -> Result<Vec<MetadataValue>, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    delimited(lexeme('['), separated(0.., value, lexeme(',')), lexeme(']')).parse_next(input)
}

fn identifier_first_char<Input, Error>(input: &mut Input) -> Result<<Input as Stream>::Token, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    one_of(('a'..='z', 'A'..='z', '0'..='9', '_', '+', '.')).parse_next(input)
}

fn identifier_char<Input, Error>(input: &mut Input) -> Result<<Input as Stream>::Token, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    alt((identifier_first_char, one_of('-'))).parse_next(input)
}

fn identifier<Input, Error, Output>(input: &mut Input) -> Result<Output, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: ParseSlice<Output>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    lexeme(
        (
            identifier_first_char,
            repeat::<_, _, (), _, _>(0.., identifier_char),
        )
            .take(),
    )
    .parse_to()
    .parse_next(input)
}

fn object_item<Input, Error>(input: &mut Input) -> Result<(String, MetadataValue), Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    (
        identifier,
        alt((
            object.map(MetadataValue::Object),
            preceded(lexeme('='), value),
        )),
    )
        .parse_next(input)
}

fn object_body<Input, Error>(input: &mut Input) -> Result<HashMap<String, MetadataValue>, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    repeat(0.., object_item).parse_next(input)
}

fn object<Input, Error>(input: &mut Input) -> Result<HashMap<String, MetadataValue>, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    delimited(lexeme('{'), object_body, lexeme('}')).parse_next(input)
}

fn value<Input, Error>(input: &mut Input) -> Result<MetadataValue, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    alt((
        integer.map(MetadataValue::Integer),
        quoted_string.map(MetadataValue::String),
        array.map(MetadataValue::Array),
        object.map(MetadataValue::Object),
    ))
    .parse_next(input)
}

fn lexeme<Input, Output, Error, ParseNext>(parser: ParseNext) -> impl Parser<Input, Output, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
    ParseNext: Parser<Input, Output, Error>,
{
    terminated(parser, multispace0)
}
