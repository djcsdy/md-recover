use crate::lvm::metadata::parse::identifier::*;
use crate::lvm::metadata::parse::lexeme::*;
use crate::lvm::metadata::parse::value::*;
use crate::lvm::metadata::MetadataValue;
use std::collections::HashMap;
use winnow::combinator::{alt, delimited, preceded, repeat};
use winnow::error::ParserError;
use winnow::stream::{AsChar, Compare, ParseSlice, Stream, StreamIsPartial};
use winnow::Parser;

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

pub fn object_body<Input, Error>(input: &mut Input) -> Result<HashMap<String, MetadataValue>, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    repeat(0.., object_item).parse_next(input)
}

pub fn object<Input, Error>(input: &mut Input) -> Result<HashMap<String, MetadataValue>, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    delimited(lexeme('{'), object_body, lexeme('}')).parse_next(input)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use winnow::error::ContextError;
    use winnow::Parser;

    #[test]
    fn object_item() -> anyhow::Result<()> {
        assert_eq!(
            super::object_item::<_, ContextError>
                .parse("seqno = 12\n")
                .map_err(anyhow::Error::msg)?,
            ("seqno".to_owned(), 12.into())
        );

        assert_eq!(
            super::object_item::<_, ContextError>
                .parse("pv0 { \nid = \"a\" device = \"/dev/sda2\" }\n")
                .map_err(anyhow::Error::msg)?,
            (
                "pv0".to_owned(),
                [
                    ("id".to_owned(), "a".into()),
                    ("device".to_owned(), "/dev/sda2".into())
                ]
                .into()
            )
        );

        Ok(())
    }

    #[test]
    fn object() -> anyhow::Result<()> {
        assert_eq!(
            super::object::<_, ContextError>
                .parse("{\nid=\"a\"\ndevice=\"/dev/sda2\"\n}\n")
                .map_err(anyhow::Error::msg)?,
            HashMap::from([
                ("id".to_owned(), "a".into()),
                ("device".to_owned(), "/dev/sda2".into())
            ])
        );

        Ok(())
    }
}
