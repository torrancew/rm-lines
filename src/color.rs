use crate::Parse;

use nom::{combinator::map_res, number::complete::le_u32};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
#[error("Invalid color specified: {value}")]
pub struct ColorError {
    value: u32,
}

#[derive(Debug, PartialEq)]
pub enum Color {
    Black,
    Grey,
    White,
    Blue,
    Red,
}

impl TryFrom<u32> for Color {
    type Error = ColorError;

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
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map_res(le_u32, Color::try_from)(input)
    }
}
