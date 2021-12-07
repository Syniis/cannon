#[rustfmt::skip]
const DEBRUIJ_T: [u8; 64] = [
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

#[inline(always)]
pub const fn bit_scan_forward(bits: u64) -> u8 {
    DEBRUIJ_T[(((bits ^ bits.wrapping_sub(1)).wrapping_mul(DEBRUIJ_M)).wrapping_shr(58)) as usize]
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

fn format_u64(input: u64) -> String {
    format!("{:064b}", input)
}
