//^ 
//^ DIFF
//^ 

//> DIFF -> STRUCT
pub struct Diff<'valid> {
    remove: usize,
    write: &'valid str
}

//> DIFF -> NEW
impl<'valid> Diff<'valid> {
    #[inline]
    pub fn new(current: &str, render: &'valid str) -> Self {
        let mut old = current.chars();
        let mut chars = render.chars();
        let mut index = 0;
        return loop {if old.next() == chars.next() {
            index += 1;
            continue
        } else {break Diff {
            remove: current.len() - index,
            write: &render[index..]
        }}}
    }
}

//> DIFF -> INTO
impl<'valid> Into<Vec<u8>> for Diff<'valid> {
    #[inline]
    fn into(self) -> Vec<u8> {
        let mut sequence = Vec::new();
        sequence.extend([b'\x1B', b'[', b'D'].repeat(self.remove));
        sequence.extend([b' '].repeat(self.remove));
        sequence.extend([b'\x1B', b'[', b'D'].repeat(self.remove));
        sequence.extend(self.write.as_bytes());
        return sequence;
    }
}