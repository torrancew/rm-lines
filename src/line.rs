use crate::{Color, Parse, Point, Tool};

use nom::{
    combinator::map,
    multi::length_count,
    number::complete::{le_f32, le_u32},
    sequence::tuple,
};

/// Data representation of a line in a reMarkable document layer
///
/// A `Line` combines a [Tool], a [Color], a size and a series of [Point]s.
/// It is used to represent a continuous, joined section of [Point]s, such
/// as a continuous pen stroke.
#[derive(Debug, PartialEq)]
pub struct Line {
    pub tool: Tool,
    pub color: Color,
    pub size: f32,
    pub points: Vec<Point>,
}

impl<'i> Parse<'i> for Line {
    /// Attempts to parse a `Line` from a byte sequence
    ///
    /// A Line is represented by a series of little-endian 32-bit values.
    /// 2 of these values serve an unknown purpose at this time, and most
    /// likely map to functionality like selections, with no obvious or
    /// useful semantic meaning in the context of a document parser.
    ///
    /// Points within a line are represented as a 32-bit point count,
    /// followed by the raw [Point] values themselves.
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map(
            tuple((
                Tool::parse,
                Color::parse,
                le_u32, // Unknown/padding
                le_f32, // Line size (float)
                le_u32, // Unknown/padding
                length_count(le_u32, Point::parse),
            )),
            |(tool, color, _, size, _, points)| Line {
                tool,
                color,
                size,
                points,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let bytes = include_bytes!("../data/test-line.rm");

        let res = Line::parse(bytes);
        assert!(res.is_ok());

        let (rest, line) = res.unwrap();
        assert_eq!(rest, &[][..]);
        assert_eq!(line.tool, Tool::FineLiner);
        assert_eq!(line.color, Color::Black);
        assert_eq!(line.size, 2.);
        assert_eq!(line.points.len(), 309);
    }
}
