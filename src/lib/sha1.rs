use crypto::digest::Digest;
use crypto::sha1::Sha1;

// TODO: Remove in favour of Oid
pub const SHA1_LEN: usize = 20;
pub type PackedSha1 = [u8; SHA1_LEN];

pub fn hash(input: &[u8]) -> PackedSha1 {
    let mut hasher = Sha1::new();
    hasher.input(input);
    let mut output = [0; SHA1_LEN];
    hasher.result(&mut output);
    output
}

#[cfg(test)]
mod tests {
    use super::hash;

    #[test]
    fn it_works() {
        let input: [u8; 6] = [0x31, 0x32, 0x33, 0x61, 0x62, 0x63];
        let output = hash(&input);

        assert_eq!(output, [0x4b, 0xe3, 0x0d, 0x98, 0x14, 0xc6, 0xd4, 0xe9, 0x80, 0x0e, 0x0d, 0x2e, 0xa9, 0xec, 0x9f, 0xb0, 0x0e, 0xfa, 0x88, 0x7b
]);
    }
}

