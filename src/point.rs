use crate::Parse;

use nom::{combinator::map, number::complete::le_f32, sequence::tuple};

/// Data representation of a point in a reMarkable document line
#[derive(Debug, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub speed: f32,
    pub direction: f32,
    pub width: f32,
    pub pressure: f32,
}

impl<'i> Parse<'i> for Point {
    /// Attempts to parse a `Point` from a byte sequence
    ///
    /// A point is represented by six little-endian, 32-bit floating point values.
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map(
            tuple((le_f32, le_f32, le_f32, le_f32, le_f32, le_f32)),
            |(x, y, speed, direction, width, pressure)| Point {
                x,
                y,
                speed,
                direction,
                width,
                pressure,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let bytes = include_bytes!("../data/test-point.rm");

        assert_eq!(
            Point::parse(bytes),
            Ok((
                &[][..],
                Point {
                    x: 421.77228,
                    y: 307.6698,
                    speed: 1.2305648,
                    direction: 1.6739763e-5,
                    width: 4.0,
                    pressure: 0.039853483
                }
            ))
        );
    }
}
