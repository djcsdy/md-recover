use crate::lvm::metadata::parse::object::*;
use crate::lvm::metadata::Metadata;
use winnow::ascii::multispace0;
use winnow::combinator::preceded;
use winnow::error::ParserError;
use winnow::stream::{AsChar, Compare, ParseSlice, Stream, StreamIsPartial};
use winnow::Parser;

mod array;
mod identifier;
mod integer;
mod lexeme;
mod object;
mod string;
mod value;

pub fn metadata<Input, Error>(input: &mut Input) -> Result<Metadata, Error>
where
    Input: StreamIsPartial + Stream + Compare<char>,
    <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
    <Input as Stream>::Token: AsChar + Clone,
    Error: ParserError<Input>,
{
    preceded(multispace0, object_body)
        .parse_next(input)
        .map(|root| Metadata { root })
}

#[cfg(test)]
mod test {
    use crate::lvm::metadata::{Metadata, MetadataValue};
    use std::collections::HashMap;
    use winnow::error::ContextError;
    use winnow::Parser;

    #[test]
    fn metadata() -> anyhow::Result<()> {
        assert_eq!(
            super::metadata::<_, ContextError>
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
            Metadata {
                root: HashMap::from([(
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
            }
        );

        Ok(())
    }
}
