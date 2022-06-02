use crate::{Line, Parse};

use nom::{combinator::map, multi::length_count, number::complete::le_u32};

/// Data representation of a layer in a reMarkable document page
#[derive(Debug, PartialEq)]
pub struct Layer {
    pub lines: Vec<Line>,
}

impl<'i> Parse<'i> for Layer {
    /// Attempts to parse a `Layer` from a byte sequence
    ///
    /// A layer is represented by a little-endian, 32-bit line count,
    /// followed by the raw [Line] values themselves
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
