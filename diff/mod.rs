//^ 
//^ DIFF
//^ 

//> DIFF -> STRUCT
pub struct Diff<'valid> {
    remove: usize,
    write: &'valid [u8]
}

//> DIFF -> ERASE
const ERASE: [u8; 7] = [b'\x1B', b'[', b'D', b' ', b'\x1B', b'[', b'D'];

//> DIFF -> IMPLEMENTATION
impl<'valid> Diff<'valid> {
    #[inline]
    pub fn new(current: &[u8], replacement: &'valid [u8]) -> Self {
        let mut old = current.into_iter();
        let mut new = replacement.into_iter();
        for index in 0.. {return if old.next() != new.next() {Diff {
            remove: current.len() - index,
            write: &replacement[index..]
        }} else {continue}}
        unreachable!();
    }
}

//> DIFF -> INTO VEC U8
impl<'valid> Into<Vec<u8>> for Diff<'valid> {
    #[inline]
    fn into(self) -> Vec<u8> {
        let mut sequence = Vec::new();
        sequence.extend(ERASE.repeat(self.remove));
        sequence.extend(self.write);
        return sequence;
    }
}