use nom::bytes::take;
use nom::{IResult, Input, Parser, ToUsize};

pub fn take_parse<C, I, O, P>(
    count: C,
    mut inner: P,
) -> impl Parser<I, Output = O, Error = nom::error::Error<I>>
where
    C: ToUsize + Clone,
    I: Input,
    P: Parser<I, Output = O, Error = nom::error::Error<I>>,
{
    move |input| -> IResult<I, O, nom::error::Error<I>> {
        let (rest, inner_input) = take(count.clone()).parse(input)?;
        let (_, output) = inner.parse(inner_input)?;
        Ok((rest, output))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::number::le_u16;

    #[test]
    fn test_take_parse() {
        let input = [0x12u8, 0x34, 0x56, 0x78, 0x9a, 0xbc];
        let mut parser = take_parse(4usize, le_u16());
        let (rest, value) = parser.parse(input.as_slice()).unwrap();
        assert_eq!(rest, &[0x9a, 0xbc]);
        assert_eq!(value, 0x3412);
    }
}
