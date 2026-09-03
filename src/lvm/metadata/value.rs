use crate::lvm::metadata::parse;
use std::collections::HashMap;
use winnow::error::{ContextError, ParseError};
use winnow::stream::{AsChar, Compare, ParseSlice, Stream, StreamIsPartial};
use winnow::Parser;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum MetadataValue {
    String(String),
    Integer(u64),
    Array(Vec<MetadataValue>),
    Object(HashMap<String, MetadataValue>),
}

impl MetadataValue {
    pub fn parse<Input>(
        input: Input,
    ) -> Result<HashMap<String, MetadataValue>, ParseError<Input, ContextError>>
    where
        Input: StreamIsPartial + Stream + Compare<char>,
        <Input as Stream>::Slice: ParseSlice<u64> + ParseSlice<String>,
        <Input as Stream>::Token: AsChar + Clone,
    {
        parse::metadata::<Input, ContextError>.parse(input)
    }
}
