use crate::lvm::metadata::parse::lexeme;
use winnow::ascii::escaped;
use winnow::combinator::delimited;
use winnow::error::ParserError;
use winnow::stream::{Accumulate, AsChar, Compare, Stream, StreamIsPartial};
use winnow::token::{any, none_of};
use winnow::Parser;

pub fn quoted_string<Input, Error, Output>(input: &mut Input) -> Result<Output, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Token: AsChar + Clone,
    Output: Accumulate<char>,
    Error: ParserError<Input>,
{
    lexeme::lexeme(delimited(
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

#[cfg(test)]
mod test {
    use winnow::error::ContextError;
    use winnow::Parser;

    #[test]
    fn quoted_string() -> anyhow::Result<()> {
        assert_eq!(
            super::quoted_string::<_, ContextError, String>
                .parse("\"\"\n")
                .map_err(anyhow::Error::msg)?,
            ""
        );

        assert_eq!(
            super::quoted_string::<_, ContextError, String>
                .parse("\"hello\"  ")
                .map_err(anyhow::Error::msg)?,
            "hello"
        );

        assert!(super::quoted_string::<_, ContextError, String>
            .parse("a\"hello\"")
            .is_err());

        assert!(super::quoted_string::<_, ContextError, String>
            .parse("\"hello\"a")
            .is_err());

        Ok(())
    }
}
