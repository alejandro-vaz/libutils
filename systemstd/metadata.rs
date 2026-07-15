//^
//^ HEAD
//^

//> HEAD -> STD
use std::fs::Metadata as MetadataInfo;


//^
//^ METADATA
//^

//> METADATA -> STRUCT
pub struct Metadata {
    metadata: MetadataInfo
}

//> METADATA -> DATA
impl Metadata {
    pub fn size(&self) -> usize {return self.metadata.len() as usize}
}