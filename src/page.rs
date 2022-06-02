use crate::{Header, Layer, Parse, Version};

use nom::{combinator::map, multi::length_count, number::complete::le_u32, sequence::tuple};

/// Data representation of a page in a reMarkable document
///
/// This is the type used to represent a parsed `.rm` file
#[derive(Debug, PartialEq)]
pub struct Page {
    pub version: Version,
    pub layers: Vec<Layer>,
}

impl<'i> Parse<'i> for Page {
    /// Attempts to parse a `Page` from a byte sequence
    ///
    /// A page is represented by a 44-byte, ASCII formatted header,
    /// followed by a little-endian, 32-bit layer count, followed by
    /// the raw [Layer] values themselves
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map(
            tuple((Header::parse, length_count(le_u32, Layer::parse))),
            |(header, layers)| Page {
                layers,
                version: header.0,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let file = include_bytes!("../data/test.rm");

        let res = Page::parse(file);
        assert!(res.is_ok());

        let (rest, page) = res.unwrap();
        assert_eq!(rest, &[][..]);
        assert_eq!(page.layers.len(), 2);
    }
}
