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
        for index in 0.. {return if old.next() != chars.next() {Diff {
            remove: current.len() - index,
            write: &render[index..]
        }} else {continue}}
        unreachable!();
    }
}

//> DIFF -> INTO
impl<'valid> Into<Vec<u8>> for Diff<'valid> {
    #[inline]
    fn into(self) -> Vec<u8> {
        const RETURN: [u8; 3] = [b'\x1B', b'[', b'D'];
        let mut sequence = Vec::new();
        sequence.extend(RETURN.repeat(self.remove));
        sequence.extend([b' '].repeat(self.remove));
        sequence.extend(RETURN.repeat(self.remove));
        sequence.extend(self.write.as_bytes());
        return sequence;
    }
}