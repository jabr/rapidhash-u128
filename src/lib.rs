const SEED: u64 = 0xbdd8_9aa9_8270_4029;
const MAGIC: [u64; 3] = [
    0x2d35_8dcc_aa6c_78a5,
    0x8bb8_4b93_962e_acc9,
    0x4b33_a62e_d433_d4a3,
];
const MAGICSEED: u64 = SEED ^ mix(SEED ^ MAGIC[0], MAGIC[1]) ^ 16;

#[inline(always)]
const fn mum(a: u64, b: u64) -> (u64, u64) {
    let r = a as u128 * b as u128;
    let a1 = (r & 0x00000000_00000000_ffffffff_ffffffff) as u64;
    let b1 = (r >> 64) as u64;
    return (a1, b1);
}

#[inline(always)]
const fn mix(a: u64, b: u64) -> u64 {
    let (a1, b1) = mum(a, b);
    return a1 ^ b1;
}

pub fn hash(key: u128) -> u64 {
    // println!("{:?}", key.to_le_bytes());
    let w3 = (key >> 96) as u32;
    let w2 = ((key & 0x00000000_ffffffff_00000000_00000000) >> 64) as u32;
    let w1 = ((key & 0x00000000_00000000_ffffffff_00000000) >> 32) as u32;
    let w0 = (key & 0x00000000_00000000_00000000_ffffffff) as u32;
    let mut a = (w0 as u64) << 32 | w3 as u64;
    let mut b = (w1 as u64) << 32 | w2 as u64;
    (a, b) = mum(a ^ MAGIC[1], b ^ MAGICSEED);
    return mix(a ^ MAGIC[0] ^ 16, b ^ MAGIC[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_hashes_as_expected() {
        assert_eq!(                                    hash(0),  8755926293314635566);
        assert_eq!(                                    hash(1), 17996969877019643443);
        assert_eq!(                                 hash(0xff),  5200326291411116507);
        assert_eq!(                               hash(0x1000),  3752997491443908878);
        assert_eq!(                          hash(0x1000_0000),  1347028408682550078);
        assert_eq!(                  hash(0x10000000_00000000),  3593052489046108800);
        assert_eq!(                  hash(0x10000000_00000001),  7365235785575411947);
        assert_eq!(         hash(0x10000000_00000000_00000000),  5399386355486589714);
        assert_eq!(hash(0x10000000_00000000_00000000_00000000), 13365378750111633005);
        assert_eq!(hash(0xffffffff_ffffffff_ffffffff_ffffffff), 10466158564987642889);
    }
}
