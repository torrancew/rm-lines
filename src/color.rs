use crate::Parse;

use nom::{combinator::map_res, number::complete::le_u32};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
#[error("Invalid color specified: {value}")]
pub struct ColorError {
    value: u32,
}

/// Data representation of an exported color in a reMarkable document line
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    Black,
    Grey,
    White,
    Blue,
    Red,
}

impl TryFrom<u32> for Color {
    /// Used to represent a [u32] which does not map to a known `Color`
    type Error = ColorError;

    /// Attempts to map a [u32] value to a known and supported `Color`
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Color::Black),
            0x01 => Ok(Color::Grey),
            0x02 => Ok(Color::White),
            0x06 => Ok(Color::Red),
            0x07 => Ok(Color::Blue),
            _ => Err(ColorError { value }),
        }
    }
}

impl<'i> Parse<'i> for Color {
    /// Attempts to parse a `Color` from a byte sequence
    ///
    /// A color is represented by a value-constrained,
    /// little-endian, 32-bit integer.
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map_res(le_u32, Color::try_from)(input)
    }
}
