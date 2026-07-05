//^
//^ HEAD
//^

//> HEAD -> CONSOLE
use libutils_console::Metadata as MetadataTrait;

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
impl MetadataTrait for Metadata {
    fn size(&self) -> usize {return self.metadata.len() as usize}
}