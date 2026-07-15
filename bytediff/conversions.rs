//^
//^ HEAD
//^

//> HEAD -> ALLOC
use alloc::vec::Vec;

//> HEAD -> SUPER
use super::Diff;


//^
//^ INTO
//^

//> INTO -> VEC U8
impl<'valid> Into<Vec<u8>> for Diff<'valid> {
    fn into(self) -> Vec<u8> {
        let mut sequence = Vec::new();
        sequence.extend(b"\x1b[P".repeat(self.remove));
        sequence.extend(self.write);
        return sequence;
    }
}