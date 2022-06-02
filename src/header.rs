use crate::{Parse, Version};

use nom::{bytes::complete::tag, combinator::map, sequence::delimited};

/// Data representation of the header found in a `.rm` file
pub(crate) struct Header(pub Version);

impl<'i> Parse<'i> for Header {
    /// Attempts to parse a `Header` from a byte sequence
    ///
    /// A header is represented by a 44-byte ASCII string
    /// of the form:
    ///     `reMarkable .lines file, version=X          `
    /// where `X` is a [Version]
    fn parse(input: &'i [u8]) -> nom::IResult<&'i [u8], Self> {
        map(
            delimited(
                tag("reMarkable .lines file, version="),
                Version::parse,
                tag("          "),
            ),
            Self,
        )(input)
    }
}
