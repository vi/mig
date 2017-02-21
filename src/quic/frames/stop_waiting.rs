use std::io;

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use quic::errors::Result;
use quic::utils::map_unexpected_eof;


pub const FRAME_STOP_WAITING: u8 = 0x06;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct StopWaitingFrame {
    pub least_acked_delta: u64,
}

impl StopWaitingFrame {
    pub fn encode(&self, write: &mut io::Write, packet_number_size: usize) -> Result<()> {
        write.write_u8(FRAME_STOP_WAITING)?;

        match packet_number_size {
            1 | 2 | 4 | 6 => {
                write.write_uint::<BigEndian>(self.least_acked_delta, packet_number_size)?;
            },
            _ => panic!("Invalid packet number size: {}", packet_number_size),
        }

        Ok(())
    }

    pub fn decode(read: &mut io::Read, packet_number_size: usize) -> Result<StopWaitingFrame> {
        if read.read_u8().map_err(map_unexpected_eof)? != FRAME_STOP_WAITING {
            panic!("Incorrect frame's decode called!")
        }

        let least_acked_delta = 
            read.read_uint::<BigEndian>(packet_number_size)
            .map_err(map_unexpected_eof)?;

        Ok(StopWaitingFrame { least_acked_delta: least_acked_delta })
    }
}
