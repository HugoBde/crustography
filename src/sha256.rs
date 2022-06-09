// 512-bit block used in SHA256
#[allow(non_camel_case_types)]
struct u512 {
    words: [u32; 8],
}

// Initial values of SHA-2
// The first 32 bits of the fractional representation of the first 8 prime numbers
// 2, 3, 5, 7, 11, 13, 17, 19
const H0: u32 = 0x6a09e667;
const H1: u32 = 0xbb67ae85;
const H2: u32 = 0x3c6ef372;
const H3: u32 = 0xa54ff53a;
const H4: u32 = 0x510e527f;
const H5: u32 = 0x9b05688c;
const H6: u32 = 0x1f83d9ab;
const H7: u32 = 0x5be0cd19;

const IV: u512 = u512 {
    words: [H0, H1, H2, H3, H4, H5, H6, H7],
};

impl u512 {
    fn new() -> u512 {
        u512 { words: [0; 8] }
    }
}

impl From<&[u8]> for u512 {
    fn from(bytes: &[u8]) -> u512 {
        let mut output = u512::new();
        let mut byte_cursor = 0;
        let max = if bytes.len() < 16 {
            bytes.len()
        } else {
            16
        };

        for _ in 0..max {
            ouput
        }
    }
}

pub fn hash(bytes: &[u8]) -> [u8; 32] {
    return [0; 32];
}
