use crate::lvm::metadata::node::MetadataNode;
use crate::lvm::metadata::parse::array::array;
use crate::lvm::metadata::parse::identifier::identifier;
use crate::lvm::metadata::parse::lexeme::lexeme;
use crate::lvm::metadata::parse::object;
use crate::lvm::metadata::parse::value::value;
use crate::lvm::metadata::MetadataValue;
use std::collections::HashMap;
use winnow::combinator::{alt, preceded};
use winnow::error::ParserError;
use winnow::stream::{AsChar, Compare, ParseSlice, Stream, StreamIsPartial};
use winnow::Parser;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum ObjectEntry {
    Object(HashMap<String, MetadataNode>),
    Array(Vec<MetadataValue>),
}

impl From<&str> for ObjectEntry {
    fn from(value: &str) -> Self {
        Self::Array(vec![value.into()])
    }
}

impl From<u64> for ObjectEntry {
    fn from(value: u64) -> Self {
        Self::Array(vec![value.into()])
    }
}

impl From<Vec<MetadataValue>> for ObjectEntry {
    fn from(value: Vec<MetadataValue>) -> Self {
        Self::Array(value)
    }
}

impl From<HashMap<String, MetadataNode>> for ObjectEntry {
    fn from(value: HashMap<String, MetadataNode>) -> Self {
        Self::Object(value)
    }
}

impl<const N: usize> From<[(String, MetadataNode); N]> for ObjectEntry {
    fn from(value: [(String, MetadataNode); N]) -> Self {
        Self::Object(value.into())
    }
}

pub fn object_entry<Input, Error>(input: &mut Input) -> Result<(String, ObjectEntry), Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    (
        identifier,
        alt((
            object::object.map(ObjectEntry::Object),
            preceded(lexeme('='), array).map(ObjectEntry::Array),
            preceded(lexeme('='), value)
                .map(|value| vec![value])
                .map(ObjectEntry::Array),
        )),
    )
        .parse_next(input)
}

#[cfg(test)]
mod test {
    use winnow::error::ContextError;
    use winnow::Parser;

    #[test]
    fn object_entry() -> anyhow::Result<()> {
        assert_eq!(
            super::object_entry::<_, ContextError>
                .parse("seqno = 12\n")
                .map_err(anyhow::Error::msg)?,
            ("seqno".to_owned(), 12.into())
        );

        assert_eq!(
            super::object_entry::<_, ContextError>
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
}
