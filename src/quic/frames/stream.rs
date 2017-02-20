use std::io;

use byteorder::{BigEndian, WriteBytesExt};
use cast;


pub const FRAME_FLAG_STREAM: u8 = 0b10000000;

pub struct StreamFrame {
    stream_id: u32,
    offset: u64,
    stream_data: Vec<u8>,
    fin: bool,
}

impl StreamFrame {
    pub fn encode(&self, write: &mut io::Write) -> io::Result<()> {
        // construct the type octet
        let mut frame_type = FRAME_FLAG_STREAM;

        if self.fin {
            frame_type |= 0b01000000
        }

        // TODO: exclude data length sometimes?
        let has_data_length = true;
        if has_data_length {
            frame_type |= 0b00100000;
        }

        // TODO: calculate this more intelligently
        let offset_length = 8;
        frame_type |= 0b00011100;

        // TODO: calculate this more intelligently
        let stream_id_length = 4;
        frame_type |= 0b00000011;

        write.write_u8(frame_type)?;

        // other fields
        if has_data_length {
            write.write_u16::<BigEndian>(
                cast::u16(self.stream_data.len())
                .expect("Stream data too big, size has to fit in 16 bits")
            )?;
        }
        write.write_uint::<BigEndian>(self.stream_id as u64, stream_id_length)?;
        write.write_uint::<BigEndian>(self.offset, offset_length)?;
        write.write_all(&self.stream_data[..])?;

        Ok(())
    }
}