use crate::lvm::metadata::parse::{integer, string};
use crate::lvm::metadata::MetadataValue;
use winnow::combinator::alt;
use winnow::error::ParserError;
use winnow::stream::{AsChar, Compare, ParseSlice, Stream, StreamIsPartial};
use winnow::Parser;

pub fn value<Input, Error>(input: &mut Input) -> Result<MetadataValue, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    alt((
        integer::integer.map(MetadataValue::Integer),
        string::quoted_string.map(MetadataValue::String),
    ))
    .parse_next(input)
}
