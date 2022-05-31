use crate::{Color, Parse, Point, Tool};

use nom::{
    combinator::map,
    multi::length_count,
    number::complete::{le_f32, le_u32},
    sequence::tuple,
};

#[derive(Debug, PartialEq)]
pub struct Line {
    pub tool: Tool,
    pub color: Color,
    pub size: f32,
    pub points: Vec<Point>,
}

impl<'i> Parse<'i> for Line {
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
