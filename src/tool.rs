use crate::Parse;

use nom::{combinator::map_res, number::complete::le_u32};
use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
#[error("Invalid tool: {value}")]
pub struct ToolError {
    value: u32,
}

#[derive(Debug, PartialEq)]
pub enum Tool {
    Brush,
    Pencil,
    BallPoint,
    Marker,
    FineLiner,
    Highlighter,
    Eraser,
    MechanicalPencil,
    EraseArea,
    EraseAll,
    SelectionBrush,
    Calligraphy,
}

impl TryFrom<u32> for Tool {
    type Error = ToolError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0x00 | 0x0c => Ok(Tool::Brush),
            0x01 | 0x0e => Ok(Tool::Pencil),
            0x02 | 0x0f => Ok(Tool::BallPoint),
            0x03 | 0x10 => Ok(Tool::Marker),
            0x04 | 0x11 => Ok(Tool::FineLiner),
            0x05 | 0x12 => Ok(Tool::Highlighter),
            0x06 => Ok(Tool::Eraser),
            0x07 | 0x0d => Ok(Tool::MechanicalPencil),
            0x08 => Ok(Tool::EraseArea),
            0x09 => Ok(Tool::EraseAll),
            0x0a | 0x0b => Ok(Tool::SelectionBrush),
            0x15 => Ok(Tool::Calligraphy),
            _ => Err(ToolError { value }),
        }
    }
}

impl<'i> Parse<'i> for Tool {
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map_res(le_u32, Tool::try_from)(input)
    }
}
