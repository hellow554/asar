#[no_mangle]
pub const extern "C" fn crc32(s: *const u8, len: usize) -> u32 {
    // SAFETY: the pointer is passed by FFI, we need to trust it
    let slice = unsafe { std::slice::from_raw_parts(s, len) };
    crate::crc32::crc32(slice)
}
