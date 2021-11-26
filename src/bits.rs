#[rustfmt::skip]
static DEBRUIJ_T: &'static [u8] = &[
    0, 47,  1, 56, 48, 27,  2, 60,
    57, 49, 41, 37, 28, 16,  3, 61,
    54, 58, 35, 52, 50, 42, 21, 44,
    38, 32, 29, 23, 17, 11,  4, 62,
    46, 55, 26, 59, 40, 36, 15, 53,
    34, 51, 20, 43, 31, 22, 10, 45,
    25, 39, 14, 33, 19, 30,  9, 24,
    13, 18,  8, 12,  7,  6,  5, 63
];

const DEBRUIJ_M: u64 = 0x03f7_9d71_b4cb_0a89;

pub fn bit_scan_forward(bits: u64) -> u8 {
    unsafe {
        *DEBRUIJ_T.get_unchecked(
            (((bits ^ bits.wrapping_sub(1)).wrapping_mul(DEBRUIJ_M)).wrapping_shr(58)) as usize,
        )
    }
}

pub fn reverse_bytes(b: u64) -> u64 {
    let mut m: u64 = 0;
    m |= (reverse_byte(((b >> 56) & 0xFF) as u8) as u64) << 56;
    m |= (reverse_byte(((b >> 48) & 0xFF) as u8) as u64) << 48;
    m |= (reverse_byte(((b >> 40) & 0xFF) as u8) as u64) << 40;
    m |= (reverse_byte(((b >> 32) & 0xFF) as u8) as u64) << 32;
    m |= (reverse_byte(((b >> 24) & 0xFF) as u8) as u64) << 24;
    m |= (reverse_byte(((b >> 16) & 0xFF) as u8) as u64) << 16;
    m |= (reverse_byte(((b >> 8) & 0xFF) as u8) as u64) << 8;
    m |= reverse_byte((b & 0xFF) as u8) as u64;
    m
}

pub fn reverse_byte(b: u8) -> u8 {
    let m: u8 = ((0b0000_0001 & b) << 7)
        | ((0b0000_0010 & b) << 5)
        | ((0b0000_0100 & b) << 3)
        | ((0b0000_1000 & b) << 1)
        | ((0b0001_0000 & b) >> 1)
        | ((0b0010_0000 & b) >> 3)
        | ((0b0100_0000 & b) >> 5)
        | ((0b1000_0000 & b) >> 7);
    m
}

pub fn string_u64(input: u64) -> String {
    let mut s = String::new();
    let format_in = format_u64(input);
    for x in 0..8 {
        let slice = &format_in[x * 8..((x * 8) + 8)];
        s += slice;
        s += "\n";
    }
    s
}

/// Returns a stringified u64 with all 64 bits being represented.
fn format_u64(input: u64) -> String {
    format!("{:064b}", input)
}
