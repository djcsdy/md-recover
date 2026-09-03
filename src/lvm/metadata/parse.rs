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

#[cfg(test)]
mod test {
    use crate::lvm::metadata::{parse, MetadataValue};
    use std::collections::HashMap;
    use winnow::error::ContextError;
    use winnow::Parser;

    #[test]
    fn metadata() -> anyhow::Result<()> {
        assert_eq!(
            parse::metadata::<_, ContextError>
                .parse(concat!(
                    "vg_name {\n",
                    "    seqno = 12\n",
                    "    extent_size = 8192\n",
                    "    \n",
                    "    physical_volumes {\n",
                    "        pv0 {\n",
                    "            device = \"/dev/sda2\"",
                    "            status = [\"ALLOCATABLE\"]\n",
                    "            pe_start = 2048\n",
                    "            pe_count = 123456\n",
                    "        }\n",
                    "    }\n",
                    "}\n",
                ))
                .map_err(anyhow::Error::msg)?,
            HashMap::from([(
                "vg_name".to_owned(),
                MetadataValue::Object(HashMap::from([
                    ("seqno".to_owned(), MetadataValue::Integer(12)),
                    ("extent_size".to_owned(), MetadataValue::Integer(8192)),
                    (
                        "physical_volumes".to_owned(),
                        MetadataValue::Object(HashMap::from([(
                            "pv0".to_owned(),
                            MetadataValue::Object(HashMap::from([
                                (
                                    "device".to_owned(),
                                    MetadataValue::String("/dev/sda2".to_owned())
                                ),
                                (
                                    "status".to_owned(),
                                    MetadataValue::Array(vec![MetadataValue::String(
                                        "ALLOCATABLE".to_owned()
                                    )])
                                ),
                                ("pe_start".to_owned(), MetadataValue::Integer(2048)),
                                ("pe_count".to_owned(), MetadataValue::Integer(123456))
                            ]))
                        )]))
                    )
                ]))
            )])
        );

        Ok(())
    }

    #[test]
    fn integer() -> anyhow::Result<()> {
        assert_eq!(
            parse::integer::<_, ContextError>
                .parse("12 ")
                .map_err(anyhow::Error::msg)?,
            12
        );

        assert!(parse::integer::<_, ContextError>.parse("_12").is_err());

        assert!(parse::integer::<_, ContextError>.parse("12a").is_err());

        Ok(())
    }

    #[test]
    fn quoted_string() -> anyhow::Result<()> {
        assert_eq!(
            parse::quoted_string::<_, ContextError, String>
                .parse("\"\"\n")
                .map_err(anyhow::Error::msg)?,
            ""
        );

        assert_eq!(
            parse::quoted_string::<_, ContextError, String>
                .parse("\"hello\"  ")
                .map_err(anyhow::Error::msg)?,
            "hello"
        );

        assert!(parse::quoted_string::<_, ContextError, String>
            .parse("a\"hello\"")
            .is_err());

        assert!(parse::quoted_string::<_, ContextError, String>
            .parse("\"hello\"a")
            .is_err());

        Ok(())
    }

    #[test]
    fn array() -> anyhow::Result<()> {
        assert_eq!(
            parse::array::<_, ContextError>
                .parse("[1, 2,\n3]\n")
                .map_err(anyhow::Error::msg)?,
            vec![
                MetadataValue::Integer(1),
                MetadataValue::Integer(2),
                MetadataValue::Integer(3)
            ]
        );

        assert!(parse::array::<_, ContextError>.parse("[1,2,3,]").is_err());

        Ok(())
    }

    #[test]
    fn identifier() -> anyhow::Result<()> {
        assert_eq!(
            parse::identifier::<_, ContextError, String>
                .parse("vg_name ")
                .map_err(anyhow::Error::msg)?,
            "vg_name"
        );

        assert_eq!(
            parse::identifier::<_, ContextError, String>
                .parse("1234-abc")
                .map_err(anyhow::Error::msg)?,
            "1234-abc"
        );

        assert!(parse::identifier::<_, ContextError, String>
            .parse("-abc")
            .is_err());

        Ok(())
    }

    #[test]
    fn object_item() -> anyhow::Result<()> {
        assert_eq!(
            parse::object_item::<_, ContextError>
                .parse("seqno = 12\n")
                .map_err(anyhow::Error::msg)?,
            ("seqno".to_owned(), MetadataValue::Integer(12))
        );

        assert_eq!(
            parse::object_item::<_, ContextError>
                .parse("pv0 { \nid = \"a\" device = \"/dev/sda2\" }\n")
                .map_err(anyhow::Error::msg)?,
            (
                "pv0".to_owned(),
                MetadataValue::Object(HashMap::from([
                    ("id".to_owned(), MetadataValue::String("a".to_owned())),
                    (
                        "device".to_owned(),
                        MetadataValue::String("/dev/sda2".to_owned())
                    )
                ]))
            )
        );

        Ok(())
    }

    #[test]
    fn object() -> anyhow::Result<()> {
        assert_eq!(
            parse::object::<_, ContextError>
                .parse("{\nid=\"a\"\ndevice=\"/dev/sda2\"\n}\n")
                .map_err(anyhow::Error::msg)?,
            HashMap::from([
                ("id".to_owned(), MetadataValue::String("a".to_owned())),
                (
                    "device".to_owned(),
                    MetadataValue::String("/dev/sda2".to_owned())
                )
            ])
        );

        Ok(())
    }

    #[test]
    fn lexeme() -> anyhow::Result<()> {
        assert_eq!(
            parse::lexeme::<_, _, ContextError, _>('a')
                .parse("a")
                .map_err(anyhow::Error::msg)?,
            'a'
        );

        assert_eq!(
            parse::lexeme::<_, _, ContextError, _>('a')
                .parse("a \t\n")
                .map_err(anyhow::Error::msg)?,
            'a'
        );

        assert!(parse::lexeme::<_, _, ContextError, _>('a')
            .parse(" ")
            .is_err());

        assert!(parse::lexeme::<_, _, ContextError, _>('a')
            .parse("b")
            .is_err());

        assert!(parse::lexeme::<_, _, ContextError, _>('a')
            .parse(" a")
            .is_err());

        Ok(())
    }
}
