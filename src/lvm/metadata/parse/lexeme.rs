use winnow::ascii::multispace0;
use winnow::combinator::terminated;
use winnow::error::ParserError;
use winnow::stream::{AsChar, Stream, StreamIsPartial};
use winnow::Parser;

pub fn lexeme<Input, Output, Error, ParseNext>(
    parser: ParseNext,
) -> impl Parser<Input, Output, Error>
where
    Input: StreamIsPartial + Stream,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
    ParseNext: Parser<Input, Output, Error>,
{
    terminated(parser, multispace0)
}

#[cfg(test)]
mod test {
    use winnow::error::ContextError;
    use winnow::Parser;

    #[test]
    fn lexeme() -> anyhow::Result<()> {
        assert_eq!(
            super::lexeme::<_, _, ContextError, _>('a')
                .parse("a")
                .map_err(anyhow::Error::msg)?,
            'a'
        );

        assert_eq!(
            super::lexeme::<_, _, ContextError, _>('a')
                .parse("a \t\n")
                .map_err(anyhow::Error::msg)?,
            'a'
        );

        assert!(super::lexeme::<_, _, ContextError, _>('a')
            .parse(" ")
            .is_err());

        assert!(super::lexeme::<_, _, ContextError, _>('a')
            .parse("b")
            .is_err());

        assert!(super::lexeme::<_, _, ContextError, _>('a')
            .parse(" a")
            .is_err());

        Ok(())
    }
}
