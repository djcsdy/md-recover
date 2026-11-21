use crate::lvm::metadata::parse::lexeme::*;
use crate::lvm::metadata::parse::value::*;
use crate::lvm::metadata::MetadataValue;
use winnow::combinator::{delimited, separated};
use winnow::error::ParserError;
use winnow::stream::{AsChar, Compare, ParseSlice, Stream, StreamIsPartial};
use winnow::Parser;

pub fn array<Input, Error>(input: &mut Input) -> Result<Vec<MetadataValue>, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    delimited(lexeme('['), separated(0.., value, lexeme(',')), lexeme(']')).parse_next(input)
}

#[cfg(test)]
mod test {
    use winnow::error::ContextError;
    use winnow::Parser;

    #[test]
    fn array() -> anyhow::Result<()> {
        assert_eq!(
            super::array::<_, ContextError>
                .parse("[1, 2,\n3]\n")
                .map_err(anyhow::Error::msg)?,
            vec![1.into(), 2.into(), 3.into()]
        );

        assert!(super::array::<_, ContextError>.parse("[1,2,3,]").is_err());

        Ok(())
    }
}
