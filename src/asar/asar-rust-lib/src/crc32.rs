use crc::{Crc, CRC_32_ISO_HDLC};

pub const fn crc32(arr: &[u8]) -> u32 {
    const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    CRC.checksum(arr)
}