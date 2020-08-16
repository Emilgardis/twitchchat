use crate::Encodable;
use std::io::{Result, Write};

use super::ByteWriter;

/// Leave a channel. This handles prepending a leading '#' for you if you omit it.
#[non_exhaustive]
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(::serde::Deserialize))]
pub struct Part<'a> {
    pub(crate) channel: &'a str,
}

/// Leave a channel. This handles prepending a leading '#' for you if you omit it.
pub const fn part(channel: &str) -> Part<'_> {
    Part { channel }
}

impl<'a> Encodable for Part<'a> {
    fn encode<W: Write + ?Sized>(&self, buf: &mut W) -> Result<()> {
        ByteWriter::new(buf).parts(&[&"PART", &self.channel])
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn part_encode() {
        test_encode(part("#museun"), "PART #museun\r\n");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn part_serde() {
        test_serde(part("#museun"), "PART #museun\r\n");
    }
}