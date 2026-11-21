use crate::lvm::metadata::node::MetadataNode;
use crate::lvm::metadata::parse::lexeme::*;
pub use crate::lvm::metadata::parse::object::body::object_body;
use std::collections::HashMap;
use winnow::combinator::delimited;
use winnow::error::ParserError;
use winnow::stream::{AsChar, Compare, ParseSlice, Stream, StreamIsPartial};
use winnow::Parser;

mod body;
mod entry;

pub fn object<Input, Error>(input: &mut Input) -> Result<HashMap<String, MetadataNode>, Error>
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
