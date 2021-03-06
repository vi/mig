use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use quic::errors::Result;
use quic::packets::utils::map_unexpected_eof;
use super::utils::{encode_reason_phrase, decode_reason_phrase};


pub const FRAME_GOAWAY: u8 = 0x03;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GoAwayFrame {
    pub error_code: u32,
    pub last_good_stream_id: u32,
    pub reason_phrase: Option<String>,
}

impl GoAwayFrame {
    pub fn encode<W: io::Write>(&self, write: &mut W) -> Result<()> {
        write.write_u8(FRAME_GOAWAY)?;

        write.write_u32::<BigEndian>(self.error_code)?;
        write.write_u32::<BigEndian>(self.last_good_stream_id)?;
        encode_reason_phrase(write, &self.reason_phrase)?;

        Ok(())
    }

    pub fn decode<R: io::Read>(read: &mut R) -> Result<GoAwayFrame> {
        let frame_type = read.read_u8()?;
        assert!(frame_type == FRAME_GOAWAY);

        let error_code = 
            read.read_u32::<BigEndian>()
            .map_err(map_unexpected_eof)?;
        let last_good_stream_id = 
            read.read_u32::<BigEndian>()
            .map_err(map_unexpected_eof)?;
        let reason_phrase = decode_reason_phrase(read)?;

        Ok(GoAwayFrame {
            error_code: error_code,
            last_good_stream_id: last_good_stream_id,
            reason_phrase: reason_phrase,
        })
    }
}
