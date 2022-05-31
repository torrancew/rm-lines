use crate::Parse;

use nom::{combinator::map_res, number::complete::le_u8};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid version reported: {value}")]
pub struct VersionError {
    value: u8,
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V3,
    V5,
}

impl TryFrom<u8> for Version {
    type Error = VersionError;

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            0x33 => Ok(Version::V3),
            0x35 => Ok(Version::V5),
            value => Err(VersionError { value }),
        }
    }
}

impl<'i> Parse<'i> for Version {
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map_res(le_u8, Version::try_from)(input)
    }
}
