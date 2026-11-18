use crate::lvm::metadata::parse::lexeme::*;
use winnow::ascii::digit1;
use winnow::error::ParserError;
use winnow::stream::{AsChar, ParseSlice, Stream, StreamIsPartial};
use winnow::Parser;

pub fn integer<Input, Error>(input: &mut Input) -> Result<u64, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Slice: ParseSlice<u64>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    lexeme(digit1).parse_to().parse_next(input)
}

#[cfg(test)]
mod test {
    use winnow::error::ContextError;
    use winnow::Parser;

    #[test]
    fn integer() -> anyhow::Result<()> {
        assert_eq!(
            super::integer::<_, ContextError>
                .parse("12 ")
                .map_err(anyhow::Error::msg)?,
            12
        );

        assert!(super::integer::<_, ContextError>.parse("_12").is_err());

        assert!(super::integer::<_, ContextError>.parse("12a").is_err());

        Ok(())
    }
}
