use crate::lvm::metadata::node::MetadataNode;
use crate::lvm::metadata::parse::object::entry;
use crate::lvm::metadata::parse::object::entry::ObjectEntry;
use std::collections::HashMap;
use winnow::combinator::repeat;
use winnow::error::ParserError;
use winnow::stream::{Accumulate, AsChar, Compare, ParseSlice, Stream, StreamIsPartial};
use winnow::Parser;

#[derive(Eq, PartialEq, Clone, Debug)]
struct ObjectBody {
    nodes: HashMap<String, MetadataNode>,
}

impl Accumulate<(String, ObjectEntry)> for ObjectBody {
    fn initial(capacity: Option<usize>) -> Self {
        Self {
            nodes: match capacity {
                Some(capacity) => HashMap::with_capacity(capacity),
                None => HashMap::new(),
            },
        }
    }

    fn accumulate(&mut self, (key, value): (String, ObjectEntry)) {
        let entry = self.nodes.entry(key).or_insert(MetadataNode::empty());
        match value {
            ObjectEntry::Object(object) => entry.object = Some(object),
            ObjectEntry::Array(array) => entry.array = Some(array),
        }
    }
}

pub fn object_body<Input, Error>(input: &mut Input) -> Result<HashMap<String, MetadataNode>, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    repeat(0.., entry::object_entry)
        .parse_next(input)
        .map(|body: ObjectBody| body.nodes)
}

#[cfg(test)]
mod test {
    use crate::lvm::metadata::node::MetadataNode;
    use winnow::error::ContextError;
    use winnow::Parser;

    #[test]
    fn object_and_value_with_same_name() -> anyhow::Result<()> {
        assert_eq!(
            super::object_body::<_, ContextError>
                .parse("contents=\"a\"\ncontents {\ndevice=\"/dev/sda2\"\n}\n")
                .map_err(anyhow::Error::msg)?,
            [(
                "contents".to_owned(),
                MetadataNode {
                    object: Some([("device".to_owned(), "/dev/sda2".into())].into()),
                    array: Some(vec!["a".into()]),
                }
            )]
            .into()
        );

        Ok(())
    }
}
