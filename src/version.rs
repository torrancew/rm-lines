use crate::Parse;

use nom::{combinator::map_res, number::complete::le_u8};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid version reported: {value}")]
pub struct VersionError {
    value: u8,
}

/// Data representation of the file format version used for a reMarkable document
#[derive(Debug, PartialEq)]
pub enum Version {
    V3,
    V5,
}

impl TryFrom<u8> for Version {
    /// Used to represent a [u8] that does not map to a `Version`
    type Error = VersionError;

    /// Attempts to map a [u8] value to a known and supported `Version`
    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            0x33 => Ok(Version::V3),
            0x35 => Ok(Version::V5),
            value => Err(VersionError { value }),
        }
    }
}

impl<'i> Parse<'i> for Version {
    /// Attempts to parse a `Version` from a byte sequence
    ///
    /// A version is represented by a value-constrained, little-endian, 8-bit integer,
    /// contained within a larger, 44-byte ASCII-encoded header.
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map_res(le_u8, Version::try_from)(input)
    }
}
