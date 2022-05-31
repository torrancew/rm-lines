use crate::{Line, Parse};

use nom::{combinator::map, multi::length_count, number::complete::le_u32};

#[derive(Debug, PartialEq)]
pub struct Layer {
    pub lines: Vec<Line>,
}

impl<'i> Parse<'i> for Layer {
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map(length_count(le_u32, Line::parse), |lines| Layer { lines })(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let bytes = include_bytes!("../data/test-layer.rm");

        let res = Layer::parse(bytes);
        assert!(res.is_ok());

        let (rest, layer) = res.unwrap();
        assert_eq!(rest, &[][..]);
        assert_eq!(layer.lines.len(), 2);
    }
}
