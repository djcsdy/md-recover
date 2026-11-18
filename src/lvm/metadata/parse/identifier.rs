use crate::lvm::metadata::parse::lexeme::*;
use winnow::combinator::{alt, repeat};
use winnow::error::ParserError;
use winnow::stream::{AsChar, ParseSlice, Stream, StreamIsPartial};
use winnow::token::one_of;
use winnow::Parser;

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

pub fn identifier<Input, Error, Output>(input: &mut Input) -> Result<Output, Error>
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

#[cfg(test)]
mod test {
    use winnow::error::ContextError;
    use winnow::Parser;

    #[test]
    fn identifier() -> anyhow::Result<()> {
        assert_eq!(
            super::identifier::<_, ContextError, String>
                .parse("vg_name ")
                .map_err(anyhow::Error::msg)?,
            "vg_name"
        );

        assert_eq!(
            super::identifier::<_, ContextError, String>
                .parse("1234-abc")
                .map_err(anyhow::Error::msg)?,
            "1234-abc"
        );

        assert!(super::identifier::<_, ContextError, String>
            .parse("-abc")
            .is_err());

        Ok(())
    }
}
