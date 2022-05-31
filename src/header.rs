use crate::{Parse, Version};

use nom::{bytes::complete::tag, combinator::map, sequence::delimited};

pub(crate) struct Header(pub Version);

impl<'i> Parse<'i> for Header {
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
